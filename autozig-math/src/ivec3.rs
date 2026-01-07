use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

include_zig!("zig/ivec3.zig", {
    fn ivec3_new(x: i32, y: i32, z: i32) -> IVec3;
});

impl IVec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };
    pub const X: Self = Self { x: 1, y: 0, z: 0 };
    pub const Y: Self = Self { x: 0, y: 1, z: 0 };
    pub const Z: Self = Self { x: 0, y: 0, z: 1 };

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        ivec3_new(x, y, z)
    }
}
