const std = @import("std");
const Table = @import("../../../zig/table.zig").Table;

const Entity = extern struct {
    id: u32,
    ver: u32,
};

/// Fetch core structure - manages query data retrieval
pub const FetchCore = extern struct {
    // Pointer to the base data array (e.g., Table column or SparseSet data)
    data_ptr: ?[*]const u8,
    // Size of each component/element in bytes
    element_size: usize,
    // Stride between elements (usually equal to element_size, but can differ for alignment)
    stride: usize,

    pub fn init() *FetchCore {
        const ptr = std.heap.c_allocator.create(FetchCore) catch @panic("OOM");
        ptr.* = .{
            .data_ptr = null,
            .element_size = 0,
            .stride = 0,
        };
        return ptr;
    }

    pub fn deinit(self: *FetchCore) void {
        std.heap.c_allocator.destroy(self);
    }

    pub fn configure(self: *FetchCore, data: [*]const u8, size: usize, stride: usize) void {
        self.data_ptr = data;
        self.element_size = size;
        self.stride = stride;
    }

    // Get pointer to component data for a specific index (table row or dense index)
    pub fn get_at(self: *FetchCore, index: usize) [*]const u8 {
        const base = self.data_ptr orelse @panic("FetchCore not configured");
        if (self.element_size == 0) return base;
        return base + (index * self.stride);
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

export fn fetch_configure(fetch: ?*FetchCore, data: [*]const u8, size: usize, stride: usize) void {
    if (fetch) |ptr| {
        ptr.configure(data, size, stride);
    }
}

export fn fetch_get_at(fetch: ?*FetchCore, index: usize) ?[*]const u8 {
    if (fetch) |ptr| {
        return ptr.get_at(index);
    }
    return null;
}

export fn fetch_set_table(fetch: ?*FetchCore, table: ?*Table, component_id: u32) void {
    if (fetch) |f_ptr| {
        if (table) |t_ptr| {
            if (t_ptr.getColumn(component_id)) |col| {
                f_ptr.configure(col.data.items.ptr, col.item_size, col.item_size);
            }
        }
    }
}
