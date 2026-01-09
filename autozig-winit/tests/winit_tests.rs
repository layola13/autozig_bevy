//! Tests for autozig-winit module

use autozig_winit::*;

// ========== EventLoop Tests ==========

#[test]
fn test_event_loop_creation() {
    let event_loop = EventLoop::new();
    assert_eq!(event_loop.state(), EventLoopState::Idle);
    assert_eq!(event_loop.frame_count(), 0);
    assert!(!event_loop.is_running());
}

#[test]
fn test_event_loop_lifecycle() {
    let mut event_loop = EventLoop::new();
    
    // Initially idle
    assert!(!event_loop.is_running());
    assert!(!event_loop.is_exiting());
    
    // Start the loop
    event_loop.start();
    assert!(event_loop.is_running());
    assert!(!event_loop.is_exiting());
    assert_eq!(event_loop.state(), EventLoopState::Running);
    
    // Stop the loop
    event_loop.stop();
    assert!(!event_loop.is_running());
    assert!(event_loop.is_exiting());
    assert_eq!(event_loop.state(), EventLoopState::Exiting);
}

#[test]
fn test_event_loop_update() {
    let mut event_loop = EventLoop::new();
    event_loop.start();
    
    // First frame
    event_loop.update(0.0);
    assert_eq!(event_loop.frame_count(), 1);
    assert_eq!(event_loop.delta_time(), 0.0);
    
    // Second frame at 16.67ms (60fps)
    event_loop.update(16.67);
    assert_eq!(event_loop.frame_count(), 2);
    // Delta time should be non-zero after first update
    assert!(event_loop.delta_time() > 0.0);
    
    // Third frame
    event_loop.update(33.34);
    assert_eq!(event_loop.frame_count(), 3);
    // Delta time should still be non-zero
    assert!(event_loop.delta_time() > 0.0);
}

// ========== KeyboardEvent Tests ==========

#[test]
fn test_keyboard_event_creation() {
    let event = KeyboardEvent::key_down(KeyCode::A);
    assert!(event.is_key_down());
    assert!(!event.is_key_up());
    assert_eq!(event.key_code, KeyCode::A);
}

#[test]
fn test_keyboard_event_key_up() {
    let event = KeyboardEvent::key_up(KeyCode::Space);
    assert!(!event.is_key_down());
    assert!(event.is_key_up());
    assert_eq!(event.key_code, KeyCode::Space);
}

#[test]
fn test_keyboard_event_modifiers() {
    let event = KeyboardEvent::key_down(KeyCode::C)
        .with_modifiers(false, true, false, false);
    
    assert!(event.is_key_down());
    assert_eq!(event.key_code, KeyCode::C);
    assert!(!event.shift);
    assert!(event.ctrl);
    assert!(!event.alt);
    assert!(!event.meta);
}

#[test]
fn test_keyboard_event_all_keys() {
    // Test letter keys
    let event_a = KeyboardEvent::key_down(KeyCode::A);
    assert_eq!(event_a.key_code, KeyCode::A);
    
    let event_z = KeyboardEvent::key_down(KeyCode::Z);
    assert_eq!(event_z.key_code, KeyCode::Z);
    
    // Test number keys
    let event_0 = KeyboardEvent::key_down(KeyCode::Num0);
    assert_eq!(event_0.key_code, KeyCode::Num0);
    
    // Test function keys
    let event_f1 = KeyboardEvent::key_down(KeyCode::F1);
    assert_eq!(event_f1.key_code, KeyCode::F1);
    
    // Test control keys
    let event_enter = KeyboardEvent::key_down(KeyCode::Enter);
    assert_eq!(event_enter.key_code, KeyCode::Enter);
    
    // Test arrow keys
    let event_up = KeyboardEvent::key_down(KeyCode::Up);
    assert_eq!(event_up.key_code, KeyCode::Up);
}

// ========== MouseEvent Tests ==========

#[test]
fn test_mouse_button_down() {
    let event = MouseEvent::button_down(MouseButton::Left, 100.0, 200.0);
    assert!(event.is_button_down());
    assert!(!event.is_button_up());
    assert_eq!(event.button, MouseButton::Left);
    assert_eq!(event.x, 100.0);
    assert_eq!(event.y, 200.0);
}

#[test]
fn test_mouse_button_up() {
    let event = MouseEvent::button_up(MouseButton::Right, 150.0, 250.0);
    assert!(!event.is_button_down());
    assert!(event.is_button_up());
    assert_eq!(event.button, MouseButton::Right);
    assert_eq!(event.x, 150.0);
    assert_eq!(event.y, 250.0);
}

