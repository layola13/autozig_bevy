const std = @import("std");
const builtin = @import("builtin");

/// Log level enumeration
pub const LogLevel = enum(u8) {
    trace = 0,
    debug = 1,
    info = 2,
    warn = 3,
    err = 4,

    pub fn toString(self: LogLevel) []const u8 {
        return switch (self) {
            .trace => "TRACE",
            .debug => "DEBUG",
            .info => "INFO",
            .warn => "WARN",
            .err => "ERROR",
        };
    }
};

/// Log record structure for FFI
pub const LogRecord = extern struct {
    level: LogLevel,
    timestamp: i64,
    module_ptr: [*]const u8,
    module_len: usize,
    message_ptr: [*]const u8,
    message_len: usize,
};

/// Buffer for formatting log messages
var log_buffer: [4096]u8 = undefined;

/// Get current timestamp in milliseconds
export fn log_timestamp() i64 {
    // For WASM/cross-platform, return milliseconds since epoch
    const timestamp = if (builtin.cpu.arch.isWasm())
        0 // WASM: 使用固定值（WASM环境不支持 clockid_t）
    else
        std.time.milliTimestamp();
    return @as(i64, @intCast(timestamp));
}

/// Check if log level is enabled
export fn log_enabled(level: LogLevel, min_level: LogLevel) bool {
    return @intFromEnum(level) >= @intFromEnum(min_level);
}

/// Format timestamp to string (HH:MM:SS.mmm)
fn formatTimestamp(timestamp: i64, buf: []u8) ![]const u8 {
    const millis = @abs(@mod(timestamp, 1000));
    const seconds = @abs(@divFloor(timestamp, 1000));
    const minutes = @abs(@divFloor(seconds, 60));
    const hours = @abs(@divFloor(minutes, 60));

    const s = @abs(@mod(seconds, 60));
    const m = @abs(@mod(minutes, 60));
    const h = @abs(@mod(hours, 24));

    return std.fmt.bufPrint(buf, "{d:0>2}:{d:0>2}:{d:0>2}.{d:0>3}", .{ h, m, s, millis });
}

/// Format log message
fn formatLogMessage(
    level: LogLevel,
    timestamp: i64,
    module_name: []const u8,
    message: []const u8,
    buf: []u8,
) ![]const u8 {
    var timestamp_buf: [32]u8 = undefined;
    const timestamp_str = formatTimestamp(timestamp, &timestamp_buf) catch "[??:??:??]";

    const level_str = level.toString();

    return std.fmt.bufPrint(
        buf,
        "[{s}] {s: <5} {s}: {s}",
        .{ timestamp_str, level_str, module_name, message },
    );
}

/// Console log functions - stub implementations for non-WASM
fn console_log(ptr: [*]const u8, len: usize) void {
    _ = ptr;
    _ = len;
}

fn console_warn(ptr: [*]const u8, len: usize) void {
    _ = ptr;
    _ = len;
}

fn console_error(ptr: [*]const u8, len: usize) void {
    _ = ptr;
    _ = len;
}

/// Check if console functions are available
var console_available: bool = false; // Default to false for non-WASM

/// Fallback log output for non-WASM environments
fn fallbackLog(message: []const u8) void {
    _ = message;
}

/// Write formatted log to console
export fn log_write(
    level: LogLevel,
    module_ptr: [*]const u8,
    module_len: usize,
    message_ptr: [*]const u8,
    message_len: usize,
) void {
    const module_name = module_ptr[0..module_len];
    const message = message_ptr[0..message_len];
    const timestamp = log_timestamp();

    const formatted = formatLogMessage(
        level,
        timestamp,
        module_name,
        message,
        &log_buffer,
    ) catch {
        // If formatting fails, output raw message
        if (console_available) {
            console_log(message_ptr, message_len);
        } else {
            fallbackLog(message);
        }
        return;
    };

    // Output to appropriate console function based on level
    if (console_available) {
        switch (level) {
            .trace, .debug, .info => console_log(formatted.ptr, formatted.len),
            .warn => console_warn(formatted.ptr, formatted.len),
            .err => console_error(formatted.ptr, formatted.len),
        }
    } else {
        fallbackLog(formatted);
    }
}

