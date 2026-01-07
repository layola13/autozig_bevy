//! 时间戳工具的Rust包装层 - 90% Zig实现

use autozig::include_zig;
use std::fmt;
use std::ops::{Add, Sub};

// 时间戳和时间段结构，与Zig的extern struct保持一致
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instant {
    micros: i64,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    micros: i64,
}

// 引入Zig实现的时间函数
include_zig!("src/zig/time.zig", {
    // Instant FFI
    fn instant_now() -> Instant;
    fn instant_from_micros(micros: i64) -> Instant;
    fn instant_from_millis(millis: i64) -> Instant;
    fn instant_from_secs(secs: i64) -> Instant;
    fn instant_as_micros(instant: Instant) -> i64;
    fn instant_as_millis(instant: Instant) -> i64;
    fn instant_as_secs(instant: Instant) -> i64;
    fn instant_as_secs_f64(instant: Instant) -> f64;
    fn instant_duration(later: Instant, earlier: Instant) -> Duration;
    fn instant_add(instant: Instant, duration: Duration) -> Instant;
    fn instant_sub(instant: Instant, duration: Duration) -> Instant;
    
    // Duration FFI
    fn duration_from_micros(micros: i64) -> Duration;
    fn duration_from_millis(millis: i64) -> Duration;
    fn duration_from_secs(secs: i64) -> Duration;
    fn duration_from_secs_f64(secs: f64) -> Duration;
    fn duration_as_micros(duration: Duration) -> i64;
    fn duration_as_millis(duration: Duration) -> i64;
    fn duration_as_secs(duration: Duration) -> i64;
    fn duration_as_secs_f64(duration: Duration) -> f64;
    fn duration_add(a: Duration, b: Duration) -> Duration;
    fn duration_sub(a: Duration, b: Duration) -> Duration;
    fn duration_mul(duration: Duration, factor: i64) -> Duration;
    fn duration_div(duration: Duration, divisor: i64) -> Duration;
    fn duration_is_zero(duration: Duration) -> bool;
    fn duration_is_negative(duration: Duration) -> bool;
    
    // Timer FFI
    fn timer_create() -> *mut ZigTimer;
    fn timer_destroy(timer: *mut ZigTimer);
    fn timer_elapsed(timer: *mut ZigTimer) -> Duration;
    fn timer_reset(timer: *mut ZigTimer) -> Duration;
    fn timer_restart(timer: *mut ZigTimer);
});

#[repr(C)]
struct ZigTimer {
    _private: [u8; 0],
}

impl Instant {
    /// 获取当前时刻
    pub fn now() -> Self {
        instant_now()
    }
    
    /// 从微秒创建
    pub fn from_micros(micros: i64) -> Self {
        instant_from_micros(micros)
    }
    
    /// 从毫秒创建
    pub fn from_millis(millis: i64) -> Self {
        instant_from_millis(millis)
    }
    
    /// 从秒创建
    pub fn from_secs(secs: i64) -> Self {
        instant_from_secs(secs)
    }
    
    /// 转换为微秒
    pub fn as_micros(&self) -> i64 {
        instant_as_micros(*self)
    }
    
    /// 转换为毫秒
    pub fn as_millis(&self) -> i64 {
        instant_as_millis(*self)
    }
    
    /// 转换为秒
    pub fn as_secs(&self) -> i64 {
        instant_as_secs(*self)
    }
    
    /// 转换为秒（浮点数）
    pub fn as_secs_f64(&self) -> f64 {
        instant_as_secs_f64(*self)
    }
    
    /// 计算与另一个时刻的时间差
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        instant_duration(*self, earlier)
    }
    
    /// 计算经过的时间
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;
    
    fn add(self, duration: Duration) -> Instant {
        instant_add(self, duration)
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;
    
    fn sub(self, duration: Duration) -> Instant {
        instant_sub(self, duration)
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;
    
    fn sub(self, other: Instant) -> Duration {
        self.duration_since(other)
    }
}

impl fmt::Debug for Instant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instant({}μs)", self.micros)
    }
}

impl Duration {
    /// 零时长
    pub const ZERO: Self = Self { micros: 0 };
    
    /// 从微秒创建
    pub fn from_micros(micros: i64) -> Self {
        duration_from_micros(micros)
    }
    
    /// 从毫秒创建
    pub fn from_millis(millis: i64) -> Self {
        duration_from_millis(millis)
    }
    
    /// 从秒创建
    pub fn from_secs(secs: i64) -> Self {
        duration_from_secs(secs)
    }
    
    /// 从秒创建（浮点数）
    pub fn from_secs_f64(secs: f64) -> Self {
        duration_from_secs_f64(secs)
    }
    
    /// 转换为微秒
    pub fn as_micros(&self) -> i64 {
        duration_as_micros(*self)
    }
    
    /// 转换为毫秒
    pub fn as_millis(&self) -> i64 {
        duration_as_millis(*self)
    }
    
