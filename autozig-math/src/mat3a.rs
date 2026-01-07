use autozig::include_zig;
use crate::{Quat, Vec3A};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3A {
    pub cols: [Vec3A; 3],
}

include_zig!("zig/mat3a.zig", {
    fn mat3a_identity() -> Mat3A;
    fn mat3a_from_quat(q: Quat) -> Mat3A;
    fn mat3a_mul(self_: Mat3A, other: Mat3A) -> Mat3A;
    fn mat3a_mul_vec3a(self_: Mat3A, v: Vec3A) -> Vec3A;
    fn mat3a_transpose(self_: Mat3A) -> Mat3A;
});

impl Mat3A {
    pub const IDENTITY: Self = Self {
        cols: [Vec3A::X, Vec3A::Y, Vec3A::Z],
    };

    pub fn identity() -> Self {
        mat3a_identity()
    }

    pub fn from_quat(q: Quat) -> Self {
        mat3a_from_quat(q)
    }

    pub fn mul_vec3a(self, v: Vec3A) -> Vec3A {
        mat3a_mul_vec3a(self, v)
    }

    pub fn transpose(self) -> Self {
        mat3a_transpose(self)
    }
}

impl std::ops::Mul for Mat3A {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        mat3a_mul(self, rhs)
    }
}

impl std::ops::Mul<Vec3A> for Mat3A {
    type Output = Vec3A;
    fn mul(self, rhs: Vec3A) -> Self::Output {
        mat3a_mul_vec3a(self, rhs)
    }
}
