//! # AutoZig Window - Bevy Window System implemented in Zig
//!
//! 90% Zig实现，10% Rust包装
//! 
//! 提供以下核心功能：
//! - Window: 窗口管理和配置
//! - WindowDescriptor: 窗口创建配置
//! - CursorIcon: 光标图标类型
//! - WindowEvent: 窗口事件系统
//! - WindowMode: 窗口显示模式
//!
//! 专注于 WebGPU/WASM 平台

use autozig::include_zig;

// ========== CursorIcon ==========

/// CursorIcon - Maps to CSS cursor property
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorIcon {
    Default = 0,
    Pointer = 1,
    Crosshair = 2,
    Hand = 3,
    Text = 4,
    Move = 5,
    NotAllowed = 6,
    NResize = 7,
    EResize = 8,
    SResize = 9,
    WResize = 10,
    NEResize = 11,
    NWResize = 12,
    SEResize = 13,
    SWResize = 14,
    EWResize = 15,
    NSResize = 16,
    Wait = 17,
    Progress = 18,
    Help = 19,
    ZoomIn = 20,
    ZoomOut = 21,
}

include_zig!("src/zig/cursor.zig", {
    fn cursor_icon_to_css_string(icon: CursorIcon, out_buffer: *mut u8, buffer_len: usize) -> usize;
});

impl CursorIcon {
    pub fn to_css_string(&self) -> String {
        let mut buffer = [0u8; 32];
        let len = cursor_icon_to_css_string(*self, buffer.as_mut_ptr(), buffer.len());
        String::from_utf8_lossy(&buffer[..len]).into_owned()
    }
}

// ========== WindowMode ==========

/// WindowMode - Window display mode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowMode {
    Windowed = 0,
    Fullscreen = 1,
}

include_zig!("src/zig/mode.zig", {
    fn window_mode_is_fullscreen(mode: WindowMode) -> bool;
    fn window_mode_is_windowed(mode: WindowMode) -> bool;
});

impl WindowMode {
    pub fn is_fullscreen(&self) -> bool {
        window_mode_is_fullscreen(*self)
    }

    pub fn is_windowed(&self) -> bool {
        window_mode_is_windowed(*self)
    }
}

// ========== WindowEvent ==========

/// WindowEventType - Types of window events
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowEventType {
    Resized = 0,
    Moved = 1,
    CloseRequested = 2,
    Focused = 3,
    Unfocused = 4,
    ScaleFactorChanged = 5,
}

/// WindowEvent - Represents a window event
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WindowEvent {
    pub event_type: WindowEventType,
    pub window_id: u32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
    pub focused: bool,
}

include_zig!("src/zig/event.zig", {
    fn window_event_create(event_type: WindowEventType, window_id: u32) -> WindowEvent;
    fn window_event_create_resized(window_id: u32, width: u32, height: u32) -> WindowEvent;
    fn window_event_create_focused(window_id: u32, focused: bool) -> WindowEvent;
    fn window_event_create_scale_factor_changed(window_id: u32, scale_factor: f32) -> WindowEvent;
    fn window_event_is_resized(event: *const WindowEvent) -> bool;
    fn window_event_is_focused(event: *const WindowEvent) -> bool;
    fn window_event_is_unfocused(event: *const WindowEvent) -> bool;
    fn window_event_is_close_requested(event: *const WindowEvent) -> bool;
});

impl WindowEvent {
    pub fn new(event_type: WindowEventType, window_id: u32) -> Self {
        window_event_create(event_type, window_id)
    }

    pub fn resized(window_id: u32, width: u32, height: u32) -> Self {
        window_event_create_resized(window_id, width, height)
    }

    pub fn focused(window_id: u32, focused: bool) -> Self {
        window_event_create_focused(window_id, focused)
    }

    pub fn scale_factor_changed(window_id: u32, scale_factor: f32) -> Self {
        window_event_create_scale_factor_changed(window_id, scale_factor)
    }

    pub fn is_resized(&self) -> bool {
        window_event_is_resized(self)
    }

    pub fn is_focused(&self) -> bool {
        window_event_is_focused(self)
    }

