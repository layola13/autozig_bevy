//! 模块 0: App 示例
//! 演示 autozig-app 和 autozig-ecs 的 App 架构
//! 
//! 参考 bevy/examples/app/plugin.rs
//! 展示：
//! - 插件系统（Plugin trait）
//! - 资源管理（Resource）
//! - 系统注册和调度
//! - Startup vs Update 系统

use autozig_ecs::prelude::*;
use autozig_app::{App, Plugin, MinimalPlugins};
use autozig_time::TimePlugin;

/// 自定义插件示例：打印消息插件
/// 类似 bevy/examples/app/plugin.rs 中的 PrintMessagePlugin
pub struct PrintMessagePlugin {
    pub message: &'static str,
    pub interval_seconds: u32,
}

impl Plugin for PrintMessagePlugin {
    fn build(&self, app: &mut App) {
        // 插件的 build 方法用于配置 App
        println!("  [Plugin] PrintMessagePlugin 正在初始化...");
        println!("    - 消息: \"{}\"", self.message);
        println!("    - 间隔: {} 秒", self.interval_seconds);
        
        // 这里可以添加资源、系统等
        // 注意：由于 autozig-ecs 的 API 限制，我们使用闭包系统
        let msg = self.message;
        app.add_systems(Update, move || {
            println!("  [{}] {}", "PrintMessagePlugin", msg);
        });
    }

    fn name(&self) -> &str {
        "PrintMessagePlugin"
    }
}

/// 游戏规则插件
pub struct GameRulesPlugin {
    pub max_players: u32,
    pub max_rounds: u32,
}

impl Plugin for GameRulesPlugin {
    fn build(&self, app: &mut App) {
        println!("  [Plugin] GameRulesPlugin 正在初始化...");
        println!("    - 最大玩家数: {}", self.max_players);
        println!("    - 最大回合数: {}", self.max_rounds);
        
        let max_players = self.max_players;
        let max_rounds = self.max_rounds;
        app.add_systems(Update, move || {
            println!("  [GameRules] 玩家: {}/{}, 回合限制: {}",
                     2, max_players, max_rounds);
        });
    }

    fn name(&self) -> &str {
        "GameRulesPlugin"
    }
}

/// 日志插件
pub struct LoggerPlugin;

impl Plugin for LoggerPlugin {
    fn build(&self, app: &mut App) {
        println!("  [Plugin] LoggerPlugin 正在初始化...");
        app.add_systems(Update, || {
            println!("  [Logger] 系统正在运行...");
        });
    }

    fn name(&self) -> &str {
        "LoggerPlugin"
    }
}

