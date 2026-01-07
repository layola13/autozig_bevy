use autozig::include_zig;
use crate::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

include_zig!("zig/vec4.zig", {
    fn vec4_new(x: f32, y: f32, z: f32, w: f32) -> Vec4;
    fn vec4_splat(value: f32) -> Vec4;
    fn vec4_dot(self_: Vec4, other: Vec4) -> f32;
    fn vec4_add(self_: Vec4, other: Vec4) -> Vec4;
    fn vec4_sub(self_: Vec4, other: Vec4) -> Vec4;
    fn vec4_mul_scalar(self_: Vec4, s: f32) -> Vec4;
    fn vec4_length(self_: Vec4) -> f32;
    fn vec4_length_squared(self_: Vec4) -> f32;
    fn vec4_normalize(self_: Vec4) -> Vec4;
    fn vec4_lerp(self_: Vec4, other: Vec4, t: f32) -> Vec4;
    fn vec4_min(self_: Vec4, other: Vec4) -> Vec4;
    fn vec4_max(self_: Vec4, other: Vec4) -> Vec4;
    fn vec4_abs(self_: Vec4) -> Vec4;
    fn vec4_truncate(self_: Vec4) -> Vec3;
});

impl Vec4 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0, w: 1.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0, w: 0.0 };
    pub const W: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        vec4_new(x, y, z, w)
    }

    pub fn splat(value: f32) -> Self {
        vec4_splat(value)
    }

    pub fn dot(self, other: Self) -> f32 {
        vec4_dot(self, other)
    }

    pub fn length(self) -> f32 {
        vec4_length(self)
    }

    pub fn length_squared(self) -> f32 {
        vec4_length_squared(self)
    }

    pub fn normalize(self) -> Self {
        vec4_normalize(self)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        vec4_lerp(self, other, t)
    }

    pub fn min(self, other: Self) -> Self {
        vec4_min(self, other)
    }

    pub fn max(self, other: Self) -> Self {
        vec4_max(self, other)
    }

    pub fn abs(self) -> Self {
        vec4_abs(self)
    }

    pub fn truncate(self) -> Vec3 {
        vec4_truncate(self)
    }
}

impl std::ops::Add for Vec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vec4_add(self, rhs)
    }
}

impl std::ops::Sub for Vec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vec4_sub(self, rhs)
    }
}

impl std::ops::Mul<f32> for Vec4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        vec4_mul_scalar(self, rhs)
    }
}

impl std::ops::Mul<Vec4> for f32 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        vec4_mul_scalar(rhs, self)
    }
}

impl std::ops::Neg for Vec4 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}
