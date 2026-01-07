use autozig::include_zig;
use crate::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBezier3d {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
}

include_zig!("zig/curve.zig", {
    fn cubic_bezier3d_new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> CubicBezier3d;
    fn cubic_bezier3d_position(self_: CubicBezier3d, t: f32) -> Vec3;
});

impl CubicBezier3d {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
        cubic_bezier3d_new(p0, p1, p2, p3)
    }

    pub fn position(&self, t: f32) -> Vec3 {
        cubic_bezier3d_position(*self, t)
    }
}
