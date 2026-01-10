const std = @import("std");
const Vec3 = @import("vec3.zig").Vec3;

/// Vec3A - SIMD-aligned 3D vector (16-byte aligned, uses 4 floats internally)
pub const Vec3A = extern struct {
    x: f32,
    y: f32,
    z: f32,
    _pad: f32 = 0.0,

    pub const ZERO = Vec3A{ .x = 0.0, .y = 0.0, .z = 0.0, ._pad = 0.0 };
    pub const ONE = Vec3A{ .x = 1.0, .y = 1.0, .z = 1.0, ._pad = 0.0 };
    pub const X = Vec3A{ .x = 1.0, .y = 0.0, .z = 0.0, ._pad = 0.0 };
    pub const Y = Vec3A{ .x = 0.0, .y = 1.0, .z = 0.0, ._pad = 0.0 };
    pub const Z = Vec3A{ .x = 0.0, .y = 0.0, .z = 1.0, ._pad = 0.0 };

    pub fn new(x: f32, y: f32, z: f32) Vec3A {
        return .{ .x = x, .y = y, .z = z, ._pad = 0.0 };
    }

    pub fn from_vec3(v: Vec3) Vec3A {
        return .{ .x = v.x, .y = v.y, .z = v.z, ._pad = 0.0 };
    }

    pub fn to_vec3(self: Vec3A) Vec3 {
        return Vec3{ .x = self.x, .y = self.y, .z = self.z };
    }

    pub fn splat(value: f32) Vec3A {
        return .{ .x = value, .y = value, .z = value, ._pad = 0.0 };
    }

    pub fn dot(self: Vec3A, other: Vec3A) f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }

    pub fn cross(self: Vec3A, other: Vec3A) Vec3A {
        return .{
            .x = self.y * other.z - self.z * other.y,
            .y = self.z * other.x - self.x * other.z,
            .z = self.x * other.y - self.y * other.x,
            ._pad = 0.0,
        };
    }

    pub fn add(self: Vec3A, other: Vec3A) Vec3A {
        return .{
            .x = self.x + other.x,
            .y = self.y + other.y,
            .z = self.z + other.z,
            ._pad = 0.0,
        };
    }

    pub fn sub(self: Vec3A, other: Vec3A) Vec3A {
        return .{
            .x = self.x - other.x,
            .y = self.y - other.y,
            .z = self.z - other.z,
            ._pad = 0.0,
        };
    }

    pub fn mul_scalar(self: Vec3A, s: f32) Vec3A {
        return .{
            .x = self.x * s,
            .y = self.y * s,
            .z = self.z * s,
            ._pad = 0.0,
        };
    }

    pub fn length_squared(self: Vec3A) f32 {
        return self.dot(self);
    }

    pub fn length(self: Vec3A) f32 {
        return @sqrt(self.length_squared());
    }

    pub fn normalize(self: Vec3A) Vec3A {
        const len = self.length();
        if (len == 0) return ZERO;
        return self.mul_scalar(1.0 / len);
    }

    pub fn lerp(self: Vec3A, other: Vec3A, t: f32) Vec3A {
        return .{
            .x = self.x + (other.x - self.x) * t,
            .y = self.y + (other.y - self.y) * t,
            .z = self.z + (other.z - self.z) * t,
            ._pad = 0.0,
        };
    }
};

/// Dir3A - SIMD-aligned 3D direction
pub const Dir3A = extern struct {
    vec: Vec3A,

    pub const X = Dir3A{ .vec = Vec3A.X };
    pub const Y = Dir3A{ .vec = Vec3A.Y };
    pub const Z = Dir3A{ .vec = Vec3A.Z };

    pub fn new(x: f32, y: f32, z: f32) Dir3A {
        const v = Vec3A.new(x, y, z);
        return .{ .vec = v.normalize() };
    }

    pub fn from_vec3a(v: Vec3A) Dir3A {
        return .{ .vec = v.normalize() };
    }
};

export fn vec3a_new(x: f32, y: f32, z: f32) Vec3A {
    return Vec3A.new(x, y, z);
}

export fn vec3a_from_vec3(v: Vec3) Vec3A {
    return Vec3A.from_vec3(v);
}

export fn vec3a_to_vec3(self: Vec3A) Vec3 {
    return self.to_vec3();
}

export fn vec3a_dot(self: Vec3A, other: Vec3A) f32 {
    return self.dot(other);
}

export fn vec3a_cross(self: Vec3A, other: Vec3A) Vec3A {
    return self.cross(other);
}

export fn vec3a_add(self: Vec3A, other: Vec3A) Vec3A {
    return self.add(other);
}

export fn vec3a_sub(self: Vec3A, other: Vec3A) Vec3A {
    return self.sub(other);
}

export fn vec3a_mul_scalar(self: Vec3A, s: f32) Vec3A {
    return self.mul_scalar(s);
}

export fn vec3a_length(self: Vec3A) f32 {
    return self.length();
}

export fn vec3a_normalize(self: Vec3A) Vec3A {
    return self.normalize();
}

export fn vec3a_lerp(self: Vec3A, other: Vec3A, t: f32) Vec3A {
    return self.lerp(other, t);
}

export fn dir3a_new(x: f32, y: f32, z: f32) Dir3A {
    return Dir3A.new(x, y, z);
}

export fn dir3a_from_vec3a(v: Vec3A) Dir3A {
    return Dir3A.from_vec3a(v);
}
