const std = @import("std");

/// 纳秒转换为秒
fn nanosToSecs(nanos: u64) f32 {
    return @as(f32, @floatFromInt(nanos)) / 1_000_000_000.0;
}

/// Stopwatch - 秒表
pub const Stopwatch = extern struct {
    elapsed_nanos: u64,
    paused: bool,

    /// 创建新的秒表
    pub fn new() Stopwatch {
        return Stopwatch{
            .elapsed_nanos = 0,
            .paused = false,
        };
    }

    /// 更新秒表（增加时间）
    pub fn tick(self: *Stopwatch, delta_nanos: u64) void {
        if (!self.paused) {
            self.elapsed_nanos += delta_nanos;
        }
    }

    /// 暂停秒表
    pub fn pause(self: *Stopwatch) void {
        self.paused = true;
    }

    /// 恢复秒表
    pub fn unpause(self: *Stopwatch) void {
        self.paused = false;
    }

    /// 重置秒表
    pub fn reset(self: *Stopwatch) void {
        self.elapsed_nanos = 0;
    }

    /// 获取已过去的时间（纳秒）
    pub fn elapsed(self: *const Stopwatch) u64 {
        return self.elapsed_nanos;
    }

    /// 获取已过去的时间（秒）
    pub fn elapsedSecs(self: *const Stopwatch) f32 {
        return nanosToSecs(self.elapsed_nanos);
    }

    /// 检查是否暂停
    pub fn isPaused(self: *const Stopwatch) bool {
        return self.paused;
    }
};

// FFI 导出函数

/// 创建新的秒表
export fn stopwatch_new() Stopwatch {
    return Stopwatch.new();
}

/// 更新秒表
export fn stopwatch_tick(stopwatch: *Stopwatch, delta_nanos: u64) void {
    stopwatch.tick(delta_nanos);
}

/// 暂停秒表
export fn stopwatch_pause(stopwatch: *Stopwatch) void {
    stopwatch.pause();
}

/// 恢复秒表
export fn stopwatch_unpause(stopwatch: *Stopwatch) void {
    stopwatch.unpause();
}

/// 重置秒表
export fn stopwatch_reset(stopwatch: *Stopwatch) void {
    stopwatch.reset();
}

/// 获取已过去的时间（纳秒）
export fn stopwatch_elapsed(stopwatch: *const Stopwatch) u64 {
    return stopwatch.elapsed();
}

/// 获取已过去的时间（秒）
export fn stopwatch_elapsed_secs(stopwatch: *const Stopwatch) f32 {
    return stopwatch.elapsedSecs();
}

/// 检查是否暂停
export fn stopwatch_is_paused(stopwatch: *const Stopwatch) bool {
    return stopwatch.isPaused();
}

// ========== 单元测试 ==========

test "Stopwatch creation" {
    const sw = Stopwatch.new();
    try std.testing.expectEqual(@as(u64, 0), sw.elapsed_nanos);
    try std.testing.expectEqual(false, sw.paused);
}

test "Stopwatch tick" {
    var sw = Stopwatch.new();
    sw.tick(1_000_000_000); // 1 second
    
    try std.testing.expectEqual(@as(u64, 1_000_000_000), sw.elapsed());
    try std.testing.expectApproxEqAbs(@as(f32, 1.0), sw.elapsedSecs(), 0.001);
}

test "Stopwatch pause and unpause" {
    var sw = Stopwatch.new();
    sw.tick(500_000_000); // 0.5 seconds
    
    sw.pause();
    try std.testing.expect(sw.isPaused());
    
    sw.tick(500_000_000); // Should not increment
    try std.testing.expectEqual(@as(u64, 500_000_000), sw.elapsed());
    
    sw.unpause();
    try std.testing.expect(!sw.isPaused());
    
    sw.tick(500_000_000); // Should increment
    try std.testing.expectEqual(@as(u64, 1_000_000_000), sw.elapsed());
}

test "Stopwatch reset" {
    var sw = Stopwatch.new();
    sw.tick(2_000_000_000); // 2 seconds
    sw.reset();
    
    try std.testing.expectEqual(@as(u64, 0), sw.elapsed());
    try std.testing.expectApproxEqAbs(@as(f32, 0.0), sw.elapsedSecs(), 0.001);
}
