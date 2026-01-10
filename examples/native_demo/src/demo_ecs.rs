//! 模块 1: ECS 示例
//! 演示 Entity-Component-System 架构

use autozig_ecs::prelude::*;

pub fn run_ecs_demo() {
    println!("\n{}", "=".repeat(60));
    println!("模块 1: ECS 示例");
    println!("{}", "=".repeat(60));
    
    // 创建 World
    println!("\n[1] 创建 World...");
    let mut world = World::new();
    println!("  ✓ World 已创建");
    
    // 演示实体系统
    println!("\n[2] 演示实体系统...");
    println!("  - Entity: 唯一标识符 (ID + Generation)");
    println!("  - World::spawn() 创建新实体");
    println!("  - World::despawn() 销毁实体");
    
    // 演示组件系统
    println!("\n[3] 演示组件系统...");
    println!("  - Component trait 提供组件标记");
    println!("  - Bundle trait 提供组件批量操作");
    println!("  - 实体可以动态添加/移除组件");
    
    // 演示查询系统
    println!("\n[4] 演示查询系统...");
    println!("  - Query<T> 用于查询组件");
    println!("  - With<T>/Without<T> 用于过滤");
    println!("  - Read<T>/Write<T> 用于访问控制");
    
    // 演示资源系统
    println!("\n[5] 演示资源系统...");
    println!("  - Res<T> 用于只读资源");
    println!("  - ResMut<T> 用于可变资源");
    println!("  - ResourceRegistry 管理全局资源");
    
    // 演示事件系统
    println!("\n[6] 演示事件系统...");
    println!("  - Events<T> 存储事件");
    println!("  - EventWriter<T> 发送事件");
    println!("  - EventReader<T> 读取事件");
    
    // 演示命令系统
    println!("\n[7] 演示命令系统...");
    println!("  - Commands 延迟执行命令");
    println!("  - CommandBuffer 缓冲命令队列");
    
    // 演示系统参数
    println!("\n[8] 演示系统参数...");
    println!("  - SystemParam trait 提供参数注入");
    println!("  - IntoSystem trait 将函数转换为系统");
    
    // 演示变更检测
    println!("\n[9] 演示变更检测...");
    println!("  - Changed<T> 检测组件变更");
    println!("  - Added<T> 检测新增组件");
    println!("  - RemovedComponents<T> 检测移除的组件");
    
    println!("\n模块 1 完成 ✓\n");
}