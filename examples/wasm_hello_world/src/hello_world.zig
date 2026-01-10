// AutoZig WASM Hello World - Zig Implementation
const std = @import("std");

// 导出函数：创建并运行 Hello World demo
export fn run_hello_world() void {
    // 在 WASM 环境中，我们使用 console.log 输出
    // 这些字符串会被 JavaScript 捕获并显示
    _ = js_console_log("[System 1] 👋 Hello World from AutoZig-Bevy!");
    _ = js_console_log("[System 2] ⚙️  使用框架: AutoZig-ECS");
    _ = js_console_log("[System 3] 🔢 执行计数: 1");
    _ = js_console_log("[System 4] 🎮 Update: 更新游戏状态");
    _ = js_console_log("[System 5] 🎨 Render: 渲染当前帧");
}

// 导出函数：获取系统计数
export fn get_system_count() u32 {
    return 5; // 我们硬编码了 5 个系统
}

// 导出函数：运行多次迭代
export fn run_multiple_times(times: u32) void {
    var i: u32 = 1;
    while (i <= times) : (i += 1) {
        const msg = std.fmt.allocPrint(
            std.heap.page_allocator,
            "━━━ 迭代 {} ━━━",
            .{i},
        ) catch return;
        defer std.heap.page_allocator.free(msg);

        _ = js_console_log(msg.ptr);

        const exec_msg = std.fmt.allocPrint(
            std.heap.page_allocator,
            "  迭代 {} 执行",
            .{i},
        ) catch return;
        defer std.heap.page_allocator.free(exec_msg);

        _ = js_console_log(exec_msg.ptr);
    }
}

// JavaScript 互操作：console.log
extern fn js_console_log(msg_ptr: [*]const u8) void;
