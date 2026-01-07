const std = @import("std");
const math = std.math;

pub const Vec3 = extern struct {
    x: f32,
    y: f32,
    z: f32,

    pub const ZERO = Vec3{ .x = 0.0, .y = 0.0, .z = 0.0 };
    pub const ONE = Vec3{ .x = 1.0, .y = 1.0, .z = 1.0 };
    pub const X = Vec3{ .x = 1.0, .y = 0.0, .z = 0.0 };
    pub const Y = Vec3{ .x = 0.0, .y = 1.0, .z = 0.0 };
    pub const Z = Vec3{ .x = 0.0, .y = 0.0, .z = 1.0 };
    pub const NEG_X = Vec3{ .x = -1.0, .y = 0.0, .z = 0.0 };
    pub const NEG_Y = Vec3{ .x = 0.0, .y = -1.0, .z = 0.0 };
    pub const NEG_Z = Vec3{ .x = 0.0, .y = 0.0, .z = -1.0 };

    pub fn new(x: f32, y: f32, z: f32) Vec3 {
        return .{ .x = x, .y = y, .z = z };
    }

    pub fn splat(value: f32) Vec3 {
        return .{ .x = value, .y = value, .z = value };
    }

    pub fn dot(self: Vec3, other: Vec3) f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(self: Vec3, other: Vec3) Vec3 {
        return .{
            .x = self.y * other.z - self.z * other.y,
            .y = self.z * other.x - self.x * other.z,
            .z = self.x * other.y - self.y * other.x,
        };
    }

    pub fn add(self: Vec3, other: Vec3) Vec3 {
        return .{
            .x = self.x + other.x,
            .y = self.y + other.y,
            .z = self.z + other.z,
        };
    }

    pub fn sub(self: Vec3, other: Vec3) Vec3 {
        return .{
            .x = self.x - other.x,
            .y = self.y - other.y,
            .z = self.z - other.z,
        };
    }

    pub fn mul(self: Vec3, other: Vec3) Vec3 {
        return .{
            .x = self.x * other.x,
            .y = self.y * other.y,
            .z = self.z * other.z,
        };
    }

    pub fn mul_scalar(self: Vec3, s: f32) Vec3 {
        return .{
            .x = self.x * s,
            .y = self.y * s,
            .z = self.z * s,
        };
    }

    pub fn div(self: Vec3, other: Vec3) Vec3 {
        return .{
            .x = self.x / other.x,
            .y = self.y / other.y,
            .z = self.z / other.z,
        };
    }

    pub fn div_scalar(self: Vec3, s: f32) Vec3 {
        return .{
            .x = self.x / s,
            .y = self.y / s,
            .z = self.z / s,
        };
    }

    pub fn length_squared(self: Vec3) f32 {
        return self.dot(self);
    }

    pub fn length(self: Vec3) f32 {
        return @sqrt(self.length_squared());
    }

    pub fn normalize(self: Vec3) Vec3 {
        const len = self.length();
        if (len == 0) return ZERO;
        return self.mul_scalar(1.0 / len);
    }

    pub fn normalize_or_zero(self: Vec3) Vec3 {
        const len = self.length();
        if (len > std.math.floatEps(f32)) {
            return self.mul_scalar(1.0 / len);
        }
        return ZERO;
    }

    pub fn distance(self: Vec3, other: Vec3) f32 {
        return self.sub(other).length();
    }

    pub fn distance_squared(self: Vec3, other: Vec3) f32 {
        return self.sub(other).length_squared();
    }

    pub fn lerp(self: Vec3, other: Vec3, t: f32) Vec3 {
        return .{
            .x = self.x + (other.x - self.x) * t,
            .y = self.y + (other.y - self.y) * t,
            .z = self.z + (other.z - self.z) * t,
        };
    }

    pub fn min(self: Vec3, other: Vec3) Vec3 {
        return .{
            .x = @min(self.x, other.x),
            .y = @min(self.y, other.y),
            .z = @min(self.z, other.z),
        };
    }

    pub fn max(self: Vec3, other: Vec3) Vec3 {
        return .{
            .x = @max(self.x, other.x),
            .y = @max(self.y, other.y),
            .z = @max(self.z, other.z),
        };
    }

    pub fn clamp(self: Vec3, min_v: Vec3, max_v: Vec3) Vec3 {
        return self.max(min_v).min(max_v);
    }

    pub fn abs(self: Vec3) Vec3 {
        return .{
            .x = @abs(self.x),
            .y = @abs(self.y),
            .z = @abs(self.z),
        };
    }

    pub fn neg(self: Vec3) Vec3 {
        return .{ .x = -self.x, .y = -self.y, .z = -self.z };
    }

    /// Reflect off a normal vector
    pub fn reflect(self: Vec3, normal: Vec3) Vec3 {
        const d = 2.0 * self.dot(normal);
        return self.sub(normal.mul_scalar(d));
    }

    /// Project self onto other
    pub fn project_onto(self: Vec3, other: Vec3) Vec3 {
        const len_sq = other.length_squared();
        if (len_sq == 0) return ZERO;
        return other.mul_scalar(self.dot(other) / len_sq);
    }

    /// Reject from other (component perpendicular to other)
    pub fn reject_from(self: Vec3, other: Vec3) Vec3 {
        return self.sub(self.project_onto(other));
    }

    /// Angle between two vectors in radians
    pub fn angle_between(self: Vec3, other: Vec3) f32 {
        const denom = @sqrt(self.length_squared() * other.length_squared());
        if (denom == 0) return 0;
        const cos_theta = @min(1.0, @max(-1.0, self.dot(other) / denom));
        return std.math.acos(cos_theta);
    }

    /// Any orthogonal vector (not unique)
    pub fn any_orthogonal_vector(self: Vec3) Vec3 {
        // Hughes-Moller method
        if (@abs(self.x) > @abs(self.z)) {
            const v = Vec3{ .x = -self.y, .y = self.x, .z = 0.0 };
            return v.normalize();
        } else {
            const v = Vec3{ .x = 0.0, .y = -self.z, .z = self.y };
            return v.normalize();
        }
    }
};

