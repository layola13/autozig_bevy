use autozig_state::prelude::*;

// 定义游戏状态
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    Menu,
    InGame,
    Paused,
}

impl States for GameState {}

fn main() {
    println!("=== AutoZig State 基础示例 ===\n");
    
    // 创建状态资源
    let mut current_state = State::new(GameState::Loading);
    let mut next_state = NextState::<GameState>::new();
    
    println!("【初始状态】");
    println!("当前状态: {:?}", current_state.get());
    
    // 转换到Menu
    println!("\n【转换 1: Loading → Menu】");
    next_state.set(GameState::Menu);
    println!("✓ 设置下一个状态: {:?}", GameState::Menu);
    
    if let Some(new_state) = next_state.take() {
        current_state.set(new_state.clone());
        println!("✓ 应用转换");
        println!("  当前状态: {:?}", current_state.get());
    }
    
    // 转换到InGame
    println!("\n【转换 2: Menu → InGame】");
    next_state.set(GameState::InGame);
    println!("✓ 设置下一个状态: {:?}", GameState::InGame);
    
    if let Some(new_state) = next_state.take() {
        current_state.set(new_state.clone());
        println!("✓ 应用转换");
        println!("  当前状态: {:?}", current_state.get());
    }
    
    // 转换到Paused
    println!("\n【转换 3: InGame → Paused】");
    next_state.set(GameState::Paused);
    println!("✓ 设置下一个状态: {:?}", GameState::Paused);
    
    if let Some(new_state) = next_state.take() {
        current_state.set(new_state.clone());
        println!("✓ 应用转换");
        println!("  当前状态: {:?}", current_state.get());
    }
    
    // 转换回InGame
    println!("\n【转换 4: Paused → InGame】");
    next_state.set(GameState::InGame);
    println!("✓ 设置下一个状态: {:?}", GameState::InGame);
    
    if let Some(new_state) = next_state.take() {
        current_state.set(new_state.clone());
        println!("✓ 应用转换");
        println!("  当前状态: {:?}", current_state.get());
    }
    
    println!("\n=== 所有状态转换完成! ===");
    println!("\n✨ State 系统特性:");
    println!("  ✓ States trait");
    println!("  ✓ State<S> 当前状态");
    println!("  ✓ NextState<S> 转换队列");
    println!("  ✓ 状态转换逻辑");
    println!("  ✓ 90% Zig + 10% Rust");
}
