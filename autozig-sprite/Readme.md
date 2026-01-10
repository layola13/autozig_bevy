# autozig-sprite

`autozig-sprite` 是 Bevy 引擎的 Zig 语言 2D 精灵渲染模块，提供高性能的精灵批处理和渲染功能。

## 核心功能
- **精灵批处理**：实现高效的精灵合批渲染
- **2D 渲染支持**：提供 2D 场景的渲染管线集成
- **Zig 原生实现**：通过 Zig 代码优化渲染性能
- **纹理管理**：处理精灵纹理的加载和绑定

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-sprite = { path = "autozig_bevy/autozig-sprite" }
```

在 Bevy 应用中集成：
```rust
use autozig_sprite::{SpritePlugin, Sprite};

fn main() {
    App::build()
        .add_plugin(SpritePlugin)
        .spawn(Sprite {
            color: Color::WHITE,
            ..Default::default()
        })
        .run();
}
```

## 核心组件
| 组件 | 说明 |
|------|------|
| `Sprite` | 2D 精灵基础组件 |
| `SpriteBatch` | 精灵批处理系统 |
| `TextureAtlas` | 纹理图集支持 |

## 渲染流程
1. 创建精灵实体并附加 `Sprite` 组件
2. 系统自动收集精灵数据
3. 批处理系统优化渲染调用
4. GPU 渲染管线执行最终绘制

## 注意事项
- 所有精灵渲染操作在 Zig 层实现
- 批处理系统自动优化渲染性能
- 纹理管理需配合 `autozig-asset` 模块
- 2D 渲染坐标系遵循 Bevy 的标准规范
- 支持透明度混合和着色器定制