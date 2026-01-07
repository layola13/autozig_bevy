use autozig::include_zig;
use crate::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicHermite3d {
    pub p0: Vec3,
    pub p1: Vec3,
    pub t0: Vec3,
    pub t1: Vec3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CatmullRom3d {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BSpline3d {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
}

include_zig!("zig/splines.zig", {
    fn cubic_hermite3d_new(p0: Vec3, p1: Vec3, t0: Vec3, t1: Vec3) -> CubicHermite3d;
    fn cubic_hermite3d_position(self_: CubicHermite3d, t: f32) -> Vec3;
    fn cubic_hermite3d_velocity(self_: CubicHermite3d, t: f32) -> Vec3;
    fn catmull_rom3d_new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> CatmullRom3d;
    fn catmull_rom3d_position(self_: CatmullRom3d, t: f32) -> Vec3;
    fn bspline3d_new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> BSpline3d;
    fn bspline3d_position(self_: BSpline3d, t: f32) -> Vec3;
});

impl CubicHermite3d {
    pub fn new(p0: Vec3, p1: Vec3, t0: Vec3, t1: Vec3) -> Self {
        cubic_hermite3d_new(p0, p1, t0, t1)
    }

    pub fn position(&self, t: f32) -> Vec3 {
        cubic_hermite3d_position(*self, t)
    }

    pub fn velocity(&self, t: f32) -> Vec3 {
        cubic_hermite3d_velocity(*self, t)
    }
}

impl CatmullRom3d {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
        catmull_rom3d_new(p0, p1, p2, p3)
    }

    pub fn position(&self, t: f32) -> Vec3 {
        catmull_rom3d_position(*self, t)
    }
}

impl BSpline3d {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) -> Self {
        bspline3d_new(p0, p1, p2, p3)
    }

    pub fn position(&self, t: f32) -> Vec3 {
        bspline3d_position(*self, t)
    }
}
