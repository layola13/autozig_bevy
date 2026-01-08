//! # AutoZig Input - Bevy Input System implemented in Zig
//!
//! 90% Zig实现，10% Rust包装
//! 
//! 提供以下核心功能：
//! - Input<T>: 通用输入状态追踪
//! - Keyboard: 键盘输入处理
//! - Mouse: 鼠标按键、移动和滚轮
//! - Touch: 触摸输入（WebGPU/WASM核心）
//! - Gamepad: 手柄输入（简化版）
//!
//! 专注于 WebGPU/WASM 平台

use autozig::include_zig;

// ========== Keyboard Input ==========

/// KeyCode enumeration - Maps to Web KeyboardEvent.code
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Letters A-Z
    KeyA = 0, KeyB = 1, KeyC = 2, KeyD = 3, KeyE = 4, KeyF = 5, KeyG = 6, KeyH = 7,
    KeyI = 8, KeyJ = 9, KeyK = 10, KeyL = 11, KeyM = 12, KeyN = 13, KeyO = 14, KeyP = 15,
    KeyQ = 16, KeyR = 17, KeyS = 18, KeyT = 19, KeyU = 20, KeyV = 21, KeyW = 22, KeyX = 23,
    KeyY = 24, KeyZ = 25,

    // Digits 0-9
    Digit0 = 26, Digit1 = 27, Digit2 = 28, Digit3 = 29, Digit4 = 30,
    Digit5 = 31, Digit6 = 32, Digit7 = 33, Digit8 = 34, Digit9 = 35,

    // Function keys
    F1 = 36, F2 = 37, F3 = 38, F4 = 39, F5 = 40, F6 = 41,
    F7 = 42, F8 = 43, F9 = 44, F10 = 45, F11 = 46, F12 = 47,

    // Control keys
    Escape = 48, Tab = 49, CapsLock = 50, ShiftLeft = 51, ShiftRight = 52,
    ControlLeft = 53, ControlRight = 54, AltLeft = 55, AltRight = 56,
    MetaLeft = 57, MetaRight = 58, Space = 59, Enter = 60, Backspace = 61,

    // Arrow keys
    ArrowLeft = 62, ArrowRight = 63, ArrowUp = 64, ArrowDown = 65,

    // Editing keys
    Insert = 66, Delete = 67, Home = 68, End = 69, PageUp = 70, PageDown = 71,

    // Symbol keys
    Minus = 72, Equal = 73, BracketLeft = 74, BracketRight = 75, Backslash = 76,
    Semicolon = 77, Quote = 78, Comma = 79, Period = 80, Slash = 81, Backquote = 82,

    // Numpad keys
    Numpad0 = 83, Numpad1 = 84, Numpad2 = 85, Numpad3 = 86, Numpad4 = 87,
    Numpad5 = 88, Numpad6 = 89, Numpad7 = 90, Numpad8 = 91, Numpad9 = 92,
    NumpadAdd = 93, NumpadSubtract = 94, NumpadMultiply = 95, NumpadDivide = 96,
    NumpadDecimal = 97, NumpadEnter = 98, NumLock = 99,

    // Additional keys
    ScrollLock = 100, Pause = 101, PrintScreen = 102, ContextMenu = 103,

    // Unknown/Other
    Unknown = 255,
}

/// KeyboardInput - Fixed-size input state (256 keys max)
/// This struct is repr(C) compatible with the Zig struct
#[repr(C)]
pub struct KeyboardInput {
    pressed: [bool; 256],
    just_pressed: [bool; 256],
    just_released: [bool; 256],
}

include_zig!("src/zig/keyboard.zig", {
    fn keyboard_input_create() -> KeyboardInput;
    fn keyboard_input_press(keyboard: *mut KeyboardInput, key_code: KeyCode) -> bool;
    fn keyboard_input_release(keyboard: *mut KeyboardInput, key_code: KeyCode) -> bool;
    fn keyboard_input_pressed(keyboard: *const KeyboardInput, key_code: KeyCode) -> bool;
    fn keyboard_input_just_pressed(keyboard: *const KeyboardInput, key_code: KeyCode) -> bool;
    fn keyboard_input_just_released(keyboard: *const KeyboardInput, key_code: KeyCode) -> bool;
    fn keyboard_input_clear(keyboard: *mut KeyboardInput);
    fn keyboard_input_reset(keyboard: *mut KeyboardInput);
});

impl KeyboardInput {
    pub fn new() -> Self {
        keyboard_input_create()
    }

    pub fn press(&mut self, key: KeyCode) -> bool {
        keyboard_input_press(self, key)
    }

    pub fn release(&mut self, key: KeyCode) -> bool {
        keyboard_input_release(self, key)
    }

    pub fn pressed(&self, key: KeyCode) -> bool {
        keyboard_input_pressed(self, key)
    }

    pub fn just_pressed(&self, key: KeyCode) -> bool {
        keyboard_input_just_pressed(self, key)
    }

