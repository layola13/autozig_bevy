const std = @import("std");

pub const Rot2 = extern struct {
    c: f32, // cosine
    s: f32, // sine

    pub const IDENTITY = Rot2{ .c = 1.0, .s = 0.0 };

    pub fn identity() Rot2 {
        return IDENTITY;
    }

    pub fn from_angle(angle: f32) Rot2 {
        return .{
            .c = @cos(angle),
            .s = @sin(angle),
        };
    }

    pub fn from_sin_cos(sin: f32, cos: f32) Rot2 {
        return .{
            .c = cos,
            .s = sin,
        };
    }

    pub fn as_angle(self: Rot2) f32 {
        return std.math.atan2(self.s, self.c);
    }

    pub fn mul(self: Rot2, other: Rot2) Rot2 {
        return .{
            .c = self.c * other.c - self.s * other.s,
            .s = self.s * other.c + self.c * other.s,
        };
    }

    pub fn inverse(self: Rot2) Rot2 {
        return .{
            .c = self.c,
            .s = -self.s,
        };
    }

    pub fn rotate_vec2(self: Rot2, v: Vec2) Vec2 {
        return .{
            .x = self.c * v.x - self.s * v.y,
            .y = self.s * v.x + self.c * v.y,
        };
    }

    pub fn length(self: Rot2) f32 {
        return @sqrt(self.c * self.c + self.s * self.s);
    }

    pub fn normalize(self: Rot2) Rot2 {
        const len = self.length();
        if (len == 0) return IDENTITY;
        return .{
            .c = self.c / len,
            .s = self.s / len,
        };
    }

    pub fn lerp(self: Rot2, other: Rot2, t: f32) Rot2 {
        const c = self.c + (other.c - self.c) * t;
        const s = self.s + (other.s - self.s) * t;
        const result = Rot2{ .c = c, .s = s };
        return result.normalize();
    }
};

export fn rot2_identity() Rot2 {
    return Rot2.IDENTITY;
}

export fn rot2_from_angle(angle: f32) Rot2 {
    return Rot2.from_angle(angle);
}

export fn rot2_as_angle(self: Rot2) f32 {
    return self.as_angle();
}

export fn rot2_mul(self: Rot2, other: Rot2) Rot2 {
    return self.mul(other);
}

export fn rot2_inverse(self: Rot2) Rot2 {
    return self.inverse();
}

export fn rot2_rotate_vec2(self: Rot2, v: Vec2) Vec2 {
    return self.rotate_vec2(v);
}

export fn rot2_lerp(self: Rot2, other: Rot2, t: f32) Rot2 {
    return self.lerp(other, t);
}
