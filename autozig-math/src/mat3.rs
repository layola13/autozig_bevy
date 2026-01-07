use autozig::include_zig;
use crate::{Vec3, Quat};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub cols: [[f32; 3]; 3],
}

include_zig!("zig/mat3.zig", {
    fn mat3_identity() -> Mat3;
    fn mat3_from_scale(scale: Vec3) -> Mat3;
    fn mat3_from_quat(q: Quat) -> Mat3;
    fn mat3_mul(self_: Mat3, other: Mat3) -> Mat3;
    fn mat3_mul_vec3(self_: Mat3, v: Vec3) -> Vec3;
    fn mat3_transpose(self_: Mat3) -> Mat3;
    fn mat3_determinant(self_: Mat3) -> f32;
    fn mat3_inverse(self_: Mat3) -> Mat3;
});

impl Mat3 {
    pub const IDENTITY: Self = Self {
        cols: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };

    pub const ZERO: Self = Self {
        cols: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
    };

    pub fn identity() -> Self {
        mat3_identity()
    }

    pub fn from_scale(scale: Vec3) -> Self {
        mat3_from_scale(scale)
    }

    pub fn from_quat(q: Quat) -> Self {
        mat3_from_quat(q)
    }

    pub fn mul_vec3(self, v: Vec3) -> Vec3 {
        mat3_mul_vec3(self, v)
    }

    pub fn transpose(self) -> Self {
        mat3_transpose(self)
    }

    pub fn determinant(self) -> f32 {
        mat3_determinant(self)
    }

    pub fn inverse(self) -> Self {
        mat3_inverse(self)
    }
}

impl std::ops::Mul for Mat3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        mat3_mul(self, rhs)
    }
}

impl std::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        mat3_mul_vec3(self, rhs)
    }
}
