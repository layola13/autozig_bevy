//! 模块 3: State 示例
//! 演示 autozig-state 的状态管理功能

use autozig_ecs::plugin::App;

pub fn run_state_demo() {
    println!("\n{}", "=".repeat(60));
    println!("模块 3: State 示例");
    println!("{}", "=".repeat(60));
    
    // 状态系统概述
    println!("\n[1] 状态系统概述...");
    println!("  ✓ States trait: 定义状态类型");
    println!("  ✓ State<T>: 当前状态资源");
    println!("  ✓ NextState<T>: 下一个状态");
    println!("  ✓ StateRegistry: 状态注册表");
    
    // 状态转换
    println!("\n[2] 状态转换系统...");
    println!("  ✓ OnEnter<S>: 进入状态时运行");
    println!("  ✓ OnExit<S>: 退出状态时运行");
    println!("  ✓ OnTransition<S1, S2>: 状态转换时运行");
    println!("  ✓ StateTransitionEvent: 状态转换事件");
    
    // 状态条件
    println!("\n[3] 状态条件系统...");
    println!("  ✓ in_state(s): 检查当前状态");
    println!("  ✓ state_changed(): 检查状态是否变更");
    println!("  ✓ on_enter(s): 进入状态条件");
    println!("  ✓ on_exit(s): 退出状态条件");
    
    // 作用域实体
    println!("\n[4] 作用域实体系统...");
    println!("  ✓ StateScoped<S>: 状态作用域标记");
    println!("  ✓ DespawnOnExit<S>: 退出时销毁");
    println!("  ✓ DespawnOnEnter<S>: 进入时销毁");
    
    // 示例：游戏状态机
    println!("\n[5] 示例：游戏状态机...");
    println!("  GameState 状态流:");
    println!("    MainMenu → Loading → InGame → Paused → GameOver");
    println!("           ↑___________________________________|");
    
    // 状态插件
    println!("\n[6] 状态插件...");
    println!("  ✓ StatePlugin: 状态系统插件");
    println!("  ✓ AppStateExt: App扩展trait");
    println!("  ✓ 自动管理状态转换和清理");
    
    // 应用状态
    println!("\n[7] 应用状态示例...");
    let mut app = App::new();
    
    println!("  - 创建 App");
    println!("  - 可以添加 StatePlugin<GameState>");
    println!("  - 可以使用 add_state<GameState>()");
    println!("  - 系统可以使用 in_state() 条件运行");
    
    println!("\n[8] 状态转换示例...");
    println!("  典型用法:");
    println!("    app.add_systems(Update, game_logic.run_if(in_state(GameState::InGame)))");
    println!("    app.add_systems(OnEnter(GameState::Loading), setup_loading)");
    println!("    app.add_systems(OnExit(GameState::InGame), cleanup_game)");
    
    println!("\n模块 3 完成 ✓\n");
}