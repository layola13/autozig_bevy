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

// ========== Additional Input Types for Bevy API Completeness ==========

/// Gamepad identifier
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gamepad {
    pub id: u32,
}

impl Gamepad {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

/// Key enum for logical key values (character-based)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Character(char),
    Unidentified,
}

/// Accumulated mouse motion for frame
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AccumulatedMouseMotion {
    pub delta: [f32; 2],
}

impl AccumulatedMouseMotion {
    pub fn new() -> Self {
        Self { delta: [0.0, 0.0] }
    }
    
    pub fn accumulate(&mut self, delta_x: f32, delta_y: f32) {
        self.delta[0] += delta_x;
        self.delta[1] += delta_y;
    }
    
    pub fn reset(&mut self) {
        self.delta = [0.0, 0.0];
    }
}

/// Accumulated mouse scroll for frame
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AccumulatedMouseScroll {
    pub delta: [f32; 2],
}

impl AccumulatedMouseScroll {
    pub fn new() -> Self {
        Self { delta: [0.0, 0.0] }
    }
    
    pub fn accumulate(&mut self, delta_x: f32, delta_y: f32) {
        self.delta[0] += delta_x;
        self.delta[1] += delta_y;
    }
    
    pub fn reset(&mut self) {
        self.delta = [0.0, 0.0];
    }
}

/// Generic input state tracker
#[derive(Debug, Clone)]
pub struct ButtonInput<T: Copy + Eq + core::hash::Hash> {
    pressed: std::collections::HashSet<T>,
    just_pressed: std::collections::HashSet<T>,
    just_released: std::collections::HashSet<T>,
}

impl<T: Copy + Eq + core::hash::Hash> ButtonInput<T> {
    pub fn new() -> Self {
        Self {
            pressed: std::collections::HashSet::new(),
            just_pressed: std::collections::HashSet::new(),
            just_released: std::collections::HashSet::new(),
        }
    }
    
    pub fn press(&mut self, input: T) {
        if !self.pressed.contains(&input) {
            self.just_pressed.insert(input);
        }
        self.pressed.insert(input);
    }
    
    pub fn release(&mut self, input: T) {
        if self.pressed.contains(&input) {
            self.just_released.insert(input);
        }
        self.pressed.remove(&input);
    }
    
    pub fn pressed(&self, input: T) -> bool {
        self.pressed.contains(&input)
    }
    
    pub fn just_pressed(&self, input: T) -> bool {
        self.just_pressed.contains(&input)
    }
    
    pub fn just_released(&self, input: T) -> bool {
        self.just_released.contains(&input)
    }
    
    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
    
    pub fn reset(&mut self) {
        self.pressed.clear();
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

impl<T: Copy + Eq + core::hash::Hash> Default for ButtonInput<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic axis for input values (-1.0 to 1.0)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Axis<T> {
    pub value: f32,
    _marker: core::marker::PhantomData<T>,
}

impl<T> Axis<T> {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(-1.0, 1.0),
            _marker: core::marker::PhantomData,
        }
    }
    
    pub fn set(&mut self, value: f32) {
        self.value = value.clamp(-1.0, 1.0);
    }
    
    pub fn get(&self) -> f32 {
        self.value
    }
}

/// Axis settings for dead zones and sensitivity
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AxisSettings {
    pub livezone_lowerbound: f32,
    pub deadzone_lowerbound: f32,
    pub deadzone_upperbound: f32,
    pub livezone_upperbound: f32,
    pub threshold: f32,
}

impl Default for AxisSettings {
    fn default() -> Self {
        Self {
            livezone_lowerbound: -0.95,
            deadzone_lowerbound: -0.05,
            deadzone_upperbound: 0.05,
            livezone_upperbound: 0.95,
            threshold: 0.01,
        }
    }
}

/// Button axis settings (for buttons used as axes)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ButtonAxisSettings {
    pub high: f32,
    pub low: f32,
    pub threshold: f32,
}

impl Default for ButtonAxisSettings {
    fn default() -> Self {
        Self {
            high: 1.0,
            low: -1.0,
            threshold: 0.01,
        }
    }
}

/// Button settings for press threshold
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ButtonSettings {
    pub press_threshold: f32,
    pub release_threshold: f32,
}

impl Default for ButtonSettings {
    fn default() -> Self {
        Self {
            press_threshold: 0.75,
            release_threshold: 0.65,
        }
    }
}

/// Gamepad settings
#[derive(Debug, Clone)]
pub struct GamepadSettings {
    pub default_axis_settings: AxisSettings,
    pub default_button_settings: ButtonSettings,
    pub default_button_axis_settings: ButtonAxisSettings,
}

impl Default for GamepadSettings {
    fn default() -> Self {
        Self {
            default_axis_settings: AxisSettings::default(),
            default_button_settings: ButtonSettings::default(),
            default_button_axis_settings: ButtonAxisSettings::default(),
        }
    }
}

/// Touch collection manager
#[derive(Debug, Clone)]
pub struct Touches {
    touches: Vec<Touch>,
}

