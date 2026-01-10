const std = @import("std");
const Vec2 = @import("vec2.zig").Vec2;
// Vec2 is available globally via autozig merge

pub const Rect = extern struct {
    min: Vec2,
    max: Vec2,

    pub fn new(min: Vec2, max: Vec2) Rect {
        return .{
            .min = min,
            .max = max,
        };
    }

    pub fn width(self: Rect) f32 {
        return self.max.x - self.min.x;
    }

    pub fn height(self: Rect) f32 {
        return self.max.y - self.min.y;
    }
};

export fn rect_new(min: Vec2, max: Vec2) Rect {
    return Rect.new(min, max);
}

export fn rect_width(self: Rect) f32 {
    return self.width();
}

export fn rect_height(self: Rect) f32 {
    return self.height();
}