    pub fn just_released(&self, key: KeyCode) -> bool {
        keyboard_input_just_released(self, key)
    }

    pub fn clear(&mut self) {
        keyboard_input_clear(self);
    }

    pub fn reset(&mut self) {
        keyboard_input_reset(self);
    }
}

impl Default for KeyboardInput {
    fn default() -> Self {
        Self::new()
    }
}

// ========== Mouse Input ==========

/// MouseButton enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Other = 3,
}

/// MouseScrollUnit - Unit for mouse wheel scrolling
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseScrollUnit {
    Line = 0,
    Pixel = 1,
}

/// MouseMotion - Mouse movement delta
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MouseMotion {
    pub delta_x: f32,
    pub delta_y: f32,
}

/// MouseWheel - Mouse wheel scroll event
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MouseWheel {
    pub unit: MouseScrollUnit,
    pub delta_x: f32,
    pub delta_y: f32,
}

/// MouseButtonInput - Fixed-size input state (4 buttons max)
#[repr(C)]
pub struct MouseButtonInput {
    pressed: [bool; 4],
    just_pressed: [bool; 4],
    just_released: [bool; 4],
}

include_zig!("src/zig/mouse.zig", {
    fn mouse_button_input_create() -> MouseButtonInput;
    fn mouse_button_input_press(mouse: *mut MouseButtonInput, button: MouseButton) -> bool;
    fn mouse_button_input_release(mouse: *mut MouseButtonInput, button: MouseButton) -> bool;
    fn mouse_button_input_pressed(mouse: *const MouseButtonInput, button: MouseButton) -> bool;
    fn mouse_button_input_just_pressed(mouse: *const MouseButtonInput, button: MouseButton) -> bool;
    fn mouse_button_input_just_released(mouse: *const MouseButtonInput, button: MouseButton) -> bool;
    fn mouse_button_input_clear(mouse: *mut MouseButtonInput);
    fn mouse_button_input_reset(mouse: *mut MouseButtonInput);
    
    fn mouse_motion_create(delta_x: f32, delta_y: f32) -> MouseMotion;
    fn mouse_wheel_create(unit: MouseScrollUnit, delta_x: f32, delta_y: f32) -> MouseWheel;
});

impl MouseButtonInput {
    pub fn new() -> Self {
        mouse_button_input_create()
    }

    pub fn press(&mut self, button: MouseButton) -> bool {
        mouse_button_input_press(self, button)
    }

    pub fn release(&mut self, button: MouseButton) -> bool {
        mouse_button_input_release(self, button)
    }

    pub fn pressed(&self, button: MouseButton) -> bool {
        mouse_button_input_pressed(self, button)
    }

    pub fn just_pressed(&self, button: MouseButton) -> bool {
        mouse_button_input_just_pressed(self, button)
    }

    pub fn just_released(&self, button: MouseButton) -> bool {
        mouse_button_input_just_released(self, button)
    }

    pub fn clear(&mut self) {
        mouse_button_input_clear(self);
    }

    pub fn reset(&mut self) {
        mouse_button_input_reset(self);
    }
}

impl Default for MouseButtonInput {
    fn default() -> Self {
        Self::new()
    }
}

impl MouseMotion {
    pub fn new(delta_x: f32, delta_y: f32) -> Self {
        mouse_motion_create(delta_x, delta_y)
    }
}

impl MouseWheel {
    pub fn new(unit: MouseScrollUnit, delta_x: f32, delta_y: f32) -> Self {
        mouse_wheel_create(unit, delta_x, delta_y)
    }
}

// ========== Touch Input ==========

/// TouchPhase - Touch event lifecycle phases
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchPhase {
    Started = 0,
    Moved = 1,
    Ended = 2,
    Cancelled = 3,
}

/// Touch - Represents a single touch point
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Touch {
    pub id: u64,
    pub phase: TouchPhase,
    pub position_x: f32,
    pub position_y: f32,
}

/// TouchInput - Fixed-size touch state (max 10 simultaneous touches)
#[repr(C)]
pub struct TouchInput {
    touches: [Touch; 10],
    active: [bool; 10],
    count: usize,
}

include_zig!("src/zig/touch.zig", {
    fn touch_input_create() -> TouchInput;
    fn touch_create(id: u64, phase: TouchPhase, position_x: f32, position_y: f32) -> Touch;
    fn touch_input_update(touch_input: *mut TouchInput, touch: Touch) -> bool;
    fn touch_input_remove(touch_input: *mut TouchInput, touch_id: u64);
    fn touch_input_get(touch_input: *const TouchInput, touch_id: u64, out_touch: *mut Touch) -> bool;
    fn touch_input_count(touch_input: *const TouchInput) -> usize;
    fn touch_input_clear(touch_input: *mut TouchInput);
});

impl TouchInput {
    pub fn new() -> Self {
        touch_input_create()
    }

