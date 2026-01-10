const std = @import("std");
const DVec2 = @import("dvec2.zig").DVec2;

pub const DMat2 = extern struct {
    cols: [2][2]f64,

    pub const IDENTITY = DMat2{ .cols = .{
        .{ 1.0, 0.0 },
        .{ 0.0, 1.0 },
    } };

    pub const ZERO = DMat2{ .cols = .{
        .{ 0.0, 0.0 },
        .{ 0.0, 0.0 },
    } };

    pub fn identity() DMat2 {
        return IDENTITY;
    }

    pub fn from_angle(angle: f64) DMat2 {
        const c = @cos(angle);
        const s = @sin(angle);
        return .{ .cols = .{
            .{ c, s },
            .{ -s, c },
        } };
    }

    pub fn from_scale(scale: DVec2) DMat2 {
        return .{ .cols = .{
            .{ scale.x, 0.0 },
            .{ 0.0, scale.y },
        } };
    }

    pub fn mul(self: DMat2, other: DMat2) DMat2 {
        var result: DMat2 = undefined;
        inline for (0..2) |col| {
            inline for (0..2) |row| {
                var sum: f64 = 0.0;
                inline for (0..2) |k| {
                    sum += self.cols[k][row] * other.cols[col][k];
                }
                result.cols[col][row] = sum;
            }
        }
        return result;
    }

    pub fn mul_dvec2(self: DMat2, v: DVec2) DVec2 {
        return .{
            .x = self.cols[0][0] * v.x + self.cols[1][0] * v.y,
            .y = self.cols[0][1] * v.x + self.cols[1][1] * v.y,
        };
    }

    pub fn transpose(self: DMat2) DMat2 {
        return .{ .cols = .{
            .{ self.cols[0][0], self.cols[1][0] },
            .{ self.cols[0][1], self.cols[1][1] },
        } };
    }

    pub fn determinant(self: DMat2) f64 {
        return self.cols[0][0] * self.cols[1][1] - self.cols[0][1] * self.cols[1][0];
    }

    pub fn inverse(self: DMat2) DMat2 {
        const det = self.determinant();
        if (@abs(det) < std.math.floatEps(f64)) {
            return IDENTITY;
        }
        const inv_det = 1.0 / det;
        return .{ .cols = .{
            .{ self.cols[1][1] * inv_det, -self.cols[0][1] * inv_det },
            .{ -self.cols[1][0] * inv_det, self.cols[0][0] * inv_det },
        } };
    }
};

export fn dmat2_identity() DMat2 {
    return DMat2.identity();
}

export fn dmat2_from_angle(angle: f64) DMat2 {
    return DMat2.from_angle(angle);
}

export fn dmat2_mul(self: DMat2, other: DMat2) DMat2 {
    return self.mul(other);
}

export fn dmat2_mul_dvec2(self: DMat2, v: DVec2) DVec2 {
    return self.mul_dvec2(v);
}

export fn dmat2_transpose(self: DMat2) DMat2 {
    return self.transpose();
}

export fn dmat2_determinant(self: DMat2) f64 {
    return self.determinant();
}

export fn dmat2_inverse(self: DMat2) DMat2 {
    return self.inverse();
}
