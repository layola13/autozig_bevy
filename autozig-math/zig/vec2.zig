const std = @import("std");

pub const Vec2 = extern struct {
    x: f32,
    y: f32,

    pub const ZERO = Vec2{ .x = 0.0, .y = 0.0 };
    pub const ONE = Vec2{ .x = 1.0, .y = 1.0 };
    pub const X = Vec2{ .x = 1.0, .y = 0.0 };
    pub const Y = Vec2{ .x = 0.0, .y = 1.0 };
    pub const NEG_X = Vec2{ .x = -1.0, .y = 0.0 };
    pub const NEG_Y = Vec2{ .x = 0.0, .y = -1.0 };

    pub fn new(x: f32, y: f32) Vec2 {
        return .{ .x = x, .y = y };
    }

    pub fn splat(value: f32) Vec2 {
        return .{ .x = value, .y = value };
    }

    pub fn dot(self: Vec2, other: Vec2) f32 {
        return self.x * other.x + self.y * other.y;
    }

    pub fn add(self: Vec2, other: Vec2) Vec2 {
        return .{ .x = self.x + other.x, .y = self.y + other.y };
    }

    pub fn sub(self: Vec2, other: Vec2) Vec2 {
        return .{ .x = self.x - other.x, .y = self.y - other.y };
    }

    pub fn mul(self: Vec2, other: Vec2) Vec2 {
        return .{ .x = self.x * other.x, .y = self.y * other.y };
    }

    pub fn mul_scalar(self: Vec2, s: f32) Vec2 {
        return .{ .x = self.x * s, .y = self.y * s };
    }

    pub fn div(self: Vec2, other: Vec2) Vec2 {
        return .{ .x = self.x / other.x, .y = self.y / other.y };
    }

    pub fn div_scalar(self: Vec2, s: f32) Vec2 {
        return .{ .x = self.x / s, .y = self.y / s };
    }

    pub fn length_squared(self: Vec2) f32 {
        return self.dot(self);
    }

    pub fn length(self: Vec2) f32 {
        return @sqrt(self.length_squared());
    }

    pub fn normalize(self: Vec2) Vec2 {
        const len = self.length();
        if (len == 0) return ZERO;
        return self.mul_scalar(1.0 / len);
    }

    pub fn normalize_or_zero(self: Vec2) Vec2 {
        const len = self.length();
        if (len > std.math.floatEps(f32)) {
            return self.mul_scalar(1.0 / len);
        }
        return ZERO;
    }

    pub fn distance(self: Vec2, other: Vec2) f32 {
        return self.sub(other).length();
    }

    pub fn distance_squared(self: Vec2, other: Vec2) f32 {
        return self.sub(other).length_squared();
    }

    pub fn lerp(self: Vec2, other: Vec2, t: f32) Vec2 {
        return .{
            .x = self.x + (other.x - self.x) * t,
            .y = self.y + (other.y - self.y) * t,
        };
    }

    pub fn min(self: Vec2, other: Vec2) Vec2 {
        return .{
            .x = @min(self.x, other.x),
            .y = @min(self.y, other.y),
        };
    }

    pub fn max(self: Vec2, other: Vec2) Vec2 {
        return .{
            .x = @max(self.x, other.x),
            .y = @max(self.y, other.y),
        };
    }

    pub fn clamp(self: Vec2, min_v: Vec2, max_v: Vec2) Vec2 {
        return self.max(min_v).min(max_v);
    }

    pub fn abs(self: Vec2) Vec2 {
        return .{
            .x = @abs(self.x),
            .y = @abs(self.y),
        };
    }

    pub fn neg(self: Vec2) Vec2 {
        return .{ .x = -self.x, .y = -self.y };
    }

    /// Perpendicular vector (rotated 90 degrees counter-clockwise)
    pub fn perp(self: Vec2) Vec2 {
        return .{ .x = -self.y, .y = self.x };
    }

    /// Perpendicular dot product (2D cross product analog)
    pub fn perp_dot(self: Vec2, other: Vec2) f32 {
        return self.x * other.y - self.y * other.x;
    }

    /// Rotate by angle in radians
    pub fn rotate(self: Vec2, angle: f32) Vec2 {
        const c = @cos(angle);
        const s = @sin(angle);
        return .{
            .x = self.x * c - self.y * s,
            .y = self.x * s + self.y * c,
        };
    }

    /// Reflect off a normal vector
    pub fn reflect(self: Vec2, normal: Vec2) Vec2 {
        const d = 2.0 * self.dot(normal);
        return self.sub(normal.mul_scalar(d));
    }

    /// Angle between two vectors in radians
    pub fn angle_between(self: Vec2, other: Vec2) f32 {
        const denom = @sqrt(self.length_squared() * other.length_squared());
        if (denom == 0) return 0;
        const cos_theta = @min(1.0, @max(-1.0, self.dot(other) / denom));
        return std.math.acos(cos_theta);
    }

    /// Project self onto other
    pub fn project_onto(self: Vec2, other: Vec2) Vec2 {
        const len_sq = other.length_squared();
        if (len_sq == 0) return ZERO;
        return other.mul_scalar(self.dot(other) / len_sq);
    }
};

export fn vec2_new(x: f32, y: f32) Vec2 {
    return Vec2.new(x, y);
}

export fn vec2_splat(value: f32) Vec2 {
    return Vec2.splat(value);
}

export fn vec2_dot(self: Vec2, other: Vec2) f32 {
    return self.dot(other);
}

export fn vec2_add(self: Vec2, other: Vec2) Vec2 {
    return self.add(other);
}

export fn vec2_sub(self: Vec2, other: Vec2) Vec2 {
    return self.sub(other);
}

export fn vec2_mul_scalar(self: Vec2, s: f32) Vec2 {
    return self.mul_scalar(s);
}

export fn vec2_length(self: Vec2) f32 {
    return self.length();
}

export fn vec2_length_squared(self: Vec2) f32 {
    return self.length_squared();
}

export fn vec2_normalize(self: Vec2) Vec2 {
    return self.normalize();
}

export fn vec2_distance(self: Vec2, other: Vec2) f32 {
    return self.distance(other);
}

export fn vec2_lerp(self: Vec2, other: Vec2, t: f32) Vec2 {
    return self.lerp(other, t);
}

export fn vec2_min(self: Vec2, other: Vec2) Vec2 {
    return self.min(other);
}

export fn vec2_max(self: Vec2, other: Vec2) Vec2 {
    return self.max(other);
}

export fn vec2_abs(self: Vec2) Vec2 {
    return self.abs();
}

export fn vec2_perp(self: Vec2) Vec2 {
    return self.perp();
}

export fn vec2_perp_dot(self: Vec2, other: Vec2) f32 {
    return self.perp_dot(other);
}

export fn vec2_rotate(self: Vec2, angle: f32) Vec2 {
    return self.rotate(angle);
}

export fn vec2_reflect(self: Vec2, normal: Vec2) Vec2 {
    return self.reflect(normal);
}

export fn vec2_angle_between(self: Vec2, other: Vec2) f32 {
    return self.angle_between(other);
}

export fn vec2_project_onto(self: Vec2, other: Vec2) Vec2 {
    return self.project_onto(other);
}
