const std = @import("std");
const ZigApp = @import("app.zig").ZigApp;
const ZigPlugin = @import("plugin.zig").ZigPlugin;

/// Plugin entry in a group - tracks plugin and enabled state
pub const PluginEntry = struct {
    plugin: *ZigPlugin,
    enabled: bool,
    type_id: u64,
};

/// Plugin Group Builder - manages ordered collection of plugins
pub const PluginGroupBuilder = struct {
    allocator: std.mem.Allocator,
    group_name: []const u8,
    plugins: std.AutoHashMap(u64, PluginEntry),
    order: std.ArrayList(u64),

    pub fn create(allocator: std.mem.Allocator, name: []const u8) !*PluginGroupBuilder {
        const builder = try allocator.create(PluginGroupBuilder);

        const name_copy = try allocator.dupe(u8, name);

        builder.* = PluginGroupBuilder{
            .allocator = allocator,
            .group_name = name_copy,
            .plugins = std.AutoHashMap(u64, PluginEntry).init(allocator),
            .order = std.ArrayList(u64){},
        };

        return builder;
    }

    pub fn destroy(self: *PluginGroupBuilder) void {
        self.allocator.free(self.group_name);

        // Note: plugins are owned by the App, we only store references
        self.plugins.deinit();
        self.order.deinit(self.allocator);

        self.allocator.destroy(self);
    }

    /// Check if builder contains a plugin by type_id
    pub fn contains(self: *PluginGroupBuilder, type_id: u64) bool {
        return self.plugins.contains(type_id);
    }

    /// Check if a plugin is enabled
    pub fn isEnabled(self: *PluginGroupBuilder, type_id: u64) bool {
        if (self.plugins.get(type_id)) |entry| {
            return entry.enabled;
        }
        return false;
    }

    /// Find index of a plugin in the order list
    fn indexOf(self: *PluginGroupBuilder, type_id: u64) ?usize {
        for (self.order.items, 0..) |id, i| {
            if (id == type_id) {
                return i;
            }
        }
        return null;
    }

    /// Add plugin at the end of the builder
    pub fn add(self: *PluginGroupBuilder, plugin: *ZigPlugin, type_id: u64) !void {
        // Remove from previous position if it exists
        if (self.indexOf(type_id)) |old_index| {
            _ = self.order.orderedRemove(old_index);
        }

        // Add to end
        try self.order.append(self.allocator, type_id);

        const entry = PluginEntry{
            .plugin = plugin,
            .enabled = true,
            .type_id = type_id,
        };

        try self.plugins.put(type_id, entry);
    }

    /// Add plugin before target plugin
    pub fn addBefore(
        self: *PluginGroupBuilder,
        plugin: *ZigPlugin,
        type_id: u64,
        target_type_id: u64,
    ) !bool {
        const target_index = self.indexOf(target_type_id) orelse return false;

        // Remove from previous position if it exists
        if (self.indexOf(type_id)) |old_index| {
            _ = self.order.orderedRemove(old_index);
        }

        // Insert before target
        try self.order.insert(self.allocator, target_index, type_id);

        const entry = PluginEntry{
            .plugin = plugin,
            .enabled = true,
            .type_id = type_id,
        };

        try self.plugins.put(type_id, entry);
        return true;
    }

    /// Add plugin after target plugin
    pub fn addAfter(
        self: *PluginGroupBuilder,
        plugin: *ZigPlugin,
        type_id: u64,
        target_type_id: u64,
    ) !bool {
        const target_index = self.indexOf(target_type_id) orelse return false;

        // Remove from previous position if it exists
        if (self.indexOf(type_id)) |old_index| {
            _ = self.order.orderedRemove(old_index);
        }

        // Insert after target
        const insert_index = target_index + 1;
        try self.order.insert(self.allocator, insert_index, type_id);

        const entry = PluginEntry{
            .plugin = plugin,
            .enabled = true,
            .type_id = type_id,
        };

        try self.plugins.put(type_id, entry);
        return true;
    }

    /// Enable a plugin
    pub fn enable(self: *PluginGroupBuilder, type_id: u64) !void {
        if (self.plugins.getPtr(type_id)) |entry| {
            entry.enabled = true;
        } else {
            return error.PluginNotFound;
        }
    }

    /// Disable a plugin
    pub fn disable(self: *PluginGroupBuilder, type_id: u64) !void {
        if (self.plugins.getPtr(type_id)) |entry| {
            entry.enabled = false;
        } else {
            return error.PluginNotFound;
        }
    }

    /// Set/replace plugin value
    pub fn set(self: *PluginGroupBuilder, plugin: *ZigPlugin, type_id: u64) !bool {
        if (self.plugins.getPtr(type_id)) |entry| {
            entry.plugin = plugin;
            return true;
        }
        return false;
    }

    /// Finish building and add all enabled plugins to app
    pub fn finish(self: *PluginGroupBuilder, app: *ZigApp) !void {
        for (self.order.items) |type_id| {
            if (self.plugins.get(type_id)) |entry| {
                if (entry.enabled) {
                    _ = try app.addPlugin(entry.plugin);
                }
            }
        }
    }

    /// Get number of plugins in builder
    pub fn len(self: *PluginGroupBuilder) usize {
        return self.order.items.len;
    }

    /// Get number of enabled plugins
    pub fn enabledCount(self: *PluginGroupBuilder) usize {
        var count: usize = 0;
        for (self.order.items) |type_id| {
            if (self.plugins.get(type_id)) |entry| {
                if (entry.enabled) {
                    count += 1;
                }
            }
        }
        return count;
    }
};

