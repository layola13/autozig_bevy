use autozig_window::{
    CursorIcon, Window, WindowDescriptor, WindowEvent, WindowEventType, WindowMode,
};

#[test]
fn test_window_create() {
    let window = Window::new(800, 600, "Test Window");
    assert_eq!(window.get_width(), 800);
    assert_eq!(window.get_height(), 600);
    assert_eq!(window.get_title(), "Test Window");
    assert!(window.is_visible());
    assert!(!window.is_focused());
}

#[test]
fn test_window_descriptor_default() {
    let desc = WindowDescriptor::default();
    assert_eq!(desc.width, 800);
    assert_eq!(desc.height, 600);
    assert_eq!(desc.get_title(), "AutoZig Window");
    assert!(desc.resizable);
    assert!(desc.decorations);
    assert!(!desc.transparent);
}

#[test]
fn test_window_descriptor_builder() {
    let desc = WindowDescriptor::default()
        .with_title("Custom Title")
        .with_size(1920, 1080)
        .with_canvas("my-canvas")
        .set_resizable(false)
        .set_decorations(false)
        .set_transparent(true);

    assert_eq!(desc.get_title(), "Custom Title");
    assert_eq!(desc.width, 1920);
    assert_eq!(desc.height, 1080);
    assert_eq!(desc.get_canvas_id(), "my-canvas");
    assert!(!desc.resizable);
    assert!(!desc.decorations);
    assert!(desc.transparent);
}

#[test]
fn test_window_set_title() {
    let mut window = Window::new(800, 600, "Initial Title");
    assert_eq!(window.get_title(), "Initial Title");

    window.set_title("Updated Title");
    assert_eq!(window.get_title(), "Updated Title");
}

#[test]
fn test_window_resize() {
    let mut window = Window::new(800, 600, "Test");
    assert_eq!(window.get_width(), 800);
    assert_eq!(window.get_height(), 600);

    window.resize(1920, 1080);
    assert_eq!(window.get_width(), 1920);
    assert_eq!(window.get_height(), 1080);
}

#[test]
fn test_window_visibility() {
    let mut window = Window::new(800, 600, "Test");
    assert!(window.is_visible());

    window.set_visible(false);
    assert!(!window.is_visible());

    window.set_visible(true);
    assert!(window.is_visible());
}

#[test]
fn test_window_focus() {
    let mut window = Window::new(800, 600, "Test");
    assert!(!window.is_focused());

    window.set_focused(true);
    assert!(window.is_focused());

    window.set_focused(false);
    assert!(!window.is_focused());
}

#[test]
fn test_cursor_visibility() {
    let mut window = Window::new(800, 600, "Test");
    assert!(window.is_cursor_visible());

    window.set_cursor_visible(false);
    assert!(!window.is_cursor_visible());

    window.set_cursor_visible(true);
    assert!(window.is_cursor_visible());
}

#[test]
fn test_cursor_lock() {
    let mut window = Window::new(800, 600, "Test");
    assert!(!window.is_cursor_locked());

    window.set_cursor_locked(true);
    assert!(window.is_cursor_locked());

    window.set_cursor_locked(false);
    assert!(!window.is_cursor_locked());
}

#[test]
fn test_cursor_icon() {
    let mut window = Window::new(800, 600, "Test");
    assert_eq!(window.get_cursor_icon(), CursorIcon::Default);

    window.set_cursor_icon(CursorIcon::Pointer);
    assert_eq!(window.get_cursor_icon(), CursorIcon::Pointer);

    window.set_cursor_icon(CursorIcon::Hand);
    assert_eq!(window.get_cursor_icon(), CursorIcon::Hand);

    window.set_cursor_icon(CursorIcon::Crosshair);
    assert_eq!(window.get_cursor_icon(), CursorIcon::Crosshair);
}

#[test]
fn test_cursor_icon_to_css() {
    assert_eq!(CursorIcon::Default.to_css_string(), "default");
    assert_eq!(CursorIcon::Pointer.to_css_string(), "pointer");
    assert_eq!(CursorIcon::Crosshair.to_css_string(), "crosshair");
    assert_eq!(CursorIcon::Hand.to_css_string(), "grab");
    assert_eq!(CursorIcon::Text.to_css_string(), "text");
    assert_eq!(CursorIcon::Move.to_css_string(), "move");
    assert_eq!(CursorIcon::NotAllowed.to_css_string(), "not-allowed");
    assert_eq!(CursorIcon::Wait.to_css_string(), "wait");
    assert_eq!(CursorIcon::Help.to_css_string(), "help");
}

#[test]
fn test_cursor_position() {
    let mut window = Window::new(800, 600, "Test");
    let (x, y) = window.get_cursor_position();
    assert_eq!(x, 0.0);
    assert_eq!(y, 0.0);

    window.set_cursor_position(100.5, 200.5);
    let (x, y) = window.get_cursor_position();
    assert_eq!(x, 100.5);
    assert_eq!(y, 200.5);
}

