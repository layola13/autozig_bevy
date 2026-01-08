const std = @import("std");

/// SubApp structure - represents a sub-application with its own world and schedule
pub const SubApp = struct {
    allocator: std.mem.Allocator,
    update_schedule: ?ScheduleLabel,
    systems: std.ArrayList(SystemFn),
    extract_fn: ?ExtractFn,

    const ScheduleLabel = enum {
        Update,
        FixedUpdate,
        PreUpdate,
        PostUpdate,
    };

    const SystemFn = *const fn () void;
    const ExtractFn = *const fn (*SubApp, *SubApp) void;

    pub fn create(allocator: std.mem.Allocator) !*SubApp {
        const sub_app = try allocator.create(SubApp);

        // 分配空的systems数组
        const empty_systems = try allocator.alloc(SystemFn, 0);

        sub_app.* = SubApp{
            .allocator = allocator,
            .update_schedule = .Update,
            .systems = .{ .items = empty_systems, .capacity = 0 },
            .extract_fn = null,
        };

        return sub_app;
    }

    pub fn destroy(self: *SubApp) void {
        self.systems.deinit(self.allocator);
        self.allocator.destroy(self);
    }

    pub fn runDefaultSchedule(self: *SubApp) void {
        if (self.update_schedule == null) return;

        // Run all systems in the update schedule
        for (self.systems.items) |system| {
            system();
        }
    }

    pub fn update(self: *SubApp) void {
        self.runDefaultSchedule();
        self.clearTrackers();
    }

    pub fn clearTrackers(self: *SubApp) void {
        // Clear change detection trackers
        // In a real implementation, this would clear ECS world trackers
        _ = self;
    }

    pub fn setUpdateSchedule(self: *SubApp, schedule: ScheduleLabel) void {
        self.update_schedule = schedule;
    }

    pub fn addSystem(self: *SubApp, system: SystemFn) !void {
        try self.systems.append(self.allocator, system);
    }

    pub fn setExtractFn(self: *SubApp, extract_fn: ExtractFn) void {
        self.extract_fn = extract_fn;
    }

    pub fn extract(self: *SubApp, main_app: *SubApp) void {
        if (self.extract_fn) |extract_fn| {
            extract_fn(self, main_app);
        }
    }
};

// FFI exports
export fn sub_app_create() ?*SubApp {
    const allocator = std.heap.page_allocator;
    return SubApp.create(allocator) catch null;
}

export fn sub_app_destroy(sub_app: *SubApp) void {
    sub_app.destroy();
}

export fn sub_app_update(sub_app: *SubApp) void {
    sub_app.update();
}

export fn sub_app_run_default_schedule(sub_app: *SubApp) void {
    sub_app.runDefaultSchedule();
}
