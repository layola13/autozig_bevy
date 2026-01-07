const std = @import("std");

pub const UVec2 = extern struct {
    x: u32,
    y: u32,

    pub fn new(x: u32, y: u32) UVec2 {
        return .{
            .x = x,
            .y = y,
        };
    }
};

export fn uvec2_new(x: u32, y: u32) UVec2 {
    return UVec2.new(x, y);
}
