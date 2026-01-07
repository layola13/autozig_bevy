use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

include_zig!("zig/dvec3.zig", {
    fn dvec3_new(x: f64, y: f64, z: f64) -> DVec3;
    fn dvec3_dot(self_: DVec3, other: DVec3) -> f64;
    fn dvec3_cross(self_: DVec3, other: DVec3) -> DVec3;
    fn dvec3_add(self_: DVec3, other: DVec3) -> DVec3;
    fn dvec3_sub(self_: DVec3, other: DVec3) -> DVec3;
    fn dvec3_mul_scalar(self_: DVec3, s: f64) -> DVec3;
    fn dvec3_length(self_: DVec3) -> f64;
    fn dvec3_normalize(self_: DVec3) -> DVec3;
});

impl DVec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0 };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        dvec3_new(x, y, z)
    }

    pub fn dot(self, other: Self) -> f64 {
        dvec3_dot(self, other)
    }

    pub fn cross(self, other: Self) -> Self {
        dvec3_cross(self, other)
    }

    pub fn length(self) -> f64 {
        dvec3_length(self)
    }

    pub fn normalize(self) -> Self {
        dvec3_normalize(self)
    }
}

impl std::ops::Add for DVec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        dvec3_add(self, rhs)
    }
}

impl std::ops::Sub for DVec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        dvec3_sub(self, rhs)
    }
}

impl std::ops::Mul<f64> for DVec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        dvec3_mul_scalar(self, rhs)
    }
}
