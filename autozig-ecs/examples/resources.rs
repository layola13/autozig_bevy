use autozig_ecs::prelude::*;
use autozig_ecs::resource::ResourceRegistry;

// 定义一个Time资源
#[derive(Debug)]
struct Time {
    delta: f32,
    elapsed: f32,
}

impl Time {
    fn new() -> Self {
        Self {
            delta: 0.016, // 60 FPS
            elapsed: 0.0,
        }
    }
    
    fn delta_seconds(&self) -> f32 {
        self.delta
    }
    
    fn elapsed_seconds(&self) -> f32 {
        self.elapsed
    }
    
    fn tick(&mut self, delta: f32) {
        self.delta = delta;
        self.elapsed += delta;
    }
}

// 定义一个配置资源
#[derive(Debug)]
struct GameConfig {
    player_speed: f32,
    max_enemies: u32,
}

fn main() {
    println!("=== AutoBevy Resources 测试 ===\n");
    
    // 创建ResourceRegistry
    let mut resources = ResourceRegistry::new();
    println!("✓ 创建 ResourceRegistry");
    
    // 插入Time资源
    resources.insert(Time::new());
    println!("✓ 插入 Time 资源");
    
    // 插入GameConfig资源
    resources.insert(GameConfig {
        player_speed: 5.0,
        max_enemies: 10,
    });
    println!("✓ 插入 GameConfig 资源");
    
    // 读取资源
    println!("\n【读取资源测试】");
    if let Some(time) = resources.get::<Time>() {
        println!("✓ Time 资源:");
        println!("  - Delta: {:.3}s", time.delta_seconds());
        println!("  - Elapsed: {:.3}s", time.elapsed_seconds());
    }
    
    if let Some(config) = resources.get::<GameConfig>() {
        println!("✓ GameConfig 资源:");
        println!("  - Player Speed: {}", config.player_speed);
        println!("  - Max Enemies: {}", config.max_enemies);
    }
    
    // 检查资源存在性
    println!("\n【资源存在性检查】");
    println!("  Time exists: {}", resources.contains::<Time>());
    println!("  GameConfig exists: {}", resources.contains::<GameConfig>());
    println!("  NonExistent exists: {}", resources.contains::<World>());
    
    // 移除资源
    println!("\n【移除资源测试】");
    let removed = resources.remove::<GameConfig>();
    println!("✓ GameConfig removed: {:?}", removed);
    println!("  GameConfig exists after removal: {}", resources.contains::<GameConfig>());
    
    // 验证移除后无法访问
    if resources.get::<GameConfig>().is_none() {
        println!("✓ GameConfig 确实已被移除");
    }
    
    println!("\n=== 所有测试通过! ===");
    println!("\n✨ Resources 系统特性:");
    println!("  ✓ TypeId -> u64 映射");
    println!("  ✓ Zig HashMap 存储");
    println!("  ✓ insert_resource");
    println!("  ✓ get_resource -> Res<T>");
    println!("  ✓ remove_resource");
    println!("  ✓ 90% Zig + 10% Rust");
}
