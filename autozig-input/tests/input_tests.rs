use autozig_input::*;

// ========== Input State Tests ==========

#[test]
fn test_input_press_release() {
    let mut keyboard = KeyboardInput::new();
    
    // Initially not pressed
    assert!(!keyboard.pressed(KeyCode::KeyA));
    
    // Press key
    assert!(keyboard.press(KeyCode::KeyA));
    assert!(keyboard.pressed(KeyCode::KeyA));
    
    // Release key
    assert!(keyboard.release(KeyCode::KeyA));
    assert!(!keyboard.pressed(KeyCode::KeyA));
}

#[test]
fn test_input_just_pressed() {
    let mut keyboard = KeyboardInput::new();
    
    // Press key
    assert!(keyboard.press(KeyCode::Space));
    assert!(keyboard.just_pressed(KeyCode::Space));
    
    // After clear, just_pressed should be false
    keyboard.clear();
    assert!(!keyboard.just_pressed(KeyCode::Space));
    assert!(keyboard.pressed(KeyCode::Space)); // Still pressed
}

#[test]
fn test_input_just_released() {
    let mut keyboard = KeyboardInput::new();
    
    // Press and then release
    assert!(keyboard.press(KeyCode::Enter));
    keyboard.clear();
    assert!(keyboard.release(KeyCode::Enter));
    
    assert!(keyboard.just_released(KeyCode::Enter));
    assert!(!keyboard.pressed(KeyCode::Enter));
    
    // After clear, just_released should be false
    keyboard.clear();
    assert!(!keyboard.just_released(KeyCode::Enter));
}

#[test]
fn test_input_clear() {
    let mut keyboard = KeyboardInput::new();
    
    // Press multiple keys
    assert!(keyboard.press(KeyCode::KeyW));
    assert!(keyboard.press(KeyCode::KeyA));
    assert!(keyboard.press(KeyCode::KeyS));
    assert!(keyboard.press(KeyCode::KeyD));
    
    // All should be just_pressed
    assert!(keyboard.just_pressed(KeyCode::KeyW));
    assert!(keyboard.just_pressed(KeyCode::KeyA));
    assert!(keyboard.just_pressed(KeyCode::KeyS));
    assert!(keyboard.just_pressed(KeyCode::KeyD));
    
    // Clear frame states
    keyboard.clear();
    
    // Just_pressed should be cleared, but pressed remains
    assert!(!keyboard.just_pressed(KeyCode::KeyW));
    assert!(!keyboard.just_pressed(KeyCode::KeyA));
    assert!(keyboard.pressed(KeyCode::KeyW));
    assert!(keyboard.pressed(KeyCode::KeyA));
}

// ========== Keyboard Tests ==========

#[test]
fn test_keyboard_input() {
    let mut keyboard = KeyboardInput::new();
    
    // Test various keys
    assert!(keyboard.press(KeyCode::KeyA));
    assert!(keyboard.press(KeyCode::Digit1));
    assert!(keyboard.press(KeyCode::F1));
    assert!(keyboard.press(KeyCode::Escape));
    
    assert!(keyboard.pressed(KeyCode::KeyA));
    assert!(keyboard.pressed(KeyCode::Digit1));
    assert!(keyboard.pressed(KeyCode::F1));
    assert!(keyboard.pressed(KeyCode::Escape));
    
    assert!(!keyboard.pressed(KeyCode::KeyB));
}

// ========== Mouse Tests ==========

#[test]
fn test_mouse_button() {
    let mut mouse = MouseButtonInput::new();
    
    // Test left button
    assert!(mouse.press(MouseButton::Left));
    assert!(mouse.pressed(MouseButton::Left));
    assert!(mouse.just_pressed(MouseButton::Left));
    
    mouse.clear();
    assert!(mouse.pressed(MouseButton::Left));
    assert!(!mouse.just_pressed(MouseButton::Left));
    
    assert!(mouse.release(MouseButton::Left));
    assert!(!mouse.pressed(MouseButton::Left));
    assert!(mouse.just_released(MouseButton::Left));
}

#[test]
fn test_mouse_motion() {
    let motion = MouseMotion::new(10.5, -20.3);
    
    assert_eq!(motion.delta_x, 10.5);
    assert_eq!(motion.delta_y, -20.3);
}

#[test]
fn test_mouse_wheel() {
    // Line scrolling
    let wheel_line = MouseWheel::new(MouseScrollUnit::Line, 0.0, 3.0);
    assert_eq!(wheel_line.unit, MouseScrollUnit::Line);
    assert_eq!(wheel_line.delta_x, 0.0);
    assert_eq!(wheel_line.delta_y, 3.0);
    
    // Pixel scrolling
    let wheel_pixel = MouseWheel::new(MouseScrollUnit::Pixel, 100.0, 50.0);
    assert_eq!(wheel_pixel.unit, MouseScrollUnit::Pixel);
    assert_eq!(wheel_pixel.delta_x, 100.0);
    assert_eq!(wheel_pixel.delta_y, 50.0);
}

