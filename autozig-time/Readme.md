# autozig-time

`autozig-time` 是 Bevy 引擎的 Zig 语言时间管理模块，提供精确的时间测量和定时器功能。

## 核心功能
- **时间测量**：实现高精度的系统时间访问
- **定时器系统**：支持单次和重复定时器
- **WASM64 兼容**：针对 WebAssembly 平台的特殊时间处理
- **Zig 原生实现**：通过 Zig 代码优化时间计算性能

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-time = { path = "autozig_bevy/autozig-time" }
```

在 Bevy 应用中集成：
```rust
use autozig_time::{TimePlugin, Timer, TimerMode};

fn main() {
    App::build()
        .add_plugin(TimePlugin)
        .insert_resource(Timer::new(2.0, TimerMode::Repeating))
        .add_system(timer_system.system())
        .run();
}

fn timer_system(time: Res<Time>, mut timer: ResMut<Timer>) {
    if timer.tick(time.delta()).just_finished() {
        println!("Timer finished!");
    }
}
```

## 核心组件
| 组件 | 说明 |
|------|------|
| `Time` | 全局时间资源，包含帧时间信息 |
| `Timer` | 单个定时器实例 |
| `Stopwatch` | 可暂停的计时器 |

## 时间类型
- **`Duration`**：表示时间间隔
- **`Instant`**：表示绝对时间点
- **`TimerMode`**：定时器模式（单次/重复）

## 注意事项
- 所有时间操作在 Zig 层实现，确保高精度
- 定时器自动处理帧时间累积
- WASM64 平台使用浏览器时间 API
- 时间资源在每帧自动更新
- 支持暂停和恢复时间流