const std = @import("std");

pub const DVec4 = extern struct {
    x: f64,
    y: f64,
    z: f64,
    w: f64,

    pub fn new(x: f64, y: f64, z: f64, w: f64) DVec4 {
        return .{ .x = x, .y = y, .z = z, .w = w };
    }

    pub fn splat(value: f64) DVec4 {
        return .{ .x = value, .y = value, .z = value, .w = value };
    }

    pub fn dot(self: DVec4, other: DVec4) f64 {
        return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
    }

    pub fn add(self: DVec4, other: DVec4) DVec4 {
        return .{
            .x = self.x + other.x,
            .y = self.y + other.y,
            .z = self.z + other.z,
            .w = self.w + other.w,
        };
    }

    pub fn sub(self: DVec4, other: DVec4) DVec4 {
        return .{
            .x = self.x - other.x,
            .y = self.y - other.y,
            .z = self.z - other.z,
            .w = self.w - other.w,
        };
    }

    pub fn mul_scalar(self: DVec4, s: f64) DVec4 {
        return .{
            .x = self.x * s,
            .y = self.y * s,
            .z = self.z * s,
            .w = self.w * s,
        };
    }

    pub fn length_squared(self: DVec4) f64 {
        return self.dot(self);
    }

    pub fn length(self: DVec4) f64 {
        return @sqrt(self.length_squared());
    }

    pub fn normalize(self: DVec4) DVec4 {
        const len = self.length();
        if (len == 0) return .{ .x = 0, .y = 0, .z = 0, .w = 0 };
        return self.mul_scalar(1.0 / len);
    }
};

export fn dvec4_new(x: f64, y: f64, z: f64, w: f64) DVec4 {
    return DVec4.new(x, y, z, w);
}

export fn dvec4_dot(self: DVec4, other: DVec4) f64 {
    return self.dot(other);
}

export fn dvec4_add(self: DVec4, other: DVec4) DVec4 {
    return self.add(other);
}

export fn dvec4_sub(self: DVec4, other: DVec4) DVec4 {
    return self.sub(other);
}

export fn dvec4_mul_scalar(self: DVec4, s: f64) DVec4 {
    return self.mul_scalar(s);
}

export fn dvec4_length(self: DVec4) f64 {
    return self.length();
}

export fn dvec4_normalize(self: DVec4) DVec4 {
    return self.normalize();
}
