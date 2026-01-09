# AutoZig FFI Allocator Fix

## 问题

`RemovedComponents` 测试失败，`removed_components_init` FFI 函数返回 `null`。

## 根本原因

Zig 的 `?*T` (可选指针) 和 Rust 的 `Option<*mut T>` 在 FFI 边界上 **ABI 不兼容**。

## 解决方案

参考工作正常的 `autozig-utils/hashmap.zig` 模式：

### 1. 本地 GPA Allocator

每个 Zig 文件直接定义自己的 `gpa_instance`，不依赖跨文件导入：

```zig
// removed_components.zig
const std = @import("std");

var gpa_instance = std.heap.GeneralPurposeAllocator(.{}){};
const allocator = gpa_instance.allocator();
```

### 2. 非可选返回类型

使用 `*T` + `catch unreachable` 而不是 `?*T` + `catch return null`：

```zig
// ❌ 错误方式
export fn removed_components_init(id: u32) ?*RemovedComponents {
    const ptr = allocator.create(RemovedComponents) catch return null;
    // ...
}

// ✅ 正确方式
export fn removed_components_init(id: u32) *RemovedComponents {
    const ptr = allocator.create(RemovedComponents) catch unreachable;
    // ...
}
```

### 3. Rust FFI 使用原始指针

```rust
// ❌ 错误方式
fn removed_components_init(id: u32) -> Option<*mut Opaque>;

// ✅ 正确方式  
fn removed_components_init(id: u32) -> *mut Opaque;
```

### 4. Build.rs 清理

在构建前删除可能损坏的文件：

```rust
// build.rs
let out_dir = std::env::var("OUT_DIR").unwrap();
let lib_path = Path::new(&out_dir).join("libautozig.a");
if lib_path.exists() {
    let _ = std::fs::remove_file(&lib_path);
}
```

## 测试结果

```
test result: ok. 28 passed; 0 failed; 0 ignored
```
