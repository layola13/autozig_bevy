//! 模块 4: Time and Task 示例
//! 演示 autozig-time 和 autozig-tasks 的时间与任务功能

use autozig_time::*;
use autozig_tasks::prelude::*;

pub fn run_time_task_demo() {
    println!("\n{}", "=".repeat(60));
    println!("模块 4: Time and Task 示例");
    println!("{}", "=".repeat(60));
    
    // ========== Time 资源 ==========
    println!("\n[1] Time 资源...");
    let mut time = Time::new();
    println!("  ✓ Time 已创建");
    println!("  - delta_seconds(): {:.6}s", time.delta_seconds());
    println!("  - elapsed_seconds(): {:.6}s", time.elapsed_seconds());
    
    // 模拟时间更新
    time.set_delta(0.016); // 模拟 60 FPS
    println!("\n  模拟帧更新 (60 FPS):");
    println!("  - 设置 delta = 0.016s");
    
    // ========== Stopwatch ==========
    println!("\n[2] Stopwatch 秒表...");
    let mut stopwatch = Stopwatch::new();
    println!("  ✓ Stopwatch 已创建");
    
    // 模拟时间流逝
    stopwatch.tick(16_666_667); // 16.67ms in nanoseconds
    stopwatch.tick(16_666_667);
    stopwatch.tick(16_666_667);
    
    println!("  - 经过 3 帧 (~50ms)");
    println!("  - elapsed_secs(): {:.3}s", stopwatch.elapsed_secs());
    println!("  - is_paused(): {}", stopwatch.is_paused());
    
    // 暂停和恢复
    println!("\n  测试暂停/恢复:");
    stopwatch.pause();
    println!("  - 已暂停");
    stopwatch.tick(16_666_667);
    println!("  - tick() 调用 (不计时)");
    println!("  - elapsed_secs(): {:.3}s (未变)", stopwatch.elapsed_secs());
    
    stopwatch.unpause();
    println!("  - 已恢复");
    
    // 重置
    stopwatch.reset();
    println!("  - 已重置");
    println!("  - elapsed_secs(): {:.3}s", stopwatch.elapsed_secs());
    
    // ========== Timer ==========
    println!("\n[3] Timer 计时器...");
    
    // 一次性计时器
    println!("\n  3.1 一次性计时器:");
    let mut timer_once = Timer::from_seconds(1.0, TimerMode::Once);
    println!("  ✓ Timer (1.0s, Once) 已创建");
    
    for i in 1..=4 {
        timer_once.tick(300_000_000); // 300ms
        println!("  - 第 {} 次 tick (300ms)", i);
        println!("    percent: {:.1}%", timer_once.percent() * 100.0);
        println!("    finished: {}", timer_once.finished());
        if timer_once.just_finished() {
            println!("    ⚡ just_finished!");
        }
    }
    
    // 循环计时器
    println!("\n  3.2 循环计时器:");
    let mut timer_repeat = Timer::from_seconds(0.5, TimerMode::Repeating);
    println!("  ✓ Timer (0.5s, Repeating) 已创建");
    
    for i in 1..=5 {
        timer_repeat.tick(200_000_000); // 200ms
        println!("  - 第 {} 次 tick (200ms)", i);
        println!("    percent: {:.1}%", timer_repeat.percent() * 100.0);
        if timer_repeat.just_finished() {
            println!("    ⚡ just_finished! (循环触发)");
        }
    }
    
    // ========== TaskPool ==========
    println!("\n[4] TaskPool 任务池...");
    let pool = TaskPool::new();
    println!("  ✓ TaskPool 已创建");
    println!("  - 线程池用于并行任务");
    println!("  - 支持异步任务调度");
    println!("  - 自动管理工作线程");
    
    // 时间工具函数
    println!("\n[5] 时间工具函数...");
    let current_nanos = now_nanos();
    println!("  - now_nanos(): {}", current_nanos);
    
    let secs = 1.5f32;
    let nanos = secs_to_nanos(secs);
    let back = nanos_to_secs(nanos);
    println!("  - secs_to_nanos({:.1}s) = {} ns", secs, nanos);
    println!("  - nanos_to_secs({} ns) = {:.1}s", nanos, back);
    
    // 应用示例
    println!("\n[6] 实际应用示例...");
    println!("  典型用法:");
    println!("    • Time 作为资源在系统中获取帧时间");
    println!("    • Stopwatch 用于性能测量和调试");
    println!("    • Timer 用于游戏逻辑计时 (技能冷却、动画等)");
    println!("    • TaskPool 用于异步资源加载和并行计算");
    
    println!("\n模块 4 完成 ✓\n");
}