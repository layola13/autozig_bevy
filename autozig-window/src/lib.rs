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

/// Common imports for autozig windowing
pub mod prelude {
    pub use crate::{
        Window, WindowPlugin, WindowDescriptor, WindowResolution, 
        WindowPosition, CursorIcon, WindowMode, Monitor, VideoMode,
        PrimaryWindow, PrimaryMonitor,
    };
}

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

// ========== Entity Type (for ECS integration) ==========

/// Entity - Bevy ECS entity identifier (u64)
pub type Entity = u64;

// ========== Window Events (Structs 1-18) ==========

/// WindowResized - Event sent when a window is resized
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowResized {
    pub entity: Entity,
    pub width: f32,
    pub height: f32,
}

/// WindowMoved - Event sent when a window is moved
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowMoved {
    pub entity: Entity,
    pub position_x: i32,
    pub position_y: i32,
}

/// WindowFocused - Event sent when a window gains or loses focus
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowFocused {
    pub entity: Entity,
    pub focused: bool,
}

/// WindowCloseRequested - Event sent when a window's close button is clicked
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowCloseRequested {
    pub entity: Entity,
}

/// WindowClosed - Event sent when a window is closed
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowClosed {
    pub entity: Entity,
}

/// WindowClosing - Event sent when a window is about to close
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowClosing {
    pub entity: Entity,
}

/// ClosingWindow - Component marking a window as closing
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClosingWindow;

/// WindowCreated - Event sent when a window is created
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowCreated {
    pub entity: Entity,
}

/// WindowDestroyed - Event sent when a window is destroyed
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowDestroyed {
    pub entity: Entity,
}

/// WindowScaleFactorChanged - Event sent when window scale factor changes
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowScaleFactorChanged {
    pub entity: Entity,
    pub scale_factor: f64,
}

/// WindowBackendScaleFactorChanged - Event sent when backend scale factor changes
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowBackendScaleFactorChanged {
    pub entity: Entity,
    pub scale_factor: f64,
}

/// WindowOccluded - Event sent when a window is occluded (hidden behind other windows)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowOccluded {
    pub entity: Entity,
    pub occluded: bool,
}

/// WindowThemeChanged - Event sent when window theme changes
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowThemeChanged {
    pub entity: Entity,
    pub theme: WindowTheme,
}

/// RequestRedraw - Event requesting a window redraw
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestRedraw;

// ========== Cursor Events (Structs 19-22) ==========

/// CursorEntered - Event sent when cursor enters a window
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorEntered {
    pub entity: Entity,
}

/// CursorLeft - Event sent when cursor leaves a window
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorLeft {
    pub entity: Entity,
}

/// CursorMoved - Event sent when cursor moves within a window
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CursorMoved {
    pub entity: Entity,
    pub position_x: f32,
    pub position_y: f32,
}

/// CursorOptions - Configuration for cursor behavior
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorOptions {
    pub grab_mode: CursorGrabMode,
    pub visible: bool,
    pub hit_test: bool,
}

impl Default for CursorOptions {
    fn default() -> Self {
        Self {
            grab_mode: CursorGrabMode::None,
            visible: true,
            hit_test: true,
        }
    }
}

// ========== Window Configuration (Structs 23-26) ==========

/// WindowResolution - Window resolution configuration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowResolution {
    pub physical_width: u32,
    pub physical_height: u32,
    pub scale_factor_override: f32,
}

impl WindowResolution {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            physical_width: width as u32,
            physical_height: height as u32,
            scale_factor_override: 1.0,
        }
    }

    pub fn with_scale_factor_override(mut self, scale_factor: f32) -> Self {
        self.scale_factor_override = scale_factor;
        self
    }

    pub fn width(&self) -> f32 {
        self.physical_width as f32
    }

    pub fn height(&self) -> f32 {
        self.physical_height as f32
    }
}

impl Default for WindowResolution {
    fn default() -> Self {
        Self::new(1280.0, 720.0)
    }
}

/// WindowResizeConstraints - Constraints for window resizing
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowResizeConstraints {
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,
}

impl Default for WindowResizeConstraints {
    fn default() -> Self {
        Self {
            min_width: 180.0,
            min_height: 120.0,
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
        }
    }
}

/// WindowPosition - Position configuration for a window
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}

impl WindowPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn centered() -> Self {
        Self { x: i32::MIN, y: i32::MIN }
    }
}

/// PrimaryWindow - Marker component for the primary window
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimaryWindow;

