use autozig_time::*;
use std::thread;
use std::time::Duration;

#[test]
fn test_time_creation() {
    let time = Time::new();
    assert_eq!(time.delta, 0.0);
    assert_eq!(time.elapsed, 0.0);
    assert_eq!(time.delta_nanos, 0);
    assert_eq!(time.elapsed_nanos, 0);
    assert!(time.startup_nanos > 0);
    assert!(time.last_update_nanos > 0);
}

#[test]
fn test_time_update() {
    let mut time = Time::new();
    
    // 等待一小段时间
    thread::sleep(Duration::from_millis(10));
    
    // 更新时间
    time.update();
    
    // 验证时间已更新
    assert!(time.delta > 0.0);
    assert!(time.elapsed > 0.0);
    assert!(time.delta_nanos > 0);
    assert!(time.elapsed_nanos > 0);
    
    // delta 应该大约是 10ms
    assert!(time.delta >= 0.008 && time.delta <= 0.020);
}

#[test]
fn test_time_delta() {
    let mut time = Time::new();
    
    // 手动设置增量时间
    time.set_delta(0.016); // 约 16ms (60 FPS)
    
    assert!((time.delta_seconds() - 0.016).abs() < 0.001);
    assert!((time.elapsed_seconds() - 0.016).abs() < 0.001);
}

#[test]
fn test_time_reset() {
    let mut time = Time::new();
    time.set_delta(1.0);
    
    assert!(time.elapsed > 0.0);
    
    time.reset();
    
    assert_eq!(time.delta, 0.0);
    assert_eq!(time.elapsed, 0.0);
    assert_eq!(time.delta_nanos, 0);
    assert_eq!(time.elapsed_nanos, 0);
}

#[test]
fn test_stopwatch_creation() {
    let stopwatch = Stopwatch::new();
    assert_eq!(stopwatch.elapsed_nanos, 0);
    assert_eq!(stopwatch.paused, false);
    assert_eq!(stopwatch.elapsed(), 0);
    assert_eq!(stopwatch.elapsed_secs(), 0.0);
}

#[test]
fn test_stopwatch_tick() {
    let mut stopwatch = Stopwatch::new();
    
    // 模拟 16ms (60 FPS)
    let delta_nanos = secs_to_nanos(0.016);
    stopwatch.tick(delta_nanos);
    
    assert!(stopwatch.elapsed() > 0);
    assert!((stopwatch.elapsed_secs() - 0.016).abs() < 0.001);
    
    // 再次 tick
    stopwatch.tick(delta_nanos);
    assert!((stopwatch.elapsed_secs() - 0.032).abs() < 0.001);
}

#[test]
fn test_stopwatch_pause() {
    let mut stopwatch = Stopwatch::new();
    let delta_nanos = secs_to_nanos(0.016);
    
    // 正常计时
    stopwatch.tick(delta_nanos);
    let elapsed_before_pause = stopwatch.elapsed();
    
    // 暂停
    stopwatch.pause();
    assert!(stopwatch.is_paused());
    
    // 暂停期间 tick 不应增加时间
    stopwatch.tick(delta_nanos);
    assert_eq!(stopwatch.elapsed(), elapsed_before_pause);
    
    // 恢复
    stopwatch.unpause();
    assert!(!stopwatch.is_paused());
    
    // 恢复后继续计时
    stopwatch.tick(delta_nanos);
    assert!(stopwatch.elapsed() > elapsed_before_pause);
}

#[test]
fn test_stopwatch_reset() {
    let mut stopwatch = Stopwatch::new();
    let delta_nanos = secs_to_nanos(1.0);
    
    stopwatch.tick(delta_nanos);
    assert!(stopwatch.elapsed() > 0);
    
    stopwatch.reset();
    assert_eq!(stopwatch.elapsed(), 0);
    assert_eq!(stopwatch.elapsed_secs(), 0.0);
}

#[test]
fn test_timer_once() {
    let mut timer = Timer::new(1.0, TimerMode::Once);
    
    assert!(!timer.finished());
    assert!(!timer.just_finished());
    assert_eq!(timer.percent(), 0.0);
    assert_eq!(timer.percent_left(), 1.0);
    
    // 模拟 0.5 秒
    let delta_nanos = secs_to_nanos(0.5);
    timer.tick(delta_nanos);
    
    assert!(!timer.finished());
    assert!(!timer.just_finished());
    assert!((timer.percent() - 0.5).abs() < 0.01);
    assert!((timer.percent_left() - 0.5).abs() < 0.01);
    
    // 再模拟 0.5 秒，应该完成
    timer.tick(delta_nanos);
    
    assert!(timer.finished());
    assert!(timer.just_finished());
    assert_eq!(timer.times_finished_this_tick(), 1);
    assert!((timer.percent() - 1.0).abs() < 0.01);
    
    // Once 模式应该暂停
    assert!(timer.is_paused());
    
    // 再次 tick 不应改变状态
    timer.tick(delta_nanos);
    assert_eq!(timer.times_finished_this_tick(), 0); // 新的 tick，应该重置
}