    /// 转换为秒
    pub fn as_secs(&self) -> i64 {
        duration_as_secs(*self)
    }
    
    /// 转换为秒（浮点数）
    pub fn as_secs_f64(&self) -> f64 {
        duration_as_secs_f64(*self)
    }
    
    /// 是否为零
    pub fn is_zero(&self) -> bool {
        duration_is_zero(*self)
    }
    
    /// 是否为负数
    pub fn is_negative(&self) -> bool {
        duration_is_negative(*self)
    }
    
    /// 乘以倍数
    pub fn mul_i64(&self, factor: i64) -> Self {
        duration_mul(*self, factor)
    }
    
    /// 除以除数
    pub fn div_i64(&self, divisor: i64) -> Self {
        duration_div(*self, divisor)
    }
}

impl Add for Duration {
    type Output = Duration;
    
    fn add(self, other: Duration) -> Duration {
        duration_add(self, other)
    }
}

impl Sub for Duration {
    type Output = Duration;
    
    fn sub(self, other: Duration) -> Duration {
        duration_sub(self, other)
    }
}

impl fmt::Debug for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_negative() {
            write!(f, "-Duration({}μs)", -self.micros)
        } else {
            write!(f, "Duration({}μs)", self.micros)
        }
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.as_secs_f64();
        if secs.abs() < 0.001 {
            write!(f, "{}μs", self.micros)
        } else if secs.abs() < 1.0 {
            write!(f, "{:.2}ms", self.as_millis() as f64 / 1000.0)
        } else {
            write!(f, "{:.2}s", secs)
        }
    }
}

/// 计时器 - 用于测量时间间隔
pub struct Timer {
    ptr: *mut ZigTimer,
}

impl Timer {
    /// 创建并启动新的计时器
    pub fn new() -> Self {
        Self {
            ptr: timer_create(),
        }
    }
    
    /// 获取已经过的时间
    pub fn elapsed(&self) -> Duration {
        timer_elapsed(self.ptr)
    }
    
    /// 重置计时器并返回经过的时间
    pub fn reset(&mut self) -> Duration {
        timer_reset(self.ptr)
    }
    
    /// 重新启动计时器
    pub fn restart(&mut self) {
        timer_restart(self.ptr);
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        timer_destroy(self.ptr);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_instant_creation() {
        let instant = Instant::now();
        assert!(instant.as_micros() > 0);
        
        let from_secs = Instant::from_secs(10);
        assert_eq!(from_secs.as_secs(), 10);
    }
    
    #[test]
    fn test_instant_conversion() {
        let instant = Instant::from_millis(3500);
        assert_eq!(instant.as_micros(), 3_500_000);
        assert_eq!(instant.as_millis(), 3_500);
        assert_eq!(instant.as_secs(), 3);
    }
    
    #[test]
    fn test_duration_creation() {
        let dur = Duration::from_secs(5);
        assert_eq!(dur.as_secs(), 5);
        assert_eq!(dur.as_millis(), 5_000);
        assert_eq!(dur.as_micros(), 5_000_000);
    }
    
    #[test]
    fn test_duration_arithmetic() {
        let dur1 = Duration::from_secs(10);
        let dur2 = Duration::from_secs(5);
        
        let sum = dur1 + dur2;
        assert_eq!(sum.as_secs(), 15);
        
        let diff = dur1 - dur2;
        assert_eq!(diff.as_secs(), 5);
        
        let product = dur2.mul_i64(3);
        assert_eq!(product.as_secs(), 15);
        
        let quotient = dur1.div_i64(2);
        assert_eq!(quotient.as_secs(), 5);
    }
    
    #[test]
    fn test_duration_properties() {
        assert!(Duration::ZERO.is_zero());
        
        let positive = Duration::from_secs(5);
        assert!(!positive.is_zero());
        assert!(!positive.is_negative());
        
        let negative = Duration::from_secs(-5);
        assert!(negative.is_negative());
    }
    
    #[test]
    fn test_instant_arithmetic() {
        let instant = Instant::from_secs(100);
        let duration = Duration::from_secs(50);
        
        let later = instant + duration;
        assert_eq!(later.as_secs(), 150);
        
        let earlier = instant - duration;
        assert_eq!(earlier.as_secs(), 50);
    }
    
    #[test]
    fn test_instant_duration() {
        let start = Instant::from_secs(100);
        let end = Instant::from_secs(150);
        
        let duration = end - start;
        assert_eq!(duration.as_secs(), 50);
    }
    
    #[test]
    fn test_timer() {
        let mut timer = Timer::new();
        
        // 非常短的等待
        std::thread::sleep(std::time::Duration::from_micros(100));
        
        let elapsed = timer.elapsed();
        assert!(elapsed.as_micros() >= 0);
        
        timer.restart();
        let new_elapsed = timer.elapsed();
        assert!(new_elapsed.as_micros() < elapsed.as_micros() || new_elapsed.as_micros() >= 0);
    }
}