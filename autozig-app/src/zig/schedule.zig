const std = @import("std");
const ZigApp = @import("app.zig").ZigApp;

/// Schedule label enumeration matching Rust MainScheduleOrder
pub const ScheduleLabel = enum(u8) {
    First = 0,
    PreStartup = 1,
    Startup = 2,
    PostStartup = 3,
    PreUpdate = 4,
    Update = 5,
    PostUpdate = 6,
    Last = 7,

    pub fn isStartup(self: ScheduleLabel) bool {
        return self == .PreStartup or self == .Startup or self == .PostStartup;
    }

    pub fn asStr(self: ScheduleLabel) []const u8 {
        return switch (self) {
            .First => "First",
            .PreStartup => "PreStartup",
            .Startup => "Startup",
            .PostStartup => "PostStartup",
            .PreUpdate => "PreUpdate",
            .Update => "Update",
            .PostUpdate => "PostUpdate",
            .Last => "Last",
        };
    }
};

/// System function pointer type
pub const SystemFn = *const fn () callconv(.c) void;

/// System entry in a schedule
pub const SystemEntry = struct {
    system_fn: SystemFn,
    set_id: u64,
};

/// Schedule container holding systems for a specific phase
pub const Schedule = struct {
    allocator: std.mem.Allocator,
    label: ScheduleLabel,
    systems: std.ArrayList(SystemEntry),
    system_sets: std.ArrayList(u64),
    has_run: bool, // Track if startup schedules have run

    pub fn create(allocator: std.mem.Allocator, label: ScheduleLabel) !*Schedule {
        const schedule = try allocator.create(Schedule);

        schedule.* = Schedule{
            .allocator = allocator,
            .label = label,
            .systems = .{},
            .system_sets = .{},
            .has_run = false,
        };

        return schedule;
    }

    pub fn destroy(self: *Schedule) void {
        self.systems.deinit(self.allocator);
        self.system_sets.deinit(self.allocator);
        self.allocator.destroy(self);
    }

    pub fn addSystem(self: *Schedule, system: SystemFn, set_id: u64) !void {
        const entry = SystemEntry{
            .system_fn = system,
            .set_id = set_id,
        };
        try self.systems.append(self.allocator, entry);
    }

    pub fn configureSet(self: *Schedule, set_id: u64) !void {
        // Check if set already exists
        for (self.system_sets.items) |existing_set| {
            if (existing_set == set_id) {
                return; // Already configured
            }
        }
        try self.system_sets.append(self.allocator, set_id);
    }

    pub fn run(self: *Schedule, _: bool) void {
        // For startup schedules, only run on first iteration
        if (self.label.isStartup()) {
            if (self.has_run) {
                return; // Already ran, skip
            }
            self.has_run = true;
        }

        // Run all systems in order
        for (self.systems.items) |entry| {
            entry.system_fn();
        }
    }

    pub fn shouldRun(self: *Schedule, is_first_run: bool) bool {
        if (self.label.isStartup()) {
            // Startup schedules only run on first iteration
            return is_first_run and !self.has_run;
        }
        // Non-startup schedules run every frame
        return true;
    }
};

/// Schedule manager integrated into ZigApp
pub const ScheduleManager = struct {
    allocator: std.mem.Allocator,
    schedules: [8]*Schedule, // 8 schedules matching enum
    is_first_run: bool,

    pub fn create(allocator: std.mem.Allocator) !*ScheduleManager {
        const manager = try allocator.create(ScheduleManager);

        // Create all 8 schedules
        manager.schedules[0] = try Schedule.create(allocator, .First);
        manager.schedules[1] = try Schedule.create(allocator, .PreStartup);
        manager.schedules[2] = try Schedule.create(allocator, .Startup);
        manager.schedules[3] = try Schedule.create(allocator, .PostStartup);
        manager.schedules[4] = try Schedule.create(allocator, .PreUpdate);
        manager.schedules[5] = try Schedule.create(allocator, .Update);
        manager.schedules[6] = try Schedule.create(allocator, .PostUpdate);
        manager.schedules[7] = try Schedule.create(allocator, .Last);

        manager.* = ScheduleManager{
            .allocator = allocator,
            .schedules = manager.schedules,
            .is_first_run = true,
        };

        return manager;
    }

    pub fn destroy(self: *ScheduleManager) void {
        for (self.schedules) |schedule| {
            schedule.destroy();
        }
        self.allocator.destroy(self);
    }

    pub fn addSystem(self: *ScheduleManager, schedule_label: u8, system: SystemFn) !void {
        if (schedule_label >= 8) return error.InvalidScheduleLabel;

        const schedule = self.schedules[schedule_label];
        try schedule.addSystem(system, 0); // Default set_id = 0
    }

    pub fn configureSet(self: *ScheduleManager, schedule_label: u8, set_id: u64) !void {
        if (schedule_label >= 8) return error.InvalidScheduleLabel;

        const schedule = self.schedules[schedule_label];
        try schedule.configureSet(set_id);
    }

    pub fn runSchedule(self: *ScheduleManager, schedule_label: u8) void {
        if (schedule_label >= 8) return;

        const schedule = self.schedules[schedule_label];
        if (schedule.shouldRun(self.is_first_run)) {
            schedule.run(self.is_first_run);
        }
    }

    pub fn runAll(self: *ScheduleManager) void {
        // Run all schedules in order
        for (self.schedules) |schedule| {
            if (schedule.shouldRun(self.is_first_run)) {
                schedule.run(self.is_first_run);
            }
        }

        // After first run, mark as no longer first
        self.is_first_run = false;
    }
};

// FFI exports
export fn schedule_add_system(app: *ZigApp, schedule: u8, system: SystemFn) void {
    // Ensure schedule manager exists
    if (app.schedule_manager == null) {
        app.schedule_manager = ScheduleManager.create(app.allocator) catch return;
    }

    if (app.schedule_manager) |manager| {
        manager.addSystem(schedule, system) catch {};
    }
}

export fn schedule_configure_set(app: *ZigApp, schedule: u8, set_id: u64) void {
    // Ensure schedule manager exists
    if (app.schedule_manager == null) {
        app.schedule_manager = ScheduleManager.create(app.allocator) catch return;
    }

    if (app.schedule_manager) |manager| {
        manager.configureSet(schedule, set_id) catch {};
    }
}

export fn schedule_run(app: *ZigApp, schedule: u8, is_first_run: bool) void {
    if (app.schedule_manager) |manager| {
        manager.is_first_run = is_first_run;
        manager.runSchedule(schedule);
    }
}

export fn schedule_init_resource(app: *ZigApp, type_id: u64) void {
    // Mark resource type as initialized for scheduling purposes
    // The resource itself is managed by app.resources
    _ = app;
    _ = type_id;
    // This is a no-op for now, but can be extended for dependency tracking
}
