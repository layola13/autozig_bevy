# autozig-core-pipeline

`autozig-core-pipeline` 是 Bevy 引擎的 Zig 语言渲染管线核心模块，提供底层图形 API 的抽象和高效渲染命令管理。

## 核心功能
- **命令编码器**：实现 GPU 命令的高效编码和提交
- **渲染通道调度**：优化渲染通道的执行顺序和资源依赖
- **管线状态管理**：维护渲染管线的动态状态配置
- **资源屏障处理**：自动管理 GPU 资源的同步和访问权限

## 使用指南
在 `Cargo.toml` 中添加依赖：
```toml
autozig-core-pipeline = { path = "autozig_bevy/autozig-core-pipeline" }
```

在 Bevy 应用中集成：
```rust
use autozig_core_pipeline::{RenderPipelinePlugin, CommandEncoder};

fn main() {
    App::build()
        .add_plugin(RenderPipelinePlugin)
        .run();
}
```

## 核心组件
| 组件 | 说明 |
|------|------|
| `CommandEncoder` | GPU 命令编码和提交接口 |
| `PassScheduler` | 渲染通道执行调度器 |
| `PipelineState` | 渲染管线状态配置 |
| `ResourceBarrier` | GPU 资源同步管理 |

## 渲染流程
1. 创建渲染管线配置
2. 通过 `CommandEncoder` 编码渲染命令
3. `PassScheduler` 优化通道执行顺序
4. 提交命令到 GPU 执行

## 注意事项
- 所有渲染操作需在渲染线程执行
- 资源屏障管理确保 GPU 内存安全
- 管线状态变更会触发自动优化
- 与 Bevy 渲染器深度集成
- 依赖 Zig 实现的底层图形 API 绑定