/// PrimaryMonitor - Marker for primary monitor selection
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimaryMonitor;

// ========== Monitor and VideoMode (Structs 27-28) ==========

/// Monitor - Represents a display monitor
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Monitor {
    pub width: u32,
    pub height: u32,
    pub refresh_rate_millihertz: u32,
    pub position_x: i32,
    pub position_y: i32,
    pub scale_factor: f64,
    name: [u8; 128],
    name_len: u32,
}

impl Monitor {
    pub fn new(width: u32, height: u32, refresh_rate_millihertz: u32) -> Self {
        Self {
            width,
            height,
            refresh_rate_millihertz,
            position_x: 0,
            position_y: 0,
            scale_factor: 1.0,
            name: [0; 128],
            name_len: 0,
        }
    }

    pub fn name(&self) -> String {
        String::from_utf8_lossy(&self.name[..self.name_len as usize]).into_owned()
    }
}

/// VideoMode - Represents a video mode for fullscreen
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate_millihertz: u32,
    pub bit_depth: u16,
}

impl VideoMode {
    pub fn new(width: u32, height: u32, refresh_rate_millihertz: u32) -> Self {
        Self {
            width,
            height,
            refresh_rate_millihertz,
            bit_depth: 24,
        }
    }
}

// ========== Platform Handle Wrappers (Structs 29-32) ==========

/// RawHandleWrapper - Wrapper for raw window handles (placeholder for cross-platform)
#[repr(C)]
#[derive(Debug)]
pub struct RawHandleWrapper {
    _platform_handle: u64,
}

impl RawHandleWrapper {
    pub fn new(handle: u64) -> Self {
        Self {
            _platform_handle: handle,
        }
    }
}

/// RawHandleWrapperHolder - Holder for raw handle wrapper
#[repr(C)]
#[derive(Debug)]
pub struct RawHandleWrapperHolder {
    pub wrapper: RawHandleWrapper,
}

/// ThreadLockedRawWindowHandleWrapper - Thread-locked raw window handle
#[repr(C)]
#[derive(Debug)]
pub struct ThreadLockedRawWindowHandleWrapper {
    _handle: u64,
}

/// WindowWrapper - Wrapper for window state
#[repr(C)]
#[derive(Debug)]
pub struct WindowWrapper {
    pub entity: Entity,
}

/// InternalWindowState - Internal window state (placeholder)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InternalWindowState {
    pub minimized: bool,
    pub maximized: bool,
    pub has_focus: bool,
}

impl Default for InternalWindowState {
    fn default() -> Self {
        Self {
            minimized: false,
            maximized: false,
            has_focus: false,
        }
    }
}

// ========== Custom Cursor (Structs 33-36) ==========

/// CustomCursorImage - Custom cursor image data
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CustomCursorImage {
    pub width: u32,
    pub height: u32,
    pub hotspot_x: u16,
    pub hotspot_y: u16,
}

/// CustomCursorUrl - Custom cursor from URL
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomCursorUrl {
    url: [u8; 256],
    url_len: u32,
}

impl CustomCursorUrl {
    pub fn new(url: &str) -> Self {
        let mut url_bytes = [0u8; 256];
        let len = url.len().min(256);
        url_bytes[..len].copy_from_slice(&url.as_bytes()[..len]);
        Self {
            url: url_bytes,
            url_len: len as u32,
        }
    }

    pub fn url(&self) -> String {
        String::from_utf8_lossy(&self.url[..self.url_len as usize]).into_owned()
    }
}

/// EnabledButtons - Configuration for enabled window buttons
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EnabledButtons {
    pub minimize: bool,
    pub maximize: bool,
    pub close: bool,
}

impl Default for EnabledButtons {
    fn default() -> Self {
        Self {
            minimize: true,
            maximize: true,
            close: true,
        }
    }
}

// ========== Window Handle Resource ==========

use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, HandleError,
    HasWindowHandle, HasDisplayHandle, WindowHandle, DisplayHandle,
};

/// Resource containing raw handles for WGPU initialization
#[derive(Debug, Clone)]
pub struct WindowRawHandle {
    pub window_handle: RawWindowHandle,
    pub display_handle: RawDisplayHandle,
}



impl HasWindowHandle for WindowRawHandle {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        // SAFETY: We hold a valid raw window handle.
        unsafe { Ok(WindowHandle::borrow_raw(self.window_handle)) }
    }
}

