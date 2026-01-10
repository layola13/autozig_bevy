const std = @import("std");

/// WASM 兼容的全局分配器
/// 在 WASM 环境使用 FixedBufferAllocator，在其他环境使用 page_allocator
pub fn getGlobalAllocator() std.mem.Allocator {
    const builtin = @import("builtin");
    if (builtin.target.cpu.arch.isWasm()) {
        // WASM 环境：使用固定大小的缓冲区分配器
        // 注意：这是一个简化的实现，实际生产环境可能需要更复杂的内存管理
        const State = struct {
            var buffer: [1024 * 1024 * 10]u8 = undefined; // 10MB 缓冲区
            var fba = std.heap.FixedBufferAllocator.init(&buffer);
            var initialized = false;
        };
        
        if (!State.initialized) {
            State.fba = std.heap.FixedBufferAllocator.init(&State.buffer);
            State.initialized = true;
        }
        
        return State.fba.allocator();
    } else {
        // 非 WASM 环境：使用标准 page_allocator
        return std.heap.page_allocator;
    }
}