pub fn run_app_demo() {
    println!("\n{}", "=".repeat(60));
    println!("模块 0: App 示例 (增强版)");
    println!("{}", "=".repeat(60));
    
    // ==================== 示例 1: 基础 App ====================
    println!("\n[示例 1] 基础 App 创建和系统注册");
    println!("{}", "-".repeat(60));
    
    let mut app = App::new();
    println!("✓ App 实例已创建");
    
    // 添加多个系统
    app.add_systems(Startup, || {
        println!("  → System 1: 初始化系统执行");
    });
    
    app.add_systems(Update, || {
        println!("  → System 2: 更新系统执行");
    });
    
    app.add_systems(Last, || {
        println!("  → System 3: 渲染系统执行");
    });
    
    let system_count = app.closure_system_count();
    println!("✓ 已注册 {} 个闭包系统", system_count);
    
    println!("\n运行系统:");
    app.run();
    println!("✓ 系统执行完毕");
    
    // ==================== 示例 2: 插件系统 ====================
    println!("\n[示例 2] 插件系统演示");
    println!("{}", "-".repeat(60));
    println!("插件是组织功能的核心方式，它们封装了:");
    println!("  • 组件定义");
    println!("  • 资源配置");
    println!("  • 系统注册");
    println!();
    
    let mut app2 = App::new();
    
    // 添加核心插件
    println!("添加插件:");
    app2.add_plugins(MinimalPlugins);
    println!("  ✓ MinimalPlugins 已添加");
    
    app2.add_plugin(TimePlugin);
    println!("  ✓ TimePlugin 已添加");
    
    // 添加自定义插件
    app2.add_plugin(PrintMessagePlugin {
        message: "Hello from Plugin!",
        interval_seconds: 1,
    });
    println!("  ✓ PrintMessagePlugin 已添加");
    
    app2.add_plugin(GameRulesPlugin {
        max_players: 4,
        max_rounds: 10,
    });
    println!("  ✓ GameRulesPlugin 已添加");
    
    app2.add_plugin(LoggerPlugin);
    println!("  ✓ LoggerPlugin 已添加");
    
    let plugin_count = app2.plugin_count();
    println!("\n✓ 总计插件数量: {}", plugin_count);
    
    // 完成插件初始化
    println!("\n执行插件初始化:");
    app2.finish();
    println!("✓ 所有插件已初始化");
    
    println!("\n运行插件系统:");
    app2.run();
    println!("✓ 插件系统执行完毕");
    
    // ==================== 示例 3: 多阶段系统 ====================
    println!("\n[示例 3] 多阶段系统调度");
    println!("{}", "-".repeat(60));
    println!("Bevy 使用不同的 Schedule 来组织系统执行顺序:");
    println!("  • Startup: 启动时执行一次");
    println!("  • Update: 每帧执行");
    println!("  • Last: 每帧最后执行");
    println!();
    
    let mut app3 = App::new();
    
    // 模拟 Startup 系统
    println!("添加 Startup 系统 (启动系统):");
    app3.add_systems(Startup, || {
        println!("  [Startup] 初始化游戏世界...");
        println!("    • 加载配置");
        println!("    • 创建玩家实体");
        println!("    • 初始化资源");
    });
    
    // 模拟 Update 系统
    println!("\n添加 Update 系统 (更新系统):");
    app3.add_systems(Update, || {
        println!("  [Update] 帧 #1: 处理游戏逻辑");
        println!("    • 移动实体");
        println!("    • 检测碰撞");
        println!("    • 更新分数");
    });
    
    // 模拟 Last 系统
    println!("\n添加 Last 系统 (清理系统):");
    app3.add_systems(Last, || {
        println!("  [Last] 帧结束清理");
        println!("    • 应用延迟命令");
        println!("    • 清理临时数据");
    });
    
    println!("\n执行系统调度:");
    app3.run();
    println!("✓ 多阶段系统执行完毕");
    
    // ==================== 示例 4: 系统链和依赖 ====================
    println!("\n[示例 4] 系统链和执行顺序");
    println!("{}", "-".repeat(60));
    println!("系统可以通过 .before() 和 .after() 指定执行顺序");
    println!("系统链确保数据依赖正确处理");
    println!();
    
    let mut app4 = App::new();
    
    app4.add_systems(Update, || {
        println!("  [1] 物理系统: 计算速度和位置");
    });
    
    app4.add_systems(Update, || {
        println!("  [2] 碰撞系统: 检测碰撞 (依赖物理系统)");
    });
    
    app4.add_systems(Update, || {
        println!("  [3] 音效系统: 播放碰撞音效 (依赖碰撞系统)");
    });
    
    app4.add_systems(Update, || {
        println!("  [4] 渲染系统: 绘制实体 (最后执行)");
    });
    
    println!("执行系统链:");
    app4.run();
    println!("✓ 系统按依赖顺序执行完毕");
    
    // ==================== 总结 ====================
    println!("\n{}", "=".repeat(60));
    println!("模块 0 总结:");
    println!("{}", "-".repeat(60));
    println!("✓ 基础 App 创建和系统注册");
    println!("✓ 插件系统 (Plugin trait)");
    println!("✓ 多阶段系统调度 (Startup/Update/Last)");
    println!("✓ 系统链和执行顺序");
    println!();
    println!("关键概念:");
    println!("  • App: 应用程序容器");
    println!("  • Plugin: 功能模块封装");
    println!("  • System: 逻辑执行单元");
    println!("  • Schedule: 系统调度器");
    println!("{}", "=".repeat(60));
    println!("\n模块 0 完成 ✓\n");
}