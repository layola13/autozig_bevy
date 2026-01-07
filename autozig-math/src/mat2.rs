use autozig::include_zig;
use crate::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat2 {
    pub cols: [[f32; 2]; 2],
}

include_zig!("zig/mat2.zig", {
    fn mat2_identity() -> Mat2;
    fn mat2_from_angle(angle: f32) -> Mat2;
    fn mat2_from_scale(scale: Vec2) -> Mat2;
    fn mat2_mul(self_: Mat2, other: Mat2) -> Mat2;
    fn mat2_mul_vec2(self_: Mat2, v: Vec2) -> Vec2;
    fn mat2_transpose(self_: Mat2) -> Mat2;
    fn mat2_determinant(self_: Mat2) -> f32;
    fn mat2_inverse(self_: Mat2) -> Mat2;
});

impl Mat2 {
    pub const IDENTITY: Self = Self {
        cols: [[1.0, 0.0], [0.0, 1.0]],
    };

    pub const ZERO: Self = Self {
        cols: [[0.0, 0.0], [0.0, 0.0]],
    };

    pub fn identity() -> Self {
        mat2_identity()
    }

    pub fn from_angle(angle: f32) -> Self {
        mat2_from_angle(angle)
    }

    pub fn from_scale(scale: Vec2) -> Self {
        mat2_from_scale(scale)
    }

    pub fn mul_vec2(self, v: Vec2) -> Vec2 {
        mat2_mul_vec2(self, v)
    }

    pub fn transpose(self) -> Self {
        mat2_transpose(self)
    }

    pub fn determinant(self) -> f32 {
        mat2_determinant(self)
    }

    pub fn inverse(self) -> Self {
        mat2_inverse(self)
    }
}

impl std::ops::Mul for Mat2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        mat2_mul(self, rhs)
    }
}

impl std::ops::Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        mat2_mul_vec2(self, rhs)
    }
}