#[test]
fn test_mouse_motion() {
    let event = MouseEvent::motion(50.0, 75.0, 10.0, -5.0);
    assert!(event.is_move());
    assert!(!event.is_button_down());
    assert_eq!(event.x, 50.0);
    assert_eq!(event.y, 75.0);
    assert_eq!(event.delta_x, 10.0);
    assert_eq!(event.delta_y, -5.0);
}

#[test]
fn test_mouse_wheel() {
    let event = MouseEvent::wheel(120.0);
    assert!(event.is_wheel());
    assert!(!event.is_move());
    assert_eq!(event.wheel_delta, 120.0);
}

#[test]
fn test_mouse_event_modifiers() {
    let event = MouseEvent::button_down(MouseButton::Left, 0.0, 0.0)
        .with_modifiers(true, false, true);
    
    assert!(event.shift);
    assert!(!event.ctrl);
    assert!(event.alt);
}

#[test]
fn test_mouse_all_buttons() {
    let left = MouseEvent::button_down(MouseButton::Left, 0.0, 0.0);
    assert_eq!(left.button, MouseButton::Left);
    
    let right = MouseEvent::button_down(MouseButton::Right, 0.0, 0.0);
    assert_eq!(right.button, MouseButton::Right);
    
    let middle = MouseEvent::button_down(MouseButton::Middle, 0.0, 0.0);
    assert_eq!(middle.button, MouseButton::Middle);
    
    let other = MouseEvent::button_down(MouseButton::Other, 0.0, 0.0);
    assert_eq!(other.button, MouseButton::Other);
}

// ========== TouchEvent Tests ==========

#[test]
fn test_touch_start() {
    let event = TouchEvent::start(1, 100.0, 200.0);
    assert!(event.is_start());
    assert!(!event.is_move());
    assert!(!event.is_end());
    assert_eq!(event.touch_id, 1);
    assert_eq!(event.x, 100.0);
    assert_eq!(event.y, 200.0);
}

#[test]
fn test_touch_move() {
    let event = TouchEvent::motion(1, 150.0, 250.0);
    assert!(!event.is_start());
    assert!(event.is_move());
    assert!(!event.is_end());
    assert_eq!(event.touch_id, 1);
    assert_eq!(event.x, 150.0);
    assert_eq!(event.y, 250.0);
}

#[test]
fn test_touch_end() {
    let event = TouchEvent::end(1, 200.0, 300.0);
    assert!(!event.is_start());
    assert!(!event.is_move());
    assert!(event.is_end());
    assert_eq!(event.touch_id, 1);
    assert_eq!(event.x, 200.0);
    assert_eq!(event.y, 300.0);
}

#[test]
fn test_touch_cancel() {
    let event = TouchEvent::cancel(1);
    assert!(!event.is_start());
    assert!(!event.is_move());
    assert!(!event.is_end());
    assert!(event.is_cancel());
    assert_eq!(event.touch_id, 1);
}

#[test]
fn test_touch_force() {
    let event = TouchEvent::start(1, 0.0, 0.0).with_force(0.8);
    assert_eq!(event.force, 0.8);
}

#[test]
fn test_touch_multiple_fingers() {
    let touch1 = TouchEvent::start(1, 100.0, 200.0);
    let touch2 = TouchEvent::start(2, 300.0, 400.0);
    
    assert_eq!(touch1.touch_id, 1);
    assert_eq!(touch2.touch_id, 2);
    assert_ne!(touch1.touch_id, touch2.touch_id);
}

// ========== Integration Tests ==========

#[test]
fn test_event_loop_with_events() {
    let mut event_loop = EventLoop::new();
    event_loop.start();
    
    // Simulate frame updates with events
    event_loop.update(0.0);
    let _keyboard = KeyboardEvent::key_down(KeyCode::W);
    
    event_loop.update(16.67);
    let _mouse = MouseEvent::motion(100.0, 100.0, 5.0, 5.0);
    
    event_loop.update(33.34);
    let _touch = TouchEvent::start(1, 50.0, 50.0);
    
    assert_eq!(event_loop.frame_count(), 3);
    assert!(event_loop.is_running());
}

#[test]
fn test_default_implementations() {
    let event_loop = EventLoop::default();
    assert_eq!(event_loop.state(), EventLoopState::Idle);
}