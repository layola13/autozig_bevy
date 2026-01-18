//! # AutoZig Time - Bevy Time System implemented in Zig
//!
//! 90% Zig实现，10% Rust包装
//!
//! 提供以下核心功能：
//! - Time 资源: 追踪帧间增量时间和总运行时间
//! - Stopwatch: 秒表功能，支持暂停/恢复/重置
//! - Timer: 计时器功能，支持一次性和循环模式
//! - 时间工具函数: 纳秒/秒转换等
//! - Fixed: 固定时间步
//! - Real: 真实时间
//! - Virtual: 虚拟时间
//! - TimePlugin: 时间插件
//! - TimeUpdateStrategy: 时间更新策略

use autozig::include_zig;
use std::fmt;
#[cfg(feature = "std")]
pub use crossbeam_channel::{Receiver, Sender, TrySendError};

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

// ========== Fixed Time Context ==========

/// 固定时间步上下文
///
/// 用于固定频率的时间更新，适用于物理模拟等需要稳定时间步长的场景
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Fixed {
    pub timestep_nanos: u64,
    pub overstep_nanos: u64,
}

impl Default for Fixed {
    fn default() -> Self {
        Self {
            timestep_nanos: 15_625_000, // 64 Hz (15.625 ms)
            overstep_nanos: 0,
        }
    }
}

impl Fixed {
    /// 创建具有指定时间步长的固定时间（秒）
    pub fn from_seconds(seconds: f32) -> Self {
        Self {
            timestep_nanos: secs_to_nanos(seconds),
            overstep_nanos: 0,
        }
    }

    /// 创建具有指定频率的固定时间（Hz）
    pub fn from_hz(hz: f32) -> Self {
        Self::from_seconds(1.0 / hz)
    }

    /// 获取时间步长（纳秒）
    pub fn timestep(&self) -> u64 {
        self.timestep_nanos
    }

    /// 设置时间步长（秒）
    pub fn set_timestep(&mut self, seconds: f32) {
        self.timestep_nanos = secs_to_nanos(seconds);
    }

    /// 获取超步时间（纳秒）
    pub fn overstep(&self) -> u64 {
        self.overstep_nanos
    }

    /// 累积超步时间
    pub fn accumulate(&mut self, delta_nanos: u64) {
        self.overstep_nanos += delta_nanos;
    }

    /// 消耗一个时间步
    pub fn expend(&mut self) -> bool {
        if self.overstep_nanos >= self.timestep_nanos {
            self.overstep_nanos -= self.timestep_nanos;
            true
        } else {
            false
        }
    }
}

// ========== Real Time Context ==========

/// 真实时间上下文
///
/// 追踪真实世界的墙钟时间，不受游戏暂停或时间缩放的影响
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Real {
    pub startup_nanos: u64,
    pub first_update_nanos: u64,
    pub last_update_nanos: u64,
}

impl Default for Real {
    fn default() -> Self {
        let now = now_nanos();
        Self {
            startup_nanos: now,
            first_update_nanos: 0,
            last_update_nanos: 0,
        }
    }
}

impl Real {
    /// 创建新的真实时间上下文
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取启动时间（纳秒）
    pub fn startup(&self) -> u64 {
        self.startup_nanos
    }

    /// 获取首次更新时间（纳秒）
    pub fn first_update(&self) -> Option<u64> {
        if self.first_update_nanos == 0 {
            None
        } else {
            Some(self.first_update_nanos)
        }
    }

    /// 获取最后更新时间（纳秒）
    pub fn last_update(&self) -> Option<u64> {
        if self.last_update_nanos == 0 {
            None
        } else {
            Some(self.last_update_nanos)
        }
    }
}

// ========== Virtual Time Context ==========

/// 虚拟时间上下文
///
/// 游戏时间，可以被暂停、加速或减速
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Virtual {
    pub max_delta_nanos: u64,
    pub paused: bool,
    pub relative_speed: f32,
    pub effective_speed: f32,
}

impl Default for Virtual {
    fn default() -> Self {
        Self {
            max_delta_nanos: 250_000_000, // 250 ms
            paused: false,
            relative_speed: 1.0,
            effective_speed: 1.0,
        }
    }
}

impl Virtual {
    /// 创建新的虚拟时间上下文
    pub fn new() -> Self {
        Self::default()
    }

    /// 暂停时间
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// 恢复时间
    pub fn unpause(&mut self) {
        self.paused = false;
    }

    /// 切换暂停状态
    pub fn toggle(&mut self) {
        self.paused = !self.paused;
    }

    /// 是否已暂停
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// 设置相对速度
    pub fn set_relative_speed(&mut self, speed: f32) {
        self.relative_speed = speed.max(0.0);
    }

