use autozig::include_zig;
use crate::{Vec3, Mat3};

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

    #[inline]
    pub fn identity() -> Self {
        quat_identity()
    }

    #[inline]
    pub fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        quat_from_xyzw(x, y, z, w)
    }

    #[inline]
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        quat_from_axis_angle(axis, angle)
    }

    #[inline]
    pub fn from_rotation_x(angle: f32) -> Self {
        quat_from_rotation_x(angle)
    }

    #[inline]
    pub fn from_rotation_y(angle: f32) -> Self {
        quat_from_rotation_y(angle)
    }

    #[inline]
    pub fn from_rotation_z(angle: f32) -> Self {
        quat_from_rotation_z(angle)
    }

    /// Creates a quaternion from a 3x3 rotation matrix.
    #[inline]
    pub fn from_mat3(m: &Mat3) -> Self {
        let m00 = m.cols[0][0];
        let m01 = m.cols[0][1];
        let m02 = m.cols[0][2];
        let m10 = m.cols[1][0];
        let m11 = m.cols[1][1];
        let m12 = m.cols[1][2];
        let m20 = m.cols[2][0];
        let m21 = m.cols[2][1];
        let m22 = m.cols[2][2];
        
        let trace = m00 + m11 + m22;
        
        if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            Self {
                w: 0.25 * s,
                x: (m21 - m12) / s,
                y: (m02 - m20) / s,
                z: (m10 - m01) / s,
            }
        } else if m00 > m11 && m00 > m22 {
            let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
            Self {
                w: (m21 - m12) / s,
                x: 0.25 * s,
                y: (m01 + m10) / s,
                z: (m02 + m20) / s,
            }
        } else if m11 > m22 {
            let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
            Self {
                w: (m02 - m20) / s,
                x: (m01 + m10) / s,
                y: 0.25 * s,
                z: (m12 + m21) / s,
            }
        } else {
            let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
            Self {
                w: (m10 - m01) / s,
                x: (m02 + m20) / s,
                y: (m12 + m21) / s,
                z: 0.25 * s,
            }
        }
    }

    #[inline]
    pub fn conjugate(self) -> Self {
        quat_conjugate(self)
    }

    #[inline]
    pub fn inverse(self) -> Self {
        quat_inverse(self)
    }

    #[inline]
    pub fn length(self) -> f32 {
        quat_length(self)
    }

    #[inline]
    pub fn normalize(self) -> Self {
        quat_normalize(self)
    }

    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        quat_dot(self, other)
    }

    #[inline]
    pub fn mul_vec3(self, v: Vec3) -> Vec3 {
        quat_mul_vec3(self, v)
    }

    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        quat_lerp(self, other, t)
    }

    #[inline]
    pub fn slerp(self, other: Self, t: f32) -> Self {
        quat_slerp(self, other, t)
    }

    /// Returns `true` if all components are finite.
    #[inline]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }
}

impl Default for Quat {
    fn default() -> Self {
        Self::IDENTITY
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

