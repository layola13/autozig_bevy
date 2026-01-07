use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UVec4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

include_zig!("zig/uvec4.zig", {
    fn uvec4_new(x: u32, y: u32, z: u32, w: u32) -> UVec4;
    fn uvec4_splat(value: u32) -> UVec4;
    fn uvec4_add(self_: UVec4, other: UVec4) -> UVec4;
    fn uvec4_sub(self_: UVec4, other: UVec4) -> UVec4;
    fn uvec4_dot(self_: UVec4, other: UVec4) -> u32;
    fn uvec4_min(self_: UVec4, other: UVec4) -> UVec4;
    fn uvec4_max(self_: UVec4, other: UVec4) -> UVec4;
});

impl UVec4 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0, w: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1, w: 1 };
    pub const X: Self = Self { x: 1, y: 0, z: 0, w: 0 };
    pub const Y: Self = Self { x: 0, y: 1, z: 0, w: 0 };
    pub const Z: Self = Self { x: 0, y: 0, z: 1, w: 0 };
    pub const W: Self = Self { x: 0, y: 0, z: 0, w: 1 };

    pub fn new(x: u32, y: u32, z: u32, w: u32) -> Self {
        uvec4_new(x, y, z, w)
    }

    pub fn splat(value: u32) -> Self {
        uvec4_splat(value)
    }

    pub fn dot(self, other: Self) -> u32 {
        uvec4_dot(self, other)
    }

    pub fn min(self, other: Self) -> Self {
        uvec4_min(self, other)
    }

    pub fn max(self, other: Self) -> Self {
        uvec4_max(self, other)
    }
}

impl std::ops::Add for UVec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        uvec4_add(self, rhs)
    }
}

impl std::ops::Sub for UVec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        uvec4_sub(self, rhs)
    }
}
