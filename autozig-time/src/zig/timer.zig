const std = @import("std");

/// 纳秒转换为秒
fn nanosToSecs(nanos: u64) f32 {
    return @as(f32, @floatFromInt(nanos)) / 1_000_000_000.0;
}

/// 秒转换为纳秒
fn secsToNanos(secs: f32) u64 {
    return @as(u64, @intFromFloat(secs * 1_000_000_000.0));
}

/// Stopwatch 结构（内嵌）
const Stopwatch = extern struct {
    elapsed_nanos: u64,
    paused: bool,

    pub fn new() Stopwatch {
        return Stopwatch{
            .elapsed_nanos = 0,
            .paused = false,
        };
    }

    pub fn tick(self: *Stopwatch, delta_nanos: u64) void {
        if (!self.paused) {
            self.elapsed_nanos += delta_nanos;
        }
    }

    pub fn pause(self: *Stopwatch) void {
        self.paused = true;
    }

    pub fn unpause(self: *Stopwatch) void {
        self.paused = false;
    }

    pub fn reset(self: *Stopwatch) void {
        self.elapsed_nanos = 0;
    }

    pub fn elapsedSecs(self: *const Stopwatch) f32 {
        return nanosToSecs(self.elapsed_nanos);
    }
};

/// Timer mode - 计时器模式
pub const TimerMode = enum(u8) {
    Once = 0, // 触发一次
    Repeating = 1, // 循环触发
};

/// Timer - 计时器
pub const Timer = extern struct {
    stopwatch: Stopwatch,
    duration_nanos: u64,
    mode: TimerMode,
    finished: bool,
    times_finished_this_tick: u32,

    /// 创建新的计时器
    pub fn new(duration_secs: f32, mode: TimerMode) Timer {
        return Timer{
            .stopwatch = Stopwatch.new(),
            .duration_nanos = secsToNanos(duration_secs),
            .mode = mode,
            .finished = false,
            .times_finished_this_tick = 0,
        };
    }

    /// 更新计时器
    pub fn tick(self: *Timer, delta_nanos: u64) void {
        if (self.stopwatch.paused) {
            self.times_finished_this_tick = 0;
            return;
        }

        self.times_finished_this_tick = 0;
        self.stopwatch.tick(delta_nanos);

        if (self.stopwatch.elapsed_nanos >= self.duration_nanos) {
            self.finished = true;

            switch (self.mode) {
                .Once => {
                    self.times_finished_this_tick = 1;
                    self.stopwatch.pause();
                },
                .Repeating => {
                    // 计算完成次数
                    const times = self.stopwatch.elapsed_nanos / self.duration_nanos;
                    self.times_finished_this_tick = @as(u32, @intCast(times));

                    // 重置秒表，保留余数
                    const remainder = self.stopwatch.elapsed_nanos % self.duration_nanos;
                    self.stopwatch.elapsed_nanos = remainder;
                },
            }
        }
    }

    /// 检查计时器是否完成
    pub fn isFinished(self: *const Timer) bool {
        return self.finished;
    }

    /// 检查计时器在本次 tick 中是否刚完成
    pub fn justFinished(self: *const Timer) bool {
        return self.times_finished_this_tick > 0;
    }

    /// 重置计时器
    pub fn reset(self: *Timer) void {
        self.stopwatch.reset();
        self.stopwatch.unpause();
        self.finished = false;
        self.times_finished_this_tick = 0;
    }

    /// 获取完成进度 (0.0 - 1.0)
    pub fn percent(self: *const Timer) f32 {
        if (self.duration_nanos == 0) {
            return 1.0;
        }
        const elapsed_f = @as(f32, @floatFromInt(self.stopwatch.elapsed_nanos));
        const duration_f = @as(f32, @floatFromInt(self.duration_nanos));
        const result = elapsed_f / duration_f;
        return @min(result, 1.0);
    }

    /// 获取剩余进度 (1.0 - 0.0)
    pub fn percentLeft(self: *const Timer) f32 {
        return 1.0 - self.percent();
    }

    /// 暂停计时器
    pub fn pause(self: *Timer) void {
        self.stopwatch.pause();
    }

    /// 恢复计时器
    pub fn unpause(self: *Timer) void {
        self.stopwatch.unpause();
    }

    /// 检查是否暂停
    pub fn isPaused(self: *const Timer) bool {
        return self.stopwatch.paused;
    }

    /// 获取已过去的时间（秒）
    pub fn elapsedSecs(self: *const Timer) f32 {
        return self.stopwatch.elapsedSecs();
    }

    /// 获取持续时间（秒）
    pub fn durationSecs(self: *const Timer) f32 {
        return nanosToSecs(self.duration_nanos);
    }

    /// 设置持续时间（秒）
    pub fn setDuration(self: *Timer, duration_secs: f32) void {
        self.duration_nanos = secsToNanos(duration_secs);
    }

    /// 获取本次 tick 中完成的次数
    pub fn timesFinishedThisTick(self: *const Timer) u32 {
        return self.times_finished_this_tick;
    }
};

// FFI 导出函数

/// 创建新的计时器
export fn timer_new(duration_secs: f32, mode: TimerMode) Timer {
    return Timer.new(duration_secs, mode);
}

