const std = @import("std");

// 事件队列 - 双缓冲实现
pub const EventQueue = struct {
    buffers: [2]std.ArrayList(u8),
    write_idx: u8,
    stride: usize, // 每个事件的字节大小
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator, event_size: usize) !*EventQueue {
        const queue = try allocator.create(EventQueue);
        queue.* = EventQueue{
            .buffers = [2]std.ArrayList(u8){
                std.ArrayList(u8){},
                std.ArrayList(u8){},
            },
            .write_idx = 0,
            .stride = event_size,
            .allocator = allocator,
        };
        return queue;
    }
    
    pub fn deinit(self: *EventQueue) void {
        self.buffers[0].deinit(self.allocator);
        self.buffers[1].deinit(self.allocator);
        self.allocator.destroy(self);
    }
    
    // 写入事件（写到当前写缓冲）
    pub fn push(self: *EventQueue, data: [*]const u8) !void {
        const buf = &self.buffers[self.write_idx];
        try buf.appendSlice(self.allocator, data[0..self.stride]);
    }
    
    // 交换缓冲区（每帧调用）
    pub fn swap(self: *EventQueue) void {
        self.write_idx ^= 1; // 0 <-> 1 切换
        self.buffers[self.write_idx].clearRetainingCapacity(); // 清空新的写缓冲
    }
    
    // 获取读缓冲（上一帧写入的）
    pub fn get_reader(self: *const EventQueue) []const u8 {
        const read_idx = self.write_idx ^ 1;
        return self.buffers[read_idx].items;
    }
    
    // 获取读缓冲中的事件数量
    pub fn get_event_count(self: *const EventQueue) usize {
        const read_idx = self.write_idx ^ 1;
        return self.buffers[read_idx].items.len / self.stride;
    }
    
    // 清空所有缓冲
    pub fn clear(self: *EventQueue) void {
        self.buffers[0].clearRetainingCapacity();
        self.buffers[1].clearRetainingCapacity();
    }
};

// 全局allocator在entity.zig中定义，合并后直接可用

// Exported C API
export fn event_queue_create(event_size: usize) ?*EventQueue {
    return EventQueue.init(g_allocator, event_size) catch null;
}

export fn event_queue_destroy(queue: *EventQueue) void {
    queue.deinit();
}

export fn event_queue_push(queue: *EventQueue, data_ptr: [*]const u8) bool {
    queue.push(data_ptr) catch return false;
    return true;
}

export fn event_queue_swap(queue: *EventQueue) void {
    queue.swap();
}

export fn event_queue_get_reader(queue: *const EventQueue, out_ptr: *[*]const u8, out_len: *usize) void {
    const reader = queue.get_reader();
    out_ptr.* = reader.ptr;
    out_len.* = reader.len;
}

export fn event_queue_get_event_count(queue: *const EventQueue) usize {
    return queue.get_event_count();
}

export fn event_queue_clear(queue: *EventQueue) void {
    queue.clear();
}
