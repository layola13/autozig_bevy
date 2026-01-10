//! AutoZig WASM 3D Demo - 简化版 Zig 实现
//!
//! 不使用 std.debug.print 以避免 wasm64-freestanding 的 POSIX 调用问题

const std = @import("std");

/// 初始化 demo
export fn demo_init() void {
    // 初始化完成（无日志输出）
}

/// 简单测试函数
export fn test_simple() u32 {
    // 简单的内存测试
    var test_value: u32 = 42;
    test_value += 1;

    return test_value; // 应该返回 43
}

/// 获取版本号
export fn get_version() u32 {
    return 100; // v1.0.0
}
