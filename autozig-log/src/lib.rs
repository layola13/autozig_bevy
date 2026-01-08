//! # AutoZig Log - Bevy Logging System implemented in Zig
//!
//! 90% Zig实现，10% Rust包装
//! 
//! 提供以下核心功能：
//! - 日志级别: trace, debug, info, warn, error
//! - 日志格式化: 时间戳、模块名、日志级别、消息
//! - 日志输出: 控制台输出（WASM环境）
//! - 日志过滤: 按级别和模块过滤日志

use autozig::include_zig;
use std::fmt;
use std::sync::atomic::{AtomicU8, Ordering};

/// Log level enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRACE" => Some(LogLevel::Trace),
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" | "WARNING" => Some(LogLevel::Warn),
            "ERROR" | "ERR" => Some(LogLevel::Error),
            _ => None,
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// Include Zig implementation
include_zig!("src/zig/logger.zig", {
    fn log_write(
        level: LogLevel,
        module_ptr: *const u8,
        module_len: usize,
        message_ptr: *const u8,
        message_len: usize,
    );
    fn log_write_formatted(
        level: LogLevel,
        message_ptr: *const u8,
        message_len: usize,
    );
    fn log_timestamp() -> i64;
    fn log_enabled(level: LogLevel, min_level: LogLevel) -> bool;
    fn log_init();
    fn log_shutdown();
    fn log_set_console_available(available: bool);
});

/// Global minimum log level (atomic for thread-safety without requiring unsafe)
static MIN_LOG_LEVEL: AtomicU8 = AtomicU8::new(LogLevel::Info as u8);

/// Set minimum log level
pub fn set_min_level(level: LogLevel) {
    MIN_LOG_LEVEL.store(level as u8, Ordering::Relaxed);
}

/// Get minimum log level
pub fn min_level() -> LogLevel {
    let level = MIN_LOG_LEVEL.load(Ordering::Relaxed);
    match level {
        0 => LogLevel::Trace,
        1 => LogLevel::Debug,
        2 => LogLevel::Info,
        3 => LogLevel::Warn,
        4 => LogLevel::Error,
        _ => LogLevel::Info, // default
    }
}

/// Check if a log level is enabled
pub fn is_enabled(level: LogLevel) -> bool {
    let min = min_level();
    log_enabled(level, min)
}

/// Initialize the logger
pub fn init() {
    log_init();
}

/// Shutdown the logger
pub fn shutdown() {
    log_shutdown();
}

/// Set whether console functions are available
pub fn set_console_available(available: bool) {
    log_set_console_available(available);
}

/// Log a message at a specific level
pub fn log(level: LogLevel, module: &str, message: &str) {
    if !is_enabled(level) {
        return;
    }
    
    log_write(
        level,
        module.as_ptr(),
        module.len(),
        message.as_ptr(),
        message.len(),
    );
}

/// Log a formatted message at a specific level
pub fn log_formatted(level: LogLevel, message: &str) {
    if !is_enabled(level) {
        return;
    }
    
    log_write_formatted(
        level,
        message.as_ptr(),
        message.len(),
    );
}

/// Trace log macro
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        if $crate::is_enabled($crate::LogLevel::Trace) {
            $crate::log($crate::LogLevel::Trace, module_path!(), &format!($($arg)*));
        }
    };
}

/// Debug log macro
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::is_enabled($crate::LogLevel::Debug) {
            $crate::log($crate::LogLevel::Debug, module_path!(), &format!($($arg)*));
        }
    };
}

/// Info log macro
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if $crate::is_enabled($crate::LogLevel::Info) {
            $crate::log($crate::LogLevel::Info, module_path!(), &format!($($arg)*));
        }
    };
}

/// Warn log macro
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if $crate::is_enabled($crate::LogLevel::Warn) {
            $crate::log($crate::LogLevel::Warn, module_path!(), &format!($($arg)*));
        }
    };
}

/// Error log macro
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if $crate::is_enabled($crate::LogLevel::Error) {
            $crate::log($crate::LogLevel::Error, module_path!(), &format!($($arg)*));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }

    #[test]
    fn test_log_level_from_str() {
        assert_eq!(LogLevel::from_str("trace"), Some(LogLevel::Trace));
        assert_eq!(LogLevel::from_str("DEBUG"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("info"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("WARN"), Some(LogLevel::Warn));
        assert_eq!(LogLevel::from_str("error"), Some(LogLevel::Error));
        assert_eq!(LogLevel::from_str("invalid"), None);
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(format!("{}", LogLevel::Trace), "TRACE");
        assert_eq!(format!("{}", LogLevel::Debug), "DEBUG");
        assert_eq!(format!("{}", LogLevel::Info), "INFO");
        assert_eq!(format!("{}", LogLevel::Warn), "WARN");
        assert_eq!(format!("{}", LogLevel::Error), "ERROR");
    }

    #[test]
    fn test_set_min_level() {
        set_min_level(LogLevel::Warn);
        assert_eq!(min_level(), LogLevel::Warn);
        
        set_min_level(LogLevel::Debug);
        assert_eq!(min_level(), LogLevel::Debug);
    }

    #[test]
    fn test_is_enabled() {
        set_min_level(LogLevel::Info);
        
        assert!(!is_enabled(LogLevel::Trace));
        assert!(!is_enabled(LogLevel::Debug));
        assert!(is_enabled(LogLevel::Info));
        assert!(is_enabled(LogLevel::Warn));
        assert!(is_enabled(LogLevel::Error));
    }
}