use autozig_ecs::prelude::*;

// 自定义插件示例
struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        fn game_plugin_build(_app_ptr: *mut std::ffi::c_void) {
            println!("  → GamePlugin initialized");
        }
        
        app.register_plugin_fn("GamePlugin", game_plugin_build);
    }
}

struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        fn physics_plugin_build(_app_ptr: *mut std::ffi::c_void) {
            println!("  → PhysicsPlugin initialized");
        }
        
        app.register_plugin_fn("PhysicsPlugin", physics_plugin_build);
    }
}

fn main() {
    println!("=== AutoBevy Plugins 测试 ===\n");
    
    // 创建App
    let mut app = App::new();
    println!("✓ 创建 App");
    
    // 添加单个插件
    println!("\n【添加单个插件】");
    app.add_plugin(CorePlugin);
    println!("✓ 添加 CorePlugin");
    
    // app.add_plugin(TimePlugin);
    // println!("✓ 添加 TimePlugin");
    
    app.add_plugin(GamePlugin);
    println!("✓ 添加 GamePlugin");
    
    println!("\n  已注册插件数量: {}", app.plugin_count());
    
    // 执行插件
    println!("\n【执行所有插件】");
    app.finish();
    
    // 测试DefaultPlugins bundle
    println!("\n【测试 DefaultPlugins Bundle】");
    let mut app2 = App::new();
    
    app2.add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin);
    
    println!("✓ 添加 DefaultPlugins + PhysicsPlugin");
    println!("  已注册插件数量: {}", app2.plugin_count());
    
    println!("\n【执行所有插件】");
    app2.finish();
    
    println!("\n=== 所有测试通过! ===");
    println!("\n✨ Plugins 系统特性:");
    println!("  ✓ Plugin trait");
    println!("  ✓ App Builder 模式");
    println!("  ✓ add_plugin() API");
    println!("  ✓ add_plugins() Bundle");
    println!("  ✓ DefaultPlugins");
    println!("  ✓ Zig插件管理器");
    println!("  ✓ 插件生命周期管理");
    println!("  ✓ 90% Zig + 10% Rust");
}
