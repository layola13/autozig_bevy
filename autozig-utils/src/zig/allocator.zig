const std = @import("std");
const builtin = @import("builtin");

// 跨平台 allocator (WASM兼容)
// 根据目标平台选择最佳allocator：
// - WASM: 使用 page_allocator (无需libc，避免POSIX函数)
// - Native: 使用 c_allocator (最佳性能)
pub const g_allocator = if (builtin.cpu.arch.isWasm())
    std.heap.page_allocator // WASM: use page allocator (no libc)
else
    std.heap.c_allocator; // Native: use C allocator (best performance)

// Dummy export to ensure this file is included by autozig scanner
export fn _allocator_init() void {}
