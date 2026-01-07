const std = @import("std");

pub const Affine3 = extern struct {
    matrix3: Mat3,
    translation: Vec3,

    pub const IDENTITY = Affine3{
        .matrix3 = Mat3.identity(),
        .translation = .{ .x = 0.0, .y = 0.0, .z = 0.0 },
    };

    pub fn from_mat3_translation(matrix3: Mat3, translation: Vec3) Affine3 {
        return .{
            .matrix3 = matrix3,
            .translation = translation,
        };
    }
};

export fn affine3_identity() Affine3 {
    return Affine3.IDENTITY;
}

export fn affine3_new(matrix3: Mat3, translation: Vec3) Affine3 {
    return Affine3.from_mat3_translation(matrix3, translation);
}
