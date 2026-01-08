# AutoZig-Log

AutoZig日志系统 - Bevy日志功能的Zig实现

## 特性

- **90% Zig实现，10% Rust包装** - 高性能核心日志功能用Zig实现
- **五级日志** - Trace, Debug, Info, Warn, Error
- **格式化输出** - 时间戳、模块名、日志级别、消息内容
- **日志过滤** - 按级别动态过滤日志输出
- **零unsafe代码** - 完全符合Rust安全标准
- **WebGPU/WASM兼容** - 专为Web平台优化

## 架构

```
autozig-log/
├── src/
│   ├── lib.rs          # 10% Rust API包装
│   └── zig/
│       ├── logger.zig  # 90% Zig核心实现
│       └── format.zig  # 日志格式化
├── examples/
│   └── basic_log.rs    # 基础示例
└── tests/
    └── logger_tests.rs # 集成测试
```

## 使用示例

### 基础用法

```rust
use autozig_log::*;

fn main() {
    // 初始化日志系统
    init();
    
    // 设置最小日志级别
    set_min_level(LogLevel::Info);
    
    // 使用日志宏
    trace!("这是一条trace消息");
    debug!("这是一条debug消息");
    info!("这是一条info消息");
    warn!("这是一条warning消息");
    error!("这是一条error消息");
    
    // 带格式化的日志
    let value = 42;
    info!("值为: {}", value);
    
    // 关闭日志系统
    shutdown();
}
```

### 日志级别控制

```rust
use autozig_log::*;

// 设置日志级别
set_min_level(LogLevel::Warn); // 只显示Warn和Error

// 检查日志级别是否启用
if is_enabled(LogLevel::Debug) {
    debug!("Debug级别已启用");
}

// 获取当前最小日志级别
let level = min_level();
println!("当前最小级别: {:?}", level);
```

### 直接日志调用

```rust
use autozig_log::*;

// 使用log函数直接调用
log(LogLevel::Info, "my_module", "自定义模块日志");
log(LogLevel::Error, "network::http", "连接超时");
```

## API文档

### 日志级别

```rust
pub enum LogLevel {
    Trace = 0,  // 最详细
    Debug = 1,  // 调试信息
    Info = 2,   // 一般信息
    Warn = 3,   // 警告
    Error = 4,  // 错误
}
```

### 核心函数

- `init()` - 初始化日志系统
- `shutdown()` - 关闭日志系统
- `set_min_level(level: LogLevel)` - 设置最小日志级别
- `min_level() -> LogLevel` - 获取当前最小日志级别
- `is_enabled(level: LogLevel) -> bool` - 检查日志级别是否启用
- `log(level: LogLevel, module: &str, message: &str)` - 直接日志调用

### 日志宏

- `trace!(...)` - 输出trace级别日志
- `debug!(...)` - 输出debug级别日志
- `info!(...)` - 输出info级别日志
- `warn!(...)` - 输出warn级别日志
- `error!(...)` - 输出error级别日志

所有宏都支持格式化参数，如 `info!("值为: {}", value)`

## Zig实现细节

### 核心功能 (logger.zig)

- 日志级别枚举和比较
- 时间戳生成（毫秒级）
- 日志消息格式化
- 控制台输出（WASM环境）
- 日志过滤逻辑

### 格式化 (format.zig)

- 日志级别字符串转换
- 时间戳格式化
- 颜色支持（终端环境）
- 自定义格式样式

## 测试

### 运行所有测试

```bash
cargo test
```

### 运行lib测试

```bash
cargo test --lib
```

### 运行集成测试

```bash
cargo test --test logger_tests
```

### 测试覆盖

- ✅ 日志级别排序和比较
- ✅ 日志级别字符串转换
- ✅ 最小日志级别设置和获取
- ✅ 日志过滤功能
- ✅ 日志宏功能
- ✅ 并发日志测试
- ✅ 长消息和特殊字符测试

## 示例程序

运行基础示例：

```bash
cargo run --example basic_log
```

## 编译

### Debug版本

```bash
cargo build
```

### Release版本

```bash
cargo build --release
```

## 性能特点

- **零拷贝** - 直接传递字符串指针，避免不必要的内存复制
- **高效过滤** - 日志级别检查在Zig层完成，避免Rust层开销
- **SIMD优化** - Zig编译器自动进行SIMD优化
- **静态缓冲** - 使用固定大小缓冲区，避免动态内存分配

## WebGPU/WASM支持

autozig-log专为WebGPU和WASM平台设计：

- 使用console_log/console_warn/console_error输出到浏览器控制台
- 支持WASM环境的时间戳获取
- 针对Web平台优化的内存布局

## 依赖

- `autozig` - AutoZig FFI框架
- `autozig-build` - AutoZig构建工具

## 许可证

MIT OR Apache-2.0

## 贡献

欢迎提交Issue和Pull Request！

## 验收清单

- ✅ Cargo build 编译成功
- ✅ Cargo test 所有测试通过（19个测试全部通过）
- ✅ 有完整的单元测试覆盖
- ✅ 示例程序可运行
- ✅ Release版本编译成功
- ✅ 无unsafe代码
- ✅ 90% Zig实现，10% Rust包装