/// Write log with pre-formatted message
export fn log_write_formatted(
    level: LogLevel,
    message_ptr: [*]const u8,
    message_len: usize,
) void {
    const message = message_ptr[0..message_len];

    if (console_available) {
        switch (level) {
            .trace, .debug, .info => console_log(message_ptr, message_len),
            .warn => console_warn(message_ptr, message_len),
            .err => console_error(message_ptr, message_len),
        }
    } else {
        fallbackLog(message);
    }
}

/// Initialize logger
export fn log_init() void {
    // Initialize any required state
    console_available = true;
}

/// Shutdown logger
export fn log_shutdown() void {
    // Cleanup if needed
}

/// Set whether console functions are available
export fn log_set_console_available(available: bool) void {
    console_available = available;
}

// 单元测试
test "log timestamp wasm64 compatibility" {
    const timestamp = log_timestamp();
    
    if (builtin.cpu.arch.isWasm()) {
        // 在 WASM 环境下，时间戳应该返回 0
        try std.testing.expectEqual(@as(i64, 0), timestamp);
    } else {
        // 在原生环境下，时间戳应该是正数（假设系统时间正确）
        try std.testing.expect(timestamp >= 0);
    }
}

test "log level enabled check" {
    try std.testing.expect(log_enabled(.info, .info));
    try std.testing.expect(log_enabled(.warn, .info));
    try std.testing.expect(log_enabled(.err, .info));
    try std.testing.expect(!log_enabled(.debug, .info));
    try std.testing.expect(!log_enabled(.trace, .info));
}

test "log level ordering" {
    try std.testing.expect(@intFromEnum(LogLevel.trace) < @intFromEnum(LogLevel.debug));
    try std.testing.expect(@intFromEnum(LogLevel.debug) < @intFromEnum(LogLevel.info));
    try std.testing.expect(@intFromEnum(LogLevel.info) < @intFromEnum(LogLevel.warn));
    try std.testing.expect(@intFromEnum(LogLevel.warn) < @intFromEnum(LogLevel.err));
}

test "log level toString" {
    try std.testing.expectEqualStrings("TRACE", LogLevel.trace.toString());
    try std.testing.expectEqualStrings("DEBUG", LogLevel.debug.toString());
    try std.testing.expectEqualStrings("INFO", LogLevel.info.toString());
    try std.testing.expectEqualStrings("WARN", LogLevel.warn.toString());
    try std.testing.expectEqualStrings("ERROR", LogLevel.err.toString());
}

test "formatTimestamp" {
    var buf: [32]u8 = undefined;
    
    // 测试 0 毫秒
    const ts1 = try formatTimestamp(0, &buf);
    try std.testing.expectEqualStrings("00:00:00.000", ts1);
    
    // 测试 1 小时 2 分 3 秒 456 毫秒 = 3723456 毫秒
    const ts2 = try formatTimestamp(3723456, &buf);
    try std.testing.expectEqualStrings("01:02:03.456", ts2);
}

test "wasm64 compatibility full workflow" {
    // 测试完整的日志工作流在 WASM64 下能正常工作
    log_init();
    log_set_console_available(false);
    
    const module = "test_module";
    const message = "test message";
    
    // 这不应该在 WASM64 下引发 clockid_t 错误
    log_write(.info, module.ptr, module.len, message.ptr, message.len);
    log_write_formatted(.warn, message.ptr, message.len);
    
    const timestamp = log_timestamp();
    
    if (builtin.cpu.arch.isWasm()) {
        try std.testing.expectEqual(@as(i64, 0), timestamp);
    }
    
    log_shutdown();
}
