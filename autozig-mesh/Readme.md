# autozig-mesh

`autozig-mesh` 是 Bevy 引擎的 Zig 语言网格处理模块，提供高效的 3D 网格数据管理和渲染支持。

## 核心功能
- **网格数据结构**：实现顶点、索引和属性的高效存储
- **几何原语**：提供立方体、球体等基础几何体生成
- **Zig 原生实现**：通过 Zig 代码优化网格处理性能
- **WASM 兼容**：针对 WebAssembly 平台的特殊优化

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-mesh = { path = "autozig_bevy/autozig-mesh" }
```

在 Bevy 应用中集成：
```rust
use autozig_mesh::{MeshPlugin, Mesh, Primitive};

fn main() {
    App::build()
        .add_plugin(MeshPlugin)
        .spawn(Mesh::from_primitive(Primitive::Cube))
        .run();
}
```

## 核心组件
| 组件 | 说明 |
|------|------|
| `Mesh` | 网格数据容器 |
| `Primitive` | 基础几何体类型 |
| `Vertex` | 顶点数据结构 |

## 几何原语
- **`Cube`**：立方体
- **`Sphere`**：球体
- **`Plane`**：平面
- **`Cylinder`**：圆柱体

## 注意事项
- 所有网格操作在 Zig 层实现，确保高性能
- 顶点数据格式需符合渲染管线要求
- 网格数据通过 Zig 优化内存布局
- 支持自定义顶点属性
- 与 `autozig-render` 模块深度集成