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
    pub fn identity() -> Self {
        isometry3d_identity()
    }

    pub fn new(translation: Vec3, rotation: Quat) -> Self {
        isometry3d_new(translation, rotation)
    }
}
