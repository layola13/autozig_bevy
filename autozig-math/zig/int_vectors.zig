//! Extended integer vector types implementation

const std = @import("std");

// ============================================================================
// I8Vec types
// ============================================================================

pub const I8Vec2 = extern struct {
    x: i8,
    y: i8,
};

pub const I8Vec3 = extern struct {
    x: i8,
    y: i8,
    z: i8,
};

pub const I8Vec4 = extern struct {
    x: i8,
    y: i8,
    z: i8,
    w: i8,
};

// ============================================================================
// I16Vec types
// ============================================================================

pub const I16Vec2 = extern struct {
    x: i16,
    y: i16,
};

pub const I16Vec3 = extern struct {
    x: i16,
    y: i16,
    z: i16,
};

pub const I16Vec4 = extern struct {
    x: i16,
    y: i16,
    z: i16,
    w: i16,
};

// ============================================================================
// I64Vec types
// ============================================================================

pub const I64Vec2 = extern struct {
    x: i64,
    y: i64,
};

pub const I64Vec3 = extern struct {
    x: i64,
    y: i64,
    z: i64,
};

pub const I64Vec4 = extern struct {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
};

// ============================================================================
// U8Vec types
// ============================================================================

pub const U8Vec2 = extern struct {
    x: u8,
    y: u8,
};

pub const U8Vec3 = extern struct {
    x: u8,
    y: u8,
    z: u8,
};

pub const U8Vec4 = extern struct {
    x: u8,
    y: u8,
    z: u8,
    w: u8,
};

// ============================================================================
// U16Vec types
// ============================================================================

pub const U16Vec2 = extern struct {
    x: u16,
    y: u16,
};

pub const U16Vec3 = extern struct {
    x: u16,
    y: u16,
    z: u16,
};

pub const U16Vec4 = extern struct {
    x: u16,
    y: u16,
    z: u16,
    w: u16,
};

// ============================================================================
// U64Vec types
// ============================================================================

pub const U64Vec2 = extern struct {
    x: u64,
    y: u64,
};

pub const U64Vec3 = extern struct {
    x: u64,
    y: u64,
    z: u64,
};

pub const U64Vec4 = extern struct {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
};

// ============================================================================
// BVec SIMD-aligned types
// ============================================================================

pub const BVec3A = extern struct {
    x: bool,
    y: bool,
    z: bool,
    _pad: bool,
};

pub const BVec4A = extern struct {
    x: bool,
    y: bool,
    z: bool,
    w: bool,
};

// ============================================================================
// FFI Exports
// ============================================================================

export fn i8vec2_new(x: i8, y: i8) I8Vec2 {
    return I8Vec2{ .x = x, .y = y };
}

export fn i8vec3_new(x: i8, y: i8, z: i8) I8Vec3 {
    return I8Vec3{ .x = x, .y = y, .z = z };
}

export fn i8vec4_new(x: i8, y: i8, z: i8, w: i8) I8Vec4 {
    return I8Vec4{ .x = x, .y = y, .z = z, .w = w };
}

export fn i16vec2_new(x: i16, y: i16) I16Vec2 {
    return I16Vec2{ .x = x, .y = y };
}

export fn i16vec3_new(x: i16, y: i16, z: i16) I16Vec3 {
    return I16Vec3{ .x = x, .y = y, .z = z };
}

export fn i16vec4_new(x: i16, y: i16, z: i16, w: i16) I16Vec4 {
    return I16Vec4{ .x = x, .y = y, .z = z, .w = w };
}

export fn i64vec2_new(x: i64, y: i64) I64Vec2 {
    return I64Vec2{ .x = x, .y = y };
}

export fn i64vec3_new(x: i64, y: i64, z: i64) I64Vec3 {
    return I64Vec3{ .x = x, .y = y, .z = z };
}

export fn i64vec4_new(x: i64, y: i64, z: i64, w: i64) I64Vec4 {
    return I64Vec4{ .x = x, .y = y, .z = z, .w = w };
}

export fn u8vec2_new(x: u8, y: u8) U8Vec2 {
    return U8Vec2{ .x = x, .y = y };
}

export fn u8vec3_new(x: u8, y: u8, z: u8) U8Vec3 {
    return U8Vec3{ .x = x, .y = y, .z = z };
}

export fn u8vec4_new(x: u8, y: u8, z: u8, w: u8) U8Vec4 {
    return U8Vec4{ .x = x, .y = y, .z = z, .w = w };
}

export fn u16vec2_new(x: u16, y: u16) U16Vec2 {
    return U16Vec2{ .x = x, .y = y };
}

export fn u16vec3_new(x: u16, y: u16, z: u16) U16Vec3 {
    return U16Vec3{ .x = x, .y = y, .z = z };
}

export fn u16vec4_new(x: u16, y: u16, z: u16, w: u16) U16Vec4 {
    return U16Vec4{ .x = x, .y = y, .z = z, .w = w };
}

export fn u64vec2_new(x: u64, y: u64) U64Vec2 {
    return U64Vec2{ .x = x, .y = y };
}

export fn u64vec3_new(x: u64, y: u64, z: u64) U64Vec3 {
    return U64Vec3{ .x = x, .y = y, .z = z };
}

export fn u64vec4_new(x: u64, y: u64, z: u64, w: u64) U64Vec4 {
    return U64Vec4{ .x = x, .y = y, .z = z, .w = w };
}

export fn bvec3a_new(x: bool, y: bool, z: bool) BVec3A {
    return BVec3A{ .x = x, .y = y, .z = z, ._pad = false };
}

export fn bvec4a_new(x: bool, y: bool, z: bool, w: bool) BVec4A {
    return BVec4A{ .x = x, .y = y, .z = z, .w = w };
}