// FFI exports
export fn plugin_group_builder_create(name_ptr: [*]const u8, name_len: usize) ?*PluginGroupBuilder {
    const allocator = std.heap.page_allocator;
    const name = name_ptr[0..name_len];
    return PluginGroupBuilder.create(allocator, name) catch null;
}

export fn plugin_group_builder_destroy(builder: *PluginGroupBuilder) void {
    builder.destroy();
}

export fn plugin_group_builder_contains(builder: *PluginGroupBuilder, type_id: u64) bool {
    return builder.contains(type_id);
}

export fn plugin_group_builder_is_enabled(builder: *PluginGroupBuilder, type_id: u64) bool {
    return builder.isEnabled(type_id);
}

export fn plugin_group_builder_add(
    builder: *PluginGroupBuilder,
    plugin: *ZigPlugin,
    type_id: u64,
) bool {
    builder.add(plugin, type_id) catch return false;
    return true;
}

export fn plugin_group_builder_add_before(
    builder: *PluginGroupBuilder,
    plugin: *ZigPlugin,
    type_id: u64,
    target_type_id: u64,
) bool {
    return builder.addBefore(plugin, type_id, target_type_id) catch false;
}

export fn plugin_group_builder_add_after(
    builder: *PluginGroupBuilder,
    plugin: *ZigPlugin,
    type_id: u64,
    target_type_id: u64,
) bool {
    return builder.addAfter(plugin, type_id, target_type_id) catch false;
}

export fn plugin_group_builder_enable(builder: *PluginGroupBuilder, type_id: u64) bool {
    builder.enable(type_id) catch return false;
    return true;
}

export fn plugin_group_builder_disable(builder: *PluginGroupBuilder, type_id: u64) bool {
    builder.disable(type_id) catch return false;
    return true;
}

export fn plugin_group_builder_set(
    builder: *PluginGroupBuilder,
    plugin: *ZigPlugin,
    type_id: u64,
) bool {
    return builder.set(plugin, type_id) catch false;
}

export fn plugin_group_builder_finish(builder: *PluginGroupBuilder, app: *ZigApp) bool {
    builder.finish(app) catch return false;
    return true;
}

export fn plugin_group_builder_len(builder: *PluginGroupBuilder) usize {
    return builder.len();
}

export fn plugin_group_builder_enabled_count(builder: *PluginGroupBuilder) usize {
    return builder.enabledCount();
}
