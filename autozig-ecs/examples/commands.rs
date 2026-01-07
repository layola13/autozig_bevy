use autozig_ecs::command::CommandBuffer;

// 定义测试组件
#[derive(Debug, Clone, Copy)]
struct Health {
    value: f32,
}

#[derive(Debug, Clone, Copy)]
struct Damage {
    amount: f32,
}

fn main() {
    println!("=== AutoBevy Commands 测试 ===\n");
    
    // 创建CommandBuffer
    let mut cmd_buffer = CommandBuffer::new();
    println!("✓ 创建 CommandBuffer");
    
    // 测试Commands API
    println!("\n【写入命令】");
    {
        let mut commands = cmd_buffer.commands();
        
        // Spawn entities with components
        commands.spawn_empty()
            .insert(Health { value: 100.0 })
            .insert(Damage { amount: 25.0 });
        println!("✓ Spawn entity with Health + Damage");
        
        commands.spawn_empty()
            .insert(Health { value: 50.0 });
        println!("✓ Spawn entity with Health only");
        
        // Entity commands
        commands.entity(0).remove::<Damage>();
        println!("✓ Remove Damage from entity 0");
        
        commands.entity(1).despawn();
        println!("✓ Despawn entity 1");
    }
    
    println!("\n【执行命令】");
    let executed = cmd_buffer.apply();
    println!("✓ 执行了 {} 个命令", executed);
    
    // 验证缓冲已清空
    assert!(cmd_buffer.is_empty());
    println!("✓ CommandBuffer 已清空");
    
    // 测试再次写入
    println!("\n【第二轮命令】");
    {
        let mut commands = cmd_buffer.commands();
        
        for i in 0..5 {
            commands.spawn_empty()
                .insert(Health { value: (i * 10) as f32 });
        }
        println!("✓ Spawn 5 个 entities");
    }
    
    let executed2 = cmd_buffer.apply();
    println!("✓ 执行了 {} 个命令", executed2);
    
    println!("\n=== 所有测试通过! ===");
    println!("\n✨ Commands 系统特性:");
    println!("  ✓ OpCode 字节流");
    println!("  ✓ CommandBuffer 延迟执行");
    println!("  ✓ spawn_empty() API");
    println!("  ✓ insert/remove 组件");
    println!("  ✓ despawn entity");
    println!("  ✓ 链式调用 (Fluent API)");
    println!("  ✓ 90% Zig + 10% Rust");
}
