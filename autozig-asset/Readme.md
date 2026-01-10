# autozig-asset

`autozig-asset` 是 Bevy 引擎的 Zig 语言资源管理模块，提供高效的资源加载、存储和事件处理功能。

## 核心功能
- **资源服务器**：实现资源的异步加载和生命周期管理
- **资源存储**：基于句柄的资源缓存与检索系统
- **事件驱动**：资源加载完成/失败事件通知机制
- **路径解析**：统一处理资源路径的标准化和验证

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-asset = { path = "autozig_bevy/autozig-asset" }
```

在 Bevy 应用中集成：
```rust
use autozig_asset::AssetPlugin;

fn main() {
    App::build()
        .add_plugin(AssetPlugin)
        .run();
}
```

## 典型工作流
1. 通过 `AssetServer` 加载资源
2. 使用 `Handle<T>` 管理资源引用
3. 监听 `AssetEvent` 处理加载状态
4. 通过 `AssetStorage` 访问已加载资源

## 注意事项
- 所有资源操作需在 Bevy 的主线程执行
- 路径格式需符合 `assets/` 前缀规范
- 内存管理通过 Zig 的自定义分配器实现