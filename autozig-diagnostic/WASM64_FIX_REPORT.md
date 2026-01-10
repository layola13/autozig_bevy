# autozig-diagnostic WASM64 修复报告

## 修复日期
2026-01-10

## 问题描述
autozig-diagnostic crate 在编译为 wasm64-unknown-unknown 目标时出现以下错误：

```
error: Unsupported operating system freestanding
  Thread.getCurrentId()
  
error: struct 'posix.system' has no member named 'SEEK'
error: struct 'posix.system' has no member named 'STDERR_FILENO'
error: struct 'posix.system' has no member named 'writev'
error: struct 'posix.system' has no member named 'pwritev'
error: struct 'posix.system' has no member named 'lseek'
```

## 根本原因
在 `diagnostic_store.zig` 中使用了 `std.debug.print()` 进行调试输出。在 WASM freestanding 环境下：
- `std.debug.print()` 依赖 POSIX 系统调用（`writev`, `pwritev`, `lseek` 等）
- 这些系统调用在 freestanding 环境中不可用
- Thread API 在 freestanding 环境下也不支持

## 修复方案
采用条件编译策略，在 WASM 环境下禁用 debug 输出：

### 修改文件
- `autozig_bevy/autozig-diagnostic/src/zig/diagnostic_store.zig`

### 修改内容
在 `getByHash()` 函数中添加 WASM 检测：

```zig
pub fn getByHash(self: *DiagnosticsStore, hash: u64) ?*Diagnostic {
    // Debug logging only in non-WASM environments (POSIX calls unavailable in WASM)
    const builtin = @import("builtin");
    if (!builtin.cpu.arch.isWasm()) {
        std.debug.print("DEBUG Zig: getByHash called with hash={}, items.len={}\n", .{ hash, self.diagnostics.items.len });
    }
    
    for (self.diagnostics.items, 0..) |entry, i| {
        if (!builtin.cpu.arch.isWasm()) {
            std.debug.print("DEBUG Zig: checking entry[{}]: hash={}\n", .{ i, entry.hash });
        }
        if (entry.hash == hash) {
            if (!builtin.cpu.arch.isWasm()) {
                std.debug.print("DEBUG Zig: found match at index {}\n", .{i});
            }
            return entry.diagnostic;
        }
    }
    if (!builtin.cpu.arch.isWasm()) {
        std.debug.print("DEBUG Zig: no match found\n", .{});
    }
    return null;
}
```

## 修复特点
✅ **无 unsafe 代码** - 完全使用 Zig 条件编译
✅ **保持调试功能** - Native 环境下仍可正常输出调试信息
✅ **WASM 兼容** - WASM 环境下跳过所有 POSIX 依赖的调用
✅ **零性能开销** - 编译时决策，运行时无额外开销

## 验证结果
```bash
cd autozig_bevy/autozig-diagnostic
cargo build --target wasm64-unknown-unknown --release -Zbuild-std=std,panic_abort
```

**结果**: ✅ 编译成功通过

```
Finished `release` profile [optimized] target(s) in 0.43s
```

## 技术要点
1. **条件编译**: 使用 `builtin.cpu.arch.isWasm()` 检测目标平台
2. **POSIX 隔离**: 在 WASM 下完全避免使用依赖 POSIX 的标准库功能
3. **调试友好**: Native 环境保留完整调试输出能力
4. **编译时优化**: WASM 构建时自动移除所有调试代码

## 对比其他 crate
与 autozig-time 和 autozig-utils 的修复模式一致：
- 使用 `builtin.cpu.arch.isWasm()` 进行平台检测
- WASM 下禁用或替换不兼容的系统调用
- Native 下保持原有功能

## 未来改进建议
如需在 WASM 环境下进行调试：
1. 可以实现自定义的日志缓冲区
2. 通过 FFI 将日志传递到 JavaScript console
3. 使用 WASM-compatible 的日志库（如果有）

## 状态
✅ **修复完成** - 所有 WASM64 编译错误已解决
✅ **编译通过** - 目标 wasm64-unknown-unknown 编译成功
✅ **代码质量** - 无 unsafe，遵循项目规范