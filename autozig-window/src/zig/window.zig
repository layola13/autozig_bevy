const std = @import("std");

// CursorIcon is defined in cursor.zig, we use u8 here for FFI compatibility
// When merged by autozig, CursorIcon will be available from cursor.zig

/// Window - Main window structure for WebGPU/WASM platform
/// Uses fixed-size arrays to avoid heap allocation
pub const Window = extern struct {
    width: u32,
    height: u32,
    title: [128]u8,
    title_len: u32,
    scale_factor: f32,
    resizable: bool,
    decorations: bool,
    transparent: bool,
    focused: bool,
    visible: bool,
    cursor_visible: bool,
    cursor_locked: bool,
    cursor_position_x: f32,
    cursor_position_y: f32,
    cursor_icon: CursorIcon,
    canvas_id: [64]u8,
    canvas_id_len: u32,

    pub fn init(width: u32, height: u32, title: []const u8) Window {
        var window = Window{
            .width = width,
            .height = height,
            .title = [_]u8{0} ** 128,
            .title_len = 0,
            .scale_factor = 1.0,
            .resizable = true,
            .decorations = true,
            .transparent = false,
            .focused = false,
            .visible = true,
            .cursor_visible = true,
            .cursor_locked = false,
            .cursor_position_x = 0.0,
            .cursor_position_y = 0.0,
            .cursor_icon = .Default,
            .canvas_id = [_]u8{0} ** 64,
            .canvas_id_len = 0,
        };

        window.setTitle(title);
        return window;
    }

    pub fn setTitle(self: *Window, title: []const u8) void {
        const len = @min(title.len, 128);
        @memcpy(self.title[0..len], title[0..len]);
        self.title_len = @intCast(len);
    }

    pub fn getTitle(self: *const Window) []const u8 {
        return self.title[0..self.title_len];
    }

    pub fn resize(self: *Window, width: u32, height: u32) void {
        self.width = width;
        self.height = height;
    }

    pub fn setVisible(self: *Window, visible: bool) void {
        self.visible = visible;
    }

    pub fn setFocused(self: *Window, focused: bool) void {
        self.focused = focused;
    }

    pub fn setCursorVisible(self: *Window, visible: bool) void {
        self.cursor_visible = visible;
    }

    pub fn setCursorLocked(self: *Window, locked: bool) void {
        self.cursor_locked = locked;
    }

    pub fn setCursorIcon(self: *Window, icon: CursorIcon) void {
        self.cursor_icon = icon;
    }

    pub fn setCursorPosition(self: *Window, x: f32, y: f32) void {
        self.cursor_position_x = x;
        self.cursor_position_y = y;
    }

    pub fn setScaleFactor(self: *Window, factor: f32) void {
        self.scale_factor = factor;
    }

    pub fn setCanvasId(self: *Window, canvas_id: []const u8) void {
        const len = @min(canvas_id.len, 64);
        @memcpy(self.canvas_id[0..len], canvas_id[0..len]);
        self.canvas_id_len = @intCast(len);
    }

    pub fn getCanvasId(self: *const Window) []const u8 {
        return self.canvas_id[0..self.canvas_id_len];
    }
};

// FFI exports for window

export fn window_create(width: u32, height: u32, title: [*]const u8, title_len: u32) Window {
    return Window.init(width, height, title[0..title_len]);
}

export fn window_set_title(window: *Window, title: [*]const u8, title_len: u32) void {
    window.setTitle(title[0..title_len]);
}

export fn window_get_title(window: *const Window, out_buffer: [*]u8, buffer_len: u32) u32 {
    const title = window.getTitle();
    const len = @min(title.len, buffer_len);
    @memcpy(out_buffer[0..len], title[0..len]);
    return @intCast(len);
}

export fn window_resize(window: *Window, width: u32, height: u32) void {
    window.resize(width, height);
}

export fn window_set_visible(window: *Window, visible: bool) void {
    window.setVisible(visible);
}

export fn window_set_focused(window: *Window, focused: bool) void {
    window.setFocused(focused);
}

export fn window_set_cursor_visible(window: *Window, visible: bool) void {
    window.setCursorVisible(visible);
}

export fn window_set_cursor_locked(window: *Window, locked: bool) void {
    window.setCursorLocked(locked);
}

export fn window_set_cursor_icon(window: *Window, icon: CursorIcon) void {
    window.setCursorIcon(icon);
}

export fn window_set_cursor_position(window: *Window, x: f32, y: f32) void {
    window.setCursorPosition(x, y);
}

export fn window_set_scale_factor(window: *Window, factor: f32) void {
    window.setScaleFactor(factor);
}

export fn window_set_canvas_id(window: *Window, canvas_id: [*]const u8, canvas_id_len: u32) void {
    window.setCanvasId(canvas_id[0..canvas_id_len]);
}

export fn window_get_canvas_id(window: *const Window, out_buffer: [*]u8, buffer_len: u32) u32 {
    const canvas = window.getCanvasId();
    const len = @min(canvas.len, buffer_len);
    @memcpy(out_buffer[0..len], canvas[0..len]);
    return @intCast(len);
}

export fn window_get_width(window: *const Window) u32 {
    return window.width;
}

