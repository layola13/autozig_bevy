//! # AutoZig Winit - Event Loop and WASM Entry Point
//!
//! 90% Zig实现，10% Rust包装
//! 
//! 提供以下核心功能：
//! - EventLoop: 事件循环管理
//! - KeyboardEvent: 键盘事件
//! - MouseEvent: 鼠标事件
//! - TouchEvent: 触摸事件
//! - WASM平台支持
//!
//! 专注于 WebGPU/WASM 平台

use autozig::include_zig;

// Re-export window types from autozig-window
pub use autozig_window::{Window, WindowDescriptor, WindowEvent, WindowEventType};

// ========== EventLoop ==========

/// EventLoopState - Tracks the state of the event loop
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventLoopState {
    Idle = 0,
    Running = 1,
    Exiting = 2,
}

/// EventLoop - Main event loop structure for WASM platform
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EventLoop {
    pub state: EventLoopState,
    pub frame_count: u64,
    pub last_frame_time: f64,
    pub delta_time: f32,
    pub is_wasm: bool,
}

include_zig!("src/zig/event_loop.zig", {
    fn event_loop_init() -> EventLoop;
    fn event_loop_start(loop_handle: *mut EventLoop);
    fn event_loop_stop(loop_handle: *mut EventLoop);
    fn event_loop_update(loop_handle: *mut EventLoop, current_time: f64);
    fn event_loop_is_running(loop_handle: *const EventLoop) -> bool;
    fn event_loop_is_exiting(loop_handle: *const EventLoop) -> bool;
    fn event_loop_get_delta_time(loop_handle: *const EventLoop) -> f32;
    fn event_loop_get_frame_count(loop_handle: *const EventLoop) -> u64;
    fn event_loop_get_state(loop_handle: *const EventLoop) -> EventLoopState;
});

impl EventLoop {
    pub fn new() -> Self {
        event_loop_init()
    }

    pub fn start(&mut self) {
        event_loop_start(self);
    }

    pub fn stop(&mut self) {
        event_loop_stop(self);
    }

    pub fn update(&mut self, current_time: f64) {
        event_loop_update(self, current_time);
    }

    pub fn is_running(&self) -> bool {
        event_loop_is_running(self)
    }

    pub fn is_exiting(&self) -> bool {
        event_loop_is_exiting(self)
    }

    pub fn delta_time(&self) -> f32 {
        event_loop_get_delta_time(self)
    }

    pub fn frame_count(&self) -> u64 {
        event_loop_get_frame_count(self)
    }

    pub fn state(&self) -> EventLoopState {
        event_loop_get_state(self)
    }
}

impl Default for EventLoop {
    fn default() -> Self {
        Self::new()
    }
}

// ========== KeyCode and Keyboard Events ==========

/// KeyCode - Keyboard key codes
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Unknown = 0,
    // Letters
    A = 1, B = 2, C = 3, D = 4, E = 5, F = 6, G = 7, H = 8,
    I = 9, J = 10, K = 11, L = 12, M = 13, N = 14, O = 15, P = 16,
    Q = 17, R = 18, S = 19, T = 20, U = 21, V = 22, W = 23, X = 24,
    Y = 25, Z = 26,
    // Numbers
    Num0 = 30, Num1 = 31, Num2 = 32, Num3 = 33, Num4 = 34,
    Num5 = 35, Num6 = 36, Num7 = 37, Num8 = 38, Num9 = 39,
    // Function keys
    F1 = 40, F2 = 41, F3 = 42, F4 = 43, F5 = 44, F6 = 45,
    F7 = 46, F8 = 47, F9 = 48, F10 = 49, F11 = 50, F12 = 51,
    // Control keys
    Escape = 60,
    Space = 61,
    Enter = 62,
    Tab = 63,
    Backspace = 64,
    Delete = 65,
    Insert = 66,
    // Arrow keys
    Left = 70,
    Right = 71,
    Up = 72,
    Down = 73,
    // Modifier keys
    LShift = 80,
    RShift = 81,
    LControl = 82,
    RControl = 83,
    LAlt = 84,
    RAlt = 85,
}

/// KeyboardEventType - Types of keyboard events
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardEventType {
    KeyDown = 0,
    KeyUp = 1,
}

