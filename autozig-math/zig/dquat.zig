const std = @import("std");

pub const DQuat = extern struct {
    x: f64,
    y: f64,
    z: f64,
    w: f64,

    pub const IDENTITY = DQuat{ .x = 0.0, .y = 0.0, .z = 0.0, .w = 1.0 };

    pub fn identity() DQuat {
        return IDENTITY;
    }

    pub fn from_xyzw(x: f64, y: f64, z: f64, w: f64) DQuat {
        return .{ .x = x, .y = y, .z = z, .w = w };
    }

    pub fn from_axis_angle(axis: DVec3, angle: f64) DQuat {
        const half_angle = angle * 0.5;
        const s = @sin(half_angle);
        const c = @cos(half_angle);
        return .{
            .x = axis.x * s,
            .y = axis.y * s,
            .z = axis.z * s,
            .w = c,
        };
    }

    pub fn mul(self: DQuat, other: DQuat) DQuat {
        return .{
            .x = self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            .y = self.w * other.y + self.y * other.w + self.z * other.x - self.x * other.z,
            .z = self.w * other.z + self.z * other.w + self.x * other.y - self.y * other.x,
            .w = self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        };
    }

    pub fn conjugate(self: DQuat) DQuat {
        return .{ .x = -self.x, .y = -self.y, .z = -self.z, .w = self.w };
    }

    pub fn length_squared(self: DQuat) f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
    }

    pub fn length(self: DQuat) f64 {
        return @sqrt(self.length_squared());
    }

    pub fn normalize(self: DQuat) DQuat {
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

    pub fn inverse(self: DQuat) DQuat {
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

    pub fn dot(self: DQuat, other: DQuat) f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
    }

    pub fn mul_dvec3(self: DQuat, v: DVec3) DVec3 {
        const u = DVec3{ .x = self.x, .y = self.y, .z = self.z };
        const s = self.w;

        const d = 2.0 * u.dot(v);
        const term1 = u.mul_scalar(d);

        const ss = s * s - u.dot(u);
        const term2 = v.mul_scalar(ss);

        const c = u.cross(v);
        const term3 = c.mul_scalar(2.0 * s);

        return term1.add(term2).add(term3);
    }

    pub fn slerp(self: DQuat, other_in: DQuat, t: f64) DQuat {
        var other = other_in;
        var d = self.dot(other);

        if (d < 0.0) {
            other = .{ .x = -other.x, .y = -other.y, .z = -other.z, .w = -other.w };
            d = -d;
        }

        if (d > 0.9995) {
            const result = DQuat{
                .x = self.x + (other.x - self.x) * t,
                .y = self.y + (other.y - self.y) * t,
                .z = self.z + (other.z - self.z) * t,
                .w = self.w + (other.w - self.w) * t,
            };
            return result.normalize();
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
};

export fn dquat_identity() DQuat {
    return DQuat.identity();
}

export fn dquat_from_xyzw(x: f64, y: f64, z: f64, w: f64) DQuat {
    return DQuat.from_xyzw(x, y, z, w);
}

export fn dquat_from_axis_angle(axis: DVec3, angle: f64) DQuat {
    return DQuat.from_axis_angle(axis, angle);
}

export fn dquat_mul(self: DQuat, other: DQuat) DQuat {
    return self.mul(other);
}

export fn dquat_conjugate(self: DQuat) DQuat {
    return self.conjugate();
}

export fn dquat_inverse(self: DQuat) DQuat {
    return self.inverse();
}

export fn dquat_normalize(self: DQuat) DQuat {
    return self.normalize();
}

export fn dquat_length(self: DQuat) f64 {
    return self.length();
}

export fn dquat_mul_dvec3(self: DQuat, v: DVec3) DVec3 {
    return self.mul_dvec3(v);
}

export fn dquat_slerp(self: DQuat, other: DQuat, t: f64) DQuat {
    return self.slerp(other, t);
}
