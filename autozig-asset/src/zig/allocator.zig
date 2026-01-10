const std = @import("std");
const builtin = @import("builtin");

// Cross-platform allocator (WASM-compatible)
// This module provides a unified allocator interface that works across
// both native and WASM targets without requiring manual conditionals
pub const g_allocator = if (builtin.cpu.arch.isWasm())
    std.heap.page_allocator // WASM: use page allocator (no libc)
else
    std.heap.c_allocator; // Native: use C allocator (best performance)

// Dummy export to ensure this file is included by autozig scanner
export fn _allocator_init() void {}