impl Touches {
    pub fn new() -> Self {
        Self {
            touches: Vec::new(),
        }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &Touch> {
        self.touches.iter()
    }
    
    pub fn get_pressed(&self, id: u64) -> Option<&Touch> {
        self.touches.iter().find(|t| t.id == id)
    }
    
    pub fn just_pressed(&self, id: u64) -> bool {
        self.touches.iter().any(|t| t.id == id && matches!(t.phase, TouchPhase::Started))
    }
    
    pub fn just_released(&self, id: u64) -> bool {
        self.touches.iter().any(|t| t.id == id && matches!(t.phase, TouchPhase::Ended))
    }
}

impl Default for Touches {
    fn default() -> Self {
        Self::new()
    }
}

/// Input plugin marker
#[derive(Debug, Clone, Copy, Default)]
pub struct InputPlugin;

/// Input systems set marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputSystems;

/// Keyboard focus lost event
#[derive(Debug, Clone, Copy)]
pub struct KeyboardFocusLost;

// ========== Gamepad Events ==========

/// Gamepad axis changed event
#[derive(Debug, Clone, Copy)]
pub struct GamepadAxisChangedEvent {
    pub gamepad: Gamepad,
    pub axis: GamepadAxis,
    pub value: f32,
}

/// Gamepad button changed event
#[derive(Debug, Clone, Copy)]
pub struct GamepadButtonChangedEvent {
    pub gamepad: Gamepad,
    pub button: GamepadButton,
    pub value: f32,
}

/// Gamepad button state changed event
#[derive(Debug, Clone, Copy)]
pub struct GamepadButtonStateChangedEvent {
    pub gamepad: Gamepad,
    pub button: GamepadButton,
    pub state: ButtonState,
}

/// Gamepad connection event
#[derive(Debug, Clone, Copy)]
pub struct GamepadConnectionEvent {
    pub gamepad: Gamepad,
    pub connection: GamepadConnection,
}

/// Raw gamepad axis changed event
#[derive(Debug, Clone, Copy)]
pub struct RawGamepadAxisChangedEvent {
    pub gamepad: Gamepad,
    pub axis: GamepadAxis,
    pub value: f32,
}

/// Raw gamepad button changed event
#[derive(Debug, Clone, Copy)]
pub struct RawGamepadButtonChangedEvent {
    pub gamepad: Gamepad,
    pub button: GamepadButton,
    pub value: f32,
}

/// Gamepad rumble intensity
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GamepadRumbleIntensity {
    pub strong_motor: f32,
    pub weak_motor: f32,
}

impl GamepadRumbleIntensity {
    pub fn new(strong: f32, weak: f32) -> Self {
        Self {
            strong_motor: strong.clamp(0.0, 1.0),
            weak_motor: weak.clamp(0.0, 1.0),
        }
    }
}

// ========== Gesture Support ==========

/// Double tap gesture
#[derive(Debug, Clone, Copy)]
pub struct DoubleTapGesture {
    pub position: [f32; 2],
    pub time_delta: f32,
}

/// Pan gesture
#[derive(Debug, Clone, Copy)]
pub struct PanGesture {
    pub delta: [f32; 2],
}

/// Pinch gesture
#[derive(Debug, Clone, Copy)]
pub struct PinchGesture {
    pub scale: f32,
}

/// Rotation gesture
#[derive(Debug, Clone, Copy)]
pub struct RotationGesture {
    pub angle: f32,
}

// ========== Enums ==========

/// Button state
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Pressed = 0,
    Released = 1,
}

/// Force touch state
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ForceTouch {
    Calibrated { force: f32, max_possible_force: f32, altitude_angle: Option<f32> },
    Normalized(f32),
}

/// Gamepad connection state
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadConnection {
    Connected = 0,
    Disconnected = 1,
}

/// Gamepad event enum
#[derive(Debug, Clone, Copy)]
pub enum GamepadEvent {
    Connection(GamepadConnectionEvent),
    Button(GamepadButtonChangedEvent),
    Axis(GamepadAxisChangedEvent),
}

/// Gamepad input enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GamepadInput {
    Button(GamepadButton),
    Axis(GamepadAxis),
}

/// Gamepad rumble request
#[derive(Debug, Clone, Copy)]
pub enum GamepadRumbleRequest {
    Start { intensity: GamepadRumbleIntensity, duration: f32 },
    Stop,
}

/// Native key code (platform-specific)
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeKeyCode {
    Unidentified = 0,
    Android(u32),
    MacOS(u16),
    Windows(u16),
    Xkb(u32),
}

/// Native key (platform-specific logical key)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeKey {
    Unidentified,
    Android(u32),
    MacOS(u16),
    Windows(u16),
    Xkb(u32),
}

/// Raw gamepad event enum
#[derive(Debug, Clone, Copy)]
pub enum RawGamepadEvent {
    Axis(RawGamepadAxisChangedEvent),
    Button(RawGamepadButtonChangedEvent),
}

// ========== Error Types ==========

/// Axis settings error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AxisSettingsError {
    LiveZoneLowerBoundOutOfRange(i32),
    DeadZoneLowerBoundOutOfRange(i32),
    DeadZoneUpperBoundOutOfRange(i32),
    LiveZoneUpperBoundOutOfRange(i32),
    LiveZoneLowerBoundGreaterThanDeadZoneLowerBound,
    DeadZoneLowerBoundGreaterThanDeadZoneUpperBound,
    DeadZoneUpperBoundGreaterThanLiveZoneUpperBound,
    ThresholdOutOfRange(i32),
}

/// Button settings error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSettingsError {
    PressThresholdOutOfRange(i32),
    ReleaseThresholdOutOfRange(i32),
    ReleaseThresholdGreaterThanPressThreshold,
}