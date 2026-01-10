# autozig-json

`autozig-json` 是 Bevy 引擎的 Zig 语言 JSON 处理模块，提供高性能的 JSON 解析和序列化功能。

## 核心功能
- **流式解析**：实现基于 tape 的高效 JSON 解析器
- **错误处理**：提供详细的解析错误定位和诊断
- **Zig 原生实现**：通过 Zig 代码优化 JSON 处理性能
- **内存安全**：使用自定义分配器管理解析过程中的内存

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-json = { path = "autozig_bevy/autozig-json" }
```

在 Bevy 应用中使用：
```rust
use autozig_json::{JsonParser, JsonValue};

fn main() {
    let json = r#"{"name": "Bevy", "version": 0.12}"#;
    let parser = JsonParser::new(json);
    let value = parser.parse().unwrap();
    
    if let JsonValue::Object(obj) = value {
        println!("Name: {}", obj.get("name").unwrap());
    }
}
```

## 核心组件
| 组件 | 说明 |
|------|------|
| `JsonParser` | JSON 解析器主接口 |
| `JsonValue` | JSON 数据的 Rust 表示 |
| `JsonTape` | 底层解析器的 tape 结构 |
| `JsonError` | 详细的解析错误类型 |

## 解析流程
1. 创建 `JsonParser` 实例
2. 调用 `parse()` 执行解析
3. 处理返回的 `JsonValue` 结果
4. 访问解析后的数据结构

## 注意事项
- 所有 JSON 操作在 Zig 层实现，确保高性能
- 支持标准 JSON 格式和常见扩展
- 内存管理通过 Zig 的自定义分配器实现
- 错误信息包含精确的行号和列号
- 适用于配置加载和网络数据处理