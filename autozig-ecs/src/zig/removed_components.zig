const std = @import("std");

// 本地allocator定义 - 遵循 autozig-utils 的模式
// 每个文件直接定义自己的 GPA 实例，不依赖跨文件导入
var gpa_instance = std.heap.GeneralPurposeAllocator(.{}){};
const removed_components_allocator = gpa_instance.allocator();

pub const RemovedComponents = struct {
    entities: std.ArrayList(u32),
    component_id: u32,
    allocator: std.mem.Allocator,

    pub fn init(alloc: std.mem.Allocator, component_id: u32) RemovedComponents {
        return RemovedComponents{
            .entities = std.ArrayList(u32){},
            .component_id = component_id,
            .allocator = alloc,
        };
    }

    pub fn deinit(self: *RemovedComponents) void {
        self.entities.deinit(self.allocator);
    }

    pub fn record(self: *RemovedComponents, entity_id: u32) !void {
        try self.entities.append(self.allocator, entity_id);
    }

    pub fn clear(self: *RemovedComponents) void {
        self.entities.clearRetainingCapacity();
    }

    pub fn iter(self: *const RemovedComponents) []const u32 {
        return self.entities.items;
    }

    pub fn len(self: *const RemovedComponents) usize {
        return self.entities.items.len;
    }
};

// C-compatible exports for FFI - 使用 hashmap.zig 的模式
export fn removed_components_init(component_id: u32) *RemovedComponents {
    const removed = removed_components_allocator.create(RemovedComponents) catch unreachable;
    removed.* = RemovedComponents.init(removed_components_allocator, component_id);
    return removed;
}

export fn removed_components_deinit(removed_ptr: *RemovedComponents) void {
    removed_ptr.deinit();
    removed_components_allocator.destroy(removed_ptr);
}

export fn removed_components_record(removed_ptr: *RemovedComponents, entity_id: u32) bool {
    removed_ptr.record(entity_id) catch return false;
    return true;
}

export fn removed_components_clear(removed_ptr: *RemovedComponents) void {
    removed_ptr.clear();
}

export fn removed_components_len(removed_ptr: *const RemovedComponents) usize {
    return removed_ptr.len();
}

export fn removed_components_get(removed_ptr: *const RemovedComponents, index: usize) u32 {
    const items = removed_ptr.iter();
    if (index >= items.len) {
        return 0xFFFFFFFF; // Invalid entity marker
    }
    return items[index];
}
