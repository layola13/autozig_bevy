const std = @import("std");

pub const CompassOctant = enum(u32) {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
};

export fn compass_octant_north() u32 {
    return @intFromEnum(CompassOctant.North);
}
