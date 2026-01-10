# autozig-tasks

`autozig-tasks` 是 Bevy 引擎的 Zig 语言任务调度模块，提供高性能的异步任务执行和线程池管理功能。

## 核心功能
- **任务调度**：实现细粒度的任务依赖管理和执行顺序控制
- **线程池管理**：优化多线程任务分配和负载均衡
- **Zig 原生实现**：通过 Zig 代码实现高效的并发原语
- **WASM 兼容**：针对 WebAssembly 平台的特殊任务调度优化

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-tasks = { path = "autozig_bevy/autozig-tasks" }
```

在 Bevy 应用中集成：
```rust
use autozig_tasks::{TaskPoolPlugin, Task};

fn main() {
    App::build()
        .add_plugin(TaskPoolPlugin::default())
        .add_system(spawn_task.system())
        .run();
}

fn spawn_task(task_pool: Res<TaskPool>) {
    let task = task_pool.spawn(async {
        // 异步任务逻辑
        42
    });
}
```

## 核心组件
| 组件 | 说明 |
|------|------|
| `TaskPool` | 线程池管理器 |
| `Task` | 异步任务句柄 |
| `TaskContext` | 任务执行上下文 |

## 任务类型
- **计算任务**：CPU 密集型操作
- **I/O 任务**：异步 I/O 操作
- **组合任务**：任务链和并行执行

## 注意事项
- 所有任务调度在 Zig 层实现，确保低开销
- 线程池大小自动适配 CPU 核心数
- 任务依赖通过显式声明管理
- WASM 平台使用单线程模拟
- 任务取消需通过显式 API 调用