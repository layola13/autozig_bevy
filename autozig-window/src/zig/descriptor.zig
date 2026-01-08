const std = @import("std");

/// WindowDescriptor - Configuration for creating a window
/// Uses fixed-size arrays to avoid heap allocation
pub const WindowDescriptor = extern struct {
    width: u32,
    height: u32,
    title: [128]u8,
    title_len: u32,
    resizable: bool,
    decorations: bool,
    transparent: bool,
    canvas_id: [64]u8,
    canvas_id_len: u32,

    pub fn init() WindowDescriptor {
        var desc = WindowDescriptor{
            .width = 800,
            .height = 600,
            .title = [_]u8{0} ** 128,
            .title_len = 0,
            .resizable = true,
            .decorations = true,
            .transparent = false,
            .canvas_id = [_]u8{0} ** 64,
            .canvas_id_len = 0,
        };

        // Set default title
        const default_title = "AutoZig Window";
        const title_len = @min(default_title.len, 128);
        @memcpy(desc.title[0..title_len], default_title[0..title_len]);
        desc.title_len = @intCast(title_len);

        return desc;
    }

    pub fn setTitle(self: *WindowDescriptor, title: []const u8) void {
        const len = @min(title.len, 128);
        @memcpy(self.title[0..len], title[0..len]);
        self.title_len = @intCast(len);
    }

    pub fn getTitle(self: *const WindowDescriptor) []const u8 {
        return self.title[0..self.title_len];
    }

    pub fn setSize(self: *WindowDescriptor, width: u32, height: u32) void {
        self.width = width;
        self.height = height;
    }

    pub fn setCanvas(self: *WindowDescriptor, canvas_id: []const u8) void {
        const len = @min(canvas_id.len, 64);
        @memcpy(self.canvas_id[0..len], canvas_id[0..len]);
        self.canvas_id_len = @intCast(len);
    }

    pub fn getCanvasId(self: *const WindowDescriptor) []const u8 {
        return self.canvas_id[0..self.canvas_id_len];
    }
};

// FFI exports for window descriptor

export fn window_descriptor_default() WindowDescriptor {
    return WindowDescriptor.init();
}

export fn window_descriptor_with_title(desc: WindowDescriptor, title: [*]const u8, title_len: u32) WindowDescriptor {
    var new_desc = desc;
    const len = @min(title_len, 128);
    @memcpy(new_desc.title[0..len], title[0..len]);
    new_desc.title_len = len;
    return new_desc;
}

export fn window_descriptor_with_size(desc: WindowDescriptor, width: u32, height: u32) WindowDescriptor {
    var new_desc = desc;
    new_desc.width = width;
    new_desc.height = height;
    return new_desc;
}

export fn window_descriptor_with_canvas(desc: WindowDescriptor, canvas_id: [*]const u8, canvas_id_len: u32) WindowDescriptor {
    var new_desc = desc;
    const len = @min(canvas_id_len, 64);
    @memcpy(new_desc.canvas_id[0..len], canvas_id[0..len]);
    new_desc.canvas_id_len = len;
    return new_desc;
}

export fn window_descriptor_set_resizable(desc: WindowDescriptor, resizable: bool) WindowDescriptor {
    var new_desc = desc;
    new_desc.resizable = resizable;
    return new_desc;
}

export fn window_descriptor_set_decorations(desc: WindowDescriptor, decorations: bool) WindowDescriptor {
    var new_desc = desc;
    new_desc.decorations = decorations;
    return new_desc;
}

export fn window_descriptor_set_transparent(desc: WindowDescriptor, transparent: bool) WindowDescriptor {
    var new_desc = desc;
    new_desc.transparent = transparent;
    return new_desc;
}

export fn window_descriptor_get_title(desc: *const WindowDescriptor, out_buffer: [*]u8, buffer_len: u32) u32 {
    const len = @min(desc.title_len, buffer_len);
    @memcpy(out_buffer[0..len], desc.title[0..len]);
    return len;
}

export fn window_descriptor_get_canvas_id(desc: *const WindowDescriptor, out_buffer: [*]u8, buffer_len: u32) u32 {
    const len = @min(desc.canvas_id_len, buffer_len);
    @memcpy(out_buffer[0..len], desc.canvas_id[0..len]);
    return len;
}

// Tests

test "WindowDescriptor default" {
    const testing = std.testing;

    const desc = window_descriptor_default();
    try testing.expectEqual(@as(u32, 800), desc.width);
    try testing.expectEqual(@as(u32, 600), desc.height);
    try testing.expect(desc.resizable);
    try testing.expect(desc.decorations);
    try testing.expect(!desc.transparent);

    const title = desc.getTitle();
    try testing.expectEqualStrings("AutoZig Window", title);
}

test "WindowDescriptor with_title" {
    const testing = std.testing;

    const desc = window_descriptor_default();
    const title = "My Custom Window";
    const new_desc = window_descriptor_with_title(desc, title.ptr, title.len);

    try testing.expectEqual(@as(u32, title.len), new_desc.title_len);
    try testing.expectEqualStrings(title, new_desc.getTitle());
}

test "WindowDescriptor with_size" {
    const testing = std.testing;

    const desc = window_descriptor_default();
    const new_desc = window_descriptor_with_size(desc, 1920, 1080);

    try testing.expectEqual(@as(u32, 1920), new_desc.width);
    try testing.expectEqual(@as(u32, 1080), new_desc.height);
}

test "WindowDescriptor with_canvas" {
    const testing = std.testing;

    const desc = window_descriptor_default();
    const canvas = "my-canvas-id";
    const new_desc = window_descriptor_with_canvas(desc, canvas.ptr, canvas.len);

    try testing.expectEqual(@as(u32, canvas.len), new_desc.canvas_id_len);
    try testing.expectEqualStrings(canvas, new_desc.getCanvasId());
}

test "WindowDescriptor builder pattern" {
    const testing = std.testing;

    var desc = window_descriptor_default();
    desc = window_descriptor_with_title(desc, "Test".ptr, 4);
    desc = window_descriptor_with_size(desc, 1024, 768);
    desc = window_descriptor_set_resizable(desc, false);
    desc = window_descriptor_set_decorations(desc, false);
    desc = window_descriptor_set_transparent(desc, true);

    try testing.expectEqualStrings("Test", desc.getTitle());
    try testing.expectEqual(@as(u32, 1024), desc.width);
    try testing.expectEqual(@as(u32, 768), desc.height);
    try testing.expect(!desc.resizable);
    try testing.expect(!desc.decorations);
    try testing.expect(desc.transparent);
}

test "WindowDescriptor get_title" {
    const testing = std.testing;

    const desc = window_descriptor_default();
    var buffer: [128]u8 = undefined;
    const len = window_descriptor_get_title(&desc, &buffer, buffer.len);

    try testing.expectEqual(@as(u32, 14), len);
    try testing.expectEqualStrings("AutoZig Window", buffer[0..len]);
}

test "WindowDescriptor get_canvas_id" {
    const testing = std.testing;

    var desc = window_descriptor_default();
    const canvas = "test-canvas";
    desc = window_descriptor_with_canvas(desc, canvas.ptr, canvas.len);

    var buffer: [64]u8 = undefined;
    const len = window_descriptor_get_canvas_id(&desc, &buffer, buffer.len);

    try testing.expectEqual(@as(u32, canvas.len), len);
    try testing.expectEqualStrings(canvas, buffer[0..len]);
}
