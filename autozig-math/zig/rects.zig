const std = @import("std");
const IVec2 = @import("ivec2.zig").IVec2;
const UVec2 = @import("uvec2.zig").UVec2;

pub const IRect = extern struct {
    min: IVec2,
    max: IVec2,
    pub fn new(min: IVec2, max: IVec2) IRect {
        return .{ .min = min, .max = max };
    }
    pub fn width(self: IRect) i32 {
        return self.max.x - self.min.x;
    }
    pub fn height(self: IRect) i32 {
        return self.max.y - self.min.y;
    }
};

pub const URect = extern struct {
    min: UVec2,
    max: UVec2,
    pub fn new(min: UVec2, max: UVec2) URect {
        return .{ .min = min, .max = max };
    }
    pub fn width(self: URect) u32 {
        return self.max.x - self.min.x;
    }
    pub fn height(self: URect) u32 {
        return self.max.y - self.min.y;
    }
};

export fn irect_new(min: IVec2, max: IVec2) IRect {
    return IRect.new(min, max);
}
export fn irect_width(self: IRect) i32 {
    return self.width();
}
export fn irect_height(self: IRect) i32 {
    return self.height();
}

export fn urect_new(min: UVec2, max: UVec2) URect {
    return URect.new(min, max);
}
export fn urect_width(self: URect) u32 {
    return self.width();
}
export fn urect_height(self: URect) u32 {
    return self.height();
}
