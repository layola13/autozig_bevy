const std = @import("std");
const Vec3 = @import("vec3.zig").Vec3;
const Quat = @import("quat.zig").Quat;

pub const Mat3 = extern struct {
    cols: [3][3]f32,

    pub const IDENTITY = Mat3{ .cols = .{
        .{ 1.0, 0.0, 0.0 },
        .{ 0.0, 1.0, 0.0 },
        .{ 0.0, 0.0, 1.0 },
    } };

    pub const ZERO = Mat3{ .cols = .{
        .{ 0.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 0.0 },
    } };

    pub fn identity() Mat3 {
        return IDENTITY;
    }

    pub fn from_cols(c0: Vec3, c1: Vec3, c2: Vec3) Mat3 {
        return .{ .cols = .{
            .{ c0.x, c0.y, c0.z },
            .{ c1.x, c1.y, c1.z },
            .{ c2.x, c2.y, c2.z },
        } };
    }

    pub fn from_scale(scale: Vec3) Mat3 {
        return .{ .cols = .{
            .{ scale.x, 0.0, 0.0 },
            .{ 0.0, scale.y, 0.0 },
            .{ 0.0, 0.0, scale.z },
        } };
    }

    pub fn from_quat(q: Quat) Mat3 {
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
            .{ 1.0 - (yy + zz), xy + wz, xz - wy },
            .{ xy - wz, 1.0 - (xx + zz), yz + wx },
            .{ xz + wy, yz - wx, 1.0 - (xx + yy) },
        } };
    }

    pub fn mul(self: Mat3, other: Mat3) Mat3 {
        var result: Mat3 = undefined;
        inline for (0..3) |col| {
            inline for (0..3) |row| {
                var sum: f32 = 0.0;
                inline for (0..3) |k| {
                    sum += self.cols[k][row] * other.cols[col][k];
                }
                result.cols[col][row] = sum;
            }
        }
        return result;
    }

    pub fn mul_vec3(self: Mat3, v: Vec3) Vec3 {
        return .{
            .x = self.cols[0][0] * v.x + self.cols[1][0] * v.y + self.cols[2][0] * v.z,
            .y = self.cols[0][1] * v.x + self.cols[1][1] * v.y + self.cols[2][1] * v.z,
            .z = self.cols[0][2] * v.x + self.cols[1][2] * v.y + self.cols[2][2] * v.z,
        };
    }

    pub fn transpose(self: Mat3) Mat3 {
        return .{ .cols = .{
            .{ self.cols[0][0], self.cols[1][0], self.cols[2][0] },
            .{ self.cols[0][1], self.cols[1][1], self.cols[2][1] },
            .{ self.cols[0][2], self.cols[1][2], self.cols[2][2] },
        } };
    }

    pub fn determinant(self: Mat3) f32 {
        const a = self.cols[0][0];
        const b = self.cols[1][0];
        const c = self.cols[2][0];
        const d = self.cols[0][1];
        const e = self.cols[1][1];
        const f = self.cols[2][1];
        const g = self.cols[0][2];
        const h = self.cols[1][2];
        const i = self.cols[2][2];
        return a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);
    }

    pub fn inverse(self: Mat3) Mat3 {
        const det = self.determinant();
        if (@abs(det) < std.math.floatEps(f32)) {
            return IDENTITY;
        }

        const a = self.cols[0][0];
        const b = self.cols[1][0];
        const c = self.cols[2][0];
        const d = self.cols[0][1];
        const e = self.cols[1][1];
        const f = self.cols[2][1];
        const g = self.cols[0][2];
        const h = self.cols[1][2];
        const i = self.cols[2][2];

        const inv_det = 1.0 / det;

        return .{ .cols = .{
            .{ (e * i - f * h) * inv_det, (c * h - b * i) * inv_det, (b * f - c * e) * inv_det },
            .{ (f * g - d * i) * inv_det, (a * i - c * g) * inv_det, (c * d - a * f) * inv_det },
            .{ (d * h - e * g) * inv_det, (b * g - a * h) * inv_det, (a * e - b * d) * inv_det },
        } };
    }
};

export fn mat3_identity() Mat3 {
    return Mat3.identity();
}

export fn mat3_from_scale(scale: Vec3) Mat3 {
    return Mat3.from_scale(scale);
}

export fn mat3_from_quat(q: Quat) Mat3 {
    return Mat3.from_quat(q);
}

export fn mat3_mul(self: Mat3, other: Mat3) Mat3 {
    return self.mul(other);
}

export fn mat3_mul_vec3(self: Mat3, v: Vec3) Vec3 {
    return self.mul_vec3(v);
}

export fn mat3_transpose(self: Mat3) Mat3 {
    return self.transpose();
}

export fn mat3_determinant(self: Mat3) f32 {
    return self.determinant();
}

export fn mat3_inverse(self: Mat3) Mat3 {
    return self.inverse();
}
