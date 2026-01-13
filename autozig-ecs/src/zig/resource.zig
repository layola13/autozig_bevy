const std = @import("std");
const common = @import("common.zig");
const Entity = common.Entity;
const g_allocator = common.g_allocator;

// 全局Resource注册表 - TypeID -> Pointer映射
pub const ResourceRegistry = struct {
    map: std.AutoHashMap(u64, *anyopaque),
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !*ResourceRegistry {
        const registry = try allocator.create(ResourceRegistry);
        registry.* = ResourceRegistry{
            .map = std.AutoHashMap(u64, *anyopaque).init(allocator),
            .allocator = allocator,
        };
        return registry;
    }

    pub fn deinit(self: *ResourceRegistry) void {
        self.map.deinit();
        self.allocator.destroy(self);
    }

    pub fn insert(self: *ResourceRegistry, type_id: u64, ptr: *anyopaque) !void {
        try self.map.put(type_id, ptr);
    }

    pub fn get(self: *const ResourceRegistry, type_id: u64) ?*anyopaque {
        return self.map.get(type_id);
    }

    pub fn remove(self: *ResourceRegistry, type_id: u64) ?*anyopaque {
        if (self.map.fetchRemove(type_id)) |kv| {
            return kv.value;
        }
        return null;
    }

    pub fn contains(self: *const ResourceRegistry, type_id: u64) bool {
        return self.map.contains(type_id);
    }
};

// 全局allocator在entity.zig中定义，合并后直接可用

// Exported C API
export fn resource_registry_create() ?*ResourceRegistry {
    return ResourceRegistry.init(g_allocator) catch null;
}

export fn resource_registry_destroy(registry: *ResourceRegistry) void {
    registry.deinit();
}

export fn resource_registry_insert(registry: *ResourceRegistry, type_id: u64, ptr: *anyopaque) bool {
    registry.insert(type_id, ptr) catch return false;
    return true;
}

export fn resource_registry_get(registry: *const ResourceRegistry, type_id: u64) ?*anyopaque {
    return registry.get(type_id);
}

export fn resource_registry_remove(registry: *ResourceRegistry, type_id: u64) ?*anyopaque {
    return registry.remove(type_id);
}

export fn resource_registry_contains(registry: *const ResourceRegistry, type_id: u64) bool {
    return registry.contains(type_id);
}
