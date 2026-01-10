const std = @import("std");
const Vec2 = @import("vec2.zig").Vec2;

pub const Dir2 = extern struct {
    vec: Vec2,

    pub const X = Dir2{ .vec = Vec2{ .x = 1.0, .y = 0.0 } };
    pub const Y = Dir2{ .vec = Vec2{ .x = 0.0, .y = 1.0 } };
    pub const NEG_X = Dir2{ .vec = Vec2{ .x = -1.0, .y = 0.0 } };
    pub const NEG_Y = Dir2{ .vec = Vec2{ .x = 0.0, .y = -1.0 } };

    pub fn new(x: f32, y: f32) Dir2 {
        const len = @sqrt(x * x + y * y);
        if (len == 0) return X;
        return .{
            .vec = .{ .x = x / len, .y = y / len },
        };
    }

    pub fn from_vec2(v: Vec2) Dir2 {
        return new(v.x, v.y);
    }

    /// Rotate by 90 degrees counter-clockwise
    pub fn perp(self: Dir2) Dir2 {
        return .{ .vec = .{ .x = -self.vec.y, .y = self.vec.x } };
    }

    /// Rotate by angle in radians
    pub fn rotate(self: Dir2, angle: f32) Dir2 {
        const c = @cos(angle);
        const s = @sin(angle);
        return .{
            .vec = .{
                .x = self.vec.x * c - self.vec.y * s,
                .y = self.vec.x * s + self.vec.y * c,
            },
        };
    }

    /// Angle of direction from positive X axis
    pub fn to_angle(self: Dir2) f32 {
        return std.math.atan2(self.vec.y, self.vec.x);
    }

    /// Create direction from angle
    pub fn from_angle(angle: f32) Dir2 {
        return .{
            .vec = .{ .x = @cos(angle), .y = @sin(angle) },
        };
    }
};

export fn dir2_from_vec2(v: Vec2) Dir2 {
    return Dir2.from_vec2(v);
}

export fn dir2_new(x: f32, y: f32) Dir2 {
    return Dir2.new(x, y);
}

export fn dir2_perp(self: Dir2) Dir2 {
    return self.perp();
}

export fn dir2_rotate(self: Dir2, angle: f32) Dir2 {
    return self.rotate(angle);
}

export fn dir2_to_angle(self: Dir2) f32 {
    return self.to_angle();
}

export fn dir2_from_angle(angle: f32) Dir2 {
    return Dir2.from_angle(angle);
}