// ========== Touch Tests ==========

#[test]
fn test_touch_input() {
    let mut touch_input = TouchInput::new();
    
    // Initially no touches
    assert_eq!(touch_input.count(), 0);
    
    // Add a touch
    let touch = Touch::new(1, TouchPhase::Started, 100.0, 200.0);
    assert!(touch_input.update(touch));
    assert_eq!(touch_input.count(), 1);
    
    // Get the touch back
    let retrieved = touch_input.get(1);
    assert!(retrieved.is_some());
    let retrieved = retrieved.unwrap();
    assert_eq!(retrieved.id, 1);
    assert_eq!(retrieved.phase, TouchPhase::Started);
    assert_eq!(retrieved.position_x, 100.0);
    assert_eq!(retrieved.position_y, 200.0);
}

#[test]
fn test_touch_phases() {
    let mut touch_input = TouchInput::new();
    
    // Started
    let touch_started = Touch::new(42, TouchPhase::Started, 50.0, 75.0);
    assert!(touch_input.update(touch_started));
    
    let retrieved = touch_input.get(42).unwrap();
    assert_eq!(retrieved.phase, TouchPhase::Started);
    
    // Moved
    let touch_moved = Touch::new(42, TouchPhase::Moved, 60.0, 85.0);
    assert!(touch_input.update(touch_moved));
    
    let retrieved = touch_input.get(42).unwrap();
    assert_eq!(retrieved.phase, TouchPhase::Moved);
    assert_eq!(retrieved.position_x, 60.0);
    assert_eq!(retrieved.position_y, 85.0);
    
    // Ended
    let touch_ended = Touch::new(42, TouchPhase::Ended, 70.0, 95.0);
    assert!(touch_input.update(touch_ended));
    
    let retrieved = touch_input.get(42).unwrap();
    assert_eq!(retrieved.phase, TouchPhase::Ended);
    
    // Remove touch
    touch_input.remove(42);
    assert_eq!(touch_input.count(), 0);
    assert!(touch_input.get(42).is_none());
}

// ========== Gamepad Tests ==========

#[test]
fn test_gamepad_button() {
    let mut gamepad = GamepadButtonInput::new();
    
    // Test South button (A/Cross)
    assert!(gamepad.press(GamepadButton::South));
    assert!(gamepad.pressed(GamepadButton::South));
    assert!(gamepad.just_pressed(GamepadButton::South));
    
    gamepad.clear();
    assert!(gamepad.pressed(GamepadButton::South));
    assert!(!gamepad.just_pressed(GamepadButton::South));
    
    assert!(gamepad.release(GamepadButton::South));
    assert!(!gamepad.pressed(GamepadButton::South));
    assert!(gamepad.just_released(GamepadButton::South));
}

#[test]
fn test_gamepad_axis() {
    let mut axis_state = GamepadAxisState::new();
    
    // Initially zero
    assert_eq!(axis_state.get(GamepadAxis::LeftStickX), 0.0);
    
    // Set values
    assert!(axis_state.set(GamepadAxis::LeftStickX, 0.75));
    assert_eq!(axis_state.get(GamepadAxis::LeftStickX), 0.75);
    
    assert!(axis_state.set(GamepadAxis::RightStickY, -0.5));
    assert_eq!(axis_state.get(GamepadAxis::RightStickY), -0.5);
    
    // Reset
    axis_state.reset();
    assert_eq!(axis_state.get(GamepadAxis::LeftStickX), 0.0);
    assert_eq!(axis_state.get(GamepadAxis::RightStickY), 0.0);
}

// ========== Multiple Inputs Tests ==========

#[test]
fn test_multiple_inputs() {
    let mut keyboard = KeyboardInput::new();
    let mut mouse = MouseButtonInput::new();
    let mut touch_input = TouchInput::new();
    
    // Press multiple keys
    assert!(keyboard.press(KeyCode::KeyW));
    assert!(keyboard.press(KeyCode::KeyA));
    assert!(keyboard.press(KeyCode::Space));
    
    // Press multiple mouse buttons
    assert!(mouse.press(MouseButton::Left));
    assert!(mouse.press(MouseButton::Right));
    
    // Add multiple touches
    let touch1 = Touch::new(1, TouchPhase::Started, 10.0, 20.0);
    let touch2 = Touch::new(2, TouchPhase::Started, 30.0, 40.0);
    assert!(touch_input.update(touch1));
    assert!(touch_input.update(touch2));
    
    // Verify all inputs
    assert!(keyboard.pressed(KeyCode::KeyW));
    assert!(keyboard.pressed(KeyCode::KeyA));
    assert!(keyboard.pressed(KeyCode::Space));
    
    assert!(mouse.pressed(MouseButton::Left));
    assert!(mouse.pressed(MouseButton::Right));
    
    assert_eq!(touch_input.count(), 2);
    assert!(touch_input.get(1).is_some());
    assert!(touch_input.get(2).is_some());
}

