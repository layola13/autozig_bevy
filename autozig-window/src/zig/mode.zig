const std = @import("std");

/// WindowMode - Window display mode for WebGPU/WASM platform
pub const WindowMode = enum(u8) {
    /// Normal windowed mode (canvas in page)
    Windowed = 0,
    /// Fullscreen mode (uses Canvas Fullscreen API)
    Fullscreen = 1,
};

// FFI exports for window mode

export fn window_mode_is_fullscreen(mode: WindowMode) bool {
    return mode == .Fullscreen;
}

export fn window_mode_is_windowed(mode: WindowMode) bool {
    return mode == .Windowed;
}

// Tests

test "WindowMode enum values" {
    const testing = std.testing;

    try testing.expectEqual(@as(u8, 0), @intFromEnum(WindowMode.Windowed));
    try testing.expectEqual(@as(u8, 1), @intFromEnum(WindowMode.Fullscreen));
}

test "window_mode_is_fullscreen" {
    const testing = std.testing;

    try testing.expect(!window_mode_is_fullscreen(WindowMode.Windowed));
    try testing.expect(window_mode_is_fullscreen(WindowMode.Fullscreen));
}

test "window_mode_is_windowed" {
    const testing = std.testing;

    try testing.expect(window_mode_is_windowed(WindowMode.Windowed));
    try testing.expect(!window_mode_is_windowed(WindowMode.Fullscreen));
}
