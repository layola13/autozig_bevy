use autozig_ecs::prelude::*;

fn main() {
    println!("=== AutoZig ECS 测试 ===\n");
    
    // 创建World
    let mut world = World::new();
    println!("✓ 创建 World");
    
    // 生成Entity
    let entity1 = world.spawn_empty();
    let entity2 = world.spawn_empty();
    let entity3 = world.spawn_empty();
    
    println!("✓ 生成了 3 个 entities");
    println!("  Entity 1: {:?}", entity1);
    println!("  Entity 2: {:?}", entity2);
    println!("  Entity 3: {:?}", entity3);
    
    // 检查entity数量
    let count = world.entity_count();
    println!("\n✓ World 中有 {} 个 entities", count);
    assert_eq!(count, 3);
    
    // 检查entity是否存在
    println!("\n测试 entity 存在性:");
    println!("  Entity 1 存在: {}", world.contains(entity1));
    println!("  Entity 2 存在: {}", world.contains(entity2));
    println!("  Entity 3 存在: {}", world.contains(entity3));
    
    // Despawn一个entity
    println!("\n删除 Entity 2...");
    let result =world.despawn(entity2);
    println!("  删除结果: {}", result);
    
    // 再次检查
    let count = world.entity_count();
    println!("\n✓ 删除后剩余 {} 个 entities", count);
    assert_eq!(count, 2);
    
    println!("  Entity 1 存在: {}", world.contains(entity1));
    println!("  Entity 2 存在: {}", world.contains(entity2));
    println!("  Entity 3 存在: {}", world.contains(entity3));
    
    // 测试Entity的bits操作
    println!("\n测试 Entity bits 转换:");
    let bits = entity1.to_bits();
    println!("  Entity 1 to_bits: 0x{:016x}", bits);
    let restored = Entity::from_bits(bits);
    println!("  from_bits 恢复: {:?}", restored);
    assert_eq!(entity1, restored);
    
    println!("\n=== 所有测试通过! ===");
    println!("\n✨ 关键特性:");
    println!("  ✓ 90% Zig 实现");
    println!("  ✓ 10% Rust 包装");
    println!("  ✓ 零 unsafe 代码 (用户侧)");
    println!("  ✓ include_zig! 宏桥接");
    println!("  ✓ Entity 生命周期管理");
}
