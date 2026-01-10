# autozig-color

`autozig-color` 是 Bevy 引擎的 Zig 语言颜色处理模块，提供高性能的颜色空间转换和标准颜色定义。

## 核心功能
- **颜色空间支持**：实现 RGBA、HSLA、HSVA 和线性 RGBA 等多种颜色空间
- **标准色库**：预定义 140+ 种 CSS 标准颜色常量
- **转换工具**：提供精确的颜色空间转换算法
- **Zig 原生实现**：通过 Zig 代码实现高性能颜色计算

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-color = { path = "autozig_bevy/autozig-color" }
```

在 Bevy 应用中使用：
```rust
use autozig_color::{Color, StandardColors};

fn main() {
    let red = Color::RED;
    let hsla = red.to_hsla();
    println!("HSLA: {:?}", hsla);
}
```

## 颜色类型
| 类型 | 说明 |
|------|------|
| `Color` | 主颜色类型，支持所有颜色空间 |
| `Hsla` | HSLA 颜色表示 |
| `Hsva` | HSVA 颜色表示 |
| `LinearRgba` | 线性 RGBA 颜色表示 |

## 注意事项
- 所有颜色计算均在 Zig 层实现，确保计算效率
- 标准颜色常量通过 `StandardColors` trait 提供
- 颜色转换时会自动处理 gamma 校正
- 线性颜色空间适用于物理渲染计算