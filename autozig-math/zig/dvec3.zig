const std = @import("std");

pub const DVec3 = extern struct {
    x: f64,
    y: f64,
    z: f64,

    pub fn new(x: f64, y: f64, z: f64) DVec3 {
        return .{ .x = x, .y = y, .z = z };
    }

    pub fn splat(value: f64) DVec3 {
        return .{ .x = value, .y = value, .z = value };
    }

    pub fn dot(self: DVec3, other: DVec3) f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(self: DVec3, other: DVec3) DVec3 {
        return .{
            .x = self.y * other.z - self.z * other.y,
            .y = self.z * other.x - self.x * other.z,
            .z = self.x * other.y - self.y * other.x,
        };
    }

    pub fn add(self: DVec3, other: DVec3) DVec3 {
        return .{
            .x = self.x + other.x,
            .y = self.y + other.y,
            .z = self.z + other.z,
        };
    }

    pub fn sub(self: DVec3, other: DVec3) DVec3 {
        return .{
            .x = self.x - other.x,
            .y = self.y - other.y,
            .z = self.z - other.z,
        };
    }

    pub fn mul_scalar(self: DVec3, s: f64) DVec3 {
        return .{
            .x = self.x * s,
            .y = self.y * s,
            .z = self.z * s,
        };
    }

    pub fn length_squared(self: DVec3) f64 {
        return self.dot(self);
    }

    pub fn length(self: DVec3) f64 {
        return @sqrt(self.length_squared());
    }

    pub fn normalize(self: DVec3) DVec3 {
        const len = self.length();
        if (len == 0) return .{ .x = 0, .y = 0, .z = 0 };
        return self.mul_scalar(1.0 / len);
    }

    pub fn distance(self: DVec3, other: DVec3) f64 {
        return self.sub(other).length();
    }

    pub fn lerp(self: DVec3, other: DVec3, t: f64) DVec3 {
        return .{
            .x = self.x + (other.x - self.x) * t,
            .y = self.y + (other.y - self.y) * t,
            .z = self.z + (other.z - self.z) * t,
        };
    }
};

export fn dvec3_new(x: f64, y: f64, z: f64) DVec3 {
    return DVec3.new(x, y, z);
}

export fn dvec3_dot(self: DVec3, other: DVec3) f64 {
    return self.dot(other);
}

export fn dvec3_cross(self: DVec3, other: DVec3) DVec3 {
    return self.cross(other);
}

export fn dvec3_add(self: DVec3, other: DVec3) DVec3 {
    return self.add(other);
}

export fn dvec3_sub(self: DVec3, other: DVec3) DVec3 {
    return self.sub(other);
}

export fn dvec3_mul_scalar(self: DVec3, s: f64) DVec3 {
    return self.mul_scalar(s);
}

export fn dvec3_length(self: DVec3) f64 {
    return self.length();
}

export fn dvec3_normalize(self: DVec3) DVec3 {
    return self.normalize();
}
