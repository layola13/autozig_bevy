const std = @import("std");
const common = @import("common.zig");
const world_mod = @import("world.zig");
const Entity = common.Entity;
const g_allocator = common.g_allocator;
const World = world_mod.World;

// 注意：autozig 会将所有 .zig 文件合并到一个 generated_autozig.zig 中
// Entity, World, g_allocator 等定义来自其他文件，合并后自动可用
// 不需要重复定义或 extern 声明

// BundleInfo - stores metadata about a bundle
pub const BundleInfo = struct {
    allocator: std.mem.Allocator,
    component_ids: []u32,

    pub fn init(alloc: std.mem.Allocator, ids: []const u32) !*BundleInfo {
        const info = try alloc.create(BundleInfo);
        const ids_copy = try alloc.alloc(u32, ids.len);
        @memcpy(ids_copy, ids);

        info.* = BundleInfo{
            .allocator = alloc,
            .component_ids = ids_copy,
        };
        return info;
    }

    pub fn deinit(self: *BundleInfo) void {
        self.allocator.free(self.component_ids);
        self.allocator.destroy(self);
    }
};

// Component storage helper
const ComponentStore = struct {
    component_id: u32,
    data_ptr: *const u8,
    data_size: usize,
};

// Bundle operations implementation
export fn bundle_info_create(
    component_ids_ptr: [*]const u32,
    component_ids_len: usize,
) ?*BundleInfo {
    const ids = component_ids_ptr[0..component_ids_len];
    return BundleInfo.init(g_allocator, ids) catch null;
}

export fn bundle_info_destroy(info: *BundleInfo) void {
    info.deinit();
}

export fn bundle_spawn(
    world_ptr: *World,
    component_ids_ptr: [*]const u32,
    component_ids_len: usize,
    component_data_ptr: [*]const *const u8,
    component_sizes_ptr: [*]const usize,
) Entity {
    // Spawn empty entity first
    const entity = world_ptr.spawnEmpty() catch return Entity{ .index = 0xFFFFFFFF, .generation = 0 };

    // Insert all components
    const ids = component_ids_ptr[0..component_ids_len];
    const data_ptrs = component_data_ptr[0..component_ids_len];
    const sizes = component_sizes_ptr[0..component_ids_len];

    var i: usize = 0;
    while (i < component_ids_len) : (i += 1) {
        // Note: Actual component insertion would require component storage
        // This is a simplified version - real implementation needs component registry
        _ = ids[i];
        _ = data_ptrs[i];
        _ = sizes[i];
    }

    return entity;
}

export fn bundle_insert(
    world_ptr: *World,
    entity: Entity,
    component_ids_ptr: [*]const u32,
    component_ids_len: usize,
    component_data_ptr: [*]const *const u8,
    component_sizes_ptr: [*]const usize,
) bool {
    // Verify entity exists
    if (!world_ptr.contains(entity)) {
        return false;
    }

    // Insert all components
    const ids = component_ids_ptr[0..component_ids_len];
    const data_ptrs = component_data_ptr[0..component_ids_len];
    const sizes = component_sizes_ptr[0..component_ids_len];

    var i: usize = 0;
    while (i < component_ids_len) : (i += 1) {
        // Note: Actual component insertion would require component storage
        // This is a simplified version - real implementation needs component registry
        _ = ids[i];
        _ = data_ptrs[i];
        _ = sizes[i];
    }

    return true;
}

export fn bundle_remove(
    world_ptr: *World,
    entity: Entity,
    component_ids_ptr: [*]const u32,
    component_ids_len: usize,
) bool {
    // Verify entity exists
    if (!world_ptr.contains(entity)) {
        return false;
    }

    // Remove all components
    const ids = component_ids_ptr[0..component_ids_len];

    var i: usize = 0;
    while (i < component_ids_len) : (i += 1) {
        // Note: Actual component removal would require component storage
        // This is a simplified version - real implementation needs component registry
        _ = ids[i];
    }

    return true;
}
