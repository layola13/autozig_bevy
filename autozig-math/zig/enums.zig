const std = @import("std");

pub const CompassQuadrant = enum(u32) {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
};

pub const EulerRot = enum(u32) {
    ZYX = 0,
    ZXY = 1,
    YXZ = 2,
    YZX = 3,
    XYZ = 4,
    XZY = 5,
};

pub const InvalidDirectionError = enum(u32) {
    Zero = 0,
    NaN = 1,
    Infinite = 2,
};
