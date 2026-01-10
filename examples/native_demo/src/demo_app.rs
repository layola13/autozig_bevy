//! 模块 0: App 示例
//! 演示 autozig-app 和 autozig-ecs 的 App 架构

use autozig_ecs::prelude::*;

pub fn run_app_demo() {
    println!("\n{}", "=".repeat(60));
    println!("模块 0: App 示例");
    println!("{}", "=".repeat(60));
    
    // 创建 App
    println!("\n[1] 创建 App 实例...");
    let mut app = App::new();
    
    // 添加系统
    println!("[2] 添加系统...");
    
    app.add_systems(|| {
        println!("  ✓ System 1: 初始化系统");
    });
    
    app.add_systems(|| {
        println!("  ✓ System 2: 更新系统");
    });
    
    app.add_systems(|| {
        println!("  ✓ System 3: 渲染系统");
    });
    
    let system_count = app.closure_system_count();
    println!("[3] 已注册 {} 个系统", system_count);
    
    // 运行 App
    println!("\n[4] 运行 App...");
    app.run();
    
    println!("\n[5] App 示例完成 ✓");
    
    // 演示插件系统
    println!("\n[6] 演示插件系统...");
    let mut app2 = App::new();
    
    app2.add_plugin(CorePlugin);
    app2.add_plugin(TimePlugin);
    
    println!("  - 已添加 CorePlugin");
    println!("  - 已添加 TimePlugin");
    println!("  - 插件数量: {}", app2.plugin_count());
    
    app2.finish();
    
    println!("\n模块 0 完成 ✓\n");
}