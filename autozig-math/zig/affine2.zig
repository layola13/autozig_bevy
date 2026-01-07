const std = @import("std");

pub const Affine2 = extern struct {
    matrix2: Mat2,
    translation: Vec2,

    pub const IDENTITY = Affine2{
        .matrix2 = Mat2.IDENTITY,
        .translation = Vec2{ .x = 0.0, .y = 0.0 },
    };

    pub fn from_mat2_translation(matrix2: Mat2, translation: Vec2) Affine2 {
        return .{
            .matrix2 = matrix2,
            .translation = translation,
        };
    }

    pub fn from_translation(translation: Vec2) Affine2 {
        return .{
            .matrix2 = Mat2.IDENTITY,
            .translation = translation,
        };
    }

    pub fn from_rotation(angle: f32) Affine2 {
        return .{
            .matrix2 = Mat2.from_angle(angle),
            .translation = Vec2{ .x = 0.0, .y = 0.0 },
        };
    }

    pub fn from_scale(scale: Vec2) Affine2 {
        return .{
            .matrix2 = Mat2.from_scale(scale),
            .translation = Vec2{ .x = 0.0, .y = 0.0 },
        };
    }

    pub fn from_scale_angle_translation(scale: Vec2, angle: f32, translation: Vec2) Affine2 {
        const rot = Mat2.from_angle(angle);
        const scaled = Mat2{ .cols = .{
            .{ rot.cols[0][0] * scale.x, rot.cols[0][1] * scale.x },
            .{ rot.cols[1][0] * scale.y, rot.cols[1][1] * scale.y },
        } };
        return .{
            .matrix2 = scaled,
            .translation = translation,
        };
    }

    pub fn transform_point(self: Affine2, point: Vec2) Vec2 {
        const rotated = self.matrix2.mul_vec2(point);
        return rotated.add(self.translation);
    }

    pub fn transform_vector(self: Affine2, vec: Vec2) Vec2 {
        return self.matrix2.mul_vec2(vec);
    }
};

export fn affine2_identity() Affine2 {
    return Affine2.IDENTITY;
}

export fn affine2_from_translation(translation: Vec2) Affine2 {
    return Affine2.from_translation(translation);
}

export fn affine2_from_rotation(angle: f32) Affine2 {
    return Affine2.from_rotation(angle);
}

export fn affine2_from_scale(scale: Vec2) Affine2 {
    return Affine2.from_scale(scale);
}

export fn affine2_from_scale_angle_translation(scale: Vec2, angle: f32, translation: Vec2) Affine2 {
    return Affine2.from_scale_angle_translation(scale, angle, translation);
}

export fn affine2_transform_point(self: Affine2, point: Vec2) Vec2 {
    return self.transform_point(point);
}

export fn affine2_transform_vector(self: Affine2, vec: Vec2) Vec2 {
    return self.transform_vector(vec);
}