    pub fn is_unfocused(&self) -> bool {
        window_event_is_unfocused(self)
    }

    pub fn is_close_requested(&self) -> bool {
        window_event_is_close_requested(self)
    }
}

// ========== WindowDescriptor ==========

/// WindowDescriptor - Configuration for creating a window
#[repr(C)]
pub struct WindowDescriptor {
    pub width: u32,
    pub height: u32,
    title: [u8; 128],
    title_len: u32,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
    canvas_id: [u8; 64],
    canvas_id_len: u32,
}

include_zig!("src/zig/descriptor.zig", {
    fn window_descriptor_default() -> WindowDescriptor;
    fn window_descriptor_with_title(desc: WindowDescriptor, title: *const u8, title_len: u32) -> WindowDescriptor;
    fn window_descriptor_with_size(desc: WindowDescriptor, width: u32, height: u32) -> WindowDescriptor;
    fn window_descriptor_with_canvas(desc: WindowDescriptor, canvas_id: *const u8, canvas_id_len: u32) -> WindowDescriptor;
    fn window_descriptor_set_resizable(desc: WindowDescriptor, resizable: bool) -> WindowDescriptor;
    fn window_descriptor_set_decorations(desc: WindowDescriptor, decorations: bool) -> WindowDescriptor;
    fn window_descriptor_set_transparent(desc: WindowDescriptor, transparent: bool) -> WindowDescriptor;
    fn window_descriptor_get_title(desc: *const WindowDescriptor, out_buffer: *mut u8, buffer_len: u32) -> u32;
    fn window_descriptor_get_canvas_id(desc: *const WindowDescriptor, out_buffer: *mut u8, buffer_len: u32) -> u32;
});

impl WindowDescriptor {
    pub fn new() -> Self {
        window_descriptor_default()
    }

    pub fn with_title(self, title: &str) -> Self {
        window_descriptor_with_title(self, title.as_ptr(), title.len() as u32)
    }

    pub fn with_size(self, width: u32, height: u32) -> Self {
        window_descriptor_with_size(self, width, height)
    }

    pub fn with_canvas(self, canvas_id: &str) -> Self {
        window_descriptor_with_canvas(self, canvas_id.as_ptr(), canvas_id.len() as u32)
    }

    pub fn set_resizable(self, resizable: bool) -> Self {
        window_descriptor_set_resizable(self, resizable)
    }

    pub fn set_decorations(self, decorations: bool) -> Self {
        window_descriptor_set_decorations(self, decorations)
    }

    pub fn set_transparent(self, transparent: bool) -> Self {
        window_descriptor_set_transparent(self, transparent)
    }

    pub fn get_title(&self) -> String {
        let mut buffer = [0u8; 128];
        let len = window_descriptor_get_title(self, buffer.as_mut_ptr(), buffer.len() as u32);
        String::from_utf8_lossy(&buffer[..len as usize]).into_owned()
    }

