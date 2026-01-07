const std = @import("std");

pub const IVec2 = extern struct {
    x: i32,
    y: i32,

    pub fn new(x: i32, y: i32) IVec2 {
        return .{
            .x = x,
            .y = y,
        };
    }
};

export fn ivec2_new(x: i32, y: i32) IVec2 {
    return IVec2.new(x, y);
}
