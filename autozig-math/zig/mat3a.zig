const std = @import("std");

/// Mat3A - SIMD-aligned 3x3 matrix (columns are Vec3A)
pub const Mat3A = extern struct {
    cols: [3]Vec3A,

    pub const IDENTITY = Mat3A{ .cols = .{
        Vec3A.X,
        Vec3A.Y,
        Vec3A.Z,
    } };

    pub const ZERO = Mat3A{ .cols = .{
        Vec3A.ZERO,
        Vec3A.ZERO,
        Vec3A.ZERO,
    } };

    pub fn identity() Mat3A {
        return IDENTITY;
    }

    pub fn from_mat3(m: Mat3) Mat3A {
        return .{ .cols = .{
            Vec3A{ .x = m.cols[0][0], .y = m.cols[0][1], .z = m.cols[0][2], ._pad = 0.0 },
            Vec3A{ .x = m.cols[1][0], .y = m.cols[1][1], .z = m.cols[1][2], ._pad = 0.0 },
            Vec3A{ .x = m.cols[2][0], .y = m.cols[2][1], .z = m.cols[2][2], ._pad = 0.0 },
        } };
    }

    pub fn to_mat3(self: Mat3A) Mat3 {
        return .{ .cols = .{
            .{ self.cols[0].x, self.cols[0].y, self.cols[0].z },
            .{ self.cols[1].x, self.cols[1].y, self.cols[1].z },
            .{ self.cols[2].x, self.cols[2].y, self.cols[2].z },
        } };
    }

    pub fn from_quat(q: Quat) Mat3A {
        const x2 = q.x + q.x;
        const y2 = q.y + q.y;
        const z2 = q.z + q.z;
        const xx = q.x * x2;
        const xy = q.x * y2;
        const xz = q.x * z2;
        const yy = q.y * y2;
        const yz = q.y * z2;
        const zz = q.z * z2;
        const wx = q.w * x2;
        const wy = q.w * y2;
        const wz = q.w * z2;

        return .{ .cols = .{
            Vec3A{ .x = 1.0 - (yy + zz), .y = xy + wz, .z = xz - wy, ._pad = 0.0 },
            Vec3A{ .x = xy - wz, .y = 1.0 - (xx + zz), .z = yz + wx, ._pad = 0.0 },
            Vec3A{ .x = xz + wy, .y = yz - wx, .z = 1.0 - (xx + yy), ._pad = 0.0 },
        } };
    }

    pub fn mul(self: Mat3A, other: Mat3A) Mat3A {
        var result: Mat3A = undefined;
        inline for (0..3) |col| {
            result.cols[col] = .{
                .x = self.cols[0].x * other.cols[col].x + self.cols[1].x * other.cols[col].y + self.cols[2].x * other.cols[col].z,
                .y = self.cols[0].y * other.cols[col].x + self.cols[1].y * other.cols[col].y + self.cols[2].y * other.cols[col].z,
                .z = self.cols[0].z * other.cols[col].x + self.cols[1].z * other.cols[col].y + self.cols[2].z * other.cols[col].z,
                ._pad = 0.0,
            };
        }
        return result;
    }

    pub fn mul_vec3a(self: Mat3A, v: Vec3A) Vec3A {
        return .{
            .x = self.cols[0].x * v.x + self.cols[1].x * v.y + self.cols[2].x * v.z,
            .y = self.cols[0].y * v.x + self.cols[1].y * v.y + self.cols[2].y * v.z,
            .z = self.cols[0].z * v.x + self.cols[1].z * v.y + self.cols[2].z * v.z,
            ._pad = 0.0,
        };
    }

    pub fn transpose(self: Mat3A) Mat3A {
        return .{ .cols = .{
            Vec3A{ .x = self.cols[0].x, .y = self.cols[1].x, .z = self.cols[2].x, ._pad = 0.0 },
            Vec3A{ .x = self.cols[0].y, .y = self.cols[1].y, .z = self.cols[2].y, ._pad = 0.0 },
            Vec3A{ .x = self.cols[0].z, .y = self.cols[1].z, .z = self.cols[2].z, ._pad = 0.0 },
        } };
    }
};

export fn mat3a_identity() Mat3A {
    return Mat3A.identity();
}

export fn mat3a_from_quat(q: Quat) Mat3A {
    return Mat3A.from_quat(q);
}

export fn mat3a_mul(self: Mat3A, other: Mat3A) Mat3A {
    return self.mul(other);
}

export fn mat3a_mul_vec3a(self: Mat3A, v: Vec3A) Vec3A {
    return self.mul_vec3a(v);
}

export fn mat3a_transpose(self: Mat3A) Mat3A {
    return self.transpose();
}
