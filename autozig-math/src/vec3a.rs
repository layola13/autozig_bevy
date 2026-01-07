use autozig::include_zig;
use crate::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3A {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _pad: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dir3A(pub Vec3A);

include_zig!("zig/vec3a.zig", {
    fn vec3a_new(x: f32, y: f32, z: f32) -> Vec3A;
    fn vec3a_from_vec3(v: Vec3) -> Vec3A;
    fn vec3a_to_vec3(self_: Vec3A) -> Vec3;
    fn vec3a_dot(self_: Vec3A, other: Vec3A) -> f32;
    fn vec3a_cross(self_: Vec3A, other: Vec3A) -> Vec3A;
    fn vec3a_add(self_: Vec3A, other: Vec3A) -> Vec3A;
    fn vec3a_sub(self_: Vec3A, other: Vec3A) -> Vec3A;
    fn vec3a_mul_scalar(self_: Vec3A, s: f32) -> Vec3A;
    fn vec3a_length(self_: Vec3A) -> f32;
    fn vec3a_normalize(self_: Vec3A) -> Vec3A;
    fn vec3a_lerp(self_: Vec3A, other: Vec3A, t: f32) -> Vec3A;
    fn dir3a_new(x: f32, y: f32, z: f32) -> Dir3A;
    fn dir3a_from_vec3a(v: Vec3A) -> Dir3A;
});

impl Vec3A {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0, _pad: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0, _pad: 0.0 };
    pub const X: Self = Self { x: 1.0, y: 0.0, z: 0.0, _pad: 0.0 };
    pub const Y: Self = Self { x: 0.0, y: 1.0, z: 0.0, _pad: 0.0 };
    pub const Z: Self = Self { x: 0.0, y: 0.0, z: 1.0, _pad: 0.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        vec3a_new(x, y, z)
    }

    pub fn from_vec3(v: Vec3) -> Self {
        vec3a_from_vec3(v)
    }

    pub fn to_vec3(self) -> Vec3 {
        vec3a_to_vec3(self)
    }

    pub fn dot(self, other: Self) -> f32 {
        vec3a_dot(self, other)
    }

    pub fn cross(self, other: Self) -> Self {
        vec3a_cross(self, other)
    }

    pub fn length(self) -> f32 {
        vec3a_length(self)
    }

    pub fn normalize(self) -> Self {
        vec3a_normalize(self)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        vec3a_lerp(self, other, t)
    }
}

impl std::ops::Add for Vec3A {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        vec3a_add(self, rhs)
    }
}

impl std::ops::Sub for Vec3A {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        vec3a_sub(self, rhs)
    }
}

impl std::ops::Mul<f32> for Vec3A {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        vec3a_mul_scalar(self, rhs)
    }
}

impl From<Vec3> for Vec3A {
    fn from(v: Vec3) -> Self {
        vec3a_from_vec3(v)
    }
}

impl From<Vec3A> for Vec3 {
    fn from(v: Vec3A) -> Self {
        vec3a_to_vec3(v)
    }
}

impl Dir3A {
    pub const X: Self = Self(Vec3A::X);
    pub const Y: Self = Self(Vec3A::Y);
    pub const Z: Self = Self(Vec3A::Z);

    pub fn new(v: Vec3A) -> Self {
        dir3a_from_vec3a(v)
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        dir3a_new(x, y, z)
    }
}
