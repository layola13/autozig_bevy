//! Extended affine transform types (double precision)
use autozig::include_zig;
use crate::{DVec2, DVec3, DMat3, DMat4};

// ============================================================================
// DAffine types (double precision affine transforms)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DAffine2 {
    pub matrix2: DMat3,
    pub translation: DVec2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DAffine3 {
    pub matrix3: DMat4,
    pub translation: DVec3,
}

include_zig!("zig/affine_ext.zig", {
    fn daffine2_identity() -> DAffine2;
    fn daffine3_identity() -> DAffine3;
});

impl DAffine2 {
    pub fn identity() -> Self {
        daffine2_identity()
    }
}

impl DAffine3 {
    pub fn identity() -> Self {
        daffine3_identity()
    }
}