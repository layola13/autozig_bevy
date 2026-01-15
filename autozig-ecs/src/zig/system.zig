const std = @import("std");
const common = @import("common.zig");
const sys_closure = @import("system_closure.zig");
const dep_graph = @import("dependency_graph.zig");
const Entity = common.Entity;
const g_allocator = common.g_allocator;

// Re-use types from system_closure
const RustClosure = sys_closure.RustClosure;
const TrampolineFn = sys_closure.TrampolineFn;
const WorldAccessFlags = sys_closure.WorldAccessFlags;
const ClosureSystem = sys_closure.ClosureSystem;

// Schedule - manages and runs systems
pub const Schedule = struct {
    allocator: std.mem.Allocator,
    systems: std.ArrayList(?ClosureSystem),
    graph: dep_graph.DependencyGraph,
    execution_order: ?std.ArrayList(usize),
    name_map: std.StringHashMap(usize),

    pub fn init(allocator: std.mem.Allocator) !*Schedule {
        const schedule = try allocator.create(Schedule);
        schedule.* = Schedule{
            .allocator = allocator,
            // Use struct literal init for ArrayList (as per dependency_graph fix)
            .systems = std.ArrayList(?ClosureSystem){},
            .graph = dep_graph.DependencyGraph.init(allocator),
            .execution_order = null,
            .name_map = std.StringHashMap(usize).init(allocator),
        };
        return schedule;
    }

    pub fn deinit(self: *Schedule) void {
        if (self.execution_order) |*order| order.deinit(self.allocator);

        var iter = self.name_map.keyIterator();
        while (iter.next()) |key| {
            self.allocator.free(key.*);
        }
        self.name_map.deinit();

        self.graph.deinit();
        self.systems.deinit(self.allocator);
        self.allocator.destroy(self);
    }

    pub fn getOrCreateNode(self: *Schedule, name: []const u8) !usize {
        if (self.name_map.get(name)) |id| return id;

        // Duplicate name for storage
        const name_copy = try self.allocator.dupe(u8, name);
        errdefer self.allocator.free(name_copy);

        const id = try self.graph.addNode();
        try self.name_map.put(name_copy, id);
        // Ensure systems list is big enough
        while (self.systems.items.len <= id) {
            try self.systems.append(self.allocator, null);
        }
        return id;
    }

    pub fn addSystem(self: *Schedule, name: []const u8, closure: RustClosure, trampoline: TrampolineFn, access: WorldAccessFlags) !void {
        const id = try self.getOrCreateNode(name);

        self.systems.items[id] = ClosureSystem{
            .name = name,
            .closure = closure,
            .trampoline = trampoline,
            .world_access = access,
        };

        // Invalidate sort
        if (self.execution_order) |*order| {
            order.deinit(self.allocator);
            self.execution_order = null;
        }
    }

    pub fn addDependency(self: *Schedule, from: usize, to: usize) !void {
        try self.graph.addEdge(from, to);
        if (self.execution_order) |*order| {
            order.deinit(self.allocator);
            self.execution_order = null;
        }
    }

    pub fn addDependencyByName(self: *Schedule, from_name: []const u8, to_name: []const u8) !void {
        const from = try self.getOrCreateNode(from_name);
        const to = try self.getOrCreateNode(to_name);
        try self.addDependency(from, to);
    }

    pub fn build(self: *Schedule) !void {
        if (self.execution_order != null) return;
        self.execution_order = try self.graph.topologicalSort();
    }

    pub fn run(self: *Schedule, world_ptr: *anyopaque) void {
        // Build if needed (lazy sort)
        if (self.execution_order == null) {
            self.build() catch |err| {
                std.debug.print("Schedule build failed: {}\n", .{err});
                return;
            };
        }

        const order = self.execution_order.?;
        for (order.items) |sys_idx| {
            if (self.systems.items[sys_idx]) |*system| {
                system.trampoline(&system.closure, world_ptr);
            }
        }
    }

    pub fn checkConflicts(self: *const Schedule) bool {
        for (self.systems.items, 0..) |sysA_opt, i| {
            if (sysA_opt) |sysA| {
                for (self.systems.items[i + 1 ..]) |sysB_opt| {
                    if (sysB_opt) |sysB| {
                        if (config_conflicts(sysA.world_access, sysB.world_access)) {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }

    pub fn systemCount(self: *const Schedule) usize {
        return self.systems.items.len;
    }
};

fn config_conflicts(a: WorldAccessFlags, b: WorldAccessFlags) bool {
    // Write-Read or Write-Write conflicts
    if (a.writes_resources and (b.reads_resources or b.writes_resources)) return true;
    if (b.writes_resources and (a.reads_resources or a.writes_resources)) return true;
    if (a.writes_components and (b.reads_components or b.writes_components)) return true;
    if (b.writes_components and (a.reads_components or a.writes_components)) return true;
    return false;
}

// Exported C API
export fn schedule_create() ?*Schedule {
    return Schedule.init(g_allocator) catch null;
}

export fn schedule_destroy(schedule: *Schedule) void {
    schedule.deinit();
}

export fn schedule_add_system(schedule: *Schedule, name_ptr: [*]const u8, name_len: usize, data_ptr: *anyopaque, vtable_ptr: *anyopaque, trampoline: TrampolineFn, access_flags: u8) bool {
    const name = name_ptr[0..name_len];
    const closure = RustClosure{
        .data_ptr = data_ptr,
        .vtable_ptr = vtable_ptr,
    };
    const access: WorldAccessFlags = @bitCast(access_flags);

    schedule.addSystem(name, closure, trampoline, access) catch return false;
    return true;
}

export fn schedule_add_dependency(schedule: *Schedule, from_ptr: [*]const u8, from_len: usize, to_ptr: [*]const u8, to_len: usize) bool {
    const from_name = from_ptr[0..from_len];
    const to_name = to_ptr[0..to_len];
    schedule.addDependencyByName(from_name, to_name) catch return false;
    return true;
}

export fn schedule_run(schedule: *Schedule, world_ptr: *anyopaque) void {
    schedule.run(world_ptr);
}

export fn schedule_system_count(schedule: *const Schedule) usize {
    return schedule.systemCount();
}

export fn schedule_check_conflicts(schedule: *const Schedule) bool {
    return schedule.checkConflicts();
}

// Observer support
pub const ObserverTrampolineFn = *const fn (
    closure: *RustClosure,
    entity: Entity,
    world: *anyopaque,
) callconv(.c) void;

pub const Observer = extern struct {
    trampoline: ObserverTrampolineFn,
    closure: RustClosure,

    pub fn deinit(self: *Observer) void {
        g_allocator.destroy(self);
    }

    pub fn trigger(self: *Observer, entity: Entity, world: *anyopaque) void {
        self.trampoline(&self.closure, entity, world);
    }
};

export fn schedule_build(schedule: *Schedule) bool {
    schedule.build() catch return false;
    return true;
}

export fn observer_create(data_ptr: *anyopaque, vtable_ptr: *anyopaque, trampoline: ObserverTrampolineFn) ?*Observer {
    var obs = g_allocator.create(Observer) catch return null;
    obs.closure = .{
        .data_ptr = data_ptr,
        .vtable_ptr = vtable_ptr,
    };
    obs.trampoline = trampoline;
    return obs;
}

export fn observer_trigger(obs: *Observer, entity: Entity, world: *anyopaque) void {
    obs.trigger(entity, world);
}

export fn observer_destroy(obs: *Observer) void {
    obs.deinit();
}
