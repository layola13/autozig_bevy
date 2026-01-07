use autozig::include_zig;
use crate::{Vec2, Rot2};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Isometry2d {
    pub rotation: Rot2,
    pub translation: Vec2,
}

include_zig!("zig/isometry2d.zig", {
    fn isometry2d_identity() -> Isometry2d;
    fn isometry2d_new(translation: Vec2, rotation: Rot2) -> Isometry2d;
});

impl Isometry2d {
    pub fn identity() -> Self {
        isometry2d_identity()
    }

    pub fn new(translation: Vec2, rotation: Rot2) -> Self {
        isometry2d_new(translation, rotation)
    }
}
