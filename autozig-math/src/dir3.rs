use autozig::include_zig;
use crate::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dir3(pub Vec3);

include_zig!("zig/dir3.zig", {
    fn dir3_from_vec3(v: Vec3) -> Dir3;
    fn dir3_new(x: f32, y: f32, z: f32) -> Dir3;
    fn dir3_neg(self_: Dir3) -> Dir3;
});

impl Dir3 {
    pub const X: Self = Self(Vec3::X);
    pub const Y: Self = Self(Vec3::Y);
    pub const Z: Self = Self(Vec3::Z);
    pub const NEG_X: Self = Self(Vec3::NEG_X);
    pub const NEG_Y: Self = Self(Vec3::NEG_Y);
    pub const NEG_Z: Self = Self(Vec3::NEG_Z);

    pub fn new(v: Vec3) -> Self {
        dir3_from_vec3(v)
    }

    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        dir3_new(x, y, z)
    }
}

impl std::ops::Neg for Dir3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        dir3_neg(self)
    }
}
