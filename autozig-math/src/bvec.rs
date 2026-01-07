use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BVec2 {
    pub x: bool,
    pub y: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BVec3 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BVec4 {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub w: bool,
}

include_zig!("zig/bvec.zig", {
    fn bvec2_new(x: bool, y: bool) -> BVec2;
    fn bvec2_all(self_: BVec2) -> bool;
    fn bvec2_any(self_: BVec2) -> bool;
    fn bvec3_new(x: bool, y: bool, z: bool) -> BVec3;
    fn bvec3_all(self_: BVec3) -> bool;
    fn bvec3_any(self_: BVec3) -> bool;
    fn bvec4_new(x: bool, y: bool, z: bool, w: bool) -> BVec4;
    fn bvec4_all(self_: BVec4) -> bool;
    fn bvec4_any(self_: BVec4) -> bool;
});

impl BVec2 {
    pub const FALSE: Self = Self { x: false, y: false };
    pub const TRUE: Self = Self { x: true, y: true };

    pub fn new(x: bool, y: bool) -> Self {
        bvec2_new(x, y)
    }

    pub fn all(self) -> bool {
        bvec2_all(self)
    }

    pub fn any(self) -> bool {
        bvec2_any(self)
    }
}

impl BVec3 {
    pub const FALSE: Self = Self { x: false, y: false, z: false };
    pub const TRUE: Self = Self { x: true, y: true, z: true };

    pub fn new(x: bool, y: bool, z: bool) -> Self {
        bvec3_new(x, y, z)
    }

    pub fn all(self) -> bool {
        bvec3_all(self)
    }

    pub fn any(self) -> bool {
        bvec3_any(self)
    }
}

impl BVec4 {
    pub const FALSE: Self = Self { x: false, y: false, z: false, w: false };
    pub const TRUE: Self = Self { x: true, y: true, z: true, w: true };

    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        bvec4_new(x, y, z, w)
    }

    pub fn all(self) -> bool {
        bvec4_all(self)
    }

    pub fn any(self) -> bool {
        bvec4_any(self)
    }
}
