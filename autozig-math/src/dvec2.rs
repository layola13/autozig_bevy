use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DVec2 {
    pub x: f64,
    pub y: f64,
}

include_zig!("zig/dvec2.zig", {
    fn dvec2_new(x: f64, y: f64) -> DVec2;
    fn dvec2_dot(self_: DVec2, other: DVec2) -> f64;
    fn dvec2_add(self_: DVec2, other: DVec2) -> DVec2;
    fn dvec2_sub(self_: DVec2, other: DVec2) -> DVec2;
    fn dvec2_mul_scalar(self_: DVec2, s: f64) -> DVec2;
    fn dvec2_length(self_: DVec2) -> f64;
    fn dvec2_normalize(self_: DVec2) -> DVec2;
});

impl DVec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0 };

    pub fn new(x: f64, y: f64) -> Self {
        dvec2_new(x, y)
    }

    pub fn dot(self, other: Self) -> f64 {
        dvec2_dot(self, other)
    }

    pub fn length(self) -> f64 {
        dvec2_length(self)
    }

    pub fn normalize(self) -> Self {
        dvec2_normalize(self)
    }
}

impl std::ops::Add for DVec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        dvec2_add(self, rhs)
    }
}

impl std::ops::Sub for DVec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        dvec2_sub(self, rhs)
    }
}

impl std::ops::Mul<f64> for DVec2 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        dvec2_mul_scalar(self, rhs)
    }
}
