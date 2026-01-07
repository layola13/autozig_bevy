const std = @import("std");

pub const DVec2 = extern struct {
    x: f64,
    y: f64,

    pub fn new(x: f64, y: f64) DVec2 {
        return .{ .x = x, .y = y };
    }

    pub fn splat(value: f64) DVec2 {
        return .{ .x = value, .y = value };
    }

    pub fn dot(self: DVec2, other: DVec2) f64 {
        return self.x * other.x + self.y * other.y;
    }

    pub fn add(self: DVec2, other: DVec2) DVec2 {
        return .{ .x = self.x + other.x, .y = self.y + other.y };
    }

    pub fn sub(self: DVec2, other: DVec2) DVec2 {
        return .{ .x = self.x - other.x, .y = self.y - other.y };
    }

    pub fn mul_scalar(self: DVec2, s: f64) DVec2 {
        return .{ .x = self.x * s, .y = self.y * s };
    }

    pub fn length_squared(self: DVec2) f64 {
        return self.dot(self);
    }

    pub fn length(self: DVec2) f64 {
        return @sqrt(self.length_squared());
    }

    pub fn normalize(self: DVec2) DVec2 {
        const len = self.length();
        if (len == 0) return .{ .x = 0, .y = 0 };
        return self.mul_scalar(1.0 / len);
    }

    pub fn distance(self: DVec2, other: DVec2) f64 {
        return self.sub(other).length();
    }

    pub fn lerp(self: DVec2, other: DVec2, t: f64) DVec2 {
        return .{
            .x = self.x + (other.x - self.x) * t,
            .y = self.y + (other.y - self.y) * t,
        };
    }
};

export fn dvec2_new(x: f64, y: f64) DVec2 {
    return DVec2.new(x, y);
}

export fn dvec2_dot(self: DVec2, other: DVec2) f64 {
    return self.dot(other);
}

export fn dvec2_add(self: DVec2, other: DVec2) DVec2 {
    return self.add(other);
}

export fn dvec2_sub(self: DVec2, other: DVec2) DVec2 {
    return self.sub(other);
}

export fn dvec2_mul_scalar(self: DVec2, s: f64) DVec2 {
    return self.mul_scalar(s);
}

export fn dvec2_length(self: DVec2) f64 {
    return self.length();
}

export fn dvec2_normalize(self: DVec2) DVec2 {
    return self.normalize();
}
