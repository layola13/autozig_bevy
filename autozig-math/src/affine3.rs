use autozig::include_zig;
use crate::{Vec3, Mat3, Mat4, Quat};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Affine3A {
    pub matrix3: Mat3,
    pub translation: Vec3,
}

include_zig!("zig/affine3.zig", {
    fn affine3_identity() -> Affine3A;
    fn affine3_new(matrix3: Mat3, translation: Vec3) -> Affine3A;
});

impl Affine3A {
    pub const IDENTITY: Self = Self {
        matrix3: Mat3::IDENTITY,
        translation: Vec3::ZERO,
    };

    #[inline]
    pub fn identity() -> Self {
        affine3_identity()
    }

    #[inline]
    pub fn new(matrix3: Mat3, translation: Vec3) -> Self {
        affine3_new(matrix3, translation)
    }

    /// Creates an affine transform from the given 3D `translation`.
    #[inline]
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            matrix3: Mat3::IDENTITY,
            translation,
        }
    }

    /// Creates an affine transform from the given 3D `rotation` and `translation`.
    #[inline]
    pub fn from_rotation_translation(rotation: Quat, translation: Vec3) -> Self {
        Self {
            matrix3: Mat3::from_quat(rotation),
            translation,
        }
    }

    /// Creates an affine transform from the given 3D `scale`, `rotation` and `translation`.
    #[inline]
    pub fn from_scale_rotation_translation(scale: Vec3, rotation: Quat, translation: Vec3) -> Self {
        let rotation_matrix = Mat3::from_quat(rotation);
        // Scale each column of the rotation matrix
        let matrix3 = Mat3 {
            cols: [
                [rotation_matrix.cols[0][0] * scale.x, rotation_matrix.cols[0][1] * scale.x, rotation_matrix.cols[0][2] * scale.x],
                [rotation_matrix.cols[1][0] * scale.y, rotation_matrix.cols[1][1] * scale.y, rotation_matrix.cols[1][2] * scale.y],
                [rotation_matrix.cols[2][0] * scale.z, rotation_matrix.cols[2][1] * scale.z, rotation_matrix.cols[2][2] * scale.z],
            ],
        };
        Self { matrix3, translation }
    }

    /// Creates an affine transform from the given 3D `scale`.
    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            matrix3: Mat3::from_scale(scale),
            translation: Vec3::ZERO,
        }
    }

    /// Creates an affine transform from the given 4x4 matrix.
    #[inline]
    pub fn from_mat4(m: Mat4) -> Self {
        Self {
            matrix3: Mat3 {
                cols: [
                    [m.cols[0][0], m.cols[0][1], m.cols[0][2]],
                    [m.cols[1][0], m.cols[1][1], m.cols[1][2]],
                    [m.cols[2][0], m.cols[2][1], m.cols[2][2]],
                ],
            },
            translation: Vec3::new(m.cols[3][0], m.cols[3][1], m.cols[3][2]),
        }
    }

    /// Returns the inverse of this affine transform.
    #[inline]
    pub fn inverse(&self) -> Self {
        let inv_matrix3 = self.matrix3.inverse();
        let inv_translation = inv_matrix3.mul_vec3(-self.translation);
        Self {
            matrix3: inv_matrix3,
            translation: inv_translation,
        }
    }

    /// Returns the `scale`, `rotation` and `translation` components of this affine transform.
    #[inline]
    pub fn to_scale_rotation_translation(&self) -> (Vec3, Quat, Vec3) {
        // Extract scale from column lengths
        let scale_x = Vec3::new(self.matrix3.cols[0][0], self.matrix3.cols[0][1], self.matrix3.cols[0][2]).length();
        let scale_y = Vec3::new(self.matrix3.cols[1][0], self.matrix3.cols[1][1], self.matrix3.cols[1][2]).length();
        let scale_z = Vec3::new(self.matrix3.cols[2][0], self.matrix3.cols[2][1], self.matrix3.cols[2][2]).length();
        
        // Handle negative scale (determinant < 0 means odd number of reflections)
        let det = self.matrix3.determinant();
        let scale = if det < 0.0 {
            Vec3::new(-scale_x, scale_y, scale_z)
        } else {
            Vec3::new(scale_x, scale_y, scale_z)
        };
        
        // Remove scale to get rotation matrix
        let inv_scale_x = if scale.x.abs() > 1e-10 { 1.0 / scale.x } else { 0.0 };
        let inv_scale_y = if scale.y.abs() > 1e-10 { 1.0 / scale.y } else { 0.0 };
        let inv_scale_z = if scale.z.abs() > 1e-10 { 1.0 / scale.z } else { 0.0 };
        
        let rotation_matrix = Mat3 {
            cols: [
                [self.matrix3.cols[0][0] * inv_scale_x, self.matrix3.cols[0][1] * inv_scale_x, self.matrix3.cols[0][2] * inv_scale_x],
                [self.matrix3.cols[1][0] * inv_scale_y, self.matrix3.cols[1][1] * inv_scale_y, self.matrix3.cols[1][2] * inv_scale_y],
                [self.matrix3.cols[2][0] * inv_scale_z, self.matrix3.cols[2][1] * inv_scale_z, self.matrix3.cols[2][2] * inv_scale_z],
            ],
        };
        
        let rotation = Quat::from_mat3(&rotation_matrix);
        
        (scale, rotation, self.translation)
    }

    /// Transforms the given 3D point, applying translation.
    #[inline]
    pub fn transform_point3(&self, point: Vec3) -> Vec3 {
        self.matrix3.mul_vec3(point) + self.translation
    }

    /// Transforms the given 3D vector (no translation).
    #[inline]
    pub fn transform_vector3(&self, vector: Vec3) -> Vec3 {
        self.matrix3.mul_vec3(vector)
    }
}

impl Default for Affine3A {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<Mat4> for Affine3A {
    fn from(m: Mat4) -> Self {
        Self::from_mat4(m)
    }
}

impl From<Affine3A> for Mat4 {
    fn from(a: Affine3A) -> Self {
        Mat4 {
            cols: [
                [a.matrix3.cols[0][0], a.matrix3.cols[0][1], a.matrix3.cols[0][2], 0.0],
                [a.matrix3.cols[1][0], a.matrix3.cols[1][1], a.matrix3.cols[1][2], 0.0],
                [a.matrix3.cols[2][0], a.matrix3.cols[2][1], a.matrix3.cols[2][2], 0.0],
                [a.translation.x, a.translation.y, a.translation.z, 1.0],
            ],
        }
    }
}

impl std::ops::Mul for Affine3A {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            matrix3: self.matrix3 * rhs.matrix3,
            translation: self.matrix3.mul_vec3(rhs.translation) + self.translation,
        }
    }
}

