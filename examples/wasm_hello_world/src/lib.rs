//! AutoZig-Bevy WASM Hello World Demo
//! 
//! 混合架构：
//! 1. AutoZig include_zig!: 生成 TypeScript 绑定 (bindings.d.ts, bindings.js)
//! 2. wasm-bindgen: 实现真正的 autozig-ecs App+System 逻辑
//! 3. JavaScript 可以选择调用 AutoZig 绑定或 wasm-bindgen 绑定

use autozig::include_zig;
use wasm_bindgen::prelude::*;
use autozig_ecs::prelude::*;

// Step 1: 使用 AutoZig 生成 TypeScript 绑定
// 这会生成 bindings.d.ts 和 bindings.js
include_zig!("src/wrapper.zig", {
    #[autozig(strategy = "dual")]
    fn run_hello_world();
    
    #[autozig(strategy = "dual")]
    fn get_system_count() -> u32;
    
    #[autozig(strategy = "dual")]
    fn run_multiple_times(times: u32);
});

// Step 2: 使用 wasm-bindgen 实现真正的 ECS 逻辑
// 这些函数会被导出到 JavaScript

/// WASM 入口点
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    web_sys::console::log_1(&"🚀 AutoZig-Bevy WASM Hello World Demo".into());
    web_sys::console::log_1(&"===================================".into());
}

/// 运行 Hello World 应用 - wasm-bindgen 版本
#[wasm_bindgen]
pub fn run_hello_world_ecs() {
    use web_sys::console;
    
    console::log_1(&"\n📦 创建 AutoZig-ECS App...".into());
    
    let mut app = App::new();
    
    // 注册系统
    app.add_systems(|| {
        web_sys::console::log_1(&"[System 1] 👋 Hello World from AutoZig-Bevy!".into());
    });
    
    let framework = "AutoZig-ECS";
    app.add_systems(move || {
        web_sys::console::log_1(&format!("[System 2] ⚙️  使用框架: {}", framework).into());
    });
    
    let counter = std::cell::Cell::new(0);
    app.add_systems(move || {
        counter.set(counter.get() + 1);
        web_sys::console::log_1(&format!("[System 3] 🔢 执行计数: {}", counter.get()).into());
    });
    
    app.add_systems(|| {
        web_sys::console::log_1(&"[System 4] 🎮 Update: 更新游戏状态".into());
    });
    
    app.add_systems(|| {
        web_sys::console::log_1(&"[System 5] 🎨 Render: 渲染当前帧".into());
    });
    
    let system_count = app.closure_system_count();
    console::log_1(&format!("\n✅ 已注册 {} 个系统", system_count).into());
    
    console::log_1(&"\n🔄 执行系统:".into());
    console::log_1(&"─────────────────".into());
    
    app.run();
    
    console::log_1(&"─────────────────".into());
    console::log_1(&"\n✅ Hello World Demo 完成!".into());
    console::log_1(&"💡 所有系统已成功执行".into());
}

/// 获取系统计数 - wasm-bindgen 版本
#[wasm_bindgen]
pub fn get_system_count_ecs() -> usize {
    let mut app = App::new();
    app.add_systems(|| {})
        .add_systems(|| {})
        .add_systems(|| {});
    app.closure_system_count()
}

/// 运行多次迭代 - wasm-bindgen 版本
#[wasm_bindgen]
pub fn run_multiple_times_ecs(times: u32) {
    use web_sys::console;
    
    console::log_1(&format!("\n🔁 运行 {} 次迭代:", times).into());
    
    for i in 1..=times {
        console::log_1(&format!("\n━━━ 迭代 {} ━━━", i).into());
        
        let mut app = App::new();
        let iteration = i;
        app.add_systems(move || {
            web_sys::console::log_1(&format!("  迭代 {} 执行", iteration).into());
        });
        
        app.run();
    }
    
    console::log_1(&format!("\n✅ {} 次迭代全部完成!", times).into());
}