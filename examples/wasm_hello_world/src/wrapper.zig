// AutoZig WASM Hello World - 纯 Zig 实现
//
// 演示 AutoZig 的 TypeScript 绑定生成功能

const std = @import("std");

/// 运行 Hello World Demo
/// 这是演示用的简化版本
export fn run_hello_world() void {
    // 在 WASM 环境中，这个函数会被 JavaScript 调用
    // 实际的日志输出需要通过 JavaScript 的 console.log
}

/// 获取系统计数
/// 返回演示用的固定值
export fn get_system_count() u32 {
    return 5; // 演示：5 个系统
}

/// 运行多次迭代
/// 演示参数传递
export fn run_multiple_times(times: u32) void {
    _ = times; // 演示：接收参数但不做实际处理
}
