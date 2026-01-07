use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IVec4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

include_zig!("zig/ivec4.zig", {
    fn ivec4_new(x: i32, y: i32, z: i32, w: i32) -> IVec4;
    fn ivec4_splat(value: i32) -> IVec4;
    fn ivec4_add(self_: IVec4, other: IVec4) -> IVec4;
    fn ivec4_sub(self_: IVec4, other: IVec4) -> IVec4;
    fn ivec4_dot(self_: IVec4, other: IVec4) -> i32;
    fn ivec4_min(self_: IVec4, other: IVec4) -> IVec4;
    fn ivec4_max(self_: IVec4, other: IVec4) -> IVec4;
    fn ivec4_abs(self_: IVec4) -> IVec4;
});

impl IVec4 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0, w: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1, w: 1 };
    pub const X: Self = Self { x: 1, y: 0, z: 0, w: 0 };
    pub const Y: Self = Self { x: 0, y: 1, z: 0, w: 0 };
    pub const Z: Self = Self { x: 0, y: 0, z: 1, w: 0 };
    pub const W: Self = Self { x: 0, y: 0, z: 0, w: 1 };

    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        ivec4_new(x, y, z, w)
    }

    pub fn splat(value: i32) -> Self {
        ivec4_splat(value)
    }

    pub fn dot(self, other: Self) -> i32 {
        ivec4_dot(self, other)
    }

    pub fn min(self, other: Self) -> Self {
        ivec4_min(self, other)
    }

    pub fn max(self, other: Self) -> Self {
        ivec4_max(self, other)
    }

    pub fn abs(self) -> Self {
        ivec4_abs(self)
    }
}

impl std::ops::Add for IVec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        ivec4_add(self, rhs)
    }
}

impl std::ops::Sub for IVec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        ivec4_sub(self, rhs)
    }
}
