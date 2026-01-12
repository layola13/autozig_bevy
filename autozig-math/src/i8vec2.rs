use autozig::include_zig;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I8Vec2 {
    pub x: i8,
    pub y: i8,
}

include_zig!("zig/i8vec2.zig", {
    fn i8vec2_new(x: i8, y: i8) -> I8Vec2;
    fn i8vec2_splat(v: i8) -> I8Vec2;
    fn i8vec2_add(a: I8Vec2, b: I8Vec2) -> I8Vec2;
    fn i8vec2_sub(a: I8Vec2, b: I8Vec2) -> I8Vec2;
    fn i8vec2_mul(a: I8Vec2, b: I8Vec2) -> I8Vec2;
    fn i8vec2_div(a: I8Vec2, b: I8Vec2) -> I8Vec2;
    fn i8vec2_dot(a: I8Vec2, b: I8Vec2) -> i8;
    fn i8vec2_min(a: I8Vec2, b: I8Vec2) -> I8Vec2;
    fn i8vec2_max(a: I8Vec2, b: I8Vec2) -> I8Vec2;
    fn i8vec2_clamp(v: I8Vec2, min: I8Vec2, max: I8Vec2) -> I8Vec2;
});

impl I8Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub const X: Self = Self { x: 1, y: 0 };
    pub const Y: Self = Self { x: 0, y: 1 };

    #[inline]
    pub const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn splat(v: i8) -> Self {
        i8vec2_splat(v)
    }

    #[inline]
    pub fn dot(self, other: Self) -> i8 {
        i8vec2_dot(self, other)
    }

    #[inline]
    pub fn min(self, other: Self) -> Self {
        i8vec2_min(self, other)
    }

    #[inline]
    pub fn max(self, other: Self) -> Self {
        i8vec2_max(self, other)
    }

    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        i8vec2_clamp(self, min, max)
    }
}

impl std::ops::Add for I8Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        i8vec2_add(self, other)
    }
}

impl std::ops::Sub for I8Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        i8vec2_sub(self, other)
    }
}

impl std::ops::Mul for I8Vec2 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        i8vec2_mul(self, other)
    }
}

impl std::ops::Div for I8Vec2 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        i8vec2_div(self, other)
    }
}