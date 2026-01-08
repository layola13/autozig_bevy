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
