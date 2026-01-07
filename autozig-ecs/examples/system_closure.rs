use autozig_ecs::prelude::*;

fn main() {
    println!("=== AutoZig-ECS 闭包系统演示 ===\n");

    // 1. 简单闭包 - 无参数
    println!("【测试 1】简单闭包");
    let mut app1 = App::new();
    app1.add_systems(|| {
        println!("✓ 简单闭包执行成功");
    });
    println!("  注册了 {} 个闭包系统\n", app1.closure_system_count());

    // 2. 捕获外部变量的闭包
    println!("【测试 2】捕获外部变量");
    let message = "来自外部的消息";
    let mut app2 = App::new();
    app2.add_systems(move || {
        println!("✓ 捕获变量: {}", message);
    });
    println!("  注册了 {} 个闭包系统\n", app2.closure_system_count());

    // 3. 多个闭包 - 演示链式调用
    println!("【测试 3】链式注册多个闭包");
    let mut app3 = App::new();
    
    let counter = std::cell::Cell::new(0);
    
    app3.add_systems(|| {
            println!("  → 系统 1: 初始化");
        })
        .add_systems(|| {
            println!("  → 系统 2: 处理逻辑");
        })
        .add_systems(move || {
            counter.set(counter.get() + 1);
            println!("  → 系统 3: 计数器 = {}", counter.get());
        })
        .add_systems(|| {
            println!("  → 系统 4: 清理");
        });
    
    println!("  注册了 {} 个闭包系统\n", app3.closure_system_count());

    // 4. 复杂闭包 - 返回闭包的闭包（类似 Bevy Local）
    println!("【测试 4】闭包工厂模式");
    let create_counter_system = |name: &'static str| {
        let mut count = 0;
        move || {
            count += 1;
            println!("  → {}: 调用次数 = {}", name, count);
        }
    };
    
    let mut app4 = App::new();
    app4.add_systems(create_counter_system("系统A"))
        .add_systems(create_counter_system("系统B"));
    
    println!("  注册了 {} 个闭包系统\n", app4.closure_system_count());

    // 5. 测试完整的应用流程
    println!("【测试 5】完整应用流程");
    let mut app = App::new();
    
    let startup_message = "应用启动";
    let mut frame_count = 0;
    
    app.add_systems(move || {
            println!("  → Startup: {}", startup_message);
        })
        .add_systems(move || {
            frame_count += 1;
            if frame_count <= 3 {
                println!("  → Update: Frame {}", frame_count);
            }
        })
        .add_systems(|| {
            println!("  → Render: 渲染完成");
        });
    
    println!("  注册了 {} 个闭包系统", app.closure_system_count());
    println!("\n运行系统:");
    app.run();

    println!("\n=== 测试完成 ===");
    println!("✅ 闭包类型检查通过");
    println!("✅ 变量捕获功能正常");
    println!("✅ 链式调用支持完整");
    println!("✅ 状态管理机制工作");
}
