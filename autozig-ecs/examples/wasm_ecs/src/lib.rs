use wasm_bindgen::prelude::*;
use autozig_ecs::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"=== AutoZig-ECS WASM 闭包系统演示 ===".into());

    // 创建应用
    let mut app = App::new();

    // 1. 简单闭包系统
    app.add_systems(|| {
        console::log_1(&"[系统 1] 简单闭包执行".into());
    });

    // 2. 捕获变量的闭包
    let message = "WASM 环境中的消息";
    app.add_systems(move || {
        console::log_1(&format!("[系统 2] 捕获变量: {}", message).into());
    });

    // 3. 计数器系统
    let counter = std::cell::Cell::new(0);
    app.add_systems(move || {
        counter.set(counter.get() + 1);
        console::log_1(&format!("[系统 3] 帧计数: {}", counter.get()).into());
    });

    // 4. 游戏循环模拟
    app.add_systems(|| {
        console::log_1(&"[系统 4] 更新游戏状态".into());
    })
    .add_systems(|| {
        console::log_1(&"[系统 5] 渲染画面".into());
    });

    let system_count = app.closure_system_count();
    console::log_1(&format!("✓ 已注册 {} 个闭包系统", system_count).into());
    
    console::log_1(&"\n执行系统:".into());
    app.run();

    console::log_1(&"\n=== WASM 测试完成 ===".into());
    console::log_1(&"✅ 闭包系统在 WASM 中正常工作!".into());
}

// 导出给 JavaScript 调用的函数
#[wasm_bindgen]
pub fn run_closure_demo() {
    main();
}

#[wasm_bindgen]
pub fn test_closure_systems() -> usize {
    let mut app = App::new();
    
    // 注册一些测试系统
    app.add_systems(|| {})
        .add_systems(|| {})
        .add_systems(|| {});
    
    app.closure_system_count()
}
