
//! 模块 1: ECS 示例 (增强版)
//! 演示 Entity-Component-System 架构
//! 
//! 参考 bevy/examples/ecs/ecs_guide.rs
//! 展示：
//! - 组件定义（Component trait）
//! - 实体创建和管理
//! - 查询系统（Query）
//! - 资源管理（Resource）
//! - 系统参数（SystemParam）

use autozig_ecs::prelude::*;

// ==================== 组件定义 ====================
// 组件是普通的 Rust 数据类型，用 Component trait 标记

/// 玩家组件
#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
}

/// 分数组件
#[derive(Clone, Debug)]
pub struct Score {
    pub value: u32,
}

/// 位置组件
#[derive(Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// 速度组件
#[derive(Clone, Debug)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

/// 生命值组件
#[derive(Clone, Debug)]
pub struct Health {
    pub current: u32,
    pub max: u32,
}

/// 玩家连胜状态
#[derive(Clone, Debug)]
pub enum PlayerStreak {
    Hot(u32),      // 连胜
    Cold(u32),     // 连败
    None,          // 无连胜/连败
}

// ==================== 资源定义 ====================
// 资源是全局共享的数据

/// 游戏状态资源
#[derive(Clone, Debug)]
pub struct GameState {
    pub current_round: u32,
    pub total_players: u32,
    pub winning_player: Option<String>,
}

/// 游戏规则资源
#[derive(Clone, Debug)]
pub struct GameRules {
    pub winning_score: u32,
    pub max_rounds: u32,
    pub max_players: u32,
}

