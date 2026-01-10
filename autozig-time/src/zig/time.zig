const std = @import("std");

/// 获取当前纳秒时间戳
fn nowNanos() u64 {
    const builtin = @import("builtin");
    if (builtin.cpu.arch.isWasm()) {
        // WASM 平台: 返回虚拟时间戳（从 0 开始计数）
        // 在 WASM 环境中，通常由宿主环境提供时间
        return 0;
    } else {
        // Native 平台: 使用系统时钟
        return @as(u64, @intCast(std.time.nanoTimestamp()));
    }
}

/// 纳秒转换为秒
fn nanosToSecs(nanos: u64) f32 {
    return @as(f32, @floatFromInt(nanos)) / 1_000_000_000.0;
}

/// 秒转换为纳秒
fn secsToNanos(secs: f32) u64 {
    return @as(u64, @intFromFloat(secs * 1_000_000_000.0));
}

/// Time - 时间资源
pub const Time = extern struct {
    delta: f32, // 帧间增量时间（秒）
    elapsed: f32, // 总运行时间（秒）
    delta_nanos: u64, // 纳秒精度的增量时间
    elapsed_nanos: u64, // 纳秒精度的总时间
    startup_nanos: u64, // 启动时间戳
    last_update_nanos: u64, // 上次更新时间戳

    /// 创建新的 Time 资源
    pub fn create() Time {
        const now = nowNanos();
        return Time{
            .delta = 0.0,
            .elapsed = 0.0,
            .delta_nanos = 0,
            .elapsed_nanos = 0,
            .startup_nanos = now,
            .last_update_nanos = now,
        };
    }

    /// 更新时间
    pub fn update(self: *Time) void {
        const now = nowNanos();

        // 计算增量时间
        self.delta_nanos = now - self.last_update_nanos;
        self.delta = nanosToSecs(self.delta_nanos);

        // 计算总运行时间
        self.elapsed_nanos = now - self.startup_nanos;
        self.elapsed = nanosToSecs(self.elapsed_nanos);

        // 更新上次更新时间戳
        self.last_update_nanos = now;
    }

    /// 手动设置增量时间（用于测试或固定时间步长）
    pub fn setDelta(self: *Time, delta_secs: f32) void {
        self.delta = delta_secs;
        self.delta_nanos = secsToNanos(delta_secs);
        self.elapsed += delta_secs;
        self.elapsed_nanos += self.delta_nanos;
    }

    /// 获取增量时间（秒）
    pub fn deltaSeconds(self: *const Time) f32 {
        return self.delta;
    }

    /// 获取总运行时间（秒）
    pub fn elapsedSeconds(self: *const Time) f32 {
        return self.elapsed;
    }

    /// 获取增量时间（纳秒）
    pub fn deltaNanos(self: *const Time) u64 {
        return self.delta_nanos;
    }

    /// 获取总运行时间（纳秒）
    pub fn elapsedNanos(self: *const Time) u64 {
        return self.elapsed_nanos;
    }

    /// 重置时间（保留启动时间戳）
    pub fn reset(self: *Time) void {
        const now = nowNanos();
        self.delta = 0.0;
        self.elapsed = 0.0;
        self.delta_nanos = 0;
        self.elapsed_nanos = 0;
        self.startup_nanos = now;
        self.last_update_nanos = now;
    }
};

// FFI 导出函数

/// 创建新的 Time 资源
export fn time_create() Time {
    return Time.create();
}

/// 更新时间
export fn time_update(time: *Time) void {
    time.update();
}

/// 手动设置增量时间
export fn time_set_delta(time: *Time, delta_secs: f32) void {
    time.setDelta(delta_secs);
}

/// 获取增量时间（秒）
export fn time_delta_seconds(time: *const Time) f32 {
    return time.deltaSeconds();
}

/// 获取总运行时间（秒）
export fn time_elapsed_seconds(time: *const Time) f32 {
    return time.elapsedSeconds();
}

/// 获取增量时间（纳秒）
export fn time_delta_nanos(time: *const Time) u64 {
    return time.deltaNanos();
}

/// 获取总运行时间（纳秒）
export fn time_elapsed_nanos(time: *const Time) u64 {
    return time.elapsedNanos();
}

/// 重置时间
export fn time_reset(time: *Time) void {
    time.reset();
}

/// 获取当前纳秒时间戳（工具函数）
export fn time_now_nanos() u64 {
    return nowNanos();
}

/// 纳秒转换为秒（工具函数）
export fn time_nanos_to_secs(nanos: u64) f32 {
    return nanosToSecs(nanos);
}

/// 秒转换为纳秒（工具函数）
export fn time_secs_to_nanos(secs: f32) u64 {
    return secsToNanos(secs);
}

// ========== 单元测试 ==========

test "Time creation" {
    const time = Time.create();
    try std.testing.expectEqual(@as(f32, 0.0), time.delta);
    try std.testing.expectEqual(@as(f32, 0.0), time.elapsed);
    try std.testing.expectEqual(@as(u64, 0), time.delta_nanos);
    try std.testing.expectEqual(@as(u64, 0), time.elapsed_nanos);
}

test "Time setDelta" {
    var time = Time.create();
    time.setDelta(0.5);
    
    try std.testing.expectEqual(@as(f32, 0.5), time.delta);
    try std.testing.expectEqual(@as(f32, 0.5), time.elapsed);
    try std.testing.expectEqual(@as(u64, 500_000_000), time.delta_nanos);
    try std.testing.expectEqual(@as(u64, 500_000_000), time.elapsed_nanos);
}

test "Time conversion functions" {
    const nanos: u64 = 1_500_000_000; // 1.5 seconds
    const secs = nanosToSecs(nanos);
    try std.testing.expectApproxEqAbs(@as(f32, 1.5), secs, 0.001);
    
    const back_to_nanos = secsToNanos(secs);
    try std.testing.expectEqual(nanos, back_to_nanos);
}

test "Time reset" {
    var time = Time.create();
    time.setDelta(1.0);
    time.reset();
    
    try std.testing.expectEqual(@as(f32, 0.0), time.delta);
    try std.testing.expectEqual(@as(f32, 0.0), time.elapsed);
    try std.testing.expectEqual(@as(u64, 0), time.delta_nanos);
    try std.testing.expectEqual(@as(u64, 0), time.elapsed_nanos);
}

