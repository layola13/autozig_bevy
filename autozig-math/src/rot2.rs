use autozig::include_zig;
use crate::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rot2 {
    pub c: f32, // cosine
    pub s: f32, // sine
}

include_zig!("zig/rot2.zig", {
    fn rot2_identity() -> Rot2;
    fn rot2_from_angle(angle: f32) -> Rot2;
    fn rot2_as_angle(self_: Rot2) -> f32;
    fn rot2_mul(self_: Rot2, other: Rot2) -> Rot2;
    fn rot2_inverse(self_: Rot2) -> Rot2;
    fn rot2_rotate_vec2(self_: Rot2, v: Vec2) -> Vec2;
    fn rot2_lerp(self_: Rot2, other: Rot2, t: f32) -> Rot2;
});

impl Rot2 {
    pub const IDENTITY: Self = Self { c: 1.0, s: 0.0 };

    pub fn identity() -> Self {
        rot2_identity()
    }

    pub fn from_angle(angle: f32) -> Self {
        rot2_from_angle(angle)
    }

    pub fn as_angle(self) -> f32 {
        rot2_as_angle(self)
    }

    pub fn inverse(self) -> Self {
        rot2_inverse(self)
    }

    pub fn rotate(self, v: Vec2) -> Vec2 {
        rot2_rotate_vec2(self, v)
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        rot2_lerp(self, other, t)
    }
}

impl std::ops::Mul for Rot2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        rot2_mul(self, rhs)
    }
}

impl std::ops::Mul<Vec2> for Rot2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        rot2_rotate_vec2(self, rhs)
    }
}
