const std = @import("std");

// System function pointer type
pub const SystemFn = *const fn (world_ptr: *anyopaque) callconv(.c) void;

// System metadata
pub const SystemMeta = struct {
    name: []const u8,
    func: SystemFn,
};

// Schedule - manages and runs systems
pub const Schedule = struct {
    allocator: std.mem.Allocator,
    systems: std.ArrayList(SystemMeta),
    
    pub fn init(allocator: std.mem.Allocator) !*Schedule {
        const schedule = try allocator.create(Schedule);
        schedule.* = Schedule{
            .allocator = allocator,
            .systems = std.ArrayList(SystemMeta){},
        };
        return schedule;
    }
    
    pub fn deinit(self: *Schedule) void {
        self.systems.deinit(self.allocator);
        self.allocator.destroy(self);
    }
    
    pub fn addSystem(self: *Schedule, name: []const u8, func: SystemFn) !void {
        try self.systems.append(self.allocator, SystemMeta{
            .name = name,
            .func = func,
        });
    }
    
    pub fn run(self: *Schedule, world_ptr: *anyopaque) void {
        for (self.systems.items) |system| {
            system.func(world_ptr);
        }
    }
    
    pub fn systemCount(self: *const Schedule) usize {
        return self.systems.items.len;
    }
};

// 全局allocator在entity.zig中定义，合并后直接可用

// Exported C API
export fn schedule_create() ?*Schedule {
    return Schedule.init(g_allocator) catch null;
}

export fn schedule_destroy(schedule: *Schedule) void {
    schedule.deinit();
}

export fn schedule_add_system(schedule: *Schedule, name_ptr: [*]const u8, name_len: usize, func: SystemFn) bool {
    const name = name_ptr[0..name_len];
    schedule.addSystem(name, func) catch return false;
    return true;
}

export fn schedule_run(schedule: *Schedule, world_ptr: *anyopaque) void {
    schedule.run(world_ptr);
}

export fn schedule_system_count(schedule: *const Schedule) usize {
    return schedule.systemCount();
}
