use autozig_state::prelude::*;

// 定义游戏菜单状态
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MenuState {
    MainMenu,
    Settings,
    Credits,
    InGame,
}

impl States for MenuState {}

fn main() {
    println!("=== AutoZig State 游戏菜单示例 ===\n");
    
    // 创建状态
    let mut current_state = State::new(MenuState::MainMenu);
    let mut next_state = NextState::<MenuState>::new();
    
    println!("【游戏启动】");
    println!("当前状态: {:?}\n", current_state.get());
    
    // 模拟用户操作序列
    let operations = vec![
        ("进入设置", MenuState::Settings),
        ("返回主菜单", MenuState::MainMenu),
        ("查看制作人员", MenuState::Credits),
        ("返回主菜单", MenuState::MainMenu),
        ("开始游戏", MenuState::InGame),
    ];
    
    for (i, (action, target_state)) in operations.iter().enumerate() {
        println!("【操作 {}】: {}", i + 1, action);
        
        // 触发OnExit
        println!("  → OnExit({:?})", current_state.get());
        
        // 设置下一个状态
        next_state.set(target_state.clone());
        
        // 应用转换
        if let Some(event) = apply_state_transition(Some(&mut current_state), &mut next_state) {
            println!("  → StateTransitionEvent:");
            if let Some(exited) = &event.exited {
                println!("      exited: {:?}", exited);
            }
            if let Some(entered) = &event.entered {
                println!("      entered: {:?}", entered);
            }
        }
        
        // 触发OnEnter
        println!("  → OnEnter({:?})", current_state.get());
        println!("  ✓ 当前状态: {:?}\n", current_state.get());
    }
    
    println!("=== 菜单导航完成! ===");
    println!("\n✨ State 转换系统特性:");
    println!("  ✓ OnEnter<S> 调度");
    println!("  ✓ OnExit<S> 调度");
    println!("  ✓ StateTransitionEvent<S>");
    println!("  ✓ apply_state_transition 函数");
    println!("  ✓ 状态机导航");
}