pub fn run_ecs_demo() {
    println!("\n{}", "=".repeat(60));
    println!("模块 1: ECS 示例 (增强版)");
    println!("{}", "=".repeat(60));
    
    // ==================== 示例 1: World 和实体基础 ====================
    println!("\n[示例 1] World 和实体基础");
    println!("{}", "-".repeat(60));
    
    let mut world = World::new();
    println!("✓ World 已创建");
    println!();
    println!("World 是 ECS 的核心容器，存储所有:");
    println!("  • 实体 (Entities): 唯一标识符");
    println!("  • 组件 (Components): 数据");
    println!("  • 资源 (Resources): 全局数据");
    
    // ==================== 示例 2: 组件和实体 ====================
    println!("\n[示例 2] 组件定义和实体创建");
    println!("{}", "-".repeat(60));
    
    println!("定义的组件类型:");
    println!("  • Player: 玩家信息 (name)");
    println!("  • Score: 分数 (value)");
    println!("  • Position: 位置 (x, y)");
    println!("  • Velocity: 速度 (dx, dy)");
    println!("  • Health: 生命值 (current, max)");
    println!("  • PlayerStreak: 连胜状态");
    println!();
    
    // 创建玩家实体
    println!("创建玩家实体:");
    
    let player1 = Player {
        name: "Alice".to_string(),
    };
    let score1 = Score { value: 0 };
    let pos1 = Position { x: 0.0, y: 0.0 };
    let vel1 = Velocity { dx: 1.5, dy: 0.0 };
    let health1 = Health { current: 100, max: 100 };
    
    println!("  玩家 1: {:?}", player1);
    println!("    - 分数: {:?}", score1);
    println!("    - 位置: {:?}", pos1);
    println!("    - 速度: {:?}", vel1);
    println!("    - 生命: {:?}", health1);
    
    let player2 = Player {
        name: "Bob".to_string(),
    };
    let score2 = Score { value: 0 };
    let pos2 = Position { x: 10.0, y: 5.0 };
    let vel2 = Velocity { dx: -0.5, dy: 2.0 };
    let health2 = Health { current: 80, max: 100 };
    
    println!();
    println!("  玩家 2: {:?}", player2);
    println!("    - 分数: {:?}", score2);
    println!("    - 位置: {:?}", pos2);
    println!("    - 速度: {:?}", vel2);
    println!("    - 生命: {:?}", health2);
    
    println!();
    println!("✓ 实体是组件的集合");
    println!("✓ 每个实体可以有不同的组件组合");
    
    // ==================== 示例 3: 资源系统 ====================
    println!("\n[示例 3] 资源系统");
    println!("{}", "-".repeat(60));
    
    let game_state = GameState {
        current_round: 0,
        total_players: 2,
        winning_player: None,
    };
    
    let game_rules = GameRules {
        winning_score: 10,
        max_rounds: 20,
        max_players: 4,
    };
    
    println!("游戏状态资源:");
    println!("  {:?}", game_state);
    println!();
    println!("游戏规则资源:");
    println!("  {:?}", game_rules);
    println!();
    println!("资源特点:");
    println!("  • 全局唯一: 每种类型只能有一个实例");
    println!("  • 随处访问: 任何系统都可以访问");
    println!("  • Res<T>: 只读访问");
    println!("  • ResMut<T>: 可变访问");
    
    // ==================== 示例 4: 查询系统 ====================
    println!("\n[示例 4] 查询系统 (Queries)");
    println!("{}", "-".repeat(60));
    
    println!("查询允许系统访问满足条件的实体:");
    println!();
    println!("示例查询:");
    println!("  • Query<&Player>: 所有有 Player 组件的实体");
    println!("  • Query<(&Player, &Score)>: 同时有两个组件的实体");
    println!("  • Query<(&mut Position, &Velocity)>: 位置可变，速度只读");
    println!();
    println!("查询过滤器:");
    println!("  • With<T>: 必须有 T 组件");
    println!("  • Without<T>: 不能有 T 组件");
    println!("  • Changed<T>: T 组件被修改过");
    println!("  • Added<T>: T 组件刚被添加");
    
    // 模拟查询处理
    println!();
    println!("模拟查询处理:");
    
    // 模拟移动系统
    println!("  [移动系统] Query<(&mut Position, &Velocity)>");
    let mut pos = pos1.clone();
    let vel = vel1.clone();
    pos.x += vel.dx;
    pos.y += vel.dy;
    println!("    • Alice 移动: ({:.1}, {:.1}) -> ({:.1}, {:.1})", 
             pos1.x, pos1.y, pos.x, pos.y);
    
    let mut pos = pos2.clone();
    let vel = vel2.clone();
    pos.x += vel.dx;
    pos.y += vel.dy;
    println!("    • Bob 移动: ({:.1}, {:.1}) -> ({:.1}, {:.1})", 
             pos2.x, pos2.y, pos.x, pos.y);
    
    // 模拟分数系统
    println!();
    println!("  [分数系统] Query<(&Player, &mut Score)>");
    let mut score = score1.clone();
    score.value += 1;
    println!("    • {} 得分: {} -> {}", player1.name, score1.value, score.value);
    
    // ==================== 示例 5: 系统参数 ====================
    println!("\n[示例 5] 系统参数 (System Parameters)");
    println!("{}", "-".repeat(60));
    
    println!("系统可以使用多种参数类型:");
    println!();
    println!("组件查询:");
    println!("  • Query<&T>: 只读查询");
    println!("  • Query<&mut T>: 可变查询");
    println!("  • Query<(&A, &B, &C)>: 多组件查询");
    println!();
    println!("资源访问:");
    println!("  • Res<T>: 只读资源");
    println!("  • ResMut<T>: 可变资源");
    println!();
    println!("命令缓冲:");
    println!("  • Commands: 延迟执行实体操作");
    println!();
    println!("局部状态:");
    println!("  • Local<T>: 系统局部状态");
    println!();
    println!("事件:");
    println!("  • EventReader<T>: 读取事件");
    println!("  • EventWriter<T>: 写入事件");
    
    // ==================== 示例 6: 系统执行示例 ====================
    println!("\n[示例 6] 完整系统执行流程");
    println!("{}", "-".repeat(60));
    
    println!("回合 1:");
    println!("  [新回合系统]");
    let mut state = game_state.clone();
    state.current_round += 1;
    println!("    • 回合 {}/{}", state.current_round, game_rules.max_rounds);
    
    println!();
    println!("  [物理系统] 更新位置");
    let mut p1_pos = Position { x: 0.0, y: 0.0 };
    let p1_vel = Velocity { dx: 1.5, dy: 0.0 };
    p1_pos.x += p1_vel.dx;
    p1_pos.y += p1_vel.dy;
    println!("    • Alice: ({:.1}, {:.1})", p1_pos.x, p1_pos.y);
    
    let mut p2_pos = Position { x: 10.0, y: 5.0 };
    let p2_vel = Velocity { dx: -0.5, dy: 2.0 };
    p2_pos.x += p2_vel.dx;
    p2_pos.y += p2_vel.dy;
    println!("    • Bob: ({:.1}, {:.1})", p2_pos.x, p2_pos.y);
    
    println!();
    println!("  [分数系统] 随机得分");
    let mut p1_score = Score { value: 0 };
    let mut p2_score = Score { value: 0 };
    
    // 模拟随机得分
    p1_score.value += 2;
    println!("    • Alice 得分: {} (+2)", p1_score.value);
    
    p2_score.value += 1;
    println!("    • Bob 得分: {} (+1)", p2_score.value);
    
    println!();
    println!("  [检查胜利系统]");
    if p1_score.value >= game_rules.winning_score {
        println!("    • {} 达到胜利分数!", player1.name);
    } else if p2_score.value >= game_rules.winning_score {
        println!("    • {} 达到胜利分数!", player2.name);
    } else {
        println!("    • 游戏继续...");
    }
    
    // ==================== 示例 7: 变更检测 ====================
    println!("\n[示例 7] 变更检测");
    println!("{}", "-".repeat(60));
    
    println!("ECS 提供变更检测功能:");
    println!();
    println!("Changed<T> 查询:");
    println!("  • 检测组件是否在上一帧被修改");
    println!("  • 优化性能，只处理变化的数据");
    println!();
    println!("Added<T> 查询:");
    println!("  • 检测组件是否刚被添加");
    println!("  • 用于初始化新实体");
    println!();
    println!("RemovedComponents<T>:");
    println!("  • 追踪被移除的组件");
    println!("  • 清理相关数据");
    println!();
    println!("示例:");
    println!("  [变更检测] Query<&Player, Changed<Score>>");
    println!("    • Alice 的分数发生变化");
    println!("    • Bob 的分数发生变化");
    println!("    → 只处理这两个实体，忽略其他未变化的实体");
    
    // ==================== 示例 8: 命令系统 ====================
    println!("\n[示例 8] 命令系统 (Commands)");
    println!("{}", "-".repeat(60));
    
    println!("Commands 提供延迟执行的实体操作:");
    println!();
    println!("为什么需要 Commands?");
    println!("  • 系统并行执行");
    println!("  • 直接修改 World 不安全");
    println!("  • Commands 收集操作，统一执行");
    println!();
    println!("常用命令:");
    println!("  • commands.spawn(bundle): 创建实体");
    println!("  • commands.entity(id).insert(component): 添加组件");
    println!("  • commands.entity(id).remove::<T>(): 移除组件");
    println!("  • commands.entity(id).despawn(): 销毁实体");
    println!();
    println!("示例:");
    println!("  commands.spawn((");
    println!("      Player {{ name: \"Charlie\" }},");
    println!("      Score {{ value: 0 }},");
    println!("      Position {{ x: 5.0, y: 5.0 }},");
    println!("  ));");
    println!("  → 在帧结束时执行");
    
    // ==================== 示例 9: 层级系统 ====================
    println!("\n[示例 9] 实体层级 (Hierarchy)");
    println!("{}", "-".repeat(60));
    
    println!("实体可以有父子关系:");
    println!();
    println!("使用场景:");
    println!("  • 场景图 (Scene Graph)");
    println!("  • UI 布局");
    println!("  • 物理关节");
    println!();
    println!("API:");
    println!("  • with_children(): 创建子实体");
    println!("  • add_child(): 添加已存在的子实体");
    println!("  • Children 组件: 存储子实体列表");
    println!("  • Parent 组件: 存储父实体引用");
    println!();
    println!("示例:");
    println!("  玩家实体");
    println!("    ├─ 武器实体");
    println!("    ├─ 护甲实体");
    println!("    └─ 特效实体");
    println!();
    println!("  变换传播:");
    println!("    • 父实体移动 → 子实体跟随移动");
    println!("    • 父实体旋转 → 子实体相对旋转");
    
    // ==================== 总结 ====================
    println!("\n{}", "=".repeat(60));
    println!("模块 1 总结:");
    println!("{}", "-".repeat(60));
    println!("✓ World: ECS 核心容器");
    println!("✓ Entity: 唯一标识符 (ID)");
    println!("✓ Component: 数据存储 (Player, Score, Position...)");
    println!("✓ Resource: 全局数据 (GameState, GameRules)");
    println!("✓ Query: 组件查询系统");
    println!("✓ System: 逻辑执行单元");
    println!("✓ Commands: 延迟命令执行");
    println!("✓ Changed/Added: 变更检测");
    println!("✓ Hierarchy: 父子实体关系");
    println!();
    println!("ECS 优势:");
    println!("  • 数据驱动设计");
    println!("  • 高性能并行执行");
    println!("  • 灵活的组件组合");
    println!("  • 清晰的代码架构");
    println!("{}", "=".repeat(60));
    println!("\n模块 1 完成 ✓\n");
}