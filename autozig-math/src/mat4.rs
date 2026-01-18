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

    #[inline]
    pub fn identity() -> Self {
        mat4_identity()
    }

    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        mat4_from_translation(translation)
    }

    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        mat4_from_scale(scale)
    }

    #[inline]
    pub fn from_quat(q: Quat) -> Self {
        mat4_from_quat(q)
    }

    #[inline]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        mat4_from_scale_rotation_translation(scale, rotation, translation)
    }

    #[inline]
    pub fn perspective_rh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> Self {
        mat4_perspective_rh(fov_y, aspect, z_near, z_far)
    }

    #[inline]
    pub fn perspective_lh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> Self {
        mat4_perspective_lh(fov_y, aspect, z_near, z_far)
    }

    #[inline]
    pub fn orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) -> Self {
        mat4_orthographic_rh(left, right, bottom, top, z_near, z_far)
    }

    #[inline]
    pub fn look_at_rh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        mat4_look_at_rh(eye, center, up)
    }

    #[inline]
    pub fn look_at_lh(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        mat4_look_at_lh(eye, center, up)
    }

    #[inline]
    pub fn mul_vec3(self, vec: Vec3) -> Vec3 {
        mat4_mul_vec3(self, vec)
    }

    #[inline]
    pub fn mul_vec4(self, vec: Vec4) -> Vec4 {
        mat4_mul_vec4(self, vec)
    }

    #[inline]
    pub fn transpose(self) -> Self {
        mat4_transpose(self)
    }

    #[inline]
    pub fn transform_point(self, point: Vec3) -> Vec3 {
        mat4_transform_point(self, point)
    }

    #[inline]
    pub fn transform_vector(self, vec: Vec3) -> Vec3 {
        mat4_transform_vector(self, vec)
    }

    #[inline]
    pub fn determinant(self) -> f32 {
        mat4_determinant(self)
    }

    #[inline]
    pub fn inverse(self) -> Self {
        mat4_inverse(self)
    }

    /// Creates a `Mat4` from a column-major `[f32; 16]` array.
    #[inline]
    pub fn from_cols_array(m: &[f32; 16]) -> Self {
        Self {
            cols: [
                [m[0], m[1], m[2], m[3]],
                [m[4], m[5], m[6], m[7]],
                [m[8], m[9], m[10], m[11]],
                [m[12], m[13], m[14], m[15]],
            ],
        }
    }

    /// Converts `self` to a column-major `[f32; 16]` array.
    #[inline]
    pub fn to_cols_array(&self) -> [f32; 16] {
        [
            self.cols[0][0], self.cols[0][1], self.cols[0][2], self.cols[0][3],
            self.cols[1][0], self.cols[1][1], self.cols[1][2], self.cols[1][3],
            self.cols[2][0], self.cols[2][1], self.cols[2][2], self.cols[2][3],
            self.cols[3][0], self.cols[3][1], self.cols[3][2], self.cols[3][3],
        ]
    }

    /// Extracts `scale`, `rotation` and `translation` from `self`.
    /// 
    /// The input matrix is expected to be a valid 3D affine transform.
    #[inline]
    pub fn to_scale_rotation_translation(&self) -> (Vec3, Quat, Vec3) {
        // Extract translation from column 3
        let translation = Vec3::new(self.cols[3][0], self.cols[3][1], self.cols[3][2]);
        
        // Extract scale from column lengths
        let scale_x = Vec3::new(self.cols[0][0], self.cols[0][1], self.cols[0][2]).length();
        let scale_y = Vec3::new(self.cols[1][0], self.cols[1][1], self.cols[1][2]).length();
        let scale_z = Vec3::new(self.cols[2][0], self.cols[2][1], self.cols[2][2]).length();
        let scale = Vec3::new(scale_x, scale_y, scale_z);
        
        // To get rotation, we need to remove scale from the rotation matrix
        let inv_scale_x = if scale_x != 0.0 { 1.0 / scale_x } else { 0.0 };
        let inv_scale_y = if scale_y != 0.0 { 1.0 / scale_y } else { 0.0 };
        let inv_scale_z = if scale_z != 0.0 { 1.0 / scale_z } else { 0.0 };
        
        // Normalized rotation matrix columns
        let m00 = self.cols[0][0] * inv_scale_x;
        let m01 = self.cols[0][1] * inv_scale_x;
        let m02 = self.cols[0][2] * inv_scale_x;
        let m10 = self.cols[1][0] * inv_scale_y;
        let m11 = self.cols[1][1] * inv_scale_y;
        let m12 = self.cols[1][2] * inv_scale_y;
        let m20 = self.cols[2][0] * inv_scale_z;
        let m21 = self.cols[2][1] * inv_scale_z;
        let m22 = self.cols[2][2] * inv_scale_z;
        
        // Convert rotation matrix to quaternion
        let rotation = quat_from_rotation_matrix(m00, m01, m02, m10, m11, m12, m20, m21, m22);
        
        (scale, rotation, translation)
    }
}

/// Convert rotation matrix to quaternion (helper function)
fn quat_from_rotation_matrix(
    m00: f32, m01: f32, m02: f32,
    m10: f32, m11: f32, m12: f32,
    m20: f32, m21: f32, m22: f32,
) -> Quat {
    let trace = m00 + m11 + m22;
    
    if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        Quat {
            w: 0.25 * s,
            x: (m21 - m12) / s,
            y: (m02 - m20) / s,
            z: (m10 - m01) / s,
        }
    } else if m00 > m11 && m00 > m22 {
        let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
        Quat {
            w: (m21 - m12) / s,
            x: 0.25 * s,
            y: (m01 + m10) / s,
            z: (m02 + m20) / s,
        }
    } else if m11 > m22 {
        let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
        Quat {
            w: (m02 - m20) / s,
            x: (m01 + m10) / s,
            y: 0.25 * s,
            z: (m12 + m21) / s,
        }
    } else {
        let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
        Quat {
            w: (m10 - m01) / s,
            x: (m02 + m20) / s,
            y: (m12 + m21) / s,
            z: 0.25 * s,
        }
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::IDENTITY
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

