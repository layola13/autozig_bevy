const std = @import("std");
const DVec3 = @import("dvec3.zig").DVec3;
const DQuat = @import("dquat.zig").DQuat;

pub const DMat3 = extern struct {
    cols: [3][3]f64,

    pub const IDENTITY = DMat3{ .cols = .{
        .{ 1.0, 0.0, 0.0 },
        .{ 0.0, 1.0, 0.0 },
        .{ 0.0, 0.0, 1.0 },
    } };

    pub const ZERO = DMat3{ .cols = .{
        .{ 0.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 0.0 },
    } };

    pub fn identity() DMat3 {
        return IDENTITY;
    }

    pub fn from_scale(scale: DVec3) DMat3 {
        return .{ .cols = .{
            .{ scale.x, 0.0, 0.0 },
            .{ 0.0, scale.y, 0.0 },
            .{ 0.0, 0.0, scale.z },
        } };
    }

    pub fn from_dquat(q: DQuat) DMat3 {
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

    pub fn mul(self: DMat3, other: DMat3) DMat3 {
        var result: DMat3 = undefined;
        inline for (0..3) |col| {
            inline for (0..3) |row| {
                var sum: f64 = 0.0;
                inline for (0..3) |k| {
                    sum += self.cols[k][row] * other.cols[col][k];
                }
                result.cols[col][row] = sum;
            }
        }
        return result;
    }

    pub fn mul_dvec3(self: DMat3, v: DVec3) DVec3 {
        return .{
            .x = self.cols[0][0] * v.x + self.cols[1][0] * v.y + self.cols[2][0] * v.z,
            .y = self.cols[0][1] * v.x + self.cols[1][1] * v.y + self.cols[2][1] * v.z,
            .z = self.cols[0][2] * v.x + self.cols[1][2] * v.y + self.cols[2][2] * v.z,
        };
    }

    pub fn transpose(self: DMat3) DMat3 {
        return .{ .cols = .{
            .{ self.cols[0][0], self.cols[1][0], self.cols[2][0] },
            .{ self.cols[0][1], self.cols[1][1], self.cols[2][1] },
            .{ self.cols[0][2], self.cols[1][2], self.cols[2][2] },
        } };
    }

    pub fn determinant(self: DMat3) f64 {
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
};

export fn dmat3_identity() DMat3 {
    return DMat3.identity();
}

export fn dmat3_from_scale(scale: DVec3) DMat3 {
    return DMat3.from_scale(scale);
}

export fn dmat3_mul(self: DMat3, other: DMat3) DMat3 {
    return self.mul(other);
}

export fn dmat3_mul_dvec3(self: DMat3, v: DVec3) DVec3 {
    return self.mul_dvec3(v);
}

export fn dmat3_transpose(self: DMat3) DMat3 {
    return self.transpose();
}

export fn dmat3_determinant(self: DMat3) f64 {
    return self.determinant();
}
