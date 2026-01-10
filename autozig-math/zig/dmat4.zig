const std = @import("std");
const DVec3 = @import("dvec3.zig").DVec3;
const DVec4 = @import("dvec4.zig").DVec4;
const DQuat = @import("dquat.zig").DQuat;

pub const DMat4 = extern struct {
    cols: [4][4]f64,

    pub const IDENTITY = DMat4{ .cols = .{
        .{ 1.0, 0.0, 0.0, 0.0 },
        .{ 0.0, 1.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 1.0, 0.0 },
        .{ 0.0, 0.0, 0.0, 1.0 },
    } };

    pub const ZERO = DMat4{ .cols = .{
        .{ 0.0, 0.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 0.0, 0.0 },
        .{ 0.0, 0.0, 0.0, 0.0 },
    } };

    pub fn identity() DMat4 {
        return IDENTITY;
    }

    pub fn from_translation(translation: DVec3) DMat4 {
        return .{ .cols = .{
            .{ 1.0, 0.0, 0.0, 0.0 },
            .{ 0.0, 1.0, 0.0, 0.0 },
            .{ 0.0, 0.0, 1.0, 0.0 },
            .{ translation.x, translation.y, translation.z, 1.0 },
        } };
    }

    pub fn from_scale(scale: DVec3) DMat4 {
        return .{ .cols = .{
            .{ scale.x, 0.0, 0.0, 0.0 },
            .{ 0.0, scale.y, 0.0, 0.0 },
            .{ 0.0, 0.0, scale.z, 0.0 },
            .{ 0.0, 0.0, 0.0, 1.0 },
        } };
    }

    pub fn from_dquat(q: DQuat) DMat4 {
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
            .{ 1.0 - (yy + zz), xy + wz, xz - wy, 0.0 },
            .{ xy - wz, 1.0 - (xx + zz), yz + wx, 0.0 },
            .{ xz + wy, yz - wx, 1.0 - (xx + yy), 0.0 },
            .{ 0.0, 0.0, 0.0, 1.0 },
        } };
    }

    pub fn mul(self: DMat4, other: DMat4) DMat4 {
        var result: DMat4 = undefined;
        inline for (0..4) |col| {
            inline for (0..4) |row| {
                var sum: f64 = 0.0;
                inline for (0..4) |k| {
                    sum += self.cols[k][row] * other.cols[col][k];
                }
                result.cols[col][row] = sum;
            }
        }
        return result;
    }

    pub fn mul_dvec4(self: DMat4, vec: DVec4) DVec4 {
        return .{
            .x = self.cols[0][0] * vec.x + self.cols[1][0] * vec.y + self.cols[2][0] * vec.z + self.cols[3][0] * vec.w,
            .y = self.cols[0][1] * vec.x + self.cols[1][1] * vec.y + self.cols[2][1] * vec.z + self.cols[3][1] * vec.w,
            .z = self.cols[0][2] * vec.x + self.cols[1][2] * vec.y + self.cols[2][2] * vec.z + self.cols[3][2] * vec.w,
            .w = self.cols[0][3] * vec.x + self.cols[1][3] * vec.y + self.cols[2][3] * vec.z + self.cols[3][3] * vec.w,
        };
    }

    pub fn transpose(self: DMat4) DMat4 {
        return .{ .cols = .{
            .{ self.cols[0][0], self.cols[1][0], self.cols[2][0], self.cols[3][0] },
            .{ self.cols[0][1], self.cols[1][1], self.cols[2][1], self.cols[3][1] },
            .{ self.cols[0][2], self.cols[1][2], self.cols[2][2], self.cols[3][2] },
            .{ self.cols[0][3], self.cols[1][3], self.cols[2][3], self.cols[3][3] },
        } };
    }

    pub fn transform_point(self: DMat4, point: DVec3) DVec3 {
        return .{
            .x = self.cols[0][0] * point.x + self.cols[1][0] * point.y + self.cols[2][0] * point.z + self.cols[3][0],
            .y = self.cols[0][1] * point.x + self.cols[1][1] * point.y + self.cols[2][1] * point.z + self.cols[3][1],
            .z = self.cols[0][2] * point.x + self.cols[1][2] * point.y + self.cols[2][2] * point.z + self.cols[3][2],
        };
    }
};

export fn dmat4_identity() DMat4 {
    return DMat4.identity();
}

export fn dmat4_from_translation(translation: DVec3) DMat4 {
    return DMat4.from_translation(translation);
}

export fn dmat4_from_scale(scale: DVec3) DMat4 {
    return DMat4.from_scale(scale);
}

export fn dmat4_mul(self: DMat4, other: DMat4) DMat4 {
    return self.mul(other);
}

export fn dmat4_mul_dvec4(self: DMat4, vec: DVec4) DVec4 {
    return self.mul_dvec4(vec);
}

export fn dmat4_transpose(self: DMat4) DMat4 {
    return self.transpose();
}

export fn dmat4_transform_point(self: DMat4, point: DVec3) DVec3 {
    return self.transform_point(point);
}
