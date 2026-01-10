# autozig-light

`autozig-light` 是 Bevy 引擎的 Zig 语言光照系统模块，提供完整的 3D 光照计算和场景管理功能。

## 核心功能
- **光照类型支持**：实现环境光、方向光、点光源和聚光灯
- **GPU 数据结构**：优化的 `GpuLightData` 用于高效渲染
- **阴影映射**：支持阴影生成与渲染管线集成
- **场景管理**：统一管理场景中的所有光源

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-light = { path = "autozig_bevy/autozig-light" }
```

在 Bevy 应用中集成：
```rust
use autozig_light::{LightPlugin, DirectionalLight};

fn main() {
    App::build()
        .add_plugin(LightPlugin)
        .insert_resource(DirectionalLight {
            illuminance: 1000.0,
            ..Default::default()
        })
        .run();
}
```

## 光源类型
| 类型 | 说明 |
|------|------|
| `AmbientLight` | 全局环境光 |
| `DirectionalLight` | 平行方向光（如太阳） |
| `PointLight` | 点状光源（如灯泡） |
| `SpotLight` | 聚光灯（带方向和角度） |

## 关键组件
- **`LightScene`**：管理场景中所有光源
- **`LightingUtils`**：提供光照计算辅助函数
- **`ShadowMap`**：处理阴影映射逻辑

## 注意事项
- 所有光照计算在 Zig 层实现，确保高性能
- 阴影映射需要 GPU 支持
- 光源参数需符合物理单位规范
- 渲染管线需正确配置光照数据结构