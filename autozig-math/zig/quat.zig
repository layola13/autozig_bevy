const std = @import("std");

pub const Quat = extern struct {
    x: f32,
    y: f32,
    z: f32,
    w: f32,

    pub const IDENTITY = Quat{ .x = 0.0, .y = 0.0, .z = 0.0, .w = 1.0 };

    pub fn identity() Quat {
        return IDENTITY;
    }

    pub fn from_xyzw(x: f32, y: f32, z: f32, w: f32) Quat {
        return .{ .x = x, .y = y, .z = z, .w = w };
    }

    pub fn from_axis_angle(axis: Vec3, angle: f32) Quat {
        const half_angle = angle * 0.5;
        const s = std.math.sin(half_angle);
        const c = std.math.cos(half_angle);
        return .{
            .x = axis.x * s,
            .y = axis.y * s,
            .z = axis.z * s,
            .w = c,
        };
    }

    pub fn from_rotation_x(angle: f32) Quat {
        const half = angle * 0.5;
        return .{
            .x = @sin(half),
            .y = 0.0,
            .z = 0.0,
            .w = @cos(half),
        };
    }

    pub fn from_rotation_y(angle: f32) Quat {
        const half = angle * 0.5;
        return .{
            .x = 0.0,
            .y = @sin(half),
            .z = 0.0,
            .w = @cos(half),
        };
    }

    pub fn from_rotation_z(angle: f32) Quat {
        const half = angle * 0.5;
        return .{
            .x = 0.0,
            .y = 0.0,
            .z = @sin(half),
            .w = @cos(half),
        };
    }

    pub fn mul(self: Quat, other: Quat) Quat {
        return .{
            .x = self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            .y = self.w * other.y + self.y * other.w + self.z * other.x - self.x * other.z,
            .z = self.w * other.z + self.z * other.w + self.x * other.y - self.y * other.x,
            .w = self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        };
    }

    pub fn conjugate(self: Quat) Quat {
        return .{ .x = -self.x, .y = -self.y, .z = -self.z, .w = self.w };
    }

    pub fn inverse(self: Quat) Quat {
        const len_sq = self.length_squared();
        if (len_sq == 0) return IDENTITY;
        const inv = 1.0 / len_sq;
        return .{
            .x = -self.x * inv,
            .y = -self.y * inv,
            .z = -self.z * inv,
            .w = self.w * inv,
        };
    }

    pub fn length_squared(self: Quat) f32 {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
    }

    pub fn length(self: Quat) f32 {
        return @sqrt(self.length_squared());
    }

    pub fn normalize(self: Quat) Quat {
        const len = self.length();
        if (len == 0) return IDENTITY;
        const inv = 1.0 / len;
        return .{
            .x = self.x * inv,
            .y = self.y * inv,
            .z = self.z * inv,
            .w = self.w * inv,
        };
    }

    pub fn is_normalized(self: Quat) bool {
        return @abs(self.length_squared() - 1.0) < 1e-5;
    }

    pub fn dot(self: Quat, other: Quat) f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
    }

    /// Rotate a Vec3 by this quaternion
    pub fn mul_vec3(self: Quat, v: Vec3) Vec3 {
        // q * v * q^-1
        const u = Vec3{ .x = self.x, .y = self.y, .z = self.z };
        const s = self.w;

        // 2 * (u . v) * u
        const d = 2.0 * u.dot(v);
        const term1 = u.mul_scalar(d);

        // (s*s - u.u) * v
        const ss = s * s - u.dot(u);
        const term2 = v.mul_scalar(ss);

        // 2*s * (u x v)
        const c = u.cross(v);
        const term3 = c.mul_scalar(2.0 * s);

        return term1.add(term2).add(term3);
    }

    /// Linear interpolation (not normalized)
    pub fn lerp(self: Quat, other: Quat, t: f32) Quat {
        return .{
            .x = self.x + (other.x - self.x) * t,
            .y = self.y + (other.y - self.y) * t,
            .z = self.z + (other.z - self.z) * t,
            .w = self.w + (other.w - self.w) * t,
        };
    }

    /// Spherical linear interpolation
    pub fn slerp(self: Quat, other_in: Quat, t: f32) Quat {
        var other = other_in;
        var d = self.dot(other);

        // If dot is negative, negate one quaternion to take shorter path
        if (d < 0.0) {
            other = .{ .x = -other.x, .y = -other.y, .z = -other.z, .w = -other.w };
            d = -d;
        }

        // If quaternions are very close, use lerp
        if (d > 0.9995) {
            return self.lerp(other, t).normalize();
        }

        const theta_0 = std.math.acos(d);
        const theta = theta_0 * t;
        const sin_theta = @sin(theta);
        const sin_theta_0 = @sin(theta_0);

        const s0 = @cos(theta) - d * sin_theta / sin_theta_0;
        const s1 = sin_theta / sin_theta_0;

        return .{
            .x = self.x * s0 + other.x * s1,
            .y = self.y * s0 + other.y * s1,
            .z = self.z * s0 + other.z * s1,
            .w = self.w * s0 + other.w * s1,
        };
    }

    /// Get the angle part of axis-angle representation
    pub fn to_angle(self: Quat) f32 {
        return 2.0 * std.math.acos(@min(1.0, @max(-1.0, self.w)));
    }

    /// Get the axis part of axis-angle representation
    pub fn to_axis(self: Quat) Vec3 {
        const sin_half = @sqrt(1.0 - self.w * self.w);
        if (sin_half < 1e-6) {
            return Vec3{ .x = 1.0, .y = 0.0, .z = 0.0 };
        }
        return Vec3{
            .x = self.x / sin_half,
            .y = self.y / sin_half,
            .z = self.z / sin_half,
        };
    }
};

export fn quat_identity() Quat {
    return Quat.identity();
}

export fn quat_from_xyzw(x: f32, y: f32, z: f32, w: f32) Quat {
    return Quat.from_xyzw(x, y, z, w);
}

export fn quat_from_axis_angle(axis: Vec3, angle: f32) Quat {
    return Quat.from_axis_angle(axis, angle);
}

export fn quat_from_rotation_x(angle: f32) Quat {
    return Quat.from_rotation_x(angle);
}

export fn quat_from_rotation_y(angle: f32) Quat {
    return Quat.from_rotation_y(angle);
}

export fn quat_from_rotation_z(angle: f32) Quat {
    return Quat.from_rotation_z(angle);
}

export fn quat_mul(self: Quat, other: Quat) Quat {
    return self.mul(other);
}

export fn quat_conjugate(self: Quat) Quat {
    return self.conjugate();
}

export fn quat_inverse(self: Quat) Quat {
    return self.inverse();
}

export fn quat_length(self: Quat) f32 {
    return self.length();
}

export fn quat_normalize(self: Quat) Quat {
    return self.normalize();
}

export fn quat_dot(self: Quat, other: Quat) f32 {
    return self.dot(other);
}

export fn quat_mul_vec3(self: Quat, v: Vec3) Vec3 {
    return self.mul_vec3(v);
}

export fn quat_lerp(self: Quat, other: Quat, t: f32) Quat {
    return self.lerp(other, t);
}

export fn quat_slerp(self: Quat, other: Quat, t: f32) Quat {
    return self.slerp(other, t);
}
