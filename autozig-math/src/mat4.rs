use autozig::include_zig;
use crate::{Vec3, Vec4, Quat};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub cols: [[f32; 4]; 4],
}

include_zig!("zig/mat4.zig", {
    fn mat4_identity() -> Mat4;
    fn mat4_from_translation(translation: Vec3) -> Mat4;
    fn mat4_from_scale(scale: Vec3) -> Mat4;
    fn mat4_from_quat(q: Quat) -> Mat4;
    fn mat4_from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Mat4;
    fn mat4_perspective_rh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> Mat4;
    fn mat4_perspective_lh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> Mat4;
    fn mat4_orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> Mat4;
    fn mat4_look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Mat4;
    fn mat4_look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Mat4;
    fn mat4_mul(self_: Mat4, other: Mat4) -> Mat4;
    fn mat4_mul_vec3(self_: Mat4, vec: Vec3) -> Vec3;
    fn mat4_mul_vec4(self_: Mat4, vec: Vec4) -> Vec4;
    fn mat4_transpose(self_: Mat4) -> Mat4;
    fn mat4_transform_point(self_: Mat4, point: Vec3) -> Vec3;
    fn mat4_transform_vector(self_: Mat4, vec: Vec3) -> Vec3;
    fn mat4_determinant(self_: Mat4) -> f32;
    fn mat4_inverse(self_: Mat4) -> Mat4;
});

impl Mat4 {
    pub const IDENTITY: Self = Self {
        cols: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub const ZERO: Self = Self {
        cols: [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ],
    };

    pub fn identity() -> Self {
        mat4_identity()
    }

    pub fn from_translation(translation: Vec3) -> Self {
        mat4_from_translation(translation)
    }

    pub fn from_scale(scale: Vec3) -> Self {
        mat4_from_scale(scale)
    }

    pub fn from_quat(q: Quat) -> Self {
        mat4_from_quat(q)
    }

    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        mat4_from_scale_rotation_translation(scale, rotation, translation)
    }

    pub fn perspective_rh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> Self {
        mat4_perspective_rh(fov_y, aspect, z_near, z_far)
    }

    pub fn perspective_lh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> Self {
        mat4_perspective_lh(fov_y, aspect, z_near, z_far)
    }

    pub fn orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> Self {
        mat4_orthographic_rh(left, right, bottom, top, z_near, z_far)
    }

    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        mat4_look_at_rh(eye, center, up)
    }

    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        mat4_look_at_lh(eye, center, up)
    }

    pub fn mul_vec3(self, vec: Vec3) -> Vec3 {
        mat4_mul_vec3(self, vec)
    }

    pub fn mul_vec4(self, vec: Vec4) -> Vec4 {
        mat4_mul_vec4(self, vec)
    }

    pub fn transpose(self) -> Self {
        mat4_transpose(self)
    }

    pub fn transform_point(self, point: Vec3) -> Vec3 {
        mat4_transform_point(self, point)
    }

    pub fn transform_vector(self, vec: Vec3) -> Vec3 {
        mat4_transform_vector(self, vec)
    }

    pub fn determinant(self) -> f32 {
        mat4_determinant(self)
    }

    pub fn inverse(self) -> Self {
        mat4_inverse(self)
    }
}

impl std::ops::Mul for Mat4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        mat4_mul(self, rhs)
    }
}

impl std::ops::Mul<Vec3> for Mat4 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        mat4_mul_vec3(self, rhs)
    }
}

impl std::ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        mat4_mul_vec4(self, rhs)
    }
}
