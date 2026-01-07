use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

include_zig!("zig/uvec2.zig", {
    fn uvec2_new(x: u32, y: u32) -> UVec2;
});

impl UVec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub const X: Self = Self { x: 1, y: 0 };
    pub const Y: Self = Self { x: 0, y: 1 };

    pub fn new(x: u32, y: u32) -> Self {
        uvec2_new(x, y)
    }
}
