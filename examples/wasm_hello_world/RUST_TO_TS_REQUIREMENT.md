# Rust 直接生成 TypeScript 绑定需求

## 问题背景

当前 wasm_hello_world 示例遇到架构冲突：

1. **需求**：
   - 逻辑全部用 Rust 编写（autozig-ecs 的 App+System）
   - 生成 TypeScript 绑定供 JavaScript 调用
   - 支持 wasm64-unknown-unknown 目标

2. **现状限制**：
   - `wasm-bindgen` 不支持 wasm64
   - `autozig::include_zig!` 只能从 Zig 代码生成绑定
   - 混合架构（Rust 逻辑 + Zig wrapper）导致链接问题

## 解决方案设计

### 方案 A：扩展 AutoZig 宏支持 Rust 函数（推荐）

添加新的宏 `#[autozig::export_rust]` 直接从 Rust 函数生成 TypeScript 绑定：

```rust
use autozig::export_rust;

#[export_rust]
pub fn run_hello_world() {
    let mut app = App::new();
    app.add_systems(|| {
        // ECS 系统逻辑
    });
    app.run();
}

#[export_rust]
pub fn get_system_count() -> u32 {
    5
}
```

**优点**：
- 纯 Rust 实现，无需 Zig wrapper
- 自动生成 TypeScript 绑定
- 支持 wasm64

**实现要点**：
1. 宏展开时生成 `#[no_mangle]` 和 `extern "C"` 的导出函数
2. 提取函数签名生成 TypeScript 类型定义
3. 生成 JavaScript 加载器代码

### 方案 B：简化的 Zig 薄包装层

保持当前架构，但让 Zig 函数为空，JavaScript 直接调用 Rust 导出：

```zig
// wrapper.zig - 只用于生成 TS 绑定
export fn run_hello_world() void {}
export fn get_system_count() u32 { return 0; }
```

```rust
// lib.rs - 实际实现
#[no_mangle]
pub extern "C" fn run_hello_world() {
    // 真正的实现
}
```

**优点**：
- 最小改动
- 利用现有 AutoZig 绑定生成

**缺点**：
- 需要手动同步 Rust 和 Zig 的函数签名
- Zig 代码完全是占位符

## 当前采用方案

**临时方案**：方案 B（简化 Zig 包装层）

**长期计划**：方案 A（扩展 AutoZig 宏）

## 下一步行动

1. 实现方案 B 让示例先运行起来
2. 在 `autozig` crate 中设计并实现 `#[export_rust]` 宏
3. 迁移示例到新的宏 API

## 相关文件

- `/autozig/macro/src/lib.rs` - 需要添加 `export_rust` 宏
- `/autozig/engine/src/lib.rs` - 需要支持解析 Rust 函数签名
- `/autozig_bevy/examples/wasm_hello_world/src/lib.rs` - 示例实现