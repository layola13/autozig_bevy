const std = @import("std");

/// Forward declaration of ScheduleManager
const ScheduleManager = @import("schedule.zig").ScheduleManager;

/// Application state
pub const ZigApp = struct {
    allocator: std.mem.Allocator,
    main_sub_app: ?*SubApp,
    sub_apps: std.StringHashMap(*SubApp),
    runner: ?*const fn (*ZigApp) callconv(.c) u8,
    plugins: std.ArrayList(*ZigPlugin),
    resources: std.AutoHashMap(u64, ResourceEntry),
    exit_code: ?u8,
    plugin_state: PluginState,
    schedule_manager: ?*ScheduleManager,
    world: ?*anyopaque,

    const PluginState = enum {
        Adding,
        Ready,
        Finished,
        Cleaned,
    };

    const ResourceEntry = struct {
        data: []const u8,
        type_id: u64,
    };

    pub fn create(world: ?*anyopaque) !*ZigApp {
        const allocator = std.heap.page_allocator;
        const app = try allocator.create(ZigApp);

        const main_sub_app = try SubApp.create(allocator);

        // 分配空的plugins数组
        const empty_plugins = try allocator.alloc(*ZigPlugin, 0);

        app.* = ZigApp{
            .allocator = allocator,
            .main_sub_app = main_sub_app,
            .sub_apps = std.StringHashMap(*SubApp).init(allocator),
            .runner = null,
            .plugins = .{ .items = empty_plugins, .capacity = 0 },
            .resources = std.AutoHashMap(u64, ResourceEntry).init(allocator),
            .exit_code = null,
            .plugin_state = .Adding,
            .schedule_manager = null,
            .world = world,
        };

        return app;
    }

    pub fn createEmpty(world: ?*anyopaque) !*ZigApp {
        const allocator = std.heap.page_allocator;
        const app = try allocator.create(ZigApp);

        // 分配空的plugins数组
        const empty_plugins = try allocator.alloc(*ZigPlugin, 0);

        app.* = ZigApp{
            .allocator = allocator,
            .main_sub_app = null,
            .sub_apps = std.StringHashMap(*SubApp).init(allocator),
            .runner = null,
            .plugins = .{ .items = empty_plugins, .capacity = 0 },
            .resources = std.AutoHashMap(u64, ResourceEntry).init(allocator),
            .exit_code = null,
            .plugin_state = .Adding,
            .schedule_manager = null,
            .world = world,
        };

        return app;
    }

    pub fn destroy(self: *ZigApp) void {
        // 0. 清理schedule_manager
        if (self.schedule_manager) |manager| {
            manager.destroy();
        }

        // 1. 先清理plugins（它们不持有外部SubApp资源）
        for (self.plugins.items) |plugin| {
            plugin.destroy();
        }
        self.plugins.deinit(self.allocator);

        // 2. 清理resources（简单数据）
        var res_it = self.resources.iterator();
        while (res_it.next()) |entry| {
            self.allocator.free(entry.value_ptr.data);
        }
        self.resources.deinit();

        // 3. 清理sub_apps的HashMap（只释放键，不销毁SubApp对象）
        // 注意：sub_apps中存储的SubApp指针可能就是main_sub_app，
        // 所以这里只清理HashMap结构和键，不调用SubApp.destroy()
        var it = self.sub_apps.iterator();
        while (it.next()) |entry| {
            // 释放存储的name_copy键
            const key = entry.key_ptr.*;
            self.allocator.free(key);
        }
        self.sub_apps.deinit();

        // 4. 最后清理main_sub_app（唯一拥有所有权的SubApp）
        // 这是唯一真正销毁SubApp的地方
        if (self.main_sub_app) |sub_app| {
            sub_app.destroy();
        }

        // 5. 最后释放self
        self.allocator.destroy(self);
    }

    pub fn update(self: *ZigApp) void {
        // Run schedule manager if it exists
        if (self.schedule_manager) |manager| {
            manager.runAll();
        }

        // Update main sub app
        if (self.main_sub_app) |sub_app| {
            sub_app.runDefaultSchedule();
        }

        // Update all sub apps
        var it = self.sub_apps.iterator();
        while (it.next()) |entry| {
            const sub_app = entry.value_ptr.*;
            sub_app.update();
        }

        // Clear trackers
        if (self.main_sub_app) |sub_app| {
            sub_app.clearTrackers();
        }
    }

    pub fn run(self: *ZigApp) u8 {
        // Use custom runner if set
        if (self.runner) |runner_fn| {
            return runner_fn(self);
        }

        // Default runner: run once
        while (self.plugin_state == .Adding) {
            // Wait for plugins to be ready
            var all_ready = true;
            for (self.plugins.items) |plugin| {
                if (!plugin.isReady(self)) {
                    all_ready = false;
                    break;
                }
            }
            if (all_ready) {
                self.plugin_state = .Ready;
            }
        }

        self.finish();
        self.cleanup();
        self.update();

        return if (self.exit_code) |code| code else 0;
    }

    pub fn setRunner(self: *ZigApp, runner: *const fn (*ZigApp) callconv(.c) u8) void {
        self.runner = runner;
    }

    pub fn shouldExit(self: *ZigApp) i32 {
        if (self.exit_code) |code| {
            return @intCast(code);
        }
        return -1;
    }

    pub fn finish(self: *ZigApp) void {
        if (self.plugin_state != .Ready) return;

        for (self.plugins.items) |plugin| {
            plugin.finish(self);
        }

        self.plugin_state = .Finished;
    }

    pub fn cleanup(self: *ZigApp) void {
        if (self.plugin_state != .Finished) return;

        for (self.plugins.items) |plugin| {
            plugin.cleanup(self);
        }

        self.plugin_state = .Cleaned;
    }

    pub fn addSubApp(self: *ZigApp, name: []const u8) !*SubApp {
        const sub_app = try SubApp.create(self.allocator);

        // Store name copy
        const name_copy = try self.allocator.dupe(u8, name);
        try self.sub_apps.put(name_copy, sub_app);

        return sub_app;
    }

    pub fn getSubApp(self: *ZigApp, name: []const u8) ?*SubApp {
        return self.sub_apps.get(name);
    }

    pub fn insertResource(self: *ZigApp, type_id: u64, data: []const u8) !void {
        // Make a copy of the data
        const data_copy = try self.allocator.dupe(u8, data);

        const entry = ResourceEntry{
            .data = data_copy,
            .type_id = type_id,
        };

        try self.resources.put(type_id, entry);
    }

    pub fn hasResource(self: *ZigApp, type_id: u64) bool {
        return self.resources.contains(type_id);
    }

    pub fn addPlugin(self: *ZigApp, plugin: *ZigPlugin) !bool {
        // Check if plugin is unique and already added
        if (plugin.isUnique()) {
            for (self.plugins.items) |existing| {
                if (std.mem.eql(u8, existing.getName(), plugin.getName())) {
                    return false; // Already added
                }
            }
        }

        try self.plugins.append(self.allocator, plugin);
        plugin.build(self);

        return true;
    }
};

