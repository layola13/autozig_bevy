# autozig-hierarchy

`autozig-hierarchy` 是 Bevy 引擎的 Zig 语言场景层级管理模块，提供高效的父子关系管理和场景图功能。

## 核心功能
- **层级结构**：实现实体间的父子关系管理
- **变换继承**：支持位置/旋转/缩放的层级传递
- **事件系统**：处理层级变更通知
- **Zig 原生实现**：通过 Zig 代码优化层级计算性能

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-hierarchy = { path = "autozig_bevy/autozig-hierarchy" }
```

在 Bevy 应用中集成：
```rust
use autozig_hierarchy::{HierarchyPlugin, Parent, Children};

fn main() {
    App::build()
        .add_plugin(HierarchyPlugin)
        .spawn()
        .insert(Parent(entity))
        .with_children(|parent| {
            parent.spawn().insert(Children::new(vec![child_entity]));
        })
        .run();
}
```

## 核心组件
| 组件 | 说明 |
|------|------|
| `Parent` | 标记实体的父级关系 |
| `Children` | 存储子实体列表 |
| `Transform` | 处理层级变换继承 |

## 层级操作
1. 创建父子关系：`entity.insert(Parent(parent))`
2. 访问子实体：`entity.get::<Children>()`
3. 变换继承：自动计算世界坐标
4. 层级事件：监听 `HierarchyEvent`

## 注意事项
- 所有层级操作需在主线程执行
- 变换继承通过 Zig 实现优化性能
- 循环引用会导致系统错误
- 层级变更会触发自动重计算
- 与 `autozig-transform` 模块深度集成