/// KeyboardEvent - Keyboard input event
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct KeyboardEvent {
    pub event_type: KeyboardEventType,
    pub key_code: KeyCode,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
    pub repeat: bool,
}

include_zig!("src/zig/input_events.zig", {
    fn keyboard_event_init(event_type: KeyboardEventType, key_code: KeyCode) -> KeyboardEvent;
    fn keyboard_event_is_key_down(event: *const KeyboardEvent) -> bool;
    fn keyboard_event_is_key_up(event: *const KeyboardEvent) -> bool;
    fn mouse_event_init(event_type: MouseEventType) -> MouseEvent;
    fn mouse_event_set_position(event: *mut MouseEvent, x: f32, y: f32);
    fn mouse_event_set_delta(event: *mut MouseEvent, delta_x: f32, delta_y: f32);
    fn mouse_event_set_button(event: *mut MouseEvent, button: MouseButton);
    fn mouse_event_set_wheel_delta(event: *mut MouseEvent, delta: f32);
    fn mouse_event_is_button_down(event: *const MouseEvent) -> bool;
    fn mouse_event_is_button_up(event: *const MouseEvent) -> bool;
    fn mouse_event_is_move(event: *const MouseEvent) -> bool;
    fn mouse_event_is_wheel(event: *const MouseEvent) -> bool;
    fn touch_event_init(event_type: TouchEventType, touch_id: u32) -> TouchEvent;
    fn touch_event_set_position(event: *mut TouchEvent, x: f32, y: f32);
    fn touch_event_set_force(event: *mut TouchEvent, force: f32);
    fn touch_event_is_start(event: *const TouchEvent) -> bool;
    fn touch_event_is_move(event: *const TouchEvent) -> bool;
    fn touch_event_is_end(event: *const TouchEvent) -> bool;
    fn touch_event_is_cancel(event: *const TouchEvent) -> bool;
});

impl KeyboardEvent {
    pub fn new(event_type: KeyboardEventType, key_code: KeyCode) -> Self {
        keyboard_event_init(event_type, key_code)
    }

    pub fn key_down(key_code: KeyCode) -> Self {
        Self::new(KeyboardEventType::KeyDown, key_code)
    }

    pub fn key_up(key_code: KeyCode) -> Self {
        Self::new(KeyboardEventType::KeyUp, key_code)
    }

    pub fn is_key_down(&self) -> bool {
        keyboard_event_is_key_down(self)
    }

    pub fn is_key_up(&self) -> bool {
        keyboard_event_is_key_up(self)
    }

    pub fn with_modifiers(mut self, shift: bool, ctrl: bool, alt: bool, meta: bool) -> Self {
        self.shift = shift;
        self.ctrl = ctrl;
        self.alt = alt;
        self.meta = meta;
        self
    }
}

// ========== Mouse Events ==========

/// MouseButton - Mouse button types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Other = 3,
}

/// MouseEventType - Types of mouse events
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    ButtonDown = 0,
    ButtonUp = 1,
    Move = 2,
    Wheel = 3,
}

/// MouseEvent - Mouse input event
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    pub event_type: MouseEventType,
    pub button: MouseButton,
    pub x: f32,
    pub y: f32,
    pub delta_x: f32,
    pub delta_y: f32,
    pub wheel_delta: f32,
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}


impl MouseEvent {
    pub fn new(event_type: MouseEventType) -> Self {
        mouse_event_init(event_type)
    }

    pub fn button_down(button: MouseButton, x: f32, y: f32) -> Self {
        let mut event = Self::new(MouseEventType::ButtonDown);
        event.set_button(button);
        event.set_position(x, y);
        event
    }

    pub fn button_up(button: MouseButton, x: f32, y: f32) -> Self {
        let mut event = Self::new(MouseEventType::ButtonUp);
        event.set_button(button);
        event.set_position(x, y);
        event
    }

    pub fn motion(x: f32, y: f32, delta_x: f32, delta_y: f32) -> Self {
        let mut event = Self::new(MouseEventType::Move);
        event.set_position(x, y);
        event.set_delta(delta_x, delta_y);
        event
    }

    pub fn wheel(delta: f32) -> Self {
        let mut event = Self::new(MouseEventType::Wheel);
        event.set_wheel_delta(delta);
        event
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        mouse_event_set_position(self, x, y);
    }

