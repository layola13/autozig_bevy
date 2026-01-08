//! # AutoZig Time - Bevy Time System implemented in Zig
//!
//! 90% Zig实现，10% Rust包装
//! 
//! 提供以下核心功能：
//! - Time 资源: 追踪帧间增量时间和总运行时间
//! - Stopwatch: 秒表功能，支持暂停/恢复/重置
//! - Timer: 计时器功能，支持一次性和循环模式
//! - 时间工具函数: 纳秒/秒转换等

use autozig::include_zig;
use std::fmt;

// ========== Timer Mode ==========

/// Timer mode enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimerMode {
    Once = 0,      // 触发一次
    Repeating = 1, // 循环触发
}

impl fmt::Display for TimerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerMode::Once => write!(f, "Once"),
            TimerMode::Repeating => write!(f, "Repeating"),
        }
    }
}

// ========== Stopwatch ==========

/// Stopwatch structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Stopwatch {
    pub elapsed_nanos: u64,
    pub paused: bool,
}

include_zig!("src/zig/stopwatch.zig", {
    fn stopwatch_new() -> Stopwatch;
    fn stopwatch_tick(stopwatch: *mut Stopwatch, delta_nanos: u64);
    fn stopwatch_pause(stopwatch: *mut Stopwatch);
    fn stopwatch_unpause(stopwatch: *mut Stopwatch);
    fn stopwatch_reset(stopwatch: *mut Stopwatch);
    fn stopwatch_elapsed(stopwatch: *const Stopwatch) -> u64;
    fn stopwatch_elapsed_secs(stopwatch: *const Stopwatch) -> f32;
    fn stopwatch_is_paused(stopwatch: *const Stopwatch) -> bool;
});

impl Stopwatch {
    pub fn new() -> Self {
        stopwatch_new()
    }
    
    pub fn tick(&mut self, delta_nanos: u64) {
        stopwatch_tick(self, delta_nanos);
    }
    
    pub fn pause(&mut self) {
        stopwatch_pause(self);
    }
    
    pub fn unpause(&mut self) {
        stopwatch_unpause(self);
    }
    
    pub fn reset(&mut self) {
        stopwatch_reset(self);
    }
    
    pub fn elapsed(&self) -> u64 {
        stopwatch_elapsed(self)
    }
    
    pub fn elapsed_secs(&self) -> f32 {
        stopwatch_elapsed_secs(self)
    }
    
    pub fn is_paused(&self) -> bool {
        stopwatch_is_paused(self)
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

// ========== Timer ==========

/// Timer structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Timer {
    pub stopwatch: Stopwatch,
    pub duration_nanos: u64,
    pub mode: TimerMode,
    pub finished: bool,
    pub times_finished_this_tick: u32,
}

include_zig!("src/zig/timer.zig", {
    fn timer_new(duration_secs: f32, mode: TimerMode) -> Timer;
    fn timer_tick(timer: *mut Timer, delta_nanos: u64);
    fn timer_finished(timer: *const Timer) -> bool;
    fn timer_just_finished(timer: *const Timer) -> bool;
    fn timer_reset(timer: *mut Timer);
    fn timer_percent(timer: *const Timer) -> f32;
    fn timer_percent_left(timer: *const Timer) -> f32;
    fn timer_pause(timer: *mut Timer);
    fn timer_unpause(timer: *mut Timer);
    fn timer_is_paused(timer: *const Timer) -> bool;
    fn timer_elapsed_secs(timer: *const Timer) -> f32;
    fn timer_duration_secs(timer: *const Timer) -> f32;
    fn timer_set_duration(timer: *mut Timer, duration_secs: f32);
    fn timer_times_finished(timer: *const Timer) -> u32;
});

impl Timer {
    pub fn new(duration_secs: f32, mode: TimerMode) -> Self {
        timer_new(duration_secs, mode)
    }
    
    pub fn from_seconds(seconds: f32, mode: TimerMode) -> Self {
        Self::new(seconds, mode)
    }
    
    pub fn tick(&mut self, delta_nanos: u64) {
        timer_tick(self, delta_nanos);
    }
    
