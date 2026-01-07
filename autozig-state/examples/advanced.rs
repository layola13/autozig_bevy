use autozig_state::prelude::*;

// 定义游戏状态
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    MainMenu,
    Playing,
    Paused,
}

impl States for GameState {}

fn main() {
    println!("=== AutoZig State 高级特性示例 ===\n");
    
    // ========== 1. in_state 运行条件 ==========
    println!("【1. in_state 运行条件】");
    let mut state = State::new(GameState::MainMenu);
    
    // 创建条件
    let is_main_menu = in_state(GameState::MainMenu);
    let is_playing = in_state(GameState::Playing);
    let is_paused = in_state(GameState::Paused);
    
    println!("当前状态: {:?}", state.get());
    println!("  in_state(MainMenu): {}", is_main_menu(&state));
    println!("  in_state(Playing): {}", is_playing(&state));
    
    // 切换到Playing
    state.set(GameState::Playing);
    println!("\n切换到 Playing");
    println!("  in_state(MainMenu): {}", is_main_menu(&state));
    println!("  in_state(Playing): {}", is_playing(&state));
    
    // ========== 2. state_changed 条件 ==========
    println!("\n【2. state_changed 条件】");
    let mut changed_detector = state_changed::<GameState>();
    
    println!("首次检查: {}", changed_detector(Some(&state)));
    println!("第二次检查（未改变）: {}", changed_detector(Some(&state)));
    
    state.set(GameState::Paused);
    println!("\n切换到 Paused");
    println!("检查变化: {}", changed_detector(Some(&state)));
    println!("再次检查（未改变）: {}", changed_detector(Some(&state)));
    
    // ========== 3. on_enter/on_exit 条件 ==========
    println!("\n【3. on_enter/on_exit 条件】");
    let mut enter_playing = on_enter(GameState::Playing);
    let mut exit_paused = on_exit(GameState::Paused);
    
    state.set(GameState::MainMenu);
    println!("状态: MainMenu");
    println!("  on_exit(Paused): {}", exit_paused(Some(&state)));
    
    state.set(GameState::Playing);
    println!("\n状态: Playing");
    println!("  on_enter(Playing): {}", enter_playing(Some(&state)));
    println!("  on_exit(Paused): {}", exit_paused(Some(&state)));
    
    // ========== 4. StateScoped 标记 ==========
    println!("\n【4. StateScoped 实体标记】");
    let scoped_playing = DespawnOnExit::new(GameState::Playing);
    let scoped_menu = DespawnOnEnter::new(GameState::MainMenu);
    
    println!("✓ 创建 DespawnOnExit(Playing)");
    println!("  → 实体将在退出Playing状态时被销毁");
    println!("✓ 创建 DespawnOnEnter(MainMenu)");
    println!("  → 实体将在进入MainMenu状态时被销毁");
    
    // ========== 模拟系统调度 ==========
    println!("\n【5. 模拟基于条件的系统调度】");
    state.set(GameState::Playing);
    
    println!("当前状态: {:?}", state.get());
    
    // 模拟不同系统的运行条件
    fn update_game(state: &State<GameState>) {
        if in_state(GameState::Playing)(state) {
            println!("  → update_game 系统运行（仅在Playing状态）");
        }
    }
    
    fn render_menu(state: &State<GameState>) {
        if in_state(GameState::MainMenu)(state) {
            println!("  → render_menu 系统运行（仅在MainMenu状态）");
        }
    }
    
    fn pause_overlay(state: &State<GameState>) {
        if in_state(GameState::Paused)(state) {
            println!("  → pause_overlay 系统运行（仅在Paused状态）");
        }
    }
    
    update_game(&state);
    render_menu(&state);
    pause_overlay(&state);
    
    state.set(GameState::MainMenu);
    println!("\n切换到 MainMenu:");
    update_game(&state);
    render_menu(&state);
    pause_overlay(&state);
    
    state.set(GameState::Paused);
    println!("\n切换到 Paused:");
    update_game(&state);
    render_menu(&state);
    pause_overlay(&state);
    
    println!("\n=== 所有高级特性演示完成! ===");
    println!("\n✨ 高级特性:");
    println!("  ✓ in_state<S> 运行条件");
    println!("  ✓ state_changed<S> 检测");
    println!("  ✓ on_enter<S> 条件");
    println!("  ✓ on_exit<S> 条件");
    println!("  ✓ DespawnOnExit<S> 标记");
    println!("  ✓ DespawnOnEnter<S> 标记");
    println!("  ✓ 基于条件的系统调度");
}