/// 更新计时器
export fn timer_tick(timer: *Timer, delta_nanos: u64) void {
    timer.tick(delta_nanos);
}

/// 检查计时器是否完成
export fn timer_finished(timer: *const Timer) bool {
    return timer.isFinished();
}

/// 检查计时器在本次 tick 中是否刚完成
export fn timer_just_finished(timer: *const Timer) bool {
    return timer.justFinished();
}

/// 重置计时器
export fn timer_reset(timer: *Timer) void {
    timer.reset();
}

/// 获取完成进度
export fn timer_percent(timer: *const Timer) f32 {
    return timer.percent();
}

/// 获取剩余进度
export fn timer_percent_left(timer: *const Timer) f32 {
    return timer.percentLeft();
}

/// 暂停计时器
export fn timer_pause(timer: *Timer) void {
    timer.pause();
}

/// 恢复计时器
export fn timer_unpause(timer: *Timer) void {
    timer.unpause();
}

/// 检查是否暂停
export fn timer_is_paused(timer: *const Timer) bool {
    return timer.isPaused();
}

/// 获取已过去的时间（秒）
export fn timer_elapsed_secs(timer: *const Timer) f32 {
    return timer.elapsedSecs();
}

/// 获取持续时间（秒）
export fn timer_duration_secs(timer: *const Timer) f32 {
    return timer.durationSecs();
}

/// 设置持续时间（秒）
export fn timer_set_duration(timer: *Timer, duration_secs: f32) void {
    timer.setDuration(duration_secs);
}

/// 获取本次 tick 中完成的次数
export fn timer_times_finished(timer: *const Timer) u32 {
    return timer.timesFinishedThisTick();
}

// ========== 单元测试 ==========

test "Timer creation" {
    const timer = Timer.new(2.0, .Once);
    try std.testing.expectEqual(@as(u64, 2_000_000_000), timer.duration_nanos);
    try std.testing.expectEqual(TimerMode.Once, timer.mode);
    try std.testing.expectEqual(false, timer.finished);
}

test "Timer Once mode" {
    var timer = Timer.new(1.0, .Once);
    
    // Tick 0.5 seconds
    timer.tick(500_000_000);
    try std.testing.expect(!timer.isFinished());
    try std.testing.expect(!timer.justFinished());
    try std.testing.expectApproxEqAbs(@as(f32, 0.5), timer.percent(), 0.01);
    
    // Tick another 0.6 seconds (total 1.1 seconds, should finish)
    timer.tick(600_000_000);
    try std.testing.expect(timer.isFinished());
    try std.testing.expect(timer.justFinished());
    try std.testing.expectEqual(@as(u32, 1), timer.timesFinishedThisTick());
    
    // Should be paused after finishing
    try std.testing.expect(timer.isPaused());
}

test "Timer Repeating mode" {
    var timer = Timer.new(1.0, .Repeating);
    
    // Tick 2.5 seconds (should complete 2 times)
    timer.tick(2_500_000_000);
    try std.testing.expect(timer.isFinished());
    try std.testing.expectEqual(@as(u32, 2), timer.timesFinishedThisTick());
    
    // Should have 0.5 seconds remaining
    try std.testing.expectApproxEqAbs(@as(f32, 0.5), timer.elapsedSecs(), 0.01);
}

test "Timer pause and unpause" {
    var timer = Timer.new(1.0, .Once);
    
    timer.tick(500_000_000); // 0.5 seconds
    timer.pause();
    
    try std.testing.expect(timer.isPaused());
    
    timer.tick(1_000_000_000); // Should not increment
    try std.testing.expectApproxEqAbs(@as(f32, 0.5), timer.elapsedSecs(), 0.01);
    
    timer.unpause();
    timer.tick(600_000_000); // Should finish now
    try std.testing.expect(timer.isFinished());
}

test "Timer reset" {
    var timer = Timer.new(1.0, .Once);
    timer.tick(2_000_000_000); // Finish it
    
    try std.testing.expect(timer.isFinished());
    
    timer.reset();
    try std.testing.expect(!timer.isFinished());
    try std.testing.expect(!timer.isPaused());
    try std.testing.expectApproxEqAbs(@as(f32, 0.0), timer.elapsedSecs(), 0.001);
}

test "Timer percent calculations" {
    var timer = Timer.new(2.0, .Once);
    
    timer.tick(500_000_000); // 0.5 / 2.0 = 25%
    try std.testing.expectApproxEqAbs(@as(f32, 0.25), timer.percent(), 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 0.75), timer.percentLeft(), 0.01);
    
    timer.tick(1_000_000_000); // 1.5 / 2.0 = 75%
    try std.testing.expectApproxEqAbs(@as(f32, 0.75), timer.percent(), 0.01);
    try std.testing.expectApproxEqAbs(@as(f32, 0.25), timer.percentLeft(), 0.01);
}

test "Timer set duration" {
    var timer = Timer.new(1.0, .Once);
    try std.testing.expectApproxEqAbs(@as(f32, 1.0), timer.durationSecs(), 0.001);
    
    timer.setDuration(3.0);
    try std.testing.expectApproxEqAbs(@as(f32, 3.0), timer.durationSecs(), 0.001);
    try std.testing.expectEqual(@as(u64, 3_000_000_000), timer.duration_nanos);
}
