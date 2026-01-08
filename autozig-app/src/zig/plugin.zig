const std = @import("std");
const ZigApp = @import("app.zig").ZigApp;

/// Plugin structure - represents a plugin with build/lifecycle hooks
pub const ZigPlugin = struct {
    allocator: std.mem.Allocator,
    name: []const u8,
    build_fn: *const fn (*ZigApp) callconv(.c) void,
    is_unique: bool,
    ready_state: bool,

    pub fn create(
        allocator: std.mem.Allocator,
        name: []const u8,
        build_fn: *const fn (*ZigApp) callconv(.c) void,
        is_unique: bool,
    ) !*ZigPlugin {
        const plugin = try allocator.create(ZigPlugin);

        // Make a copy of the name
        const name_copy = try allocator.dupe(u8, name);

        plugin.* = ZigPlugin{
            .allocator = allocator,
            .name = name_copy,
            .build_fn = build_fn,
            .is_unique = is_unique,
            .ready_state = true,
        };

        return plugin;
    }

    pub fn destroy(self: *ZigPlugin) void {
        self.allocator.free(self.name);
        self.allocator.destroy(self);
    }

    pub fn build(self: *ZigPlugin, app: *ZigApp) void {
        self.build_fn(app);
    }

    pub fn isReady(self: *ZigPlugin, app: *ZigApp) bool {
        _ = app;
        return self.ready_state;
    }

    pub fn finish(self: *ZigPlugin, app: *ZigApp) void {
        // Default implementation - do nothing
        _ = self;
        _ = app;
    }

    pub fn cleanup(self: *ZigPlugin, app: *ZigApp) void {
        // Default implementation - do nothing
        _ = self;
        _ = app;
    }

    pub fn getName(self: *ZigPlugin) []const u8 {
        return self.name;
    }

    pub fn isUnique(self: *ZigPlugin) bool {
        return self.is_unique;
    }
};

// FFI exports
export fn plugin_create(
    name_ptr: [*]const u8,
    name_len: usize,
    build_fn: *const fn (*ZigApp) callconv(.c) void,
    is_unique: bool,
) ?*ZigPlugin {
    const allocator = std.heap.page_allocator;
    const name = name_ptr[0..name_len];
    return ZigPlugin.create(allocator, name, build_fn, is_unique) catch null;
}

export fn plugin_destroy(plugin: *ZigPlugin) void {
    plugin.destroy();
}

export fn plugin_build(plugin: *ZigPlugin, app: *ZigApp) void {
    plugin.build(app);
}

export fn plugin_name(plugin: *ZigPlugin, out_ptr: *[*]const u8, out_len: *usize) void {
    const name = plugin.getName();
    out_ptr.* = name.ptr;
    out_len.* = name.len;
}

export fn plugin_is_unique(plugin: *ZigPlugin) bool {
    return plugin.isUnique();
}

export fn app_add_plugin(app: *ZigApp, plugin: *ZigPlugin) bool {
    return app.addPlugin(plugin) catch false;
}
