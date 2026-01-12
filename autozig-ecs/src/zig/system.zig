const std = @import("std");
const common = @import("common.zig");
const sys_closure = @import("system_closure.zig");
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
    systems: std.ArrayList(ClosureSystem),

    pub fn init(allocator: std.mem.Allocator) !*Schedule {
        const schedule = try allocator.create(Schedule);
        schedule.* = Schedule{
            .allocator = allocator,
            .systems = std.ArrayList(ClosureSystem){},
        };
        return schedule;
    }

    pub fn deinit(self: *Schedule) void {
        self.systems.deinit(self.allocator);
        self.allocator.destroy(self);
    }

    pub fn addSystem(self: *Schedule, name: []const u8, closure: RustClosure, trampoline: TrampolineFn, access: WorldAccessFlags) !void {
        try self.systems.append(self.allocator, ClosureSystem{
            .name = name,
            .closure = closure,
            .trampoline = trampoline,
            .world_access = access,
        });
    }

    pub fn run(self: *Schedule, world_ptr: *anyopaque) void {
        for (self.systems.items) |*system| {
            // Call the trampoline with the closure data and world pointer
            system.trampoline(&system.closure, world_ptr);
        }
    }

    pub fn checkConflicts(self: *const Schedule) bool {
        for (self.systems.items, 0..) |sysA, i| {
            for (self.systems.items[i + 1 ..]) |sysB| {
                if (config_conflicts(sysA.world_access, sysB.world_access)) {
                    // In a real implementation we would return the conflicting pair
                    return true;
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

pub const Observer = struct {
    closure: RustClosure,
    trampoline: ObserverTrampolineFn,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, closure: RustClosure, trampoline: ObserverTrampolineFn) !*Observer {
        const obs = try allocator.create(Observer);
        obs.* = .{
            .allocator = allocator,
            .closure = closure,
            .trampoline = trampoline,
        };
        return obs;
    }

    pub fn deinit(self: *Observer) void {
        self.allocator.destroy(self);
    }

    pub fn trigger(self: *Observer, entity: Entity, world: *anyopaque) void {
        self.trampoline(&self.closure, entity, world);
    }
};

export fn observer_create(data_ptr: *anyopaque, vtable_ptr: *anyopaque, trampoline: ObserverTrampolineFn) ?*Observer {
    const closure = RustClosure{
        .data_ptr = data_ptr,
        .vtable_ptr = vtable_ptr,
    };
    return Observer.init(g_allocator, closure, trampoline) catch null;
}

export fn observer_trigger(obs: *Observer, entity: Entity, world: *anyopaque) void {
    obs.trigger(entity, world);
}

export fn observer_destroy(obs: *Observer) void {
    obs.deinit();
}
