const std = @import("std");
const table_mod = @import("../../../zig/table.zig");
const Table = table_mod.Table;
const change_detection = @import("../../../zig/change_detection.zig");
const ComponentTicks = change_detection.ComponentTicks;

const Entity = extern struct {
    id: u32,
    ver: u32,
};

/// Fetch core structure - manages query data retrieval
pub const FetchCore = extern struct {
    // Pointer to the base data array (e.g., Table column or SparseSet data)
    data_ptr: ?[*]const u8,
    // Pointer to component ticks (for change detection)
    ticks_ptr: ?[*]const ComponentTicks,
    // Size of each component/element in bytes
    element_size: usize,
    // Stride between elements (usually equal to element_size, but can differ for alignment)
    stride: usize,

    pub fn init() *FetchCore {
        const ptr = std.heap.c_allocator.create(FetchCore) catch @panic("OOM");
        ptr.* = .{
            .data_ptr = null,
            .ticks_ptr = null,
            .element_size = 0,
            .stride = 0,
        };
        return ptr;
    }

    pub fn deinit(self: *FetchCore) void {
        std.heap.c_allocator.destroy(self);
    }

    pub fn configure(self: *FetchCore, data: [*]const u8, ticks: [*]const ComponentTicks, size: usize, stride: usize) void {
        self.data_ptr = data;
        self.ticks_ptr = ticks;
        self.element_size = size;
        self.stride = stride;
    }

    // Get pointer to component data for a specific index (table row or dense index)
    pub fn get_at(self: *FetchCore, index: usize) ?[*]const u8 {
        const base = self.data_ptr orelse return null;
        if (self.element_size == 0) return base;
        return base + (index * self.stride);
    }

    // Get pointer to component ticks for a specific index
    pub fn get_ticks_at(self: *FetchCore, index: usize) ?*ComponentTicks {
        const ticks = self.ticks_ptr orelse return null;
        // We return a pointer to the specific ComponentTicks struct
        return @ptrCast(@as([*]ComponentTicks, @constCast(ticks)) + index);
    }
};

export fn fetch_create() *FetchCore {
    return FetchCore.init();
}

export fn fetch_destroy(fetch: ?*FetchCore) void {
    if (fetch) |ptr| {
        ptr.deinit();
    }
}

export fn fetch_configure(fetch: ?*FetchCore, data: [*]const u8, ticks: [*]const ComponentTicks, size: usize, stride: usize) void {
    if (fetch) |ptr| {
        ptr.configure(data, ticks, size, stride);
    }
}

export fn fetch_get_at(fetch: ?*FetchCore, index: usize) ?[*]const u8 {
    if (fetch) |ptr| {
        return ptr.get_at(index);
    }
    return null;
}

export fn fetch_get_ticks_at(fetch: ?*FetchCore, index: usize) ?*ComponentTicks {
    if (fetch) |ptr| {
        return ptr.get_ticks_at(index);
    }
    return null;
}

export fn fetch_set_table(fetch: ?*FetchCore, table: ?*Table, component_id: u32) void {
    if (fetch) |f_ptr| {
        if (table) |t_ptr| {
            if (t_ptr.getColumn(component_id)) |col| {
                f_ptr.configure(col.data.items.ptr, col.ticks.items.ptr, col.item_size, col.item_size);
            } else {
                f_ptr.data_ptr = null;
                f_ptr.ticks_ptr = null;
                f_ptr.element_size = 0;
                f_ptr.stride = 0;
            }
        }
    }
}