    pub fn set_delta(&mut self, delta_x: f32, delta_y: f32) {
        mouse_event_set_delta(self, delta_x, delta_y);
    }

    pub fn set_button(&mut self, button: MouseButton) {
        mouse_event_set_button(self, button);
    }

    pub fn set_wheel_delta(&mut self, delta: f32) {
        mouse_event_set_wheel_delta(self, delta);
    }

    pub fn is_button_down(&self) -> bool {
        mouse_event_is_button_down(self)
    }

    pub fn is_button_up(&self) -> bool {
        mouse_event_is_button_up(self)
    }

    pub fn is_move(&self) -> bool {
        mouse_event_is_move(self)
    }

    pub fn is_wheel(&self) -> bool {
        mouse_event_is_wheel(self)
    }

    pub fn with_modifiers(mut self, shift: bool, ctrl: bool, alt: bool) -> Self {
        self.shift = shift;
        self.ctrl = ctrl;
        self.alt = alt;
        self
    }
}

// ========== Touch Events ==========

/// TouchEventType - Types of touch events
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchEventType {
    Start = 0,
    Move = 1,
    End = 2,
    Cancel = 3,
}

/// TouchEvent - Touch input event
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TouchEvent {
    pub event_type: TouchEventType,
    pub touch_id: u32,
    pub x: f32,
    pub y: f32,
    pub force: f32,
}


impl TouchEvent {
    pub fn new(event_type: TouchEventType, touch_id: u32) -> Self {
        touch_event_init(event_type, touch_id)
    }

    pub fn start(touch_id: u32, x: f32, y: f32) -> Self {
        let mut event = Self::new(TouchEventType::Start, touch_id);
        event.set_position(x, y);
        event
    }

    pub fn motion(touch_id: u32, x: f32, y: f32) -> Self {
        let mut event = Self::new(TouchEventType::Move, touch_id);
        event.set_position(x, y);
        event
    }

    pub fn end(touch_id: u32, x: f32, y: f32) -> Self {
        let mut event = Self::new(TouchEventType::End, touch_id);
        event.set_position(x, y);
        event
    }

    pub fn cancel(touch_id: u32) -> Self {
        Self::new(TouchEventType::Cancel, touch_id)
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        touch_event_set_position(self, x, y);
    }

    pub fn set_force(&mut self, force: f32) {
        touch_event_set_force(self, force);
    }

    pub fn is_start(&self) -> bool {
        touch_event_is_start(self)
    }

    pub fn is_move(&self) -> bool {
        touch_event_is_move(self)
    }

    pub fn is_end(&self) -> bool {
        touch_event_is_end(self)
    }

    pub fn is_cancel(&self) -> bool {
        touch_event_is_cancel(self)
    }

    pub fn with_force(mut self, force: f32) -> Self {
        self.force = force;
        self
    }
}

// ========== Winit Plugin & Runner ==========

use autozig_app::{App, Plugin, ZigApp};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

/// WinitPlugin - Configures the application to use the Winit event loop
#[derive(Default)]
pub struct WinitPlugin;

impl Plugin for WinitPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(winit_runner);
    }
    
    fn name(&self) -> &str {
        "WinitPlugin"
    }
}

/// The runner function that drives the event loop
pub extern "C" fn winit_runner(app_ptr: *mut ZigApp) -> u8 {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title("AutoZig 3D Demo")
        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720))
        .build(&event_loop)
        .unwrap();

    let raw_handle = autozig_window::WindowRawHandle {
        window_handle: window.raw_window_handle().unwrap(),
        display_handle: window.raw_display_handle().unwrap(),
    };
    
    unsafe {
        App::insert_resource_raw(app_ptr, raw_handle);
    }

    event_loop.run(move |event, target| {
        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                ..
            } => {
                target.exit();
            }
            winit::event::Event::AboutToWait => {
                // Main update loop
                unsafe {
                    autozig_app::App::update_raw(app_ptr);
                }
                
                if let Some(_exit) = unsafe { autozig_app::App::should_exit_raw(app_ptr) } {
                    target.exit();
                }
                
                window.request_redraw();
            }
            _ => {}
        }
    }).unwrap();
    
    0
}