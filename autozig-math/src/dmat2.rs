use autozig::include_zig;
use crate::DVec2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DMat2 {
    pub cols: [[f64; 2]; 2],
}

include_zig!("zig/dmat2.zig", {
    fn dmat2_identity() -> DMat2;
    fn dmat2_from_angle(angle: f64) -> DMat2;
    fn dmat2_mul(self_: DMat2, other: DMat2) -> DMat2;
    fn dmat2_mul_dvec2(self_: DMat2, v: DVec2) -> DVec2;
    fn dmat2_transpose(self_: DMat2) -> DMat2;
    fn dmat2_determinant(self_: DMat2) -> f64;
    fn dmat2_inverse(self_: DMat2) -> DMat2;
});

impl DMat2 {
    pub const IDENTITY: Self = Self {
        cols: [[1.0, 0.0], [0.0, 1.0]],
    };

    pub fn identity() -> Self {
        dmat2_identity()
    }

    pub fn from_angle(angle: f64) -> Self {
        dmat2_from_angle(angle)
    }

    pub fn mul_dvec2(self, v: DVec2) -> DVec2 {
        dmat2_mul_dvec2(self, v)
    }

    pub fn transpose(self) -> Self {
        dmat2_transpose(self)
    }

    pub fn determinant(self) -> f64 {
        dmat2_determinant(self)
    }

    pub fn inverse(self) -> Self {
        dmat2_inverse(self)
    }
}

impl std::ops::Mul for DMat2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        dmat2_mul(self, rhs)
    }
}

impl std::ops::Mul<DVec2> for DMat2 {
    type Output = DVec2;
    fn mul(self, rhs: DVec2) -> Self::Output {
        dmat2_mul_dvec2(self, rhs)
    }
}
