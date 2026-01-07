use autozig::include_zig;
use crate::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dir2(pub Vec2);

include_zig!("zig/dir2.zig", {
    fn dir2_from_vec2(v: Vec2) -> Dir2;
    fn dir2_new(x: f32, y: f32) -> Dir2;
    fn dir2_perp(self_: Dir2) -> Dir2;
    fn dir2_rotate(self_: Dir2, angle: f32) -> Dir2;
    fn dir2_to_angle(self_: Dir2) -> f32;
    fn dir2_from_angle(angle: f32) -> Dir2;
});

impl Dir2 {
    pub const X: Self = Self(Vec2::X);
    pub const Y: Self = Self(Vec2::Y);
    pub const NEG_X: Self = Self(Vec2::NEG_X);
    pub const NEG_Y: Self = Self(Vec2::NEG_Y);

    pub fn new(v: Vec2) -> Self {
        dir2_from_vec2(v)
    }

    pub fn from_xy(x: f32, y: f32) -> Self {
        dir2_new(x, y)
    }

    pub fn from_angle(angle: f32) -> Self {
        dir2_from_angle(angle)
    }

    pub fn perp(self) -> Self {
        dir2_perp(self)
    }

    pub fn rotate(self, angle: f32) -> Self {
        dir2_rotate(self, angle)
    }

    pub fn to_angle(self) -> f32 {
        dir2_to_angle(self)
    }
}

impl std::ops::Neg for Dir2 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(Vec2 { x: -self.0.x, y: -self.0.y })
    }
}
