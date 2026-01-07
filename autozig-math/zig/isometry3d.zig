const std = @import("std");

pub const Isometry3d = extern struct {
    rotation: Quat,
    translation: Vec3,

    // Quat.IDENTITY is {0, 0, 0, 1}
    pub const IDENTITY = Isometry3d{
        .rotation = .{ .x = 0.0, .y = 0.0, .z = 0.0, .w = 1.0 },
        .translation = .{ .x = 0.0, .y = 0.0, .z = 0.0 },
    };

    pub fn new(translation: Vec3, rotation: Quat) Isometry3d {
        return .{
            .translation = translation,
            .rotation = rotation,
        };
    }
};

export fn isometry3d_identity() Isometry3d {
    return Isometry3d.IDENTITY;
}

export fn isometry3d_new(translation: Vec3, rotation: Quat) Isometry3d {
    return Isometry3d.new(translation, rotation);
}
