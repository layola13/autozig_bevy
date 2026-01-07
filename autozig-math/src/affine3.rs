use autozig::include_zig;
use crate::{Vec3, Mat3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Affine3 {
    pub matrix3: Mat3,
    pub translation: Vec3,
}

include_zig!("zig/affine3.zig", {
    fn affine3_identity() -> Affine3;
    fn affine3_new(matrix3: Mat3, translation: Vec3) -> Affine3;
});

impl Affine3 {
    pub fn identity() -> Self {
        affine3_identity()
    }

    pub fn new(matrix3: Mat3, translation: Vec3) -> Self {
        affine3_new(matrix3, translation)
    }
}
