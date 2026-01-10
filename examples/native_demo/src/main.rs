//! AutoZig-Bevy Native Target Demo
//! 
//! 这是一个完整的 native target 示例，展示 autozig_bevy 的所有核心功能。
//! 
//! 包含模块：
//! - 模块 0: App - 应用架构和插件系统
//! - 模块 1: ECS - Entity-Component-System 架构
//! - 模块 2: Math - 数学库 (向量、矩阵、几何)
//! - 模块 3: State - 状态管理系统
//! - 模块 4: Time & Task - 时间系统和任务池
//! - 模块 5: JSON - 高性能 JSON 解析
//! 
//! 编译运行:
//! ```bash
//! cargo run --bin native_demo
//! ```

mod demo_app;
mod demo_ecs;
mod demo_math;
mod demo_state;
mod demo_time_task;
mod demo_json;

use demo_app::run_app_demo;
use demo_ecs::run_ecs_demo;
use demo_math::run_math_demo;
use demo_state::run_state_demo;
use demo_time_task::run_time_task_demo;
use demo_json::run_json_demo;

fn print_banner() {
    println!("\n");
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                                                               ║");
    println!("║            AutoZig-Bevy Native Target Demo                    ║");
    println!("║                                                               ║");
    println!("║  高性能游戏引擎核心功能展示 (Zig + Rust)                      ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\n");
    println!("🎯 Target: Native (非 WASM)");
    println!("🦀 Language: Rust + Zig");
    println!("⚡ Performance: SIMD 优化");
    println!("📦 Dependencies: 仅 autozig_bevy crates");
    println!("\n");
}

fn print_menu() {
    println!("\n{}", "=".repeat(60));
    println!("演示菜单");
    println!("{}", "=".repeat(60));
    println!("  0. 运行所有模块 (推荐)");
    println!("  1. 模块 0: App & Plugin System");
    println!("  2. 模块 1: ECS Architecture");
    println!("  3. 模块 2: Math Library");
    println!("  4. 模块 3: State Management");
    println!("  5. 模块 4: Time & Task System");
    println!("  6. 模块 5: JSON Parsing");
    println!("  q. 退出");
    println!("{}\n", "=".repeat(60));
}

fn run_all_demos() {
    println!("\n🚀 运行所有演示模块...\n");
    
    // 模块 0: App
    run_app_demo();
    
    // 模块 1: ECS
    run_ecs_demo();
    
    // 模块 2: Math
    run_math_demo();
    
    // 模块 3: State
    run_state_demo();
    
    // 模块 4: Time & Task
    run_time_task_demo();
    
    // 模块 5: JSON
    run_json_demo();
    
    println!("\n{}", "=".repeat(60));
    println!("✅ 所有模块演示完成！");
    println!("{}\n", "=".repeat(60));
}

fn print_summary() {
    println!("\n");
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                      演示总结                                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!("\n已演示的 AutoZig-Bevy 核心功能:");
    println!("\n✓ App & Plugin System");
    println!("  - App 应用架构");
    println!("  - Plugin 插件系统");
    println!("  - System 系统调度");
    println!("\n✓ ECS Architecture");
    println!("  - Entity 实体管理");
    println!("  - Component 组件系统");
    println!("  - Query 查询系统");
    println!("  - Resource 资源管理");
    println!("  - Event 事件系统");
    println!("\n✓ Math Library");
    println!("  - Vec2/Vec3/Vec4 向量运算");
    println!("  - Mat2/Mat3/Mat4 矩阵运算");
    println!("  - Quat 四元数旋转");
    println!("  - 几何图元和边界盒");
    println!("  - 曲线和变换系统");
    println!("\n✓ State Management");
    println!("  - State<T> 状态管理");
    println!("  - OnEnter/OnExit 转换");
    println!("  - StateScoped 作用域");
    println!("\n✓ Time & Task System");
    println!("  - Time 资源");
    println!("  - Stopwatch 秒表");
    println!("  - Timer 计时器");
    println!("  - TaskPool 任务池");
    println!("\n✓ JSON Parsing");
    println!("  - SIMD 优化解析");
    println!("  - 零依赖设计");
    println!("  - json! 宏支持");
    println!("\n{}", "=".repeat(60));
    println!("🎉 AutoZig-Bevy Native Demo 完成!");
    println!("{}\n", "=".repeat(60));
}

fn main() {
    print_banner();
    
    // 自动运行所有演示（非交互模式）
    println!("🎬 自动运行所有演示模块...\n");
    println!("按 Ctrl+C 可以随时退出\n");
    
    // 等待用户确认
    println!("按 Enter 键开始演示...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok();
    
    // 运行所有演示
    run_all_demos();
    
    // 打印总结
    print_summary();
    
    println!("感谢使用 AutoZig-Bevy!");
    println!("项目地址: https://github.com/your-repo/autozig\n");
}

// 备选：交互式菜单模式 (可选)
#[allow(dead_code)]
fn interactive_mode() {
    use std::io::{self, Write};
    
    print_banner();
    
    loop {
        print_menu();
        print!("请选择 (0-6, q): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        let choice = input.trim();
        
        match choice {
            "0" => run_all_demos(),
            "1" => run_app_demo(),
            "2" => run_ecs_demo(),
            "3" => run_math_demo(),
            "4" => run_state_demo(),
            "5" => run_time_task_demo(),
            "6" => run_json_demo(),
            "q" | "Q" => {
                println!("\n👋 再见!\n");
                break;
            }
            _ => {
                println!("\n❌ 无效选择，请重试。\n");
            }
        }
    }
}