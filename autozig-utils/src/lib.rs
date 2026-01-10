//! # AutoZig Utils - Bevy Utils implemented in Zig
//!
//! 90% Zig实现，10% Rust包装
//!
//! 提供以下核心功能：
//! - HashMap/HashSet: 高性能哈希数据结构
//! - UUID: UUID v4生成和操作
//! - Concurrency: 原子操作和并发原语
//! - Time: 高精度时间戳和计时工具

// 引入跨平台 allocator 模块 (WASM 兼容)
autozig::include_zig!("src/zig/allocator.zig", {
    fn _allocator_init();
});

pub mod hashmap;
pub mod uuid;
pub mod concurrency;
pub mod time;

pub mod prelude {
    pub use crate::{
        hashmap::{HashMap, HashSet},
        uuid::Uuid,
        concurrency::{AtomicCounter, AtomicBool, SpinLock, OnceFlag},
        time::{Instant, Duration, Timer},
    };
}