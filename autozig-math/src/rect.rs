use autozig::include_zig;
use crate::Vec2;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

include_zig!("zig/rect.zig", {
    fn rect_new(min: Vec2, max: Vec2) -> Rect;
    fn rect_width(self_: Rect) -> f32;
    fn rect_height(self_: Rect) -> f32;
});

impl Rect {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        rect_new(min, max)
    }

    pub fn width(&self) -> f32 {
        rect_width(*self)
    }

    pub fn height(&self) -> f32 {
        rect_height(*self)
    }
}