    pub fn get_canvas_id(&self) -> String {
        let mut buffer = [0u8; 64];
        let len = window_descriptor_get_canvas_id(self, buffer.as_mut_ptr(), buffer.len() as u32);
        String::from_utf8_lossy(&buffer[..len as usize]).into_owned()
    }
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// ========== Window ==========

/// Window - Main window structure
#[repr(C)]
pub struct Window {
    pub width: u32,
    pub height: u32,
    title: [u8; 128],
    title_len: u32,
    pub scale_factor: f32,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
    pub focused: bool,
    pub visible: bool,
    pub cursor_visible: bool,
    pub cursor_locked: bool,
    pub cursor_position_x: f32,
    pub cursor_position_y: f32,
    pub cursor_icon: CursorIcon,
    canvas_id: [u8; 64],
    canvas_id_len: u32,
}

include_zig!("src/zig/window.zig", {
    fn window_create(width: u32, height: u32, title: *const u8, title_len: u32) -> Window;
    fn window_set_title(window: *mut Window, title: *const u8, title_len: u32);
    fn window_get_title(window: *const Window, out_buffer: *mut u8, buffer_len: u32) -> u32;
    fn window_resize(window: *mut Window, width: u32, height: u32);
    fn window_set_visible(window: *mut Window, visible: bool);
    fn window_set_focused(window: *mut Window, focused: bool);
    fn window_set_cursor_visible(window: *mut Window, visible: bool);
    fn window_set_cursor_locked(window: *mut Window, locked: bool);
    fn window_set_cursor_icon(window: *mut Window, icon: CursorIcon);
    fn window_set_cursor_position(window: *mut Window, x: f32, y: f32);
    fn window_set_scale_factor(window: *mut Window, factor: f32);
    fn window_set_canvas_id(window: *mut Window, canvas_id: *const u8, canvas_id_len: u32);
    fn window_get_canvas_id(window: *const Window, out_buffer: *mut u8, buffer_len: u32) -> u32;
    fn window_get_width(window: *const Window) -> u32;
    fn window_get_height(window: *const Window) -> u32;
    fn window_get_scale_factor(window: *const Window) -> f32;
    fn window_is_visible(window: *const Window) -> bool;
    fn window_is_focused(window: *const Window) -> bool;
    fn window_is_cursor_visible(window: *const Window) -> bool;
    fn window_is_cursor_locked(window: *const Window) -> bool;
    fn window_get_cursor_icon(window: *const Window) -> CursorIcon;
    fn window_get_cursor_position_x(window: *const Window) -> f32;
    fn window_get_cursor_position_y(window: *const Window) -> f32;
});

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        window_create(width, height, title.as_ptr(), title.len() as u32)
    }

    pub fn set_title(&mut self, title: &str) {
        window_set_title(self, title.as_ptr(), title.len() as u32);
    }

    pub fn get_title(&self) -> String {
        let mut buffer = [0u8; 128];
        let len = window_get_title(self, buffer.as_mut_ptr(), buffer.len() as u32);
        String::from_utf8_lossy(&buffer[..len as usize]).into_owned()
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        window_resize(self, width, height);
    }

    pub fn set_visible(&mut self, visible: bool) {
        window_set_visible(self, visible);
    }

    pub fn set_focused(&mut self, focused: bool) {
        window_set_focused(self, focused);
    }

    pub fn set_cursor_visible(&mut self, visible: bool) {
        window_set_cursor_visible(self, visible);
    }

    pub fn set_cursor_locked(&mut self, locked: bool) {
        window_set_cursor_locked(self, locked);
    }

    pub fn set_cursor_icon(&mut self, icon: CursorIcon) {
        window_set_cursor_icon(self, icon);
    }

    pub fn set_cursor_position(&mut self, x: f32, y: f32) {
        window_set_cursor_position(self, x, y);
    }

    pub fn set_scale_factor(&mut self, factor: f32) {
        window_set_scale_factor(self, factor);
    }

    pub fn set_canvas_id(&mut self, canvas_id: &str) {
        window_set_canvas_id(self, canvas_id.as_ptr(), canvas_id.len() as u32);
    }

    pub fn get_canvas_id(&self) -> String {
        let mut buffer = [0u8; 64];
        let len = window_get_canvas_id(self, buffer.as_mut_ptr(), buffer.len() as u32);
        String::from_utf8_lossy(&buffer[..len as usize]).into_owned()
    }

    pub fn get_width(&self) -> u32 {
        window_get_width(self)
    }

    pub fn get_height(&self) -> u32 {
        window_get_height(self)
    }

    pub fn get_scale_factor(&self) -> f32 {
        window_get_scale_factor(self)
    }

    pub fn is_visible(&self) -> bool {
        window_is_visible(self)
    }

    pub fn is_focused(&self) -> bool {
        window_is_focused(self)
    }

    pub fn is_cursor_visible(&self) -> bool {
        window_is_cursor_visible(self)
    }

    pub fn is_cursor_locked(&self) -> bool {
        window_is_cursor_locked(self)
    }

    pub fn get_cursor_icon(&self) -> CursorIcon {
        window_get_cursor_icon(self)
    }

    pub fn get_cursor_position(&self) -> (f32, f32) {
        (window_get_cursor_position_x(self), window_get_cursor_position_y(self))
    }
}