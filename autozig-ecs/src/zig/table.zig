const std = @import("std");
const common = @import("common.zig");
const change_detection = @import("change_detection.zig");
const Entity = common.Entity;
const ComponentTicks = change_detection.ComponentTicks;
const Tick = change_detection.Tick;
const g_allocator = common.g_allocator;

/// Column - 列式存储，每个组件类型一列
pub const Column = struct {
    component_id: u32,
    data: std.ArrayList(u8), // 原始字节存储
    ticks: std.ArrayList(ComponentTicks), // 变更检测周期
    item_size: usize,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, component_id: u32, item_size: usize) Column {
        return Column{
            .component_id = component_id,
            .data = std.ArrayList(u8){},
            .ticks = std.ArrayList(ComponentTicks){},
            .item_size = item_size,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Column) void {
        self.data.deinit(self.allocator);
        self.ticks.deinit(self.allocator);
    }

    /// 添加一行数据（从指针复制）
    pub fn pushFromPtr(self: *Column, data_ptr: [*]const u8, tick: Tick) !void {
        const start = self.data.items.len;
        try self.data.resize(self.allocator, start + self.item_size);
        @memcpy(self.data.items[start..][0..self.item_size], data_ptr[0..self.item_size]);
        try self.ticks.append(self.allocator, ComponentTicks.new(tick));
    }

    /// 获取指定行的数据指针
    pub fn getPtr(self: *Column, row: usize) ?[*]u8 {
        const start = row * self.item_size;
        if (start + self.item_size > self.data.items.len) return null;
        return self.data.items[start..].ptr;
    }

    /// 获取指定行的Ticks指针
    pub fn getTicksPtr(self: *Column, row: usize) ?*ComponentTicks {
        if (row >= self.ticks.items.len) return null;
        return &self.ticks.items[row];
    }

    /// 获取只读数据指针
    pub fn getConstPtr(self: *const Column, row: usize) ?[*]const u8 {
        const start = row * self.item_size;
        if (start + self.item_size > self.data.items.len) return null;
        return self.data.items[start..].ptr;
    }

    /// swap-remove：用最后一行替换指定行
    pub fn swapRemove(self: *Column, row: usize) bool {
        const row_count = self.rowCount();
        if (row >= row_count) return false;

        if (row == row_count - 1) {
            // 如果是最后一行，直接删除
            self.data.shrinkRetainingCapacity(self.data.items.len - self.item_size);
            _ = self.ticks.pop();
            return true;
        }

        // 将最后一行的数据复制到要删除的行
        const row_start = row * self.item_size;
        const last_start = (row_count - 1) * self.item_size;
        @memcpy(
            self.data.items[row_start..][0..self.item_size],
            self.data.items[last_start..][0..self.item_size],
        );

        // swap-remove ticks
        self.ticks.items[row] = self.ticks.items[row_count - 1];
        _ = self.ticks.pop();

        // 删除最后一行数据
        self.data.shrinkRetainingCapacity(self.data.items.len - self.item_size);
        return true;
    }

    /// 获取行数
    pub fn rowCount(self: *const Column) usize {
        return self.ticks.items.len;
    }

    /// 清空所有数据
    pub fn clear(self: *Column) void {
        self.data.clearRetainingCapacity();
        self.ticks.clearRetainingCapacity();
    }
};