#[test]
fn test_timer_repeating() {
    let mut timer = Timer::new(1.0, TimerMode::Repeating);
    
    // 模拟 2.5 秒
    let delta_nanos = secs_to_nanos(2.5);
    timer.tick(delta_nanos);
    
    assert!(timer.finished());
    assert!(timer.just_finished());
    
    // 应该完成 2 次
    assert_eq!(timer.times_finished_this_tick(), 2);
    
    // 剩余进度应该是 0.5
    assert!((timer.elapsed_secs() - 0.5).abs() < 0.01);
    
    // Repeating 模式不应该暂停
    assert!(!timer.is_paused());
}

#[test]
fn test_timer_percent() {
    let mut timer = Timer::new(2.0, TimerMode::Once);
    
    // 0%
    assert_eq!(timer.percent(), 0.0);
    assert_eq!(timer.percent_left(), 1.0);
    
    // 25%
    timer.tick(secs_to_nanos(0.5));
    assert!((timer.percent() - 0.25).abs() < 0.01);
    assert!((timer.percent_left() - 0.75).abs() < 0.01);
    
    // 75%
    timer.tick(secs_to_nanos(1.0));
    assert!((timer.percent() - 0.75).abs() < 0.01);
    assert!((timer.percent_left() - 0.25).abs() < 0.01);
    
    // 100%
    timer.tick(secs_to_nanos(0.5));
    assert!((timer.percent() - 1.0).abs() < 0.01);
    assert!((timer.percent_left() - 0.0).abs() < 0.01);
}

#[test]
fn test_timer_reset() {
    let mut timer = Timer::new(1.0, TimerMode::Once);
    
    // 完成计时器
    timer.tick(secs_to_nanos(1.0));
    assert!(timer.finished());
    assert!(timer.is_paused());
    
    // 重置
    timer.reset();
    assert!(!timer.finished());
    assert!(!timer.is_paused());
    assert_eq!(timer.percent(), 0.0);
    assert_eq!(timer.elapsed_secs(), 0.0);
}

#[test]
fn test_timer_set_duration() {
    let mut timer = Timer::new(1.0, TimerMode::Once);
    assert!((timer.duration() - 1.0).abs() < 0.001);
    
    timer.set_duration(2.0);
    assert!((timer.duration() - 2.0).abs() < 0.001);
}

#[test]
fn test_timer_pause_unpause() {
    let mut timer = Timer::new(1.0, TimerMode::Once);
    
    timer.tick(secs_to_nanos(0.5));
    let elapsed_before = timer.elapsed_secs();
    
    timer.pause();
    assert!(timer.is_paused());
    
    // 暂停期间不应增加时间
    timer.tick(secs_to_nanos(0.5));
    assert!((timer.elapsed_secs() - elapsed_before).abs() < 0.001);
    
    timer.unpause();
    assert!(!timer.is_paused());
    
    // 恢复后继续计时
    timer.tick(secs_to_nanos(0.5));
    assert!(timer.elapsed_secs() > elapsed_before);
}

#[test]
fn test_nanos_conversion() {
    // 测试秒转纳秒
    let secs = 1.5f32;
    let nanos = secs_to_nanos(secs);
    assert_eq!(nanos, 1_500_000_000);
    
    // 测试纳秒转秒
    let back = nanos_to_secs(nanos);
    assert!((back - secs).abs() < 0.001);
    
    // 测试零值
    assert_eq!(secs_to_nanos(0.0), 0);
    assert_eq!(nanos_to_secs(0), 0.0);
}

#[test]
fn test_now_nanos() {
    let now1 = now_nanos();
    thread::sleep(Duration::from_millis(1));
    let now2 = now_nanos();
    
    assert!(now2 > now1);
    assert!(now2 - now1 >= 1_000_000); // 至少 1ms
}

#[test]
fn test_time_system_integration() {
    // 模拟游戏循环
    let mut time = Time::new();
    let mut timer = Timer::new(1.0, TimerMode::Repeating);
    let mut stopwatch = Stopwatch::new();
    
    // 模拟 10 帧，每帧 16ms (约 60 FPS)
    for _ in 0..10 {
        thread::sleep(Duration::from_millis(16));
        time.update();
        
        let delta = time.delta_nanos();
        timer.tick(delta);
        stopwatch.tick(delta);
    }
    
    // 验证时间系统工作正常
    assert!(time.elapsed_seconds() >= 0.15); // 至少 150ms
    assert!(stopwatch.elapsed_secs() >= 0.15);
    
    // Timer 应该还没完成（需要 1 秒）
    assert!(!timer.finished() || timer.times_finished_this_tick() > 0);
}

#[test]
fn test_timer_from_seconds() {
    let timer1 = Timer::from_seconds(2.5, TimerMode::Once);
    let timer2 = Timer::new(2.5, TimerMode::Once);
    
    assert!((timer1.duration() - timer2.duration()).abs() < 0.001);
}

#[test]
fn test_stopwatch_default() {
    let sw1 = Stopwatch::default();
    let sw2 = Stopwatch::new();
    
    assert_eq!(sw1.elapsed_nanos, sw2.elapsed_nanos);
    assert_eq!(sw1.paused, sw2.paused);
}

#[test]
fn test_time_default() {
    let time1 = Time::default();
    let time2 = Time::new();
    
    assert_eq!(time1.delta, time2.delta);
    assert_eq!(time1.elapsed, time2.elapsed);
}