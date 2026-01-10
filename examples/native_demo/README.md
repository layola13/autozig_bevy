# AutoZig-Bevy Native & WASM64 Demo

完整的双目标示例，展示 autozig_bevy 的所有核心功能。

## 🎯 目标

- **Target**: Native + WASM64 双目标
- **Language**: Rust + Zig
- **Performance**: SIMD 优化
- **Dependencies**: 仅使用 autozig_bevy crates
- **Export**: 使用 AutoZig 的 `#[autozig_export]` (无 wasm-bindgen)

## 📦 包含模块

### 模块 0: App & Plugin System
- App 应用架构
- Plugin 插件系统
- System 系统调度
- 演示文件: [`src/demo_app.rs`](src/demo_app.rs)

### 模块 1: ECS Architecture
- Entity 实体管理
- Component 组件系统
- Query 查询系统
- Resource 资源管理
- Event 事件系统
- Command 命令系统
- SystemParam 参数注入
- 变更检测
- 演示文件: [`src/demo_ecs.rs`](src/demo_ecs.rs)

### 模块 2: Math Library
- Vec2/Vec3/Vec4 向量运算
- Mat2/Mat3/Mat4 矩阵运算
- Quat 四元数旋转
- 几何图元 (Circle, Sphere, Cuboid, Cylinder, Capsule)
- 边界盒 (Aabb2d, Aabb3d)
- 曲线系统 (Bezier, Hermite, Catmull-Rom, B-Spline)
- 方向和旋转 (Dir2, Dir3, Rot2)
- 变换系统 (Isometry, Affine)
- 实用工具 (AspectRatio, EaseFunction, FloatOrd)
- 演示文件: [`src/demo_math.rs`](src/demo_math.rs)

### 模块 3: State Management
- State<T> 状态管理
- OnEnter/OnExit 转换钩子
- StateScoped 作用域实体
- 状态条件系统
- 游戏状态机示例
- 演示文件: [`src/demo_state.rs`](src/demo_state.rs)

### 模块 4: Time & Task System
- Time 资源 (帧时间、总运行时间)
- Stopwatch 秒表 (性能测量)
- Timer 计时器 (一次性/循环)
- TaskPool 任务池 (并行计算)
- 时间工具函数
- 演示文件: [`src/demo_time_task.rs`](src/demo_time_task.rs)

### 模块 5: JSON Parsing
- SIMD 优化解析
- Tape-based 架构
- 零依赖设计 (无 serde)
- json! 宏支持
- 类型安全的 AutoDeserialize
- 演示文件: [`src/demo_json.rs`](src/demo_json.rs)

## 🚀 编译运行

### Native 编译

#### Debug 版本
```bash
cd autozig_bevy/examples/native_demo
cargo build
```

#### 运行
```bash
cargo run --bin native_demo
```

或者直接：
```bash
cargo run
```

#### Release 版本
```bash
cargo build --release
cargo run --release
```

### WASM64 编译

#### 前置要求
```bash
# 安装 Rust nightly（WASM64 是 tier-3 target）
rustup install nightly
rustup component add rust-src --toolchain nightly
```

#### 编译 WASM64
```bash
# 使用 nightly 编译 WASM64 target
cargo +nightly build --target wasm64-unknown-unknown -Z build-std=std,panic_abort --release
```

#### WASM 文件位置
编译成功后，WASM 文件位于：
```
target/wasm64-unknown-unknown/release/native_demo.wasm
```

#### 运行 WASM Demo
```bash
# 复制 WASM 文件到 www 目录
cp target/wasm64-unknown-unknown/release/native_demo.wasm www/

# 启动本地服务器
cd www
python3 -m http.server 8000

# 浏览器访问
open http://localhost:8000
```

## 📊 测试结果

### Native Target
✅ **编译成功**: 所有模块编译通过
✅ **运行成功**: 所有演示正常执行
✅ **依赖正确**: 仅使用 autozig_bevy crates
✅ **Native Binary**: 成功编译为 native 二进制

### WASM64 Target
✅ **编译成功**: WASM64 编译通过
✅ **导出函数**: 使用 `#[autozig_export]` 导出
✅ **HTML 页面**: 完整的 Web 演示界面
✅ **64-bit 指针**: 支持 WASM64 大内存

## 🔧 依赖的 AutoZig-Bevy Crates

```toml
autozig-app = { path = "../../autozig-app" }
autozig-ecs = { path = "../../autozig-ecs" }
autozig-math = { path = "../../autozig-math" }
autozig-state = { path = "../../autozig-state" }
autozig-time = { path = "../../autozig-time" }
autozig-tasks = { path = "../../autozig-tasks" }
autozig_json = { path = "../../autozig_json" }
```

## 📁 项目结构

```
native_demo/
├── Cargo.toml          # 项目配置（支持 Native + WASM64）
├── build.rs            # 构建脚本（自动检测目标）
├── README.md           # 本文档
├── src/
│   ├── main.rs         # Native 入口
│   ├── lib.rs          # WASM 入口（使用 autozig_export）
│   ├── demo_app.rs     # 模块 0: App 示例
│   ├── demo_ecs.rs     # 模块 1: ECS 示例
│   ├── demo_math.rs    # 模块 2: Math 示例
│   ├── demo_state.rs   # 模块 3: State 示例
│   ├── demo_time_task.rs  # 模块 4: Time & Task 示例
│   └── demo_json.rs    # 模块 5: JSON 示例
└── www/
    └── index.html      # WASM 测试页面
```

## 💡 特性亮点

1. **纯 autozig_bevy 实现**: 不引入任何外部依赖
2. **双目标支持**: Native + WASM64 双编译
3. **无 wasm-bindgen**: 使用 AutoZig 原生导出 `#[autozig_export]`
4. **完整功能展示**: 覆盖所有核心模块
5. **清晰的代码结构**: 每个模块独立展示
6. **详细的输出日志**: 运行时输出详细信息
7. **实用的示例**: 每个模块都有实际应用场景说明
8. **Web 演示页面**: 精美的 HTML 界面展示

## 🎓 学习建议

1. 先运行完整 demo 了解整体功能
2. 逐个模块查看源代码
3. 尝试修改示例代码进行实验
4. 参考各模块的文档和测试用例

## 🔗 相关链接

- AutoZig 主项目: [../../README.md](../../README.md)
- AutoZig-Bevy 文档: [../../autozig_bevy/README.md](../../autozig_bevy/README.md)
- 其他示例: [../](../)

## ✨ 总结

这个 native_demo 展示了 autozig_bevy 作为游戏引擎核心的完整能力：

- ✅ 强大的 ECS 架构
- ✅ 完整的数学库
- ✅ 灵活的状态管理
- ✅ 精确的时间系统
- ✅ 高性能的 JSON 解析
- ✅ 可扩展的插件系统

所有功能均基于 Zig + Rust 实现，提供 SIMD 优化和高性能支持。

## 📝 运行输出示例

### Native 运行输出

程序运行时会显示精美的 Banner 和详细的模块演示：

```
╔═══════════════════════════════════════════════════════════════╗
║            AutoZig-Bevy Native Target Demo                    ║
║  高性能游戏引擎核心功能展示 (Zig + Rust)                      ║
╚═══════════════════════════════════════════════════════════════╝

🎯 Target: Native (非 WASM)
🦀 Language: Rust + Zig
⚡ Performance: SIMD 优化
📦 Dependencies: 仅 autozig_bevy crates

[运行所有 6 个模块示例...]
```

每个模块都会输出：
- 功能说明
- 使用示例
- API 演示
- 实际应用场景

### WASM 运行输出

在浏览器中打开 `www/index.html`，可以看到：
- 精美的 Web 界面
- 实时模块执行日志
- 交互式按钮控制
- 系统信息显示（版本、指针大小、架构）

## 🔧 技术细节

### AutoZig Export vs wasm-bindgen

本项目使用 AutoZig 原生的 `#[autozig_export]` 宏而非 wasm-bindgen：

**优势：**
- ✅ 零额外依赖
- ✅ 更小的 WASM 文件
- ✅ 更快的编译速度
- ✅ 与 Zig 代码无缝集成
- ✅ 支持 WASM64 (64-bit 指针)

**示例：**
```rust
use autozig::autozig_export;

#[autozig_export]
pub fn demo_run_math() -> u32 {
    demo_math::run_demo();
    1 // success
}
```

JavaScript 可以直接调用：
```javascript
const result = wasmModule.demo_run_math();
```

### WASM64 特性

- **64-bit 指针**: 支持超过 4GB 内存
- **Tier-3 Target**: 需要 nightly 工具链
- **build-std**: 需要重新编译标准库
- **兼容性**: 现代浏览器支持（Chrome 91+, Firefox 89+）

## � 已知问题

### Native
无 - 所有功能正常工作

### WASM64
- WASM64 是 tier-3 target，需要 nightly 工具链
- 某些浏览器可能不支持 WASM64（建议使用最新版 Chrome/Firefox）
- SIMD 优化在 WASM 中可能受限

## 📄 许可证

MIT OR Apache-2.0