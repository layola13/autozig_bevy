const std = @import("std");
const Vec2 = @import("vec2.zig").Vec2;
const Rot2 = @import("rot2.zig").Rot2;

pub const Isometry2d = extern struct {
    rotation: Rot2,
    translation: Vec2,

    pub const IDENTITY = Isometry2d{
        .rotation = Rot2.IDENTITY,
        .translation = .{ .x = 0.0, .y = 0.0 },
    };

    pub fn new(translation: Vec2, rotation: Rot2) Isometry2d {
        return .{
            .translation = translation,
            .rotation = rotation,
        };
    }

    pub fn from_translation(translation: Vec2) Isometry2d {
        return .{
            .translation = translation,
            .rotation = Rot2.IDENTITY,
        };
    }

    pub fn from_rotation(rotation: Rot2) Isometry2d {
        return .{
            .translation = .{ .x = 0.0, .y = 0.0 },
            .rotation = rotation,
        };
    }
};

export fn isometry2d_identity() Isometry2d {
    return Isometry2d.IDENTITY;
}

export fn isometry2d_new(translation: Vec2, rotation: Rot2) Isometry2d {
    return Isometry2d.new(translation, rotation);
}
