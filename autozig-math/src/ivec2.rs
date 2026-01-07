use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}

include_zig!("zig/ivec2.zig", {
    fn ivec2_new(x: i32, y: i32) -> IVec2;
});

impl IVec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub const X: Self = Self { x: 1, y: 0 };
    pub const Y: Self = Self { x: 0, y: 1 };

    pub fn new(x: i32, y: i32) -> Self {
        ivec2_new(x, y)
    }
}