export fn window_get_height(window: *const Window) u32 {
    return window.height;
}

export fn window_get_scale_factor(window: *const Window) f32 {
    return window.scale_factor;
}

export fn window_is_visible(window: *const Window) bool {
    return window.visible;
}

export fn window_is_focused(window: *const Window) bool {
    return window.focused;
}

export fn window_is_cursor_visible(window: *const Window) bool {
    return window.cursor_visible;
}

export fn window_is_cursor_locked(window: *const Window) bool {
    return window.cursor_locked;
}

export fn window_get_cursor_icon(window: *const Window) CursorIcon {
    return window.cursor_icon;
}

export fn window_get_cursor_position_x(window: *const Window) f32 {
    return window.cursor_position_x;
}

export fn window_get_cursor_position_y(window: *const Window) f32 {
    return window.cursor_position_y;
}

// Tests

test "Window create" {
    const testing = std.testing;

    const title = "Test Window";
    const window = window_create(800, 600, title.ptr, title.len);

    try testing.expectEqual(@as(u32, 800), window.width);
    try testing.expectEqual(@as(u32, 600), window.height);
    try testing.expectEqualStrings(title, window.getTitle());
    try testing.expect(window.visible);
    try testing.expect(!window.focused);
}

test "Window set_title and get_title" {
    const testing = std.testing;

    var window = window_create(800, 600, "Initial".ptr, 7);

    const new_title = "Updated Title";
    window_set_title(&window, new_title.ptr, new_title.len);

    var buffer: [128]u8 = undefined;
    const len = window_get_title(&window, &buffer, buffer.len);

    try testing.expectEqual(@as(u32, new_title.len), len);
    try testing.expectEqualStrings(new_title, buffer[0..len]);
}

test "Window resize" {
    const testing = std.testing;

    var window = window_create(800, 600, "Test".ptr, 4);
    try testing.expectEqual(@as(u32, 800), window_get_width(&window));
    try testing.expectEqual(@as(u32, 600), window_get_height(&window));

    window_resize(&window, 1920, 1080);
    try testing.expectEqual(@as(u32, 1920), window_get_width(&window));
    try testing.expectEqual(@as(u32, 1080), window_get_height(&window));
}

test "Window visibility" {
    const testing = std.testing;

    var window = window_create(800, 600, "Test".ptr, 4);
    try testing.expect(window_is_visible(&window));

    window_set_visible(&window, false);
    try testing.expect(!window_is_visible(&window));

    window_set_visible(&window, true);
    try testing.expect(window_is_visible(&window));
}

test "Window focus" {
    const testing = std.testing;

    var window = window_create(800, 600, "Test".ptr, 4);
    try testing.expect(!window_is_focused(&window));

    window_set_focused(&window, true);
    try testing.expect(window_is_focused(&window));

    window_set_focused(&window, false);
    try testing.expect(!window_is_focused(&window));
}

test "Window cursor visibility" {
    const testing = std.testing;

    var window = window_create(800, 600, "Test".ptr, 4);
    try testing.expect(window_is_cursor_visible(&window));

    window_set_cursor_visible(&window, false);
    try testing.expect(!window_is_cursor_visible(&window));
}

test "Window cursor lock" {
    const testing = std.testing;

    var window = window_create(800, 600, "Test".ptr, 4);
    try testing.expect(!window_is_cursor_locked(&window));

    window_set_cursor_locked(&window, true);
    try testing.expect(window_is_cursor_locked(&window));
}

test "Window cursor icon" {
    const testing = std.testing;

    var window = window_create(800, 600, "Test".ptr, 4);
    try testing.expectEqual(CursorIcon.Default, window_get_cursor_icon(&window));

    window_set_cursor_icon(&window, CursorIcon.Pointer);
    try testing.expectEqual(CursorIcon.Pointer, window_get_cursor_icon(&window));
}

test "Window cursor position" {
    const testing = std.testing;

    var window = window_create(800, 600, "Test".ptr, 4);
    try testing.expectEqual(@as(f32, 0.0), window_get_cursor_position_x(&window));
    try testing.expectEqual(@as(f32, 0.0), window_get_cursor_position_y(&window));

    window_set_cursor_position(&window, 100.5, 200.5);
    try testing.expectEqual(@as(f32, 100.5), window_get_cursor_position_x(&window));
    try testing.expectEqual(@as(f32, 200.5), window_get_cursor_position_y(&window));
}

test "Window scale factor" {
    const testing = std.testing;

    var window = window_create(800, 600, "Test".ptr, 4);
    try testing.expectEqual(@as(f32, 1.0), window_get_scale_factor(&window));

    window_set_scale_factor(&window, 2.0);
    try testing.expectEqual(@as(f32, 2.0), window_get_scale_factor(&window));
}

test "Window canvas id" {
    const testing = std.testing;

    var window = window_create(800, 600, "Test".ptr, 4);

    const canvas = "my-canvas";
    window_set_canvas_id(&window, canvas.ptr, canvas.len);

    var buffer: [64]u8 = undefined;
    const len = window_get_canvas_id(&window, &buffer, buffer.len);

    try testing.expectEqual(@as(u32, canvas.len), len);
    try testing.expectEqualStrings(canvas, buffer[0..len]);
}
