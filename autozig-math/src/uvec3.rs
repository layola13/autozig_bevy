use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UVec3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

include_zig!("zig/uvec3.zig", {
    fn uvec3_new(x: u32, y: u32, z: u32) -> UVec3;
});

impl UVec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };
    pub const X: Self = Self { x: 1, y: 0, z: 0 };
    pub const Y: Self = Self { x: 0, y: 1, z: 0 };
    pub const Z: Self = Self { x: 0, y: 0, z: 1 };

    pub fn new(x: u32, y: u32, z: u32) -> Self {
        uvec3_new(x, y, z)
    }
}
