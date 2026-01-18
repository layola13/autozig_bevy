use autozig::include_zig;
use crate::{Vec3, Quat};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Isometry3d {
    pub rotation: Quat,
    pub translation: Vec3,
}

include_zig!("zig/isometry3d.zig", {
    fn isometry3d_identity() -> Isometry3d;
    fn isometry3d_new(translation: Vec3, rotation: Quat) -> Isometry3d;
});

impl Isometry3d {
    pub const IDENTITY: Self = Self {
        rotation: Quat::IDENTITY,
        translation: Vec3::ZERO,
    };

    #[inline]
    pub fn identity() -> Self {
        isometry3d_identity()
    }

    #[inline]
    pub fn new(translation: Vec3, rotation: Quat) -> Self {
        isometry3d_new(translation, rotation)
    }

    /// Creates an isometry from the given `translation` and `rotation`.
    #[inline]
    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Self { rotation, translation }
    }

    /// Returns the translation component.
    #[inline]
    pub fn translation(&self) -> Vec3 {
        self.translation
    }

    /// Returns the rotation component.
    #[inline]
    pub fn rotation(&self) -> Quat {
        self.rotation
    }
}

impl Default for Isometry3d {
    fn default() -> Self {
        Self::IDENTITY
    }
}

