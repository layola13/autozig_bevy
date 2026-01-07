const std = @import("std");

pub const Mat2 = extern struct {
    cols: [2][2]f32,

    pub const IDENTITY = Mat2{ .cols = .{
        .{ 1.0, 0.0 },
        .{ 0.0, 1.0 },
    } };

    pub const ZERO = Mat2{ .cols = .{
        .{ 0.0, 0.0 },
        .{ 0.0, 0.0 },
    } };

    pub fn identity() Mat2 {
        return IDENTITY;
    }

    pub fn from_cols(c0: Vec2, c1: Vec2) Mat2 {
        return .{ .cols = .{
            .{ c0.x, c0.y },
            .{ c1.x, c1.y },
        } };
    }

    pub fn from_angle(angle: f32) Mat2 {
        const c = @cos(angle);
        const s = @sin(angle);
        return .{ .cols = .{
            .{ c, s },
            .{ -s, c },
        } };
    }

    pub fn from_scale(scale: Vec2) Mat2 {
        return .{ .cols = .{
            .{ scale.x, 0.0 },
            .{ 0.0, scale.y },
        } };
    }

    pub fn mul(self: Mat2, other: Mat2) Mat2 {
        var result: Mat2 = undefined;
        inline for (0..2) |col| {
            inline for (0..2) |row| {
                var sum: f32 = 0.0;
                inline for (0..2) |k| {
                    sum += self.cols[k][row] * other.cols[col][k];
                }
                result.cols[col][row] = sum;
            }
        }
        return result;
    }

    pub fn mul_vec2(self: Mat2, v: Vec2) Vec2 {
        return .{
            .x = self.cols[0][0] * v.x + self.cols[1][0] * v.y,
            .y = self.cols[0][1] * v.x + self.cols[1][1] * v.y,
        };
    }

    pub fn transpose(self: Mat2) Mat2 {
        return .{ .cols = .{
            .{ self.cols[0][0], self.cols[1][0] },
            .{ self.cols[0][1], self.cols[1][1] },
        } };
    }

    pub fn determinant(self: Mat2) f32 {
        return self.cols[0][0] * self.cols[1][1] - self.cols[0][1] * self.cols[1][0];
    }

    pub fn inverse(self: Mat2) Mat2 {
        const det = self.determinant();
        if (@abs(det) < std.math.floatEps(f32)) {
            return IDENTITY;
        }
        const inv_det = 1.0 / det;
        return .{ .cols = .{
            .{ self.cols[1][1] * inv_det, -self.cols[0][1] * inv_det },
            .{ -self.cols[1][0] * inv_det, self.cols[0][0] * inv_det },
        } };
    }
};

export fn mat2_identity() Mat2 {
    return Mat2.identity();
}

export fn mat2_from_angle(angle: f32) Mat2 {
    return Mat2.from_angle(angle);
}

export fn mat2_from_scale(scale: Vec2) Mat2 {
    return Mat2.from_scale(scale);
}

export fn mat2_mul(self: Mat2, other: Mat2) Mat2 {
    return self.mul(other);
}

export fn mat2_mul_vec2(self: Mat2, v: Vec2) Vec2 {
    return self.mul_vec2(v);
}

export fn mat2_transpose(self: Mat2) Mat2 {
    return self.transpose();
}

export fn mat2_determinant(self: Mat2) f32 {
    return self.determinant();
}

export fn mat2_inverse(self: Mat2) Mat2 {
    return self.inverse();
}
