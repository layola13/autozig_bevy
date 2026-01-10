# autozig-macro-utils

`autozig-macro-utils` 是 Bevy 引擎的 Zig 语言宏工具模块，提供基础的宏处理和代码生成支持。

## 核心功能
- **宏属性处理**：实现 Bevy 宏属性的解析和验证
- **符号管理**：提供 Rust 标识符和符号的处理工具
- **结果筛选**：实现宏展开结果的过滤和验证
- **Zig 原生实现**：通过 Zig 代码优化宏处理性能

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-macro-utils = { path = "autozig_bevy/autozig-macro-utils" }
```

在 Bevy 宏实现中使用：
```rust
use autozig_macro_utils::{BevyManifest, FqStd, Label};

#[derive(Label)]
struct MyLabel;

fn main() {
    let manifest = BevyManifest::load();
    println!("Bevy version: {}", manifest.version);
}
```

## 核心组件
| 组件 | 说明 |
|------|------|
| `BevyManifest` | Bevy 项目元数据访问 |
| `FqStd` | 标准库路径处理 |
| `Label` | 标签系统宏支持 |
| `ResultSifter` | 宏展开结果过滤 |

## 典型用例
1. Bevy 宏的属性解析
2. 代码生成过程中的符号处理
3. 项目元数据访问
4. 宏展开结果验证

## 注意事项
- 所有宏处理在 Zig 层实现，确保编译期性能
- 依赖 Bevy 项目的 `Cargo.toml` 配置
- 仅在编译期使用，不参与运行时
- 与 Bevy 的宏系统深度集成
- 需配合其他 autozig 模块使用