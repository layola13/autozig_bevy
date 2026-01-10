# autozig-reflect

`autozig-reflect` 是 Bevy 引擎的 Zig 语言反射系统模块，提供运行时类型信息和序列化支持。

## 核心功能
- **类型反射**：实现 Rust 类型的运行时元数据访问
- **序列化支持**：提供类型安全的序列化/反序列化功能
- **WASM64 兼容**：针对 WebAssembly 平台的特殊优化
- **Zig 原生实现**：通过 Zig 代码优化反射性能

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-reflect = { path = "autozig_bevy/autozig-reflect" }
```

在 Bevy 应用中集成：
```rust
use autozig_reflect::{ReflectPlugin, Reflect};

#[derive(Reflect)]
struct Player {
    health: f32,
    position: Vec3,
}

fn main() {
    App::build()
        .add_plugin(ReflectPlugin)
        .register_type::<Player>()
        .run();
}
```

## 核心特性
- **类型注册**：通过 `register_type` 注册可反射类型
- **动态访问**：运行时获取字段和方法信息
- **序列化**：支持 JSON 和二进制格式
- **类型转换**：安全的类型转换和验证

## 典型用例
1. 编辑器中的组件检查器
2. 网络同步数据序列化
3. 动态资源加载和配置
4. 脚本系统类型绑定

## 注意事项
- 所有反射操作需在主线程执行
- 类型需实现 `Reflect` trait
- WASM64 平台有内存限制
- 反射数据通过 Zig 层高效管理
- 需配合 `autozig-macro-utils` 使用