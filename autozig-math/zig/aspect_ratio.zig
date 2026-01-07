const std = @import("std");

pub const AspectRatio = extern struct {
    ratio: f32,

    pub fn new(w: f32, h: f32) AspectRatio {
        return .{ .ratio = w / h };
    }

    pub fn width(self: AspectRatio, h: f32) f32 {
        return h * self.ratio;
    }

    pub fn height(self: AspectRatio, w: f32) f32 {
        return w / self.ratio;
    }
};

export fn aspect_ratio_new(width: f32, height: f32) AspectRatio {
    return AspectRatio.new(width, height);
}

export fn aspect_ratio_width(self: AspectRatio, height: f32) f32 {
    return self.width(height);
}

export fn aspect_ratio_height(self: AspectRatio, width: f32) f32 {
    return self.height(width);
}
