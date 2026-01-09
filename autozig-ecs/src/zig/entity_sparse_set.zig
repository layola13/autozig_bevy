const std = @import("std");
const common = @import("common.zig");
const Entity = common.Entity;
const g_allocator = common.g_allocator;

/// EntitySparseSet - 纯entity索引的稀疏集合存储（用于Archetype系统）
/// 与component.zig中的SparseSet不同，这个只存储entity索引，不存储组件数据
/// 适合Tag组件、罕见标记等稀疏数据
/// O(1) 插入/删除/查询性能
pub const EntitySparseSet = struct {
    sparse: std.ArrayList(?u32), // entity_id -> dense_index映射
    dense: std.ArrayList(u32), // dense_index -> entity_id数组
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) EntitySparseSet {
        return EntitySparseSet{
            .sparse = std.ArrayList(?u32){},
            .dense = std.ArrayList(u32){},
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *EntitySparseSet) void {
        self.sparse.deinit(self.allocator);
        self.dense.deinit(self.allocator);
    }

    /// 插入entity到稀疏集合
    pub fn insert(self: *EntitySparseSet, entity: u32) !void {
        // 扩展sparse数组以容纳entity索引
        while (self.sparse.items.len <= entity) {
            try self.sparse.append(self.allocator, null);
        }

        // 如果已存在，直接返回
        if (self.sparse.items[entity]) |_| {
            return;
        }

        // 添加到dense数组末尾
        const dense_index = @as(u32, @intCast(self.dense.items.len));
        try self.dense.append(self.allocator, entity);
        self.sparse.items[entity] = dense_index;
    }

    /// 从稀疏集合移除entity
    pub fn remove(self: *EntitySparseSet, entity: u32) bool {
        if (entity >= self.sparse.items.len) return false;

        const dense_index = self.sparse.items[entity] orelse return false;

        // 获取dense数组最后一个元素
        const last_entity = self.dense.items[self.dense.items.len - 1];

        // 将最后一个元素移动到被删除位置（swap-remove）
        self.dense.items[dense_index] = last_entity;
        self.sparse.items[last_entity] = dense_index;

        // 移除最后一个元素
        _ = self.dense.pop();
        self.sparse.items[entity] = null;

        return true;
    }

    /// 检查entity是否存在
    pub fn contains(self: *const EntitySparseSet, entity: u32) bool {
        if (entity >= self.sparse.items.len) return false;
        return self.sparse.items[entity] != null;
    }

    /// 获取entity在dense数组中的索引
    pub fn getDenseIndex(self: *const EntitySparseSet, entity: u32) ?u32 {
        if (entity >= self.sparse.items.len) return null;
        return self.sparse.items[entity];
    }

    /// 获取dense数组长度
    pub fn len(self: *const EntitySparseSet) usize {
        return self.dense.items.len;
    }

    /// 清空集合
    pub fn clear(self: *EntitySparseSet) void {
        self.dense.clearRetainingCapacity();
        for (self.sparse.items) |*item| {
            item.* = null;
        }
    }
};

// FFI导出函数
export fn entity_sparse_set_create() ?*EntitySparseSet {
    const sparse_set = g_allocator.create(EntitySparseSet) catch return null;
    sparse_set.* = EntitySparseSet.init(g_allocator);
    return sparse_set;
}

export fn entity_sparse_set_destroy(set_ptr: *EntitySparseSet) void {
    set_ptr.deinit();
    g_allocator.destroy(set_ptr);
}

export fn entity_sparse_set_insert(set_ptr: *EntitySparseSet, entity: u32) bool {
    set_ptr.insert(entity) catch return false;
    return true;
}

export fn entity_sparse_set_remove(set_ptr: *EntitySparseSet, entity: u32) bool {
    return set_ptr.remove(entity);
}

export fn entity_sparse_set_contains(set_ptr: *const EntitySparseSet, entity: u32) bool {
    return set_ptr.contains(entity);
}

export fn entity_sparse_set_len(set_ptr: *const EntitySparseSet) usize {
    return set_ptr.len();
}

export fn entity_sparse_set_clear(set_ptr: *EntitySparseSet) void {
    set_ptr.clear();
}
