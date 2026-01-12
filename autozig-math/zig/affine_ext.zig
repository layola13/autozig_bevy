//! Extended affine transform types implementation

const std = @import("std");

// Import existing types (assuming they're available)
const Vec2 = extern struct { x: f32, y: f32 };
const Vec3 = extern struct { x: f32, y: f32, z: f32 };
const DVec2 = extern struct { x: f64, y: f64 };
const DVec3 = extern struct { x: f64, y: f64, z: f64 };
const Mat3 = extern struct { data: [9]f32 };
const DMat3 = extern struct { data: [9]f64 };
const DMat4 = extern struct { data: [16]f64 };

pub const DAffine2 = extern struct {
    matrix2: DMat3,
    translation: DVec2,
};

pub const DAffine3 = extern struct {
    matrix3: DMat4,
    translation: DVec3,
};

pub const Affine3A = extern struct {
    matrix3: Mat3,
    translation: Vec3,
};

export fn daffine2_identity() DAffine2 {
    return DAffine2{
        .matrix2 = DMat3{ .data = [_]f64{ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 } },
        .translation = DVec2{ .x = 0.0, .y = 0.0 },
    };
}

export fn daffine3_identity() DAffine3 {
    return DAffine3{
        .matrix3 = DMat4{ .data = [_]f64{ 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0 } },
        .translation = DVec3{ .x = 0.0, .y = 0.0, .z = 0.0 },
    };
}

export fn affine3a_identity() Affine3A {
    return Affine3A{
        .matrix3 = Mat3{ .data = [_]f32{ 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0 } },
        .translation = Vec3{ .x = 0.0, .y = 0.0, .z = 0.0 },
    };
}
