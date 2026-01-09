const std = @import("std");
const common = @import("common.zig");
const Entity = common.Entity;
const g_allocator = common.g_allocator;

/// 组件存储类型
pub const ComponentStorageType = enum(u8) {
    Table, // 密集表存储
    SparseSet, // 稀疏集合存储
};

/// Archetype - ECS原型，表示具有相同组件集合的实体分组
/// 每个Archetype维护两种存储类型的组件
pub const Archetype = struct {
    id: u32,
    table_components: std.ArrayList(u32), // 存储在Table中的组件ID列表
    sparse_set_components: std.ArrayList(u32), // 存储在SparseSet中的组件ID列表
    entities: std.ArrayList(u32), // 属于此Archetype的所有entity ID
    entity_to_row: std.AutoHashMap(u32, usize), // entity -> row映射（用于快速查找）
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, id: u32) Archetype {
        return Archetype{
            .id = id,
            .table_components = std.ArrayList(u32){},
            .sparse_set_components = std.ArrayList(u32){},
            .entities = std.ArrayList(u32){},
            .entity_to_row = std.AutoHashMap(u32, usize).init(allocator),
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Archetype) void {
        self.table_components.deinit(self.allocator);
        self.sparse_set_components.deinit(self.allocator);
        self.entities.deinit(self.allocator);
        self.entity_to_row.deinit();
    }

    /// 添加Table组件ID
    pub fn addTableComponent(self: *Archetype, component_id: u32) !void {
        // 检查是否已存在
        for (self.table_components.items) |id| {
            if (id == component_id) return;
        }
        try self.table_components.append(self.allocator, component_id);
    }

    /// 添加SparseSet组件ID
    pub fn addSparseSetComponent(self: *Archetype, component_id: u32) !void {
        // 检查是否已存在
        for (self.sparse_set_components.items) |id| {
            if (id == component_id) return;
        }
        try self.sparse_set_components.append(self.allocator, component_id);
    }

    /// 添加entity到此Archetype
    pub fn addEntity(self: *Archetype, entity: u32) !usize {
        // 检查是否已存在
        if (self.entity_to_row.get(entity)) |_| {
            return error.EntityAlreadyExists;
        }

        const row = self.entities.items.len;
        try self.entities.append(self.allocator, entity);
        try self.entity_to_row.put(entity, row);
        return row;
    }

    /// 从Archetype移除entity
    pub fn removeEntity(self: *Archetype, entity: u32) bool {
        const row = self.entity_to_row.get(entity) orelse return false;

        // swap-remove：用最后一个entity替换被删除的entity
        const last_idx = self.entities.items.len - 1;
        if (row < last_idx) {
            const last_entity = self.entities.items[last_idx];
            self.entities.items[row] = last_entity;
            self.entity_to_row.put(last_entity, row) catch return false;
        }

        _ = self.entities.pop();
        _ = self.entity_to_row.remove(entity);
        return true;
    }

    /// 获取entity的行号
    pub fn getEntityRow(self: *const Archetype, entity: u32) ?usize {
        return self.entity_to_row.get(entity);
    }

    /// 检查是否包含entity
    pub fn containsEntity(self: *const Archetype, entity: u32) bool {
        return self.entity_to_row.contains(entity);
    }

    /// 获取entity数量
    pub fn entityCount(self: *const Archetype) usize {
        return self.entities.items.len;
    }

    /// 检查是否包含指定的Table组件
    pub fn hasTableComponent(self: *const Archetype, component_id: u32) bool {
        for (self.table_components.items) |id| {
            if (id == component_id) return true;
        }
        return false;
    }

    /// 检查是否包含指定的SparseSet组件
    pub fn hasSparseSetComponent(self: *const Archetype, component_id: u32) bool {
        for (self.sparse_set_components.items) |id| {
            if (id == component_id) return true;
        }
        return false;
    }

    /// 检查是否包含指定组件（任一存储类型）
    pub fn hasComponent(self: *const Archetype, component_id: u32) bool {
        return self.hasTableComponent(component_id) or self.hasSparseSetComponent(component_id);
    }

    /// 获取所有组件ID（Table + SparseSet）
    pub fn getAllComponents(self: *const Archetype, out_buffer: []u32) usize {
        var count: usize = 0;

        for (self.table_components.items) |id| {
            if (count < out_buffer.len) {
                out_buffer[count] = id;
                count += 1;
            }
        }

        for (self.sparse_set_components.items) |id| {
            if (count < out_buffer.len) {
                out_buffer[count] = id;
                count += 1;
            }
        }

        return count;
    }

    /// 清空所有entities
    pub fn clear(self: *Archetype) void {
        self.entities.clearRetainingCapacity();
        self.entity_to_row.clearRetainingCapacity();
    }
};

// FFI导出函数
export fn archetype_create(id: u32) ?*Archetype {
    const archetype = g_allocator.create(Archetype) catch return null;
    archetype.* = Archetype.init(g_allocator, id);
    return archetype;
}

export fn archetype_destroy(arch_ptr: *Archetype) void {
    arch_ptr.deinit();
    g_allocator.destroy(arch_ptr);
}

export fn archetype_add_table_component(arch_ptr: *Archetype, component_id: u32) bool {
    arch_ptr.addTableComponent(component_id) catch return false;
    return true;
}

export fn archetype_add_sparse_set_component(arch_ptr: *Archetype, component_id: u32) bool {
    arch_ptr.addSparseSetComponent(component_id) catch return false;
    return true;
}

export fn archetype_add_entity(arch_ptr: *Archetype, entity: u32) i64 {
    const row = arch_ptr.addEntity(entity) catch return -1;
    return @as(i64, @intCast(row));
}

export fn archetype_remove_entity(arch_ptr: *Archetype, entity: u32) bool {
    return arch_ptr.removeEntity(entity);
}

export fn archetype_get_entity_row(arch_ptr: *const Archetype, entity: u32) i64 {
    return if (arch_ptr.getEntityRow(entity)) |row| @as(i64, @intCast(row)) else -1;
}

export fn archetype_contains_entity(arch_ptr: *const Archetype, entity: u32) bool {
    return arch_ptr.containsEntity(entity);
}

export fn archetype_entity_count(arch_ptr: *const Archetype) usize {
    return arch_ptr.entityCount();
}

export fn archetype_has_component(arch_ptr: *const Archetype, component_id: u32) bool {
    return arch_ptr.hasComponent(component_id);
}

export fn archetype_has_table_component(arch_ptr: *const Archetype, component_id: u32) bool {
    return arch_ptr.hasTableComponent(component_id);
}

export fn archetype_has_sparse_set_component(arch_ptr: *const Archetype, component_id: u32) bool {
    return arch_ptr.hasSparseSetComponent(component_id);
}

export fn archetype_get_id(arch_ptr: *const Archetype) u32 {
    return arch_ptr.id;
}

export fn archetype_table_component_count(arch_ptr: *const Archetype) usize {
    return arch_ptr.table_components.items.len;
}

export fn archetype_sparse_set_component_count(arch_ptr: *const Archetype) usize {
    return arch_ptr.sparse_set_components.items.len;
}

export fn archetype_get_entities(arch_ptr: *const Archetype, out_buffer: [*]u32, buffer_len: usize) usize {
    const count = @min(arch_ptr.entities.items.len, buffer_len);
    @memcpy(out_buffer[0..count], arch_ptr.entities.items[0..count]);
    return count;
}

export fn archetype_clear(arch_ptr: *Archetype) void {
    arch_ptr.clear();
}
