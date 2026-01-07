use autozig::include_zig;
use crate::{DVec3, DVec4};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DMat4 {
    pub cols: [[f64; 4]; 4],
}

include_zig!("zig/dmat4.zig", {
    fn dmat4_identity() -> DMat4;
    fn dmat4_from_translation(translation: DVec3) -> DMat4;
    fn dmat4_from_scale(scale: DVec3) -> DMat4;
    fn dmat4_mul(self_: DMat4, other: DMat4) -> DMat4;
    fn dmat4_mul_dvec4(self_: DMat4, vec: DVec4) -> DVec4;
    fn dmat4_transpose(self_: DMat4) -> DMat4;
    fn dmat4_transform_point(self_: DMat4, point: DVec3) -> DVec3;
});

impl DMat4 {
    pub const IDENTITY: Self = Self {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub fn identity() -> Self {
        dmat4_identity()
    }

    pub fn from_translation(translation: DVec3) -> Self {
        dmat4_from_translation(translation)
    }

    pub fn from_scale(scale: DVec3) -> Self {
        dmat4_from_scale(scale)
    }

    pub fn mul_dvec4(self, vec: DVec4) -> DVec4 {
        dmat4_mul_dvec4(self, vec)
    }

    pub fn transpose(self) -> Self {
        dmat4_transpose(self)
    }

    pub fn transform_point(self, point: DVec3) -> DVec3 {
        dmat4_transform_point(self, point)
    }
}

impl std::ops::Mul for DMat4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        dmat4_mul(self, rhs)
    }
}

impl std::ops::Mul<DVec4> for DMat4 {
    type Output = DVec4;
    fn mul(self, rhs: DVec4) -> Self::Output {
        dmat4_mul_dvec4(self, rhs)
    }
}
