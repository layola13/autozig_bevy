const std = @import("std");

pub const IVec3 = extern struct {
    x: i32,
    y: i32,
    z: i32,

    pub fn new(x: i32, y: i32, z: i32) IVec3 {
        return .{
            .x = x,
            .y = y,
            .z = z,
        };
    }
};

export fn ivec3_new(x: i32, y: i32, z: i32) IVec3 {
    return IVec3.new(x, y, z);
}