export fn vec3_new(x: f32, y: f32, z: f32) Vec3 {
    return Vec3.new(x, y, z);
}

export fn vec3_splat(value: f32) Vec3 {
    return Vec3.splat(value);
}

export fn vec3_dot(self: Vec3, other: Vec3) f32 {
    return self.dot(other);
}

export fn vec3_length_squared(self: Vec3) f32 {
    return self.length_squared();
}

export fn vec3_length(self: Vec3) f32 {
    return self.length();
}

export fn vec3_cross(self: Vec3, other: Vec3) Vec3 {
    return self.cross(other);
}

export fn vec3_add(self: Vec3, other: Vec3) Vec3 {
    return self.add(other);
}

export fn vec3_sub(self: Vec3, other: Vec3) Vec3 {
    return self.sub(other);
}

export fn vec3_mul_scalar(self: Vec3, s: f32) Vec3 {
    return self.mul_scalar(s);
}

export fn vec3_normalize(self: Vec3) Vec3 {
    return self.normalize();
}

export fn vec3_distance(self: Vec3, other: Vec3) f32 {
    return self.distance(other);
}

export fn vec3_lerp(self: Vec3, other: Vec3, t: f32) Vec3 {
    return self.lerp(other, t);
}

export fn vec3_min(self: Vec3, other: Vec3) Vec3 {
    return self.min(other);
}

export fn vec3_max(self: Vec3, other: Vec3) Vec3 {
    return self.max(other);
}

export fn vec3_abs(self: Vec3) Vec3 {
    return self.abs();
}

export fn vec3_reflect(self: Vec3, normal: Vec3) Vec3 {
    return self.reflect(normal);
}

export fn vec3_project_onto(self: Vec3, other: Vec3) Vec3 {
    return self.project_onto(other);
}

export fn vec3_angle_between(self: Vec3, other: Vec3) f32 {
    return self.angle_between(other);
}

export fn vec3_any_orthogonal_vector(self: Vec3) Vec3 {
    return self.any_orthogonal_vector();
}
