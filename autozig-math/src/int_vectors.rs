//! Extended integer vector types (8-bit, 16-bit, 64-bit)
use autozig::include_zig;

// ============================================================================
// I8Vec types (8-bit signed integer vectors)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I8Vec2 {
    pub x: i8,
    pub y: i8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I8Vec3 {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I8Vec4 {
    pub x: i8,
    pub y: i8,
    pub z: i8,
    pub w: i8,
}

// ============================================================================
// I16Vec types (16-bit signed integer vectors)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I16Vec2 {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I16Vec3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I16Vec4 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}

// ============================================================================
// I64Vec types (64-bit signed integer vectors)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I64Vec2 {
    pub x: i64,
    pub y: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I64Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I64Vec4 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}

// ============================================================================
// U8Vec types (8-bit unsigned integer vectors)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U8Vec2 {
    pub x: u8,
    pub y: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U8Vec3 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U8Vec4 {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub w: u8,
}

// ============================================================================
// U16Vec types (16-bit unsigned integer vectors)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U16Vec2 {
    pub x: u16,
    pub y: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U16Vec3 {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U16Vec4 {
    pub x: u16,
    pub y: u16,
    pub z: u16,
    pub w: u16,
}

// ============================================================================
// U64Vec types (64-bit unsigned integer vectors)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U64Vec2 {
    pub x: u64,
    pub y: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U64Vec3 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct U64Vec4 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64,
}

// ============================================================================
// BVec SIMD-aligned types
// ============================================================================

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BVec3A {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    _pad: bool,
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BVec4A {
    pub x: bool,
    pub y: bool,
    pub z: bool,
    pub w: bool,
}

include_zig!("zig/int_vectors.zig", {
    fn i8vec2_new(x: i8, y: i8) -> I8Vec2;
    fn i8vec3_new(x: i8, y: i8, z: i8) -> I8Vec3;
    fn i8vec4_new(x: i8, y: i8, z: i8, w: i8) -> I8Vec4;
    fn i16vec2_new(x: i16, y: i16) -> I16Vec2;
    fn i16vec3_new(x: i16, y: i16, z: i16) -> I16Vec3;
    fn i16vec4_new(x: i16, y: i16, z: i16, w: i16) -> I16Vec4;
    fn i64vec2_new(x: i64, y: i64) -> I64Vec2;
    fn i64vec3_new(x: i64, y: i64, z: i64) -> I64Vec3;
    fn i64vec4_new(x: i64, y: i64, z: i64, w: i64) -> I64Vec4;
    fn u8vec2_new(x: u8, y: u8) -> U8Vec2;
    fn u8vec3_new(x: u8, y: u8, z: u8) -> U8Vec3;
    fn u8vec4_new(x: u8, y: u8, z: u8, w: u8) -> U8Vec4;
    fn u16vec2_new(x: u16, y: u16) -> U16Vec2;
    fn u16vec3_new(x: u16, y: u16, z: u16) -> U16Vec3;
    fn u16vec4_new(x: u16, y: u16, z: u16, w: u16) -> U16Vec4;
    fn u64vec2_new(x: u64, y: u64) -> U64Vec2;
    fn u64vec3_new(x: u64, y: u64, z: u64) -> U64Vec3;
    fn u64vec4_new(x: u64, y: u64, z: u64, w: u64) -> U64Vec4;
    fn bvec3a_new(x: bool, y: bool, z: bool) -> BVec3A;
    fn bvec4a_new(x: bool, y: bool, z: bool, w: bool) -> BVec4A;
});

// ============================================================================
// Implementations
// ============================================================================

impl I8Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub fn new(x: i8, y: i8) -> Self {
        i8vec2_new(x, y)
    }
}

impl I8Vec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        i8vec3_new(x, y, z)
    }
}

impl I8Vec4 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0, w: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1, w: 1 };
    pub fn new(x: i8, y: i8, z: i8, w: i8) -> Self {
        i8vec4_new(x, y, z, w)
    }
}

impl I16Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub fn new(x: i16, y: i16) -> Self {
        i16vec2_new(x, y)
    }
}

impl I16Vec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };
    pub fn new(x: i16, y: i16, z: i16) -> Self {
        i16vec3_new(x, y, z)
    }
}

impl I16Vec4 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0, w: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1, w: 1 };
    pub fn new(x: i16, y: i16, z: i16, w: i16) -> Self {
        i16vec4_new(x, y, z, w)
    }
}

impl I64Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub fn new(x: i64, y: i64) -> Self {
        i64vec2_new(x, y)
    }
}

impl I64Vec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        i64vec3_new(x, y, z)
    }
}

impl I64Vec4 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0, w: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1, w: 1 };
    pub fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        i64vec4_new(x, y, z, w)
    }
}

impl U8Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub fn new(x: u8, y: u8) -> Self {
        u8vec2_new(x, y)
    }
}

impl U8Vec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };
    pub fn new(x: u8, y: u8, z: u8) -> Self {
        u8vec3_new(x, y, z)
    }
}

impl U8Vec4 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0, w: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1, w: 1 };
    pub fn new(x: u8, y: u8, z: u8, w: u8) -> Self {
        u8vec4_new(x, y, z, w)
    }
}

impl U16Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub fn new(x: u16, y: u16) -> Self {
        u16vec2_new(x, y)
    }
}

impl U16Vec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };
    pub fn new(x: u16, y: u16, z: u16) -> Self {
        u16vec3_new(x, y, z)
    }
}

impl U16Vec4 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0, w: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1, w: 1 };
    pub fn new(x: u16, y: u16, z: u16, w: u16) -> Self {
        u16vec4_new(x, y, z, w)
    }
}

impl U64Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };
    pub const ONE: Self = Self { x: 1, y: 1 };
    pub fn new(x: u64, y: u64) -> Self {
        u64vec2_new(x, y)
    }
}

impl U64Vec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1 };
    pub fn new(x: u64, y: u64, z: u64) -> Self {
        u64vec3_new(x, y, z)
    }
}

impl U64Vec4 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0, w: 0 };
    pub const ONE: Self = Self { x: 1, y: 1, z: 1, w: 1 };
    pub fn new(x: u64, y: u64, z: u64, w: u64) -> Self {
        u64vec4_new(x, y, z, w)
    }
}

impl BVec3A {
    pub const FALSE: Self = Self { x: false, y: false, z: false, _pad: false };
    pub const TRUE: Self = Self { x: true, y: true, z: true, _pad: false };
    pub fn new(x: bool, y: bool, z: bool) -> Self {
        bvec3a_new(x, y, z)
    }
}

impl BVec4A {
    pub const FALSE: Self = Self { x: false, y: false, z: false, w: false };
    pub const TRUE: Self = Self { x: true, y: true, z: true, w: true };
    pub fn new(x: bool, y: bool, z: bool, w: bool) -> Self {
        bvec4a_new(x, y, z, w)
    }
}