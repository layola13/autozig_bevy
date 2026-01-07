use autozig::include_zig;
use crate::{Vec2, Mat2};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Affine2 {
    pub matrix2: Mat2,
    pub translation: Vec2,
}

include_zig!("zig/affine2.zig", {
    fn affine2_identity() -> Affine2;
    fn affine2_from_translation(translation: Vec2) -> Affine2;
    fn affine2_from_rotation(angle: f32) -> Affine2;
    fn affine2_from_scale(scale: Vec2) -> Affine2;
    fn affine2_from_scale_angle_translation(scale: Vec2, angle: f32, translation: Vec2) -> Affine2;
    fn affine2_transform_point(self_: Affine2, point: Vec2) -> Vec2;
    fn affine2_transform_vector(self_: Affine2, vec: Vec2) -> Vec2;
});

impl Affine2 {
    pub const IDENTITY: Self = Self {
        matrix2: Mat2::IDENTITY,
        translation: Vec2::ZERO,
    };

    pub fn identity() -> Self {
        affine2_identity()
    }

    pub fn from_translation(translation: Vec2) -> Self {
        affine2_from_translation(translation)
    }

    pub fn from_rotation(angle: f32) -> Self {
        affine2_from_rotation(angle)
    }

    pub fn from_scale(scale: Vec2) -> Self {
        affine2_from_scale(scale)
    }

    pub fn from_scale_angle_translation(scale: Vec2, angle: f32, translation: Vec2) -> Self {
        affine2_from_scale_angle_translation(scale, angle, translation)
    }

    pub fn transform_point(self, point: Vec2) -> Vec2 {
        affine2_transform_point(self, point)
    }

    pub fn transform_vector(self, vec: Vec2) -> Vec2 {
        affine2_transform_vector(self, vec)
    }
}