    pub fn update(&mut self, touch: Touch) -> bool {
        touch_input_update(self, touch)
    }

    pub fn remove(&mut self, touch_id: u64) {
        touch_input_remove(self, touch_id);
    }

    pub fn get(&self, touch_id: u64) -> Option<Touch> {
        let mut touch = Touch {
            id: 0,
            phase: TouchPhase::Started,
            position_x: 0.0,
            position_y: 0.0,
        };
        if touch_input_get(self, touch_id, &mut touch) {
            Some(touch)
        } else {
            None
        }
    }

    pub fn count(&self) -> usize {
        touch_input_count(self)
    }

    pub fn clear(&mut self) {
        touch_input_clear(self);
    }
}

impl Default for TouchInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Touch {
    pub fn new(id: u64, phase: TouchPhase, position_x: f32, position_y: f32) -> Self {
        touch_create(id, phase, position_x, position_y)
    }
}

// ========== Gamepad Input ==========

/// GamepadButton enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadButton {
    South = 0, East = 1, North = 2, West = 3,
    DPadUp = 4, DPadDown = 5, DPadLeft = 6, DPadRight = 7,
    LeftShoulder = 8, RightShoulder = 9, LeftTrigger = 10, RightTrigger = 11,
    LeftThumb = 12, RightThumb = 13,
    Select = 14, Start = 15, Mode = 16,
}

/// GamepadAxis enumeration
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadAxis {
    LeftStickX = 0,
    LeftStickY = 1,
    RightStickX = 2,
    RightStickY = 3,
    LeftTrigger = 4,
    RightTrigger = 5,
}

/// GamepadButtonInput - Fixed-size input state (17 buttons max)
#[repr(C)]
pub struct GamepadButtonInput {
    pressed: [bool; 17],
    just_pressed: [bool; 17],
    just_released: [bool; 17],
}

/// GamepadAxisState - Fixed-size axis state (6 axes)
#[repr(C)]
pub struct GamepadAxisState {
    values: [f32; 6],
}

include_zig!("src/zig/gamepad.zig", {
    fn gamepad_button_input_create() -> GamepadButtonInput;
    fn gamepad_button_input_press(gamepad: *mut GamepadButtonInput, button: GamepadButton) -> bool;
    fn gamepad_button_input_release(gamepad: *mut GamepadButtonInput, button: GamepadButton) -> bool;
    fn gamepad_button_input_pressed(gamepad: *const GamepadButtonInput, button: GamepadButton) -> bool;
    fn gamepad_button_input_just_pressed(gamepad: *const GamepadButtonInput, button: GamepadButton) -> bool;
    fn gamepad_button_input_just_released(gamepad: *const GamepadButtonInput, button: GamepadButton) -> bool;
    fn gamepad_button_input_clear(gamepad: *mut GamepadButtonInput);
    fn gamepad_button_input_reset(gamepad: *mut GamepadButtonInput);
    
    fn gamepad_axis_state_create() -> GamepadAxisState;
    fn gamepad_axis_state_set(axis_state: *mut GamepadAxisState, axis: GamepadAxis, value: f32) -> bool;
    fn gamepad_axis_state_get(axis_state: *const GamepadAxisState, axis: GamepadAxis) -> f32;
    fn gamepad_axis_state_reset(axis_state: *mut GamepadAxisState);
});

impl GamepadButtonInput {
    pub fn new() -> Self {
        gamepad_button_input_create()
    }

    pub fn press(&mut self, button: GamepadButton) -> bool {
        gamepad_button_input_press(self, button)
    }

    pub fn release(&mut self, button: GamepadButton) -> bool {
        gamepad_button_input_release(self, button)
    }

    pub fn pressed(&self, button: GamepadButton) -> bool {
        gamepad_button_input_pressed(self, button)
    }

    pub fn just_pressed(&self, button: GamepadButton) -> bool {
        gamepad_button_input_just_pressed(self, button)
    }

    pub fn just_released(&self, button: GamepadButton) -> bool {
        gamepad_button_input_just_released(self, button)
    }

    pub fn clear(&mut self) {
        gamepad_button_input_clear(self);
    }

    pub fn reset(&mut self) {
        gamepad_button_input_reset(self);
    }
}

impl Default for GamepadButtonInput {
    fn default() -> Self {
        Self::new()
    }
}

impl GamepadAxisState {
    pub fn new() -> Self {
        gamepad_axis_state_create()
    }

    pub fn set(&mut self, axis: GamepadAxis, value: f32) -> bool {
        gamepad_axis_state_set(self, axis, value)
    }

    pub fn get(&self, axis: GamepadAxis) -> f32 {
        gamepad_axis_state_get(self, axis)
    }

    pub fn reset(&mut self) {
        gamepad_axis_state_reset(self);
    }
}

impl Default for GamepadAxisState {
    fn default() -> Self {
        Self::new()
    }
}