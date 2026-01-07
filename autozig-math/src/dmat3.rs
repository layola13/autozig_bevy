use autozig::include_zig;
use crate::DVec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DMat3 {
    pub cols: [[f64; 3]; 3],
}

include_zig!("zig/dmat3.zig", {
    fn dmat3_identity() -> DMat3;
    fn dmat3_from_scale(scale: DVec3) -> DMat3;
    fn dmat3_mul(self_: DMat3, other: DMat3) -> DMat3;
    fn dmat3_mul_dvec3(self_: DMat3, v: DVec3) -> DVec3;
    fn dmat3_transpose(self_: DMat3) -> DMat3;
    fn dmat3_determinant(self_: DMat3) -> f64;
});

impl DMat3 {
    pub const IDENTITY: Self = Self {
        cols: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };

    pub fn identity() -> Self {
        dmat3_identity()
    }

    pub fn from_scale(scale: DVec3) -> Self {
        dmat3_from_scale(scale)
    }

    pub fn mul_dvec3(self, v: DVec3) -> DVec3 {
        dmat3_mul_dvec3(self, v)
    }

    pub fn transpose(self) -> Self {
        dmat3_transpose(self)
    }

    pub fn determinant(self) -> f64 {
        dmat3_determinant(self)
    }
}

impl std::ops::Mul for DMat3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        dmat3_mul(self, rhs)
    }
}

impl std::ops::Mul<DVec3> for DMat3 {
    type Output = DVec3;
    fn mul(self, rhs: DVec3) -> Self::Output {
        dmat3_mul_dvec3(self, rhs)
    }
}
