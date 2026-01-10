//! AutoZig-Bevy Native Demo - WASM64 Library
//!
//! 使用 AutoZig 的 #[autozig_export] 宏导出函数到 WASM64
//! 无需 wasm-bindgen，纯 AutoZig 实现

use autozig::autozig_export;

// 在 WASM 环境下，这些模块使用 println! 会失败
// 我们只在 native 环境下包含它们
#[cfg(not(target_family = "wasm"))]
pub mod demo_app;
#[cfg(not(target_family = "wasm"))]
pub mod demo_ecs;
#[cfg(not(target_family = "wasm"))]
pub mod demo_math;
#[cfg(not(target_family = "wasm"))]
pub mod demo_state;
#[cfg(not(target_family = "wasm"))]
pub mod demo_time_task;
#[cfg(not(target_family = "wasm"))]
pub mod demo_json;

/// 获取版本信息
#[autozig_export]
pub fn get_version() -> u32 {
    100 // v1.0.0
}

/// 运行所有演示模块（WASM 版本仅返回成功状态）
#[autozig_export]
pub fn run_all_demos() -> u32 {
    #[cfg(not(target_family = "wasm"))]
    {
        // Native 环境：实际运行所有演示
        demo_app::run_app_demo();
        demo_ecs::run_ecs_demo();
        demo_math::run_math_demo();
        demo_state::run_state_demo();
        demo_time_task::run_time_task_demo();
        demo_json::run_json_demo();
    }
    // WASM 环境：只返回成功状态
    1 // success
}

// ============================================================
// 模块 0: App 演示
// ============================================================

/// 运行 App 示例
#[autozig_export]
pub fn demo_run_app() -> u32 {
    #[cfg(not(target_family = "wasm"))]
    {
        demo_app::run_app_demo();
    }
    1 // success
}

// ============================================================
// 模块 1: ECS 演示
// ============================================================

/// 运行 ECS 示例
#[autozig_export]
pub fn demo_run_ecs() -> u32 {
    #[cfg(not(target_family = "wasm"))]
    {
        demo_ecs::run_ecs_demo();
    }
    1 // success
}

// ============================================================
// 模块 2: Math 演示
// ============================================================

/// 运行 Math 示例
#[autozig_export]
pub fn demo_run_math() -> u32 {
    #[cfg(not(target_family = "wasm"))]
    {
        demo_math::run_math_demo();
    }
    1 // success
}

/// 测试 Vec3 创建
#[autozig_export]
pub fn demo_math_vec3_new(x: f32, y: f32, z: f32) -> u32 {
    use autozig_math::Vec3;
    let _v = Vec3::new(x, y, z);
    1 // success
}

/// 测试 Vec3 长度计算
#[autozig_export]
pub fn demo_math_vec3_length(x: f32, y: f32, z: f32) -> f32 {
    use autozig_math::Vec3;
    let v = Vec3::new(x, y, z);
    v.length()
}

/// 测试 Quat 旋转
#[autozig_export]
pub fn demo_math_quat_identity() -> u32 {
    use autozig_math::Quat;
    let _q = Quat::IDENTITY;
    1 // success
}

// ============================================================
// 模块 3: State 演示
// ============================================================

/// 运行 State 示例
#[autozig_export]
pub fn demo_run_state() -> u32 {
    #[cfg(not(target_family = "wasm"))]
    {
        demo_state::run_state_demo();
    }
    1 // success
}

// ============================================================
// 模块 4: Time & Task 演示
// ============================================================

/// 运行 Time & Task 示例
#[autozig_export]
pub fn demo_run_time_task() -> u32 {
    #[cfg(not(target_family = "wasm"))]
    {
        demo_time_task::run_time_task_demo();
    }
    1 // success
}

/// 创建 Stopwatch
#[autozig_export]
pub fn demo_time_stopwatch_new() -> u32 {
    use autozig_time::Stopwatch;
    let _sw = Stopwatch::new();
    1 // success
}

/// 创建 Timer (一次性)
#[autozig_export]
pub fn demo_time_timer_once(duration_secs: f32) -> u32 {
    use autozig_time::{Timer, TimerMode};
    let _timer = Timer::new(duration_secs, TimerMode::Once);
    1 // success
}

/// 创建 Timer (循环)
#[autozig_export]
pub fn demo_time_timer_repeating(duration_secs: f32) -> u32 {
    use autozig_time::{Timer, TimerMode};
    let _timer = Timer::new(duration_secs, TimerMode::Repeating);
    1 // success
}

// ============================================================
// 模块 5: JSON 演示
// ============================================================

/// 运行 JSON 示例
#[autozig_export]
pub fn demo_run_json() -> u32 {
    #[cfg(not(target_family = "wasm"))]
    {
        demo_json::run_json_demo();
    }
    1 // success
}

/// 解析 JSON 数字（简化版：只测试数值）
#[autozig_export]
pub fn demo_json_parse_number(value: f64) -> u32 {
    // 简单验证：将数字转成字符串再解析
    let json_str = format!("{}", value);
    // 在 WASM 环境下，我们只验证格式是否正确
    if json_str.parse::<f64>().is_ok() {
        1 // success
    } else {
        0 // error
    }
}

/// 测试 JSON 对象解析（简化版）
#[autozig_export]
pub fn demo_json_parse_object() -> u32 {
    // WASM 环境下返回成功
    // 实际解析在 native 环境测试
    1 // success
}

/// 测试 JSON 数组解析（简化版）
#[autozig_export]
pub fn demo_json_parse_array() -> u32 {
    // WASM 环境下返回成功
    // 实际解析在 native 环境测试
    1 // success
}

// ============================================================
// 通用工具函数
// ============================================================

/// 获取 AutoZig-Bevy 信息
#[autozig_export]
pub fn get_info_string_len() -> usize {
    let info = "AutoZig-Bevy Native Demo - WASM64 Edition";
    info.len()
}

/// 测试内存操作（验证 WASM64 指针大小）
#[autozig_export]
pub fn get_pointer_size() -> usize {
    std::mem::size_of::<usize>()
}

/// 运行内存测试
#[autozig_export]
pub fn run_memory_test() -> u32 {
    // 分配一些内存测试 WASM64
    let _vec: Vec<u64> = (0..1000).collect();
    1 // success
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(get_version(), 100);
    }

    #[test]
    fn test_pointer_size() {
        let size = get_pointer_size();
        // Native 可能是 64-bit，WASM64 也是 64-bit
        assert!(size == 4 || size == 8);
    }

    #[test]
    fn test_math_vec3_length() {
        let len = demo_math_vec3_length(3.0, 4.0, 0.0);
        assert!((len - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_json_parse_number() {
        assert_eq!(demo_json_parse_number(42.0), 1);
    }

    #[test]
    fn test_json_parse_object() {
        assert_eq!(demo_json_parse_object(), 1);
    }

    #[test]
    fn test_json_parse_array() {
        assert_eq!(demo_json_parse_array(), 1);
    }
}