    /// 获取相对速度
    pub fn relative_speed(&self) -> f32 {
        self.relative_speed
    }

    /// 获取有效速度（考虑暂停状态）
    pub fn effective_speed(&self) -> f32 {
        self.effective_speed
    }

    /// 设置最大增量时间（秒）
    pub fn set_max_delta(&mut self, seconds: f32) {
        self.max_delta_nanos = secs_to_nanos(seconds);
    }

    /// 获取最大增量时间（纳秒）
    pub fn max_delta(&self) -> u64 {
        self.max_delta_nanos
    }
}

// ========== Time Update Strategy ==========

/// 时间更新策略
///
/// 控制时间系统如何更新
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimeUpdateStrategy {
    /// 自动更新（使用系统时钟）
    Automatic,
    /// 手动指定时间点（纳秒）
    ManualInstant(u64),
    /// 手动指定增量时间（纳秒）
    ManualDuration(u64),
    /// 固定时间步数（每次更新运行n个固定步）
    FixedTimesteps(u32),
}

impl Default for TimeUpdateStrategy {
    fn default() -> Self {
        TimeUpdateStrategy::Automatic
    }
}

impl TimeUpdateStrategy {
    /// 创建手动增量时间策略（秒）
    pub fn manual_duration_secs(seconds: f32) -> Self {
        TimeUpdateStrategy::ManualDuration(secs_to_nanos(seconds))
    }
}

// ========== Time Plugin ==========

/// 时间插件
///
/// 向应用添加时间功能的插件
#[derive(Debug, Default, Clone, Copy)]
pub struct TimePlugin;

impl TimePlugin {
    /// 创建新的时间插件
    pub fn new() -> Self {
        Self
    }
}

use autozig_app::{App, Plugin};

impl Plugin for TimePlugin {
    fn name(&self) -> &str {
        "TimePlugin"
    }
    
    fn build(&self, app: &mut App) {
        // 1. Initialize Time resource
        let time = Time::new();
        app.insert_resource(time);
        
        // 2. Register time update system
        // Note: Using a raw function pointer for now as system registration
        // needs to interact with ECS directly.
        // In a full implementation, we would use:
        // app.add_systems(First, time_system);
        
        app.add_systems::<autozig_ecs::into_system::ExclusiveSystemMarker>(autozig_ecs::schedule::First, time_system_wrapper);
    }
}

/// Rust system wrapper for time_system
pub fn time_system_wrapper(_world: &mut autozig_ecs::world::World) {
    time_system_c_impl();
}

/// System to update the Time resource (C implementation)
#[no_mangle]
pub extern "C" fn time_system_c_impl() {
    unsafe {
        // TODO: retrieve APP_PTR safely or context
        // For now, we rely on the fact that App::update_raw calls this
        // But wait, systems don't take arguments in this FFI model yet?
        // We need to fetch the resource from the World.
        
        // Use the autozig-render APP_PTR hack for now until Tier 1 cleanup
        let app_ptr = autozig_render::APP_PTR;
        if !app_ptr.is_null() {
            if let Some(time) = autozig_app::App::get_resource_raw::<Time>(app_ptr) {
                // Must cast const reference to mut pointer to call update
                let time_mut = time as *const Time as *mut Time;
                (*time_mut).update();
            }
        }
    }
}

// ========== Time Systems ==========

/// 时间系统标签
///
/// 用于系统调度的标签，任何与Time交互的系统都应在此之后运行
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeSystems;

// ========== Time Channels (仅在std特性启用时可用) ==========

/// 时间接收器
///
/// 用于从渲染世界接收时间的通道资源
#[cfg(feature = "std")]
pub struct TimeReceiver(pub Receiver<u64>);

/// 时间发送器
///
/// 用于向主世界发送时间的通道资源
#[cfg(feature = "std")]
pub struct TimeSender(pub Sender<u64>);

/// 创建时间通道
///
/// 创建用于在渲染世界和主世界之间发送时间的通道
#[cfg(feature = "std")]
pub fn create_time_channels() -> (TimeSender, TimeReceiver) {
    let (s, r) = crossbeam_channel::bounded::<u64>(2);
    (TimeSender(s), TimeReceiver(r))
}

// ========== Trait Implementations ==========
// Implement Resource/Component for Time types to allow ECS storage

use autozig_ecs::resource::Resource;
use autozig_ecs::component::Component;

// impl Resource for Time {} -> Covered by blanket impl
impl Component for Time {} // Resources are often implemented as Components in AutoZig ECS for flexibility

// impl Resource for Fixed {} -> Covered by blanket impl
impl Component for Fixed {}

// impl Resource for Virtual {} -> Covered by blanket impl
impl Component for Virtual {}

// impl Resource for Real {} -> Covered by blanket impl
impl Component for Real {}