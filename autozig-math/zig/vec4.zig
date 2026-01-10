const std = @import("std");
const Vec3 = @import("vec3.zig").Vec3;

pub const Vec4 = extern struct {
    x: f32,
    y: f32,
    z: f32,
    w: f32,

    pub const ZERO = Vec4{ .x = 0.0, .y = 0.0, .z = 0.0, .w = 0.0 };
    pub const ONE = Vec4{ .x = 1.0, .y = 1.0, .z = 1.0, .w = 1.0 };
    pub const X = Vec4{ .x = 1.0, .y = 0.0, .z = 0.0, .w = 0.0 };
    pub const Y = Vec4{ .x = 0.0, .y = 1.0, .z = 0.0, .w = 0.0 };
    pub const Z = Vec4{ .x = 0.0, .y = 0.0, .z = 1.0, .w = 0.0 };
    pub const W = Vec4{ .x = 0.0, .y = 0.0, .z = 0.0, .w = 1.0 };

    pub fn new(x: f32, y: f32, z: f32, w: f32) Vec4 {
        return .{ .x = x, .y = y, .z = z, .w = w };
    }

    pub fn splat(value: f32) Vec4 {
        return .{ .x = value, .y = value, .z = value, .w = value };
    }

    pub fn dot(self: Vec4, other: Vec4) f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
    }

    pub fn add(self: Vec4, other: Vec4) Vec4 {
        return .{
            .x = self.x + other.x,
            .y = self.y + other.y,
            .z = self.z + other.z,
            .w = self.w + other.w,
        };
    }

    pub fn sub(self: Vec4, other: Vec4) Vec4 {
        return .{
            .x = self.x - other.x,
            .y = self.y - other.y,
            .z = self.z - other.z,
            .w = self.w - other.w,
        };
    }

    pub fn mul(self: Vec4, other: Vec4) Vec4 {
        return .{
            .x = self.x * other.x,
            .y = self.y * other.y,
            .z = self.z * other.z,
            .w = self.w * other.w,
        };
    }

    pub fn mul_scalar(self: Vec4, s: f32) Vec4 {
        return .{
            .x = self.x * s,
            .y = self.y * s,
            .z = self.z * s,
            .w = self.w * s,
        };
    }

    pub fn div(self: Vec4, other: Vec4) Vec4 {
        return .{
            .x = self.x / other.x,
            .y = self.y / other.y,
            .z = self.z / other.z,
            .w = self.w / other.w,
        };
    }

    pub fn div_scalar(self: Vec4, s: f32) Vec4 {
        return .{
            .x = self.x / s,
            .y = self.y / s,
            .z = self.z / s,
            .w = self.w / s,
        };
    }

    pub fn length_squared(self: Vec4) f32 {
        return self.dot(self);
    }

    pub fn length(self: Vec4) f32 {
        return @sqrt(self.length_squared());
    }

    pub fn normalize(self: Vec4) Vec4 {
        const len = self.length();
        if (len == 0) return ZERO;
        return self.mul_scalar(1.0 / len);
    }

    pub fn distance(self: Vec4, other: Vec4) f32 {
        return self.sub(other).length();
    }

    pub fn lerp(self: Vec4, other: Vec4, t: f32) Vec4 {
        return .{
            .x = self.x + (other.x - self.x) * t,
            .y = self.y + (other.y - self.y) * t,
            .z = self.z + (other.z - self.z) * t,
            .w = self.w + (other.w - self.w) * t,
        };
    }

    pub fn min(self: Vec4, other: Vec4) Vec4 {
        return .{
            .x = @min(self.x, other.x),
            .y = @min(self.y, other.y),
            .z = @min(self.z, other.z),
            .w = @min(self.w, other.w),
        };
    }

    pub fn max(self: Vec4, other: Vec4) Vec4 {
        return .{
            .x = @max(self.x, other.x),
            .y = @max(self.y, other.y),
            .z = @max(self.z, other.z),
            .w = @max(self.w, other.w),
        };
    }

    pub fn abs(self: Vec4) Vec4 {
        return .{
            .x = @abs(self.x),
            .y = @abs(self.y),
            .z = @abs(self.z),
            .w = @abs(self.w),
        };
    }

    pub fn neg(self: Vec4) Vec4 {
        return .{ .x = -self.x, .y = -self.y, .z = -self.z, .w = -self.w };
    }

    /// Truncate to Vec3
    pub fn truncate(self: Vec4) Vec3 {
        return Vec3{ .x = self.x, .y = self.y, .z = self.z };
    }
};

export fn vec4_new(x: f32, y: f32, z: f32, w: f32) Vec4 {
    return Vec4.new(x, y, z, w);
}

export fn vec4_splat(value: f32) Vec4 {
    return Vec4.splat(value);
}

export fn vec4_dot(self: Vec4, other: Vec4) f32 {
    return self.dot(other);
}

export fn vec4_add(self: Vec4, other: Vec4) Vec4 {
    return self.add(other);
}

export fn vec4_sub(self: Vec4, other: Vec4) Vec4 {
    return self.sub(other);
}

export fn vec4_mul_scalar(self: Vec4, s: f32) Vec4 {
    return self.mul_scalar(s);
}

export fn vec4_length(self: Vec4) f32 {
    return self.length();
}

export fn vec4_length_squared(self: Vec4) f32 {
    return self.length_squared();
}

export fn vec4_normalize(self: Vec4) Vec4 {
    return self.normalize();
}

export fn vec4_lerp(self: Vec4, other: Vec4, t: f32) Vec4 {
    return self.lerp(other, t);
}

export fn vec4_min(self: Vec4, other: Vec4) Vec4 {
    return self.min(other);
}

export fn vec4_max(self: Vec4, other: Vec4) Vec4 {
    return self.max(other);
}

export fn vec4_abs(self: Vec4) Vec4 {
    return self.abs();
}

export fn vec4_truncate(self: Vec4) Vec3 {
    return self.truncate();
}