#[test]
fn test_input_frame_clear() {
    let mut keyboard = KeyboardInput::new();
    
    // Simulate frame 1: Press key
    assert!(keyboard.press(KeyCode::KeyA));
    assert!(keyboard.just_pressed(KeyCode::KeyA));
    assert!(keyboard.pressed(KeyCode::KeyA));
    
    // End of frame 1: Clear
    keyboard.clear();
    
    // Simulate frame 2: Key still held
    assert!(!keyboard.just_pressed(KeyCode::KeyA)); // Not just pressed anymore
    assert!(keyboard.pressed(KeyCode::KeyA)); // Still pressed
    
    // Simulate frame 3: Release key
    assert!(keyboard.release(KeyCode::KeyA));
    assert!(keyboard.just_released(KeyCode::KeyA));
    assert!(!keyboard.pressed(KeyCode::KeyA));
    
    // End of frame 3: Clear
    keyboard.clear();
    
    // Simulate frame 4: Key released
    assert!(!keyboard.just_released(KeyCode::KeyA)); // Not just released anymore
    assert!(!keyboard.pressed(KeyCode::KeyA)); // Not pressed
}

// ========== Additional Integration Tests ==========

#[test]
fn test_keyboard_reset() {
    let mut keyboard = KeyboardInput::new();
    
    // Press multiple keys
    assert!(keyboard.press(KeyCode::KeyA));
    assert!(keyboard.press(KeyCode::KeyB));
    assert!(keyboard.press(KeyCode::KeyC));
    
    // Reset
    keyboard.reset();
    
    // All should be cleared
    assert!(!keyboard.pressed(KeyCode::KeyA));
    assert!(!keyboard.pressed(KeyCode::KeyB));
    assert!(!keyboard.pressed(KeyCode::KeyC));
}

#[test]
fn test_mouse_multiple_buttons() {
    let mut mouse = MouseButtonInput::new();
    
    // Press all buttons
    assert!(mouse.press(MouseButton::Left));
    assert!(mouse.press(MouseButton::Right));
    assert!(mouse.press(MouseButton::Middle));
    
    assert!(mouse.pressed(MouseButton::Left));
    assert!(mouse.pressed(MouseButton::Right));
    assert!(mouse.pressed(MouseButton::Middle));
    
    // Release one
    assert!(mouse.release(MouseButton::Right));
    assert!(mouse.pressed(MouseButton::Left));
    assert!(!mouse.pressed(MouseButton::Right));
    assert!(mouse.pressed(MouseButton::Middle));
}

#[test]
fn test_touch_multiple_touches() {
    let mut touch_input = TouchInput::new();
    
    // Add 5 touches
    for i in 1..=5 {
        let touch = Touch::new(i, TouchPhase::Started, i as f32 * 10.0, i as f32 * 20.0);
        assert!(touch_input.update(touch));
    }
    
    assert_eq!(touch_input.count(), 5);
    
    // Remove touch 3
    touch_input.remove(3);
    assert_eq!(touch_input.count(), 4);
    assert!(touch_input.get(3).is_none());
    
    // Clear all
    touch_input.clear();
    assert_eq!(touch_input.count(), 0);
}

#[test]
fn test_gamepad_multiple_buttons_and_axes() {
    let mut gamepad = GamepadButtonInput::new();
    let mut axes = GamepadAxisState::new();
    
    // Press multiple buttons
    assert!(gamepad.press(GamepadButton::South));
    assert!(gamepad.press(GamepadButton::West));
    assert!(gamepad.press(GamepadButton::LeftShoulder));
    
    // Set multiple axes
    assert!(axes.set(GamepadAxis::LeftStickX, 1.0));
    assert!(axes.set(GamepadAxis::LeftStickY, -1.0));
    assert!(axes.set(GamepadAxis::RightTrigger, 0.5));
    
    // Verify
    assert!(gamepad.pressed(GamepadButton::South));
    assert!(gamepad.pressed(GamepadButton::West));
    assert!(gamepad.pressed(GamepadButton::LeftShoulder));
    
    assert_eq!(axes.get(GamepadAxis::LeftStickX), 1.0);
    assert_eq!(axes.get(GamepadAxis::LeftStickY), -1.0);
    assert_eq!(axes.get(GamepadAxis::RightTrigger), 0.5);
}