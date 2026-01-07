use autozig::include_zig;
use crate::{IVec2, UVec2};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IRect {
    pub min: IVec2,
    pub max: IVec2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct URect {
    pub min: UVec2,
    pub max: UVec2,
}

include_zig!("zig/rects.zig", {
    fn irect_new(min: IVec2, max: IVec2) -> IRect;
    fn irect_width(self_: IRect) -> i32;
    fn irect_height(self_: IRect) -> i32;
    fn urect_new(min: UVec2, max: UVec2) -> URect;
    fn urect_width(self_: URect) -> u32;
    fn urect_height(self_: URect) -> u32;
});

impl IRect {
    pub fn new(min: IVec2, max: IVec2) -> Self { irect_new(min, max) }
    pub fn width(&self) -> i32 { irect_width(*self) }
    pub fn height(&self) -> i32 { irect_height(*self) }
}

impl URect {
    pub fn new(min: UVec2, max: UVec2) -> Self { urect_new(min, max) }
    pub fn width(&self) -> u32 { urect_width(*self) }
    pub fn height(&self) -> u32 { urect_height(*self) }
}
