const std = @import("std");

/// WindowEventType - Types of window events
pub const WindowEventType = enum(u8) {
    Resized = 0,
    Moved = 1,
    CloseRequested = 2,
    Focused = 3,
    Unfocused = 4,
    ScaleFactorChanged = 5,
};

/// WindowEvent - Represents a window event
pub const WindowEvent = extern struct {
    event_type: WindowEventType,
    window_id: u32,
    width: u32,
    height: u32,
    scale_factor: f32,
    focused: bool,

    pub fn init(event_type: WindowEventType, window_id: u32) WindowEvent {
        return WindowEvent{
            .event_type = event_type,
            .window_id = window_id,
            .width = 0,
            .height = 0,
            .scale_factor = 1.0,
            .focused = false,
        };
    }

    pub fn initResized(window_id: u32, width: u32, height: u32) WindowEvent {
        return WindowEvent{
            .event_type = .Resized,
            .window_id = window_id,
            .width = width,
            .height = height,
            .scale_factor = 1.0,
            .focused = false,
        };
    }

    pub fn initFocused(window_id: u32, focused: bool) WindowEvent {
        return WindowEvent{
            .event_type = if (focused) .Focused else .Unfocused,
            .window_id = window_id,
            .width = 0,
            .height = 0,
            .scale_factor = 1.0,
            .focused = focused,
        };
    }

    pub fn initScaleFactorChanged(window_id: u32, scale_factor: f32) WindowEvent {
        return WindowEvent{
            .event_type = .ScaleFactorChanged,
            .window_id = window_id,
            .width = 0,
            .height = 0,
            .scale_factor = scale_factor,
            .focused = false,
        };
    }
};

// FFI exports for window events

export fn window_event_create(event_type: WindowEventType, window_id: u32) WindowEvent {
    return WindowEvent.init(event_type, window_id);
}

export fn window_event_create_resized(window_id: u32, width: u32, height: u32) WindowEvent {
    return WindowEvent.initResized(window_id, width, height);
}

export fn window_event_create_focused(window_id: u32, focused: bool) WindowEvent {
    return WindowEvent.initFocused(window_id, focused);
}

export fn window_event_create_scale_factor_changed(window_id: u32, scale_factor: f32) WindowEvent {
    return WindowEvent.initScaleFactorChanged(window_id, scale_factor);
}

export fn window_event_is_resized(event: *const WindowEvent) bool {
    return event.event_type == .Resized;
}

export fn window_event_is_focused(event: *const WindowEvent) bool {
    return event.event_type == .Focused;
}

export fn window_event_is_unfocused(event: *const WindowEvent) bool {
    return event.event_type == .Unfocused;
}

export fn window_event_is_close_requested(event: *const WindowEvent) bool {
    return event.event_type == .CloseRequested;
}

// Tests

test "WindowEventType enum values" {
    const testing = std.testing;

    try testing.expectEqual(@as(u8, 0), @intFromEnum(WindowEventType.Resized));
    try testing.expectEqual(@as(u8, 5), @intFromEnum(WindowEventType.ScaleFactorChanged));
}

test "WindowEvent create" {
    const testing = std.testing;

    const event = window_event_create(WindowEventType.CloseRequested, 1);
    try testing.expectEqual(WindowEventType.CloseRequested, event.event_type);
    try testing.expectEqual(@as(u32, 1), event.window_id);
}

test "WindowEvent create_resized" {
    const testing = std.testing;

    const event = window_event_create_resized(1, 800, 600);
    try testing.expectEqual(WindowEventType.Resized, event.event_type);
    try testing.expectEqual(@as(u32, 1), event.window_id);
    try testing.expectEqual(@as(u32, 800), event.width);
    try testing.expectEqual(@as(u32, 600), event.height);
}

test "WindowEvent create_focused" {
    const testing = std.testing;

    const event_focused = window_event_create_focused(1, true);
    try testing.expectEqual(WindowEventType.Focused, event_focused.event_type);
    try testing.expect(event_focused.focused);

    const event_unfocused = window_event_create_focused(1, false);
    try testing.expectEqual(WindowEventType.Unfocused, event_unfocused.event_type);
    try testing.expect(!event_unfocused.focused);
}

test "WindowEvent create_scale_factor_changed" {
    const testing = std.testing;

    const event = window_event_create_scale_factor_changed(1, 2.0);
    try testing.expectEqual(WindowEventType.ScaleFactorChanged, event.event_type);
    try testing.expectEqual(@as(u32, 1), event.window_id);
    try testing.expectEqual(@as(f32, 2.0), event.scale_factor);
}

test "WindowEvent type checks" {
    const testing = std.testing;

    const resized = window_event_create_resized(1, 800, 600);
    try testing.expect(window_event_is_resized(&resized));
    try testing.expect(!window_event_is_focused(&resized));

    const focused = window_event_create_focused(1, true);
    try testing.expect(window_event_is_focused(&focused));
    try testing.expect(!window_event_is_unfocused(&focused));

    const close = window_event_create(WindowEventType.CloseRequested, 1);
    try testing.expect(window_event_is_close_requested(&close));
}
