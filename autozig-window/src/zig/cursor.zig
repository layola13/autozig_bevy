const std = @import("std");

/// CursorIcon - Maps to CSS cursor property values
/// Reference: https://developer.mozilla.org/en-US/docs/Web/CSS/cursor
pub const CursorIcon = enum(u8) {
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
};

// FFI exports for cursor icon

export fn cursor_icon_to_css_string(icon: CursorIcon, out_buffer: [*]u8, buffer_len: usize) usize {
    const css_name = switch (icon) {
        .Default => "default",
        .Pointer => "pointer",
        .Crosshair => "crosshair",
        .Hand => "grab",
        .Text => "text",
        .Move => "move",
        .NotAllowed => "not-allowed",
        .NResize => "n-resize",
        .EResize => "e-resize",
        .SResize => "s-resize",
        .WResize => "w-resize",
        .NEResize => "ne-resize",
        .NWResize => "nw-resize",
        .SEResize => "se-resize",
        .SWResize => "sw-resize",
        .EWResize => "ew-resize",
        .NSResize => "ns-resize",
        .Wait => "wait",
        .Progress => "progress",
        .Help => "help",
        .ZoomIn => "zoom-in",
        .ZoomOut => "zoom-out",
    };

    const copy_len = @min(css_name.len, buffer_len);
    @memcpy(out_buffer[0..copy_len], css_name[0..copy_len]);
    return copy_len;
}

// Tests

test "CursorIcon enum values" {
    const testing = std.testing;

    try testing.expectEqual(@as(u8, 0), @intFromEnum(CursorIcon.Default));
    try testing.expectEqual(@as(u8, 1), @intFromEnum(CursorIcon.Pointer));
    try testing.expectEqual(@as(u8, 21), @intFromEnum(CursorIcon.ZoomOut));
}

test "cursor_icon_to_css_string" {
    const testing = std.testing;

    var buffer: [32]u8 = undefined;

    var len = cursor_icon_to_css_string(CursorIcon.Default, &buffer, buffer.len);
    try testing.expectEqual(@as(usize, 7), len);
    try testing.expectEqualStrings("default", buffer[0..len]);

    len = cursor_icon_to_css_string(CursorIcon.Pointer, &buffer, buffer.len);
    try testing.expectEqual(@as(usize, 7), len);
    try testing.expectEqualStrings("pointer", buffer[0..len]);

    len = cursor_icon_to_css_string(CursorIcon.NotAllowed, &buffer, buffer.len);
    try testing.expectEqual(@as(usize, 11), len);
    try testing.expectEqualStrings("not-allowed", buffer[0..len]);
}
