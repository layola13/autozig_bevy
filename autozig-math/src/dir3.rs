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

    /// Creates a [`Dir3`] by normalizing the given `Vec3`.
    #[inline]
    pub fn new(v: Vec3) -> Self {
        dir3_from_vec3(v)
    }

    /// Creates a [`Dir3`] from the given components without normalization.
    /// 
    /// # Safety
    /// The caller must ensure the input vector is already normalized.
    #[inline]
    pub fn new_unchecked(v: Vec3) -> Self {
        Self(v)
    }

    #[inline]
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        dir3_new(x, y, z)
    }

    /// Returns the inner [`Vec3`].
    #[inline]
    pub fn as_vec3(&self) -> Vec3 {
        self.0
    }
}

impl From<Dir3> for Vec3 {
    #[inline]
    fn from(dir: Dir3) -> Self {
        dir.0
    }
}

impl std::ops::Deref for Dir3 {
    type Target = Vec3;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Neg for Dir3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        dir3_neg(self)
    }
}

