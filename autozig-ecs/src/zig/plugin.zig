const std = @import("std");
const common = @import("common.zig");
const Entity = common.Entity;
const g_allocator = common.g_allocator;

// Plugin function pointer type
pub const PluginBuildFn = *const fn (app_ptr: *anyopaque) callconv(.c) void;

// Plugin metadata
pub const PluginMeta = struct {
    name: []const u8,
    build_fn: PluginBuildFn,
    initialized: bool,
};

// Plugin manager - 管理插件注册和执行
pub const PluginManager = struct {
    plugins: std.ArrayList(PluginMeta),
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator) !*PluginManager {
        const manager = try allocator.create(PluginManager);
        manager.* = PluginManager{
            .plugins = std.ArrayList(PluginMeta){},
            .allocator = allocator,
        };
        return manager;
    }
    
    pub fn deinit(self: *PluginManager) void {
        self.plugins.deinit(self.allocator);
        self.allocator.destroy(self);
    }
    
    pub fn add(self: *PluginManager, name: []const u8, build_fn: PluginBuildFn) !void {
        try self.plugins.append(self.allocator, PluginMeta{
            .name = name,
            .build_fn = build_fn,
            .initialized = false,
        });
    }
    
    pub fn runAll(self: *PluginManager, app_ptr: *anyopaque) void {
        for (self.plugins.items) |*plugin| {
            if (!plugin.initialized) {
                plugin.build_fn(app_ptr);
                plugin.initialized = true;
            }
        }
    }
    
    pub fn count(self: *const PluginManager) usize {
        return self.plugins.items.len;
    }
};

// 全局allocator在entity.zig中定义，合并后直接可用

// Exported C API
export fn plugin_manager_create() ?*PluginManager {
    return PluginManager.init(g_allocator) catch null;
}

export fn plugin_manager_destroy(manager: *PluginManager) void {
    manager.deinit();
}

export fn plugin_manager_add(
    manager: *PluginManager,
    name_ptr: [*]const u8,
    name_len: usize,
    build_fn: PluginBuildFn,
) bool {
    const name = name_ptr[0..name_len];
    manager.add(name, build_fn) catch return false;
    return true;
}

export fn plugin_manager_run_all(manager: *PluginManager, app_ptr: *anyopaque) void {
    manager.runAll(app_ptr);
}

export fn plugin_manager_count(manager: *const PluginManager) usize {
    return manager.count();
}
