# autozig-ecs

`autozig-ecs` 是 Bevy 引擎的 Zig 语言实体组件系统（ECS）核心实现，提供高性能的实体-组件-系统架构支持。

## 核心功能
- **实体管理**：高效创建、销毁和查询实体
- **组件系统**：基于 Zig 的组件存储和访问机制
- **系统调度**：支持并行系统执行和依赖管理
- **资源管理**：全局资源的生命周期控制
- **事件系统**：跨系统通信的事件发布/订阅机制

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-ecs = { path = "autozig_bevy/autozig-ecs" }
```

在 Bevy 应用中使用：
```rust
use autozig_ecs::{App, SystemStage, IntoSystem};

fn main() {
    App::build()
        .add_system(hello_world_system.system())
        .run();
}

fn hello_world_system() {
    println!("Hello from ECS system!");
}
```

## 核心概念
| 概念 | 说明 |
|------|------|
| `Entity` | 唯一标识符，代表游戏世界中的对象 |
| `Component` | 附加到实体的数据单元 |
| `System` | 处理组件数据的逻辑单元 |
| `Resource` | 全局可访问的共享数据 |
| `Query` | 高效访问组件数据的机制 |

## 系统执行流程
1. 初始化阶段：创建实体和初始组件
2. 系统执行阶段：按依赖顺序执行系统
3. 事件处理阶段：处理系统间通信事件
4. 资源更新阶段：同步全局资源状态

## 注意事项
- 所有 ECS 操作需在主线程执行
- 组件存储采用 Zig 实现的内存布局优化
- 系统调度器支持并行执行
- 事件系统提供跨系统通信能力
- 资源管理遵循 Rust 的所有权模型