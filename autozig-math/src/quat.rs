use autozig::include_zig;
use crate::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

include_zig!("zig/quat.zig", {
    fn quat_identity() -> Quat;
    fn quat_from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Quat;
    fn quat_from_axis_angle(axis: Vec3, angle: f32) -> Quat;
    fn quat_from_rotation_x(angle: f32) -> Quat;
    fn quat_from_rotation_y(angle: f32) -> Quat;
    fn quat_from_rotation_z(angle: f32) -> Quat;
    fn quat_mul(self_: Quat, other: Quat) -> Quat;
    fn quat_conjugate(self_: Quat) -> Quat;
    fn quat_inverse(self_: Quat) -> Quat;
    fn quat_length(self_: Quat) -> f32;
    fn quat_normalize(self_: Quat) -> Quat;
    fn quat_dot(self_: Quat, other: Quat) -> f32;
    fn quat_mul_vec3(self_: Quat, v: Vec3) -> Vec3;
    fn quat_lerp(self_: Quat, other: Quat, t: f32) -> Quat;
    fn quat_slerp(self_: Quat, other: Quat, t: f32) -> Quat;
});

impl Quat {
    pub const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn identity() -> Self {
        quat_identity()
    }

    pub fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        quat_from_xyzw(x, y, z, w)
    }

    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        quat_from_axis_angle(axis, angle)
    }

    pub fn from_rotation_x(angle: f32) -> Self {
        quat_from_rotation_x(angle)
    }

    pub fn from_rotation_y(angle: f32) -> Self {
        quat_from_rotation_y(angle)
    }

    pub fn from_rotation_z(angle: f32) -> Self {
        quat_from_rotation_z(angle)
    }

    pub fn conjugate(self) -> Self {
        quat_conjugate(self)
    }

    pub fn inverse(self) -> Self {
        quat_inverse(self)
    }

    pub fn length(self) -> f32 {
        quat_length(self)
    }

    pub fn normalize(self) -> Self {
        quat_normalize(self)
    }

    pub fn dot(self, other: Self) -> f32 {
        quat_dot(self, other)
    }

    pub fn mul_vec3(self, v: Vec3) -> Vec3 {
        quat_mul_vec3(self, v)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        quat_lerp(self, other, t)
    }

    pub fn slerp(self, other: Self, t: f32) -> Self {
        quat_slerp(self, other, t)
    }
}

impl std::ops::Mul for Quat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        quat_mul(self, rhs)
    }
}

impl std::ops::Mul<Vec3> for Quat {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        quat_mul_vec3(self, rhs)
    }
}