impl HasDisplayHandle for WindowRawHandle {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        // SAFETY: We hold a valid raw display handle.
        unsafe { Ok(DisplayHandle::borrow_raw(self.display_handle)) }
    }
}

unsafe impl Send for WindowRawHandle {}
unsafe impl Sync for WindowRawHandle {}

/// WindowPlugin - Plugin configuration for window system
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPlugin {
    pub primary_window_enabled: bool,
    pub exit_condition: ExitCondition,
    pub close_when_requested: bool,
}

impl Default for WindowPlugin {
    fn default() -> Self {
        Self {
            primary_window_enabled: true,
            exit_condition: ExitCondition::OnPrimaryClosed,
            close_when_requested: true,
        }
    }
}

impl autozig_app::Plugin for WindowPlugin {
    fn build(&self, app: &mut autozig_app::App) {
        // Init default descriptor if missing
        app.init_resource::<WindowDescriptor>();
        
        if self.primary_window_enabled {
            // Create the window
            let window = Window::new(1280, 720, "3D Cube Demo");
            app.insert_resource(window);
        }
    }
    
    fn name(&self) -> &str {
        "WindowPlugin"
    }
}

// ========== Enums (14 types) ==========

/// WindowTheme - Window theme (light/dark)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowTheme {
    Light = 0,
    Dark = 1,
}

/// CursorGrabMode - Cursor grab/lock mode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CursorGrabMode {
    None = 0,
    Confined = 1,
    Locked = 2,
}

/// WindowLevel - Window z-order level
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindowLevel {
    AlwaysOnBottom = 0,
    Normal = 1,
    AlwaysOnTop = 2,
}

/// PresentMode - GPU present mode for rendering
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresentMode {
    AutoVsync = 0,
    AutoNoVsync = 1,
    Fifo = 2,
    FifoRelaxed = 3,
    Immediate = 4,
    Mailbox = 5,
}

impl Default for PresentMode {
    fn default() -> Self {
        Self::AutoVsync
    }
}

/// CompositeAlphaMode - Alpha compositing mode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompositeAlphaMode {
    Auto = 0,
    Opaque = 1,
    PreMultiplied = 2,
    PostMultiplied = 3,
    Inherit = 4,
}

impl Default for CompositeAlphaMode {
    fn default() -> Self {
        Self::Auto
    }
}

/// ExitCondition - Condition for application exit
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExitCondition {
    OnPrimaryClosed = 0,
    OnAllClosed = 1,
    DontExit = 2,
}

/// AppLifecycle - Application lifecycle state
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppLifecycle {
    Idle = 0,
    Running = 1,
    Suspended = 2,
    WillSuspend = 3,
    WillResume = 4,
}

/// MonitorSelection - Monitor selection strategy
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonitorSelection {
    Primary,
    Number(usize),
    Current,
}

/// VideoModeSelection - Video mode selection strategy
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoModeSelection {
    Auto,
    Specific(VideoMode),
}

/// CustomCursor - Custom cursor type
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomCursor {
    Image(CustomCursorImage),
    Url(CustomCursorUrl),
}

/// SystemCursorIcon - System cursor icons (extended version)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemCursorIcon {
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
    Grab = 22,
    Grabbing = 23,
    ContextMenu = 24,
    Copy = 25,
    Alias = 26,
    NoDrop = 27,
    AllScroll = 28,
    Cell = 29,
    VerticalText = 30,
    ColResize = 31,
    RowResize = 32,
}

/// FileDragAndDrop - File drag and drop events
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum FileDragAndDrop {
    DroppedFile { entity: Entity, path_len: u32 },
    HoveredFile { entity: Entity, path_len: u32 },
    HoveredFileCanceled { entity: Entity },
}

/// Ime - Input Method Editor events
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub enum Ime {
    Enabled { entity: Entity },
    Preedit { entity: Entity, value_len: u32, cursor: Option<(usize, usize)> },
    Commit { entity: Entity, value_len: u32 },
    Disabled { entity: Entity },
}

/// WindowRef - Reference to a window (enum or entity)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowRef {
    Entity(Entity),
    Primary,
}

/// NormalizedWindowRef - Normalized window reference
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NormalizedWindowRef {
    pub entity: Entity,
}

impl NormalizedWindowRef {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

/// ScreenEdge - Screen edge for window positioning
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScreenEdge {
    Top = 0,
    Bottom = 1,
    Left = 2,
    Right = 3,
}