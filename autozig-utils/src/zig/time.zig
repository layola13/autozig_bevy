const std = @import("std");
const hashmap = @import("hashmap.zig");

// 使用hashmap.zig中定义的allocator
const g_allocator = hashmap.g_allocator;

// 时间戳结构 (微秒精度)
pub const Instant = extern struct {
    micros: i64,

    // 获取当前时间戳
    pub fn now() Instant {
        const builtin = @import("builtin");
        const timestamp = if (builtin.cpu.arch.isWasm())
            0 // WASM: 返回虚拟时间戳
        else
            std.time.microTimestamp(); // Native: 使用系统时钟
        return Instant{ .micros = timestamp };
    }

    // 从微秒创建
    pub fn fromMicros(micros: i64) Instant {
        return Instant{ .micros = micros };
    }

    // 从毫秒创建
    pub fn fromMillis(millis: i64) Instant {
        return Instant{ .micros = millis * 1000 };
    }

    // 从秒创建
    pub fn fromSecs(secs: i64) Instant {
        return Instant{ .micros = secs * 1_000_000 };
    }

    // 转换为微秒
    pub fn asMicros(self: Instant) i64 {
        return self.micros;
    }

    // 转换为毫秒
    pub fn asMillis(self: Instant) i64 {
        return @divFloor(self.micros, 1000);
    }

    // 转换为秒
    pub fn asSecs(self: Instant) i64 {
        return @divFloor(self.micros, 1_000_000);
    }

    // 转换为秒（浮点数）
    pub fn asSecsF64(self: Instant) f64 {
        return @as(f64, @floatFromInt(self.micros)) / 1_000_000.0;
    }

    // 计算时间差
    pub fn duration(self: Instant, earlier: Instant) Duration {
        return Duration{ .micros = self.micros - earlier.micros };
    }

    // 添加时间
    pub fn add(self: Instant, dur: Duration) Instant {
        return Instant{ .micros = self.micros + dur.micros };
    }

    // 减去时间
    pub fn sub(self: Instant, dur: Duration) Instant {
        return Instant{ .micros = self.micros - dur.micros };
    }
};

// 时间段结构
pub const Duration = extern struct {
    micros: i64,

    // 创建时间段（微秒）
    pub fn fromMicros(micros: i64) Duration {
        return Duration{ .micros = micros };
    }

    // 创建时间段（毫秒）
    pub fn fromMillis(millis: i64) Duration {
        return Duration{ .micros = millis * 1000 };
    }

    // 创建时间段（秒）
    pub fn fromSecs(secs: i64) Duration {
        return Duration{ .micros = secs * 1_000_000 };
    }

    // 创建时间段（秒，浮点数）
    pub fn fromSecsF64(secs: f64) Duration {
        return Duration{ .micros = @intFromFloat(secs * 1_000_000.0) };
    }

    // 转换为微秒
    pub fn asMicros(self: Duration) i64 {
        return self.micros;
    }

    // 转换为毫秒
    pub fn asMillis(self: Duration) i64 {
        return @divFloor(self.micros, 1000);
    }

    // 转换为秒
    pub fn asSecs(self: Duration) i64 {
        return @divFloor(self.micros, 1_000_000);
    }

    // 转换为秒（浮点数）
    pub fn asSecsF64(self: Duration) f64 {
        return @as(f64, @floatFromInt(self.micros)) / 1_000_000.0;
    }

    // 加法
    pub fn add(self: Duration, other: Duration) Duration {
        return Duration{ .micros = self.micros + other.micros };
    }

    // 减法
    pub fn sub(self: Duration, other: Duration) Duration {
        return Duration{ .micros = self.micros - other.micros };
    }

    // 乘法
    pub fn mul(self: Duration, factor: i64) Duration {
        return Duration{ .micros = self.micros * factor };
    }

    // 除法
    pub fn div(self: Duration, divisor: i64) Duration {
        return Duration{ .micros = @divFloor(self.micros, divisor) };
    }

    // 是否为零
    pub fn isZero(self: Duration) bool {
        return self.micros == 0;
    }

    // 是否为负数
    pub fn isNegative(self: Duration) bool {
        return self.micros < 0;
    }
};

// 计时器
pub const Timer = struct {
    start: Instant,

    pub fn init() Timer {
        return Timer{ .start = Instant.now() };
    }

    pub fn elapsed(self: Timer) Duration {
        const now = Instant.now();
        return now.duration(self.start);
    }

    pub fn reset(self: *Timer) Duration {
        const elapsed_time = self.elapsed();
        self.start = Instant.now();
        return elapsed_time;
    }

    pub fn restart(self: *Timer) void {
        self.start = Instant.now();
    }
};

// FFI导出 - Instant
export fn instant_now() Instant {
    return Instant.now();
}

export fn instant_from_micros(micros: i64) Instant {
    return Instant.fromMicros(micros);
}

export fn instant_from_millis(millis: i64) Instant {
    return Instant.fromMillis(millis);
}

export fn instant_from_secs(secs: i64) Instant {
    return Instant.fromSecs(secs);
}

export fn instant_as_micros(instant: Instant) i64 {
    return instant.asMicros();
}

export fn instant_as_millis(instant: Instant) i64 {
    return instant.asMillis();
}

export fn instant_as_secs(instant: Instant) i64 {
    return instant.asSecs();
}

export fn instant_as_secs_f64(instant: Instant) f64 {
    return instant.asSecsF64();
}

export fn instant_duration(later: Instant, earlier: Instant) Duration {
    return later.duration(earlier);
}

export fn instant_add(instant: Instant, duration: Duration) Instant {
    return instant.add(duration);
}

export fn instant_sub(instant: Instant, duration: Duration) Instant {
    return instant.sub(duration);
}

// FFI导出 - Duration
export fn duration_from_micros(micros: i64) Duration {
    return Duration.fromMicros(micros);
}

export fn duration_from_millis(millis: i64) Duration {
    return Duration.fromMillis(millis);
}

export fn duration_from_secs(secs: i64) Duration {
    return Duration.fromSecs(secs);
}

export fn duration_from_secs_f64(secs: f64) Duration {
    return Duration.fromSecsF64(secs);
}

export fn duration_as_micros(duration: Duration) i64 {
    return duration.asMicros();
}

export fn duration_as_millis(duration: Duration) i64 {
    return duration.asMillis();
}

export fn duration_as_secs(duration: Duration) i64 {
    return duration.asSecs();
}

export fn duration_as_secs_f64(duration: Duration) f64 {
    return duration.asSecsF64();
}

export fn duration_add(a: Duration, b: Duration) Duration {
    return a.add(b);
}

export fn duration_sub(a: Duration, b: Duration) Duration {
    return a.sub(b);
}

export fn duration_mul(duration: Duration, factor: i64) Duration {
    return duration.mul(factor);
}

export fn duration_div(duration: Duration, divisor: i64) Duration {
    return duration.div(divisor);
}

export fn duration_is_zero(duration: Duration) bool {
    return duration.isZero();
}

export fn duration_is_negative(duration: Duration) bool {
    return duration.isNegative();
}

// FFI导出 - Timer
export fn timer_create() *Timer {
    const timer = g_allocator.create(Timer) catch unreachable;
    timer.* = Timer.init();
    return timer;
}

export fn timer_destroy(timer: *Timer) void {
    g_allocator.destroy(timer);
}

export fn timer_elapsed(timer: *Timer) Duration {
    return timer.elapsed();
}

export fn timer_reset(timer: *Timer) Duration {
    return timer.reset();
}

export fn timer_restart(timer: *Timer) void {
    timer.restart();
}

// 单元测试
test "Instant creation" {
    const instant = Instant.now();
    try std.testing.expect(instant.micros > 0);

    const from_secs = Instant.fromSecs(10);
    try std.testing.expectEqual(@as(i64, 10_000_000), from_secs.micros);

    const from_millis = Instant.fromMillis(5000);
    try std.testing.expectEqual(@as(i64, 5_000_000), from_millis.micros);
}

test "Instant conversion" {
    const instant = Instant.fromMicros(3_500_000);

    try std.testing.expectEqual(@as(i64, 3_500_000), instant.asMicros());
    try std.testing.expectEqual(@as(i64, 3_500), instant.asMillis());
    try std.testing.expectEqual(@as(i64, 3), instant.asSecs());
    try std.testing.expectApproxEqAbs(@as(f64, 3.5), instant.asSecsF64(), 0.001);
}

test "Duration creation and conversion" {
    const dur = Duration.fromSecs(5);
    try std.testing.expectEqual(@as(i64, 5_000_000), dur.asMicros());
    try std.testing.expectEqual(@as(i64, 5_000), dur.asMillis());
    try std.testing.expectEqual(@as(i64, 5), dur.asSecs());
}

test "Duration arithmetic" {
    const dur1 = Duration.fromSecs(10);
    const dur2 = Duration.fromSecs(5);

    const sum = dur1.add(dur2);
    try std.testing.expectEqual(@as(i64, 15), sum.asSecs());

    const diff = dur1.sub(dur2);
    try std.testing.expectEqual(@as(i64, 5), diff.asSecs());

    const product = dur2.mul(3);
    try std.testing.expectEqual(@as(i64, 15), product.asSecs());

    const quotient = dur1.div(2);
    try std.testing.expectEqual(@as(i64, 5), quotient.asSecs());
}

test "Duration properties" {
    const zero = Duration.fromMicros(0);
    try std.testing.expect(zero.isZero());

    const positive = Duration.fromSecs(5);
    try std.testing.expect(!positive.isZero());
    try std.testing.expect(!positive.isNegative());

    const negative = Duration.fromSecs(-5);
    try std.testing.expect(negative.isNegative());
}

test "Instant duration calculation" {
    const start = Instant.fromSecs(100);
    const end = Instant.fromSecs(150);

    const duration = end.duration(start);
    try std.testing.expectEqual(@as(i64, 50), duration.asSecs());
}

test "Timer operations" {
    var timer = Timer.init();

    // 模拟延迟（在测试中很短）
    std.time.sleep(1000); // 1微秒

    const elapsed = timer.elapsed();
    try std.testing.expect(elapsed.micros >= 0);

    timer.restart();
    const new_elapsed = timer.elapsed();
    try std.testing.expect(new_elapsed.micros < elapsed.micros or new_elapsed.micros >= 0);
}