    pub fn finished(&self) -> bool {
        timer_finished(self)
    }
    
    pub fn just_finished(&self) -> bool {
        timer_just_finished(self)
    }
    
    pub fn reset(&mut self) {
        timer_reset(self);
    }
    
    pub fn percent(&self) -> f32 {
        timer_percent(self)
    }
    
    pub fn percent_left(&self) -> f32 {
        timer_percent_left(self)
    }
    
    pub fn pause(&mut self) {
        timer_pause(self);
    }
    
    pub fn unpause(&mut self) {
        timer_unpause(self);
    }
    
    pub fn is_paused(&self) -> bool {
        timer_is_paused(self)
    }
    
    pub fn elapsed_secs(&self) -> f32 {
        timer_elapsed_secs(self)
    }
    
    pub fn duration(&self) -> f32 {
        timer_duration_secs(self)
    }
    
    pub fn set_duration(&mut self, duration_secs: f32) {
        timer_set_duration(self, duration_secs);
    }
    
    pub fn times_finished_this_tick(&self) -> u32 {
        timer_times_finished(self)
    }
}

// ========== Time Resource ==========

/// Time resource structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub delta: f32,
    pub elapsed: f32,
    pub delta_nanos: u64,
    pub elapsed_nanos: u64,
    pub startup_nanos: u64,
    pub last_update_nanos: u64,
}

include_zig!("src/zig/time.zig", {
    fn time_create() -> Time;
    fn time_update(time: *mut Time);
    fn time_set_delta(time: *mut Time, delta_secs: f32);
    fn time_delta_seconds(time: *const Time) -> f32;
    fn time_elapsed_seconds(time: *const Time) -> f32;
    fn time_delta_nanos(time: *const Time) -> u64;
    fn time_elapsed_nanos(time: *const Time) -> u64;
    fn time_reset(time: *mut Time);
    fn time_now_nanos() -> u64;
    fn time_nanos_to_secs(nanos: u64) -> f32;
    fn time_secs_to_nanos(secs: f32) -> u64;
});

impl Time {
    pub fn new() -> Self {
        time_create()
    }
    
    pub fn update(&mut self) {
        time_update(self);
    }
    
    pub fn set_delta(&mut self, delta_secs: f32) {
        time_set_delta(self, delta_secs);
    }
    
    pub fn delta_seconds(&self) -> f32 {
        time_delta_seconds(self)
    }
    
    pub fn elapsed_seconds(&self) -> f32 {
        time_elapsed_seconds(self)
    }
    
    pub fn delta_nanos(&self) -> u64 {
        time_delta_nanos(self)
    }
    
    pub fn elapsed_nanos(&self) -> u64 {
        time_elapsed_nanos(self)
    }
    
    pub fn reset(&mut self) {
        time_reset(self);
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

// ========== Utility Functions ==========

/// 获取当前纳秒时间戳
pub fn now_nanos() -> u64 {
    time_now_nanos()
}

/// 纳秒转换为秒
pub fn nanos_to_secs(nanos: u64) -> f32 {
    time_nanos_to_secs(nanos)
}

/// 秒转换为纳秒
pub fn secs_to_nanos(secs: f32) -> u64 {
    time_secs_to_nanos(secs)
}

// ========== Tests ==========

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_mode_display() {
        assert_eq!(format!("{}", TimerMode::Once), "Once");
        assert_eq!(format!("{}", TimerMode::Repeating), "Repeating");
    }

    #[test]
    fn test_stopwatch_default() {
        let stopwatch = Stopwatch::default();
        assert_eq!(stopwatch.elapsed_nanos, 0);
        assert_eq!(stopwatch.paused, false);
    }

    #[test]
    fn test_time_default() {
        let time = Time::default();
        assert_eq!(time.delta, 0.0);
        assert_eq!(time.elapsed, 0.0);
    }

    #[test]
    fn test_utility_functions() {
        let secs = 1.5f32;
        let nanos = secs_to_nanos(secs);
        let back_to_secs = nanos_to_secs(nanos);
        
        // 允许浮点误差
        assert!((back_to_secs - secs).abs() < 0.001);
    }
}