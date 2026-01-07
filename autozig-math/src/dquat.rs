use autozig::include_zig;
use crate::DVec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DQuat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

include_zig!("zig/dquat.zig", {
    fn dquat_identity() -> DQuat;
    fn dquat_from_xyzw(x: f64, y: f64, z: f64, w: f64) -> DQuat;
    fn dquat_from_axis_angle(axis: DVec3, angle: f64) -> DQuat;
    fn dquat_mul(self_: DQuat, other: DQuat) -> DQuat;
    fn dquat_conjugate(self_: DQuat) -> DQuat;
    fn dquat_inverse(self_: DQuat) -> DQuat;
    fn dquat_normalize(self_: DQuat) -> DQuat;
    fn dquat_length(self_: DQuat) -> f64;
    fn dquat_mul_dvec3(self_: DQuat, v: DVec3) -> DVec3;
    fn dquat_slerp(self_: DQuat, other: DQuat, t: f64) -> DQuat;
});

impl DQuat {
    pub const IDENTITY: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };

    pub fn identity() -> Self {
        dquat_identity()
    }

    pub fn from_xyzw(x: f64, y: f64, z: f64, w: f64) -> Self {
        dquat_from_xyzw(x, y, z, w)
    }

    pub fn from_axis_angle(axis: DVec3, angle: f64) -> Self {
        dquat_from_axis_angle(axis, angle)
    }

    pub fn conjugate(self) -> Self {
        dquat_conjugate(self)
    }

    pub fn inverse(self) -> Self {
        dquat_inverse(self)
    }

    pub fn normalize(self) -> Self {
        dquat_normalize(self)
    }

    pub fn length(self) -> f64 {
        dquat_length(self)
    }

    pub fn mul_dvec3(self, v: DVec3) -> DVec3 {
        dquat_mul_dvec3(self, v)
    }

    pub fn slerp(self, other: Self, t: f64) -> Self {
        dquat_slerp(self, other, t)
    }
}

impl std::ops::Mul for DQuat {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        dquat_mul(self, rhs)
    }
}

impl std::ops::Mul<DVec3> for DQuat {
    type Output = DVec3;
    fn mul(self, rhs: DVec3) -> Self::Output {
        dquat_mul_dvec3(self, rhs)
    }
}
