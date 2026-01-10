# autozig-input

`autozig-input` 是 Bevy 引擎的 Zig 语言输入处理模块，提供跨平台的输入事件处理和设备状态管理功能。

## 核心功能
- **多设备支持**：实现键盘、鼠标、游戏手柄和触摸屏输入处理
- **事件驱动架构**：基于事件的输入状态变更通知
- **Zig 原生实现**：通过 Zig 代码优化输入处理性能
- **WASM 兼容**：针对 Web 平台的特殊输入处理

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-input = { path = "autozig_bevy/autozig-input" }
```

在 Bevy 应用中集成：
```rust
use autozig_input::{InputPlugin, KeyboardInput};

fn main() {
    App::build()
        .add_plugin(InputPlugin)
        .add_system(keyboard_system.system())
        .run();
}

fn keyboard_system(input: Res<KeyboardInput>) {
    if input.pressed(KeyCode::Space) {
        println!("Space pressed!");
    }
}
```

## 输入类型
| 类型 | 说明 |
|------|------|
| `KeyboardInput` | 键盘按键状态管理 |
| `MouseInput` | 鼠标位置和按键状态 |
| `GamepadInput` | 游戏手柄输入处理 |
| `TouchInput` | 触摸屏事件处理 |

## 核心特性
- **按键状态**：支持按下、释放、持续按住状态检测
- **坐标系统**：统一处理屏幕坐标和世界坐标转换
- **设备枚举**：自动检测和管理连接的输入设备
- **事件队列**：缓冲输入事件确保不丢失

## 注意事项
- 所有输入处理在 Zig 层实现，确保低延迟
- 鼠标坐标系遵循 Bevy 的屏幕坐标规范
- 游戏手柄支持标准 XInput 和 DirectInput
- Web 平台需处理浏览器安全限制
- 输入事件在每帧开始时重置状态