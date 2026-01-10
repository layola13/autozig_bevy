# autozig-image

`autozig-image` 是 Bevy 引擎的 Zig 语言图像处理模块，提供高效的图像加载、解码和纹理管理功能。

## 核心功能
- **图像格式支持**：实现 PNG、JPEG、BMP 等常见格式的解码
- **纹理管理**：优化图像数据到 GPU 纹理的转换流程
- **WASM64 兼容**：针对 WebAssembly 平台的特殊优化
- **Zig 原生实现**：通过 Zig 代码实现高性能图像处理

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-image = { path = "autozig_bevy/autozig-image" }
```

在 Bevy 应用中集成：
```rust
use autozig_image::{ImagePlugin, Image};

fn main() {
    App::build()
        .add_plugin(ImagePlugin)
        .insert_resource(Image::load("assets/textures/sprite.png"))
        .run();
}
```

## 核心组件
| 组件 | 说明 |
|------|------|
| `Image` | 图像数据容器 |
| `ImageLoader` | 异步图像加载器 |
| `TextureAtlas` | 纹理图集管理 |

## 处理流程
1. 通过 `ImageLoader` 加载图像文件
2. 解码为原始像素数据
3. 转换为 GPU 友好的纹理格式
4. 上传到 GPU 供渲染使用

## 注意事项
- 所有图像操作需在主线程执行
- 支持的格式包括 PNG、JPEG、BMP、TGA
- 内存管理通过 Zig 的自定义分配器实现
- WASM64 平台有特殊内存限制
- 纹理尺寸需符合 GPU 规范（2的幂次方）