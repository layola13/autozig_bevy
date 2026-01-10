# AutoZig-Bevy WASM Hello World

这是一个使用 **AutoZig-Bevy ECS** 框架的完整 WASM 示例，展示了如何在 Web 环境中使用 App 和 System 架构。

## 🎯 特性

- ✅ 使用 `autozig-ecs` 的 App 和 System 架构
- ✅ 在浏览器控制台输出 "Hello World"
- ✅ 5 个不同的闭包系统演示
- ✅ 计数器状态管理示例
- ✅ 模拟游戏循环（Update + Render）
- ✅ 完全符合安全约束（无指针保存、同步执行）

## 📋 系统列表

1. **System 1**: 输出 Hello World 消息
2. **System 2**: 显示使用的框架名称（捕获变量演示）
3. **System 3**: 执行计数器（状态管理演示）
4. **System 4**: 模拟游戏状态更新
5. **System 5**: 模拟渲染管线

## 🔒 安全约束

遵循 AutoZig 的安全规则：

1. ✅ 所有 Zig 函数调用立即使用并返回
2. ✅ 不保存 raw pointer 到 Rust struct
3. ✅ 必须同步完成，不使用 spawn
4. ✅ 避免跨 FFI 边界的复杂数据结构

## 🚀 构建和运行

### 方法 1: 使用 wasm-pack（推荐）

```bash
# 构建
wasm-pack build --target web --release

# 启动服务器
cd www
python3 -m http.server 8080

# 打开浏览器
# http://localhost:8080
```

### 方法 2: 手动构建

```bash
# 构建 WASM
cargo build --target wasm32-unknown-unknown --release

# 使用 wasm-bindgen 生成绑定
wasm-bindgen \
    --out-dir www/pkg \
    --target web \
    target/wasm32-unknown-unknown/release/autozig_wasm_hello_world.wasm

# 启动服务器
cd www
python3 -m http.server 8080
```

## 📖 使用说明

1. 打开浏览器访问 `http://localhost:8080`
2. **打开开发者工具（F12）查看控制台**
3. 点击页面上的按钮：
   - **运行 Hello World**: 执行完整的 App 和所有系统
   - **获取系统数量**: 测试系统注册功能
   - **运行 3 次迭代**: 演示多次执行

## 🎮 架构说明

```rust
// 创建应用
let mut app = App::new();

// 注册系统
app.add_systems(|| {
    console::log_1(&"Hello World!".into());
});

// 运行应用（执行所有系统）
app.run();
```

## 📦 依赖

- `autozig-ecs`: AutoZig ECS 核心库
- `wasm-bindgen`: Rust 到 JavaScript 绑定
- `web-sys`: Web API 访问（console 日志）

## 🔧 技术细节

- **目标**: wasm32-unknown-unknown / wasm64-unknown-unknown
- **编译模式**: MODULAR_BUILDZIG（避免代码重复）
- **优化**: LTO 关闭（防止符号被优化掉）
- **运行时**: 浏览器 WebAssembly 环境

## 📝 代码结构

```
wasm_hello_world/
├── Cargo.toml          # 依赖配置
├── build.rs            # 构建脚本（设置 MODULAR_BUILDZIG）
├── src/
│   └── lib.rs          # 主代码（App + Systems）
└── www/
    ├── index.html      # Web 界面
    └── pkg/            # wasm-bindgen 生成的文件
```

## ✅ 验证测试

运行后在浏览器控制台应该看到：

```
🚀 AutoZig-Bevy WASM Hello World Demo
===================================

📦 创建 AutoZig-ECS App...

✅ 已注册 5 个系统

🔄 执行系统:
─────────────────
[System 1] 👋 Hello World from AutoZig-Bevy!
[System 2] ⚙️  使用框架: AutoZig-ECS
[System 3] 🔢 执行计数: 1
[System 4] 🎮 Update: 更新游戏状态
[System 5] 🎨 Render: 渲染当前帧
─────────────────

✅ Hello World Demo 完成!
💡 所有系统已成功执行
```

## 🎓 学习要点

1. **App 创建**: `App::new()` 创建应用实例
2. **系统注册**: `app.add_systems(closure)` 注册闭包系统
3. **系统执行**: `app.run()` 执行所有注册的系统
4. **状态管理**: 使用 `Cell` 在闭包中管理状态
5. **WASM 集成**: 使用 `#[wasm_bindgen]` 导出函数给 JavaScript

## 🔗 相关资源

- [AutoZig-ECS 文档](../../autozig-ecs/README.md)
- [WASM ECS 示例](../../autozig-ecs/examples/wasm_ecs/)
- [Bevy ECS 概念](https://bevyengine.org/learn/book/getting-started/ecs/)