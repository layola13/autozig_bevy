const std = @import("std");
const Vec3 = @import("vec3.zig").Vec3;
const Quat = @import("quat.zig").Quat;

pub const Dir3 = extern struct {
    vec: Vec3,

    pub const X = Dir3{ .vec = Vec3{ .x = 1.0, .y = 0.0, .z = 0.0 } };
    pub const Y = Dir3{ .vec = Vec3{ .x = 0.0, .y = 1.0, .z = 0.0 } };
    pub const Z = Dir3{ .vec = Vec3{ .x = 0.0, .y = 0.0, .z = 1.0 } };
    pub const NEG_X = Dir3{ .vec = Vec3{ .x = -1.0, .y = 0.0, .z = 0.0 } };
    pub const NEG_Y = Dir3{ .vec = Vec3{ .x = 0.0, .y = -1.0, .z = 0.0 } };
    pub const NEG_Z = Dir3{ .vec = Vec3{ .x = 0.0, .y = 0.0, .z = -1.0 } };

    pub fn new(x: f32, y: f32, z: f32) Dir3 {
        const len = @sqrt(x * x + y * y + z * z);
        if (len == 0) return X;
        return .{
            .vec = .{ .x = x / len, .y = y / len, .z = z / len },
        };
    }

    pub fn from_vec3(v: Vec3) Dir3 {
        return new(v.x, v.y, v.z);
    }

    /// Negate the direction
    pub fn neg(self: Dir3) Dir3 {
        return .{
            .vec = .{ .x = -self.vec.x, .y = -self.vec.y, .z = -self.vec.z },
        };
    }

    /// Rotate by quaternion
    pub fn rotate(self: Dir3, quat: Quat) Dir3 {
        const rotated = quat.mul_vec3(self.vec);
        return .{ .vec = rotated };
    }
};

export fn dir3_from_vec3(v: Vec3) Dir3 {
    return Dir3.from_vec3(v);
}

export fn dir3_new(x: f32, y: f32, z: f32) Dir3 {
    return Dir3.new(x, y, z);
}

export fn dir3_neg(self: Dir3) Dir3 {
    return self.neg();
}
