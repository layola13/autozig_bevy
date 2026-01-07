const std = @import("std");

pub const UVec3 = extern struct {
    x: u32,
    y: u32,
    z: u32,

    pub fn new(x: u32, y: u32, z: u32) UVec3 {
        return .{
            .x = x,
            .y = y,
            .z = z,
        };
    }
};

export fn uvec3_new(x: u32, y: u32, z: u32) UVec3 {
    return UVec3.new(x, y, z);
}