/// Table - 列式存储，适合频繁迭代的组件
pub const Table = struct {
    columns: std.ArrayList(Column),
    entity_list: std.ArrayList(u32), // 每行对应的entity ID
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) Table {
        return Table{
            .columns = std.ArrayList(Column){},
            .entity_list = std.ArrayList(u32){},
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *Table) void {
        for (self.columns.items) |*column| {
            column.deinit();
        }
        self.columns.deinit(self.allocator);
        self.entity_list.deinit(self.allocator);
    }

    /// 添加一列（组件类型）
    pub fn addColumn(self: *Table, component_id: u32, item_size: usize) !void {
        // 检查是否已存在
        for (self.columns.items) |column| {
            if (column.component_id == component_id) return;
        }

        var column = Column.init(self.allocator, component_id, item_size);

        // 为新列填充空数据以匹配现有行数
        const row_count = self.entityCount();
        if (row_count > 0) {
            try column.data.resize(self.allocator, row_count * item_size);
            @memset(column.data.items, 0);
            try column.ticks.resize(self.allocator, row_count);
            @memset(column.ticks.items, ComponentTicks.new(Tick.new(0)));
        }

        try self.columns.append(self.allocator, column);
    }

    /// 获取列索引
    pub fn getColumnIndex(self: *const Table, component_id: u32) ?usize {
        for (self.columns.items, 0..) |column, i| {
            if (column.component_id == component_id) return i;
        }
        return null;
    }

    /// 获取列
    pub fn getColumn(self: *Table, component_id: u32) ?*Column {
        const idx = self.getColumnIndex(component_id) orelse return null;
        return &self.columns.items[idx];
    }

    pub fn getColumnConst(self: *const Table, component_id: u32) ?*const Column {
        const idx = self.getColumnIndex(component_id) orelse return null;
        return &self.columns.items[idx];
    }

    /// 添加一行（为entity添加数据）
    pub fn pushRow(self: *Table, entity: u32, tick: Tick) !usize {
        const row = self.entityCount();
        try self.entity_list.append(self.allocator, entity);

        for (self.columns.items) |*column| {
            const start = column.data.items.len;
            try column.data.resize(self.allocator, start + column.item_size);
            @memset(column.data.items[start..], 0);
            try column.ticks.append(self.allocator, ComponentTicks.new(tick));
            // std.debug.print("Table.pushRow: table={}, col={}, row={}, ticks_ptr={*}\n", .{ @intFromPtr(self), column.component_id, column.ticks.items.len - 1, &column.ticks.items[column.ticks.items.len - 1] });
        }

        return row;
    }

    /// swap-remove：删除指定行
    pub fn swapRemove(self: *Table, row: usize) bool {
        if (row >= self.entityCount()) return false;

        // swap-remove entity
        const last_idx = self.entity_list.items.len - 1;
        self.entity_list.items[row] = self.entity_list.items[last_idx];
        _ = self.entity_list.pop();

        for (self.columns.items) |*column| {
            _ = column.swapRemove(row);
        }

        return true;
    }

    /// 获取entity在table中的行号
    pub fn getEntityRow(self: *const Table, entity: u32) ?usize {
        for (self.entity_list.items, 0..) |e, i| {
            if (e == entity) return i;
        }
        return null;
    }

    /// 获取实体数量（行数）
    pub fn entityCount(self: *const Table) usize {
        return self.entity_list.items.len;
    }

    /// 清空所有数据
    pub fn clear(self: *Table) void {
        for (self.columns.items) |*column| {
            column.clear();
        }
        self.entity_list.clearRetainingCapacity();
    }
};

// FFI导出函数
export fn table_create() ?*Table {
    const table = g_allocator.create(Table) catch return null;
    table.* = Table.init(g_allocator);
    return table;
}

export fn table_destroy(table_ptr: *Table) void {
    table_ptr.deinit();
    g_allocator.destroy(table_ptr);
}

export fn table_add_column(table_ptr: *Table, component_id: u32, item_size: usize) bool {
    table_ptr.addColumn(component_id, item_size) catch return false;
    return true;
}

export fn table_push_row(table_ptr: *Table, entity: u32, tick: Tick) usize {
    return table_ptr.pushRow(entity, tick) catch std.math.maxInt(usize);
}

export fn table_swap_remove(table_ptr: *Table, row: usize) bool {
    return table_ptr.swapRemove(row);
}

export fn table_entity_count(table_ptr: *const Table) usize {
    return table_ptr.entityCount();
}

export fn table_get_entity_row(table_ptr: *const Table, entity: u32) i64 {
    return if (table_ptr.getEntityRow(entity)) |row| @as(i64, @intCast(row)) else -1;
}

export fn table_clear(table_ptr: *Table) void {
    table_ptr.clear();
}

export fn table_get_column_ptr(table_ptr: *Table, component_id: u32, row: usize) ?[*]u8 {
    const column = table_ptr.getColumn(component_id) orelse return null;
    return column.getPtr(row);
}

export fn table_get_column_ticks_ptr(table_ptr: *Table, component_id: u32, row: usize) ?*ComponentTicks {
    const column = table_ptr.getColumn(component_id) orelse return null;
    return column.getTicksPtr(row);
}

export fn table_get_entity(table_ptr: *const Table, row: usize) Entity {
    if (row >= table_ptr.entity_list.items.len) return Entity{ .index = 0, .generation = 0 };
    return Entity{ .index = table_ptr.entity_list.items[row], .generation = 0 };
}

export fn table_get_entity_list_ptr(table_ptr: *const Table) [*]const u32 {
    return table_ptr.entity_list.items.ptr;
}

export fn table_get_column_data_ptr(table_ptr: *const Table, component_id: u32) ?[*]u8 {
    const column = table_ptr.getColumnConst(component_id) orelse return null;
    return column.data.items.ptr;
}

export fn table_get_column_ticks_base_ptr(table_ptr: *const Table, component_id: u32) ?[*]ComponentTicks {
    const column = table_ptr.getColumnConst(component_id) orelse return null;
    return column.ticks.items.ptr;
}