/// SubApp structure (forward declaration, defined in sub_app.zig)
pub const SubApp = @import("sub_app.zig").SubApp;

/// Plugin structure (forward declaration, defined in plugin.zig)
pub const ZigPlugin = @import("plugin.zig").ZigPlugin;

// FFI exports
export fn app_create(world: ?*anyopaque) ?*ZigApp {
    return ZigApp.create(world) catch null;
}

export fn app_create_empty(world: ?*anyopaque) ?*ZigApp {
    return ZigApp.createEmpty(world) catch null;
}

export fn app_get_world(app: *ZigApp) ?*anyopaque {
    return app.world;
}

export fn app_destroy(app: *ZigApp) void {
    app.destroy();
}

export fn app_update(app: *ZigApp) void {
    app.update();
}

export fn app_run(app: *ZigApp) u8 {
    return app.run();
}

export fn app_set_runner(app: *ZigApp, runner: *const fn (*ZigApp) callconv(.c) u8) void {
    app.setRunner(runner);
}

export fn app_should_exit(app: *ZigApp) i32 {
    return app.shouldExit();
}

export fn app_finish(app: *ZigApp) void {
    app.finish();
}

export fn app_cleanup(app: *ZigApp) void {
    app.cleanup();
}

export fn app_add_sub_app(app: *ZigApp, name_ptr: [*]const u8, name_len: usize) ?*SubApp {
    const name = name_ptr[0..name_len];
    return app.addSubApp(name) catch null;
}

export fn app_get_sub_app(app: *ZigApp, name_ptr: [*]const u8, name_len: usize) ?*SubApp {
    const name = name_ptr[0..name_len];
    return app.getSubApp(name);
}

export fn app_insert_resource(app: *ZigApp, type_id: u64, data_ptr: [*]const u8, data_len: usize) void {
    const data = data_ptr[0..data_len];
    app.insertResource(type_id, data) catch {};
}

export fn app_has_resource(app: *ZigApp, type_id: u64) bool {
    return app.hasResource(type_id);
}

export fn app_get_resource(app: *ZigApp, type_id: u64) ?*anyopaque {
    if (app.resources.get(type_id)) |entry| {
        // Remove const-ness to matches C API
        return @constCast(entry.data.ptr);
    }
    return null;
}
