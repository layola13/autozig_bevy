# autozig-math

`autozig-math` 是 Bevy 引擎的 Zig 语言数学计算模块，提供高性能的向量、矩阵和几何运算支持。

## 核心功能
- **向量运算**：实现 2D/3D/4D 向量的完整数学操作
- **矩阵计算**：提供 3x3/4x4 矩阵的变换和投影功能
- **几何工具**：包含射线、平面、AABB 等几何体操作
- **Zig 原生实现**：通过 Zig 代码优化数学计算性能

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-math = { path = "autozig_bevy/autozig-math" }
```

在 Bevy 应用中使用：
```rust
use autozig_math::{Vec3, Mat4};

fn main() {
    let position = Vec3::new(1.0, 2.0, 3.0);
    let rotation = Mat4::from_rotation_y(0.5);
    let transformed = rotation * position;
}
```

## 核心类型
| 类型 | 说明 |
|------|------|
| `Vec2`/`Vec3`/`Vec4` | 2D/3D/4D 向量类型 |
| `Mat3`/`Mat4` | 3x3/4x4 矩阵类型 |
| `Quat` | 四元数旋转表示 |
| `Ray` | 射线几何体 |
| `Aabb` | 轴对齐包围盒 |

## 常用操作
- **向量运算**：加减乘除、点积、叉积、归一化
- **矩阵变换**：平移、旋转、缩放、投影
- **几何测试**：射线-平面相交、AABB 碰撞检测
- **坐标转换**：世界坐标↔屏幕坐标转换

## 注意事项
- 所有数学计算在 Zig 层实现，确保计算效率
- 矩阵采用列优先存储格式
- 旋转操作支持欧拉角和四元数两种表示
- 几何工具需配合渲染管线使用
- 数值精度遵循 IEEE 754 标准