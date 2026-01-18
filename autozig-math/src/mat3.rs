use autozig::include_zig;
use crate::{Vec3, Quat};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub cols: [[f32; 3]; 3],
}

include_zig!("zig/mat3.zig", {
    fn mat3_identity() -> Mat3;
    fn mat3_from_scale(scale: Vec3) -> Mat3;
    fn mat3_from_quat(q: Quat) -> Mat3;
    fn mat3_mul(self_: Mat3, other: Mat3) -> Mat3;
    fn mat3_mul_vec3(self_: Mat3, v: Vec3) -> Vec3;
    fn mat3_transpose(self_: Mat3) -> Mat3;
    fn mat3_determinant(self_: Mat3) -> f32;
    fn mat3_inverse(self_: Mat3) -> Mat3;
});

impl Mat3 {
    pub const IDENTITY: Self = Self {
        cols: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };

    pub const ZERO: Self = Self {
        cols: [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
    };

    #[inline]
    pub fn identity() -> Self {
        mat3_identity()
    }

    /// Creates a 3x3 matrix from three column vectors.
    #[inline]
    pub fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            cols: [
                [x_axis.x, x_axis.y, x_axis.z],
                [y_axis.x, y_axis.y, y_axis.z],
                [z_axis.x, z_axis.y, z_axis.z],
            ],
        }
    }

    /// Creates a 3x3 matrix from a scale vector.
    #[inline]
    pub fn from_scale(scale: Vec3) -> Self {
        mat3_from_scale(scale)
    }

    /// Creates a 3x3 matrix from a quaternion.
    #[inline]
    pub fn from_quat(q: Quat) -> Self {
        mat3_from_quat(q)
    }

    /// Returns the first column (X axis).
    #[inline]
    pub fn x_axis(&self) -> Vec3 {
        Vec3::new(self.cols[0][0], self.cols[0][1], self.cols[0][2])
    }

    /// Returns the second column (Y axis).
    #[inline]
    pub fn y_axis(&self) -> Vec3 {
        Vec3::new(self.cols[1][0], self.cols[1][1], self.cols[1][2])
    }

    /// Returns the third column (Z axis).
    #[inline]
    pub fn z_axis(&self) -> Vec3 {
        Vec3::new(self.cols[2][0], self.cols[2][1], self.cols[2][2])
    }

    #[inline]
    pub fn mul_vec3(self, v: Vec3) -> Vec3 {
        mat3_mul_vec3(self, v)
    }

    #[inline]
    pub fn transpose(self) -> Self {
        mat3_transpose(self)
    }

    #[inline]
    pub fn determinant(self) -> f32 {
        mat3_determinant(self)
    }

    #[inline]
    pub fn inverse(self) -> Self {
        mat3_inverse(self)
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl std::ops::Mul for Mat3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        mat3_mul(self, rhs)
    }
}

impl std::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        mat3_mul_vec3(self, rhs)
    }
}