#[test]
fn test_scale_factor() {
    let mut window = Window::new(800, 600, "Test");
    assert_eq!(window.get_scale_factor(), 1.0);

    window.set_scale_factor(2.0);
    assert_eq!(window.get_scale_factor(), 2.0);

    window.set_scale_factor(1.5);
    assert_eq!(window.get_scale_factor(), 1.5);
}

#[test]
fn test_window_event_resize() {
    let event = WindowEvent::resized(1, 1920, 1080);
    assert_eq!(event.event_type, WindowEventType::Resized);
    assert_eq!(event.window_id, 1);
    assert_eq!(event.width, 1920);
    assert_eq!(event.height, 1080);
    assert!(event.is_resized());
    assert!(!event.is_focused());
}

#[test]
fn test_window_event_focus() {
    let focused_event = WindowEvent::focused(1, true);
    assert_eq!(focused_event.event_type, WindowEventType::Focused);
    assert_eq!(focused_event.window_id, 1);
    assert!(focused_event.focused);
    assert!(focused_event.is_focused());
    assert!(!focused_event.is_unfocused());

    let unfocused_event = WindowEvent::focused(1, false);
    assert_eq!(unfocused_event.event_type, WindowEventType::Unfocused);
    assert!(!unfocused_event.focused);
    assert!(unfocused_event.is_unfocused());
    assert!(!unfocused_event.is_focused());
}

#[test]
fn test_canvas_id() {
    let mut window = Window::new(800, 600, "Test");
    assert_eq!(window.get_canvas_id(), "");

    window.set_canvas_id("my-canvas-element");
    assert_eq!(window.get_canvas_id(), "my-canvas-element");

    window.set_canvas_id("another-canvas");
    assert_eq!(window.get_canvas_id(), "another-canvas");
}

#[test]
fn test_fullscreen_mode() {
    assert!(WindowMode::Fullscreen.is_fullscreen());
    assert!(!WindowMode::Fullscreen.is_windowed());

    assert!(WindowMode::Windowed.is_windowed());
    assert!(!WindowMode::Windowed.is_fullscreen());
}

#[test]
fn test_window_event_scale_factor_changed() {
    let event = WindowEvent::scale_factor_changed(1, 2.0);
    assert_eq!(event.event_type, WindowEventType::ScaleFactorChanged);
    assert_eq!(event.window_id, 1);
    assert_eq!(event.scale_factor, 2.0);
}

#[test]
fn test_window_event_close_requested() {
    let event = WindowEvent::new(WindowEventType::CloseRequested, 1);
    assert_eq!(event.event_type, WindowEventType::CloseRequested);
    assert_eq!(event.window_id, 1);
    assert!(event.is_close_requested());
    assert!(!event.is_resized());
}

#[test]
fn test_window_multiple_properties() {
    let mut window = Window::new(1024, 768, "Multi Test");

    // Set multiple properties
    window.set_visible(true);
    window.set_focused(true);
    window.set_cursor_visible(false);
    window.set_cursor_locked(true);
    window.set_cursor_icon(CursorIcon::ZoomIn);
    window.set_cursor_position(512.0, 384.0);
    window.set_scale_factor(1.5);
    window.set_canvas_id("game-canvas");

    // Verify all properties
    assert_eq!(window.get_width(), 1024);
    assert_eq!(window.get_height(), 768);
    assert_eq!(window.get_title(), "Multi Test");
    assert!(window.is_visible());
    assert!(window.is_focused());
    assert!(!window.is_cursor_visible());
    assert!(window.is_cursor_locked());
    assert_eq!(window.get_cursor_icon(), CursorIcon::ZoomIn);
    let (x, y) = window.get_cursor_position();
    assert_eq!(x, 512.0);
    assert_eq!(y, 384.0);
    assert_eq!(window.get_scale_factor(), 1.5);
    assert_eq!(window.get_canvas_id(), "game-canvas");
}

#[test]
fn test_long_title_truncation() {
    let long_title = "A".repeat(200); // 200 characters, should be truncated to 128
    let window = Window::new(800, 600, &long_title);
    let result_title = window.get_title();
    assert!(result_title.len() <= 128);
    assert_eq!(result_title.len(), 128);
}

#[test]
fn test_long_canvas_id_truncation() {
    let long_canvas = "canvas-".to_string() + &"x".repeat(100); // Should be truncated to 64
    let mut window = Window::new(800, 600, "Test");
    window.set_canvas_id(&long_canvas);
    let result_canvas = window.get_canvas_id();
    assert!(result_canvas.len() <= 64);
    assert_eq!(result_canvas.len(), 64);
}