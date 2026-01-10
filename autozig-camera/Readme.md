# autozig-camera

`autozig-camera` 是 Bevy 引擎的 Zig 语言相机系统模块，提供 2D/3D 场景的视角管理和投影变换功能。

## 核心功能
- **相机类型支持**：实现透视相机和正交相机
- **视口管理**：控制渲染区域和分辨率适配
- **投影变换**：处理 3D 到 2D 的坐标转换
- **Zig 原生实现**：通过 Zig 代码优化相机计算性能

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-camera = { path = "autozig_bevy/autozig-camera" }
```

在 Bevy 应用中集成：
```rust
use autozig_camera::{CameraPlugin, Camera3d};

fn main() {
    App::build()
        .add_plugin(CameraPlugin)
        .spawn(Camera3d::default())
        .run();
}
```

## 相机类型
| 类型 | 说明 |
|------|------|
| `Camera2d` | 2D 正交投影相机 |
| `Camera3d` | 3D 透视投影相机 |
| `Camera` | 通用相机组件基类 |

## 核心参数
- **`projection`**：定义投影类型和参数
- **`viewport`**：控制渲染区域
- **`transform`**：相机在世界中的位置和朝向
- **`order`**：渲染顺序优先级

## 注意事项
- 相机实体需附加 `Camera` 组件
- 3D 相机默认使用透视投影
- 2D 相机默认使用正交投影
- 视口设置需与渲染目标匹配
- 所有相机计算在 Zig 层实现