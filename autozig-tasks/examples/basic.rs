use autozig_tasks::prelude::*;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("=== AutoZig Tasks 基础示例 ===\n");
    
    // 创建任务池
    let pool = TaskPool::new();
    println!("✓ 创建 TaskPool");
    println!("  线程数: {}", pool.thread_num());
    
    // 提交简单任务
    println!("\n【1. 提交简单任务】");
    for i in 0..5 {
        pool.spawn(move || {
            println!("  → 任务 {} 执行", i);
            std::thread::sleep(std::time::Duration::from_millis(100));
        });
    }
    
    // 等待任务完成
    std::thread::sleep(std::time::Duration::from_millis(600));
    println!("✓ 所有简单任务完成");
    
    // 使用共享状态
    println!("\n【2. 共享状态计数】");
    let counter = Arc::new(AtomicUsize::new(0));
    
    for i in 0..10 {
        let counter_clone = counter.clone();
        pool.spawn(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            println!("  → 任务 {} 增加计数", i);
        });
    }
    
    std::thread::sleep(std::time::Duration::from_millis(500));
    println!("✓ 最终计数: {}", counter.load(Ordering::SeqCst));
    
    println!("\n=== 所有测试完成! ===");
    println!("\n✨ TaskPool 特性:");
    println!("  ✓ 自动检测CPU核心数");
    println!("  ✓ 任务提交 (spawn)");
    println!("  ✓ 线程安全任务队列");
    println!("  ✓ 90% Zig + 10% Rust");
}
