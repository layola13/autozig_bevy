use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

include_zig!("zig/dvec4.zig", {
    fn dvec4_new(x: f64, y: f64, z: f64, w: f64) -> DVec4;
    fn dvec4_dot(self_: DVec4, other: DVec4) -> f64;
    fn dvec4_add(self_: DVec4, other: DVec4) -> DVec4;
    fn dvec4_sub(self_: DVec4, other: DVec4) -> DVec4;
    fn dvec4_mul_scalar(self_: DVec4, s: f64) -> DVec4;
    fn dvec4_length(self_: DVec4) -> f64;
    fn dvec4_normalize(self_: DVec4) -> DVec4;
});

impl DVec4 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0, w: 0.0 };
    pub const W: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        dvec4_new(x, y, z, w)
    }

    pub fn dot(self, other: Self) -> f64 {
        dvec4_dot(self, other)
    }

    pub fn length(self) -> f64 {
        dvec4_length(self)
    }

    pub fn normalize(self) -> Self {
        dvec4_normalize(self)
    }
}

impl std::ops::Add for DVec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        dvec4_add(self, rhs)
    }
}

impl std::ops::Sub for DVec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        dvec4_sub(self, rhs)
    }
}

impl std::ops::Mul<f64> for DVec4 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        dvec4_mul_scalar(self, rhs)
    }
}
