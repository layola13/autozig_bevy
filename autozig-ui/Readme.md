# autozig-ui

`autozig-ui` 是 Bevy 引擎的 Zig 语言用户界面模块，提供高性能的 2D UI 渲染和交互系统。

## 核心功能
- **UI 组件系统**：实现按钮、文本、容器等基础 UI 组件
- **布局引擎**：支持弹性布局和网格布局
- **事件处理**：处理鼠标、触摸等 UI 交互事件
- **Zig 原生实现**：通过 Zig 代码优化 UI 渲染性能

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-ui = { path = "autozig_bevy/autozig-ui" }
```

在 Bevy 应用中集成：
```rust
use autozig_ui::{UiPlugin, Button, Text};

fn main() {
    App::build()
        .add_plugin(UiPlugin)
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(50.0)),
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section("Click Me", default()),
                    ..Default::default()
                });
            });
        })
        .run();
}
```

## 核心组件
| 组件 | 说明 |
|------|------|
| `Node` | UI 布局容器 |
| `Button` | 可点击按钮组件 |
| `Text` | 文本显示组件 |
| `Image` | 图像显示组件 |

## 布局系统
- **弹性布局**：基于 Flexbox 的布局模型
- **网格布局**：支持复杂的网格排列
- **尺寸单位**：支持像素、百分比、自动等单位
- **响应式设计**：自动适应不同屏幕尺寸

## 注意事项
- 所有 UI 操作在 Zig 层实现，确保渲染效率
- 布局计算使用 Zig 优化的算法
- 与 `autozig-sprite` 模块深度集成
- 支持多点触控和鼠标交互
- 事件系统与输入模块协同工作