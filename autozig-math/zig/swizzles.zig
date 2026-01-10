const std = @import("std");
const Vec2 = @import("vec2.zig").Vec2;
const Vec3 = @import("vec3.zig").Vec3;
const Vec4 = @import("vec4.zig").Vec4;

/// Swizzle operations for Vec2
pub const Vec2Swizzles = struct {
    pub fn xx(v: Vec2) Vec2 {
        return Vec2{ .x = v.x, .y = v.x };
    }
    pub fn xy(v: Vec2) Vec2 {
        return v;
    }
    pub fn yx(v: Vec2) Vec2 {
        return Vec2{ .x = v.y, .y = v.x };
    }
    pub fn yy(v: Vec2) Vec2 {
        return Vec2{ .x = v.y, .y = v.y };
    }
    pub fn xxx(v: Vec2) Vec3 {
        return Vec3{ .x = v.x, .y = v.x, .z = v.x };
    }
    pub fn xxy(v: Vec2) Vec3 {
        return Vec3{ .x = v.x, .y = v.x, .z = v.y };
    }
    pub fn xyx(v: Vec2) Vec3 {
        return Vec3{ .x = v.x, .y = v.y, .z = v.x };
    }
    pub fn xyy(v: Vec2) Vec3 {
        return Vec3{ .x = v.x, .y = v.y, .z = v.y };
    }
};

/// Swizzle operations for Vec3
pub const Vec3Swizzles = struct {
    pub fn xx(v: Vec3) Vec2 {
        return Vec2{ .x = v.x, .y = v.x };
    }
    pub fn xy(v: Vec3) Vec2 {
        return Vec2{ .x = v.x, .y = v.y };
    }
    pub fn xz(v: Vec3) Vec2 {
        return Vec2{ .x = v.x, .y = v.z };
    }
    pub fn yx(v: Vec3) Vec2 {
        return Vec2{ .x = v.y, .y = v.x };
    }
    pub fn yy(v: Vec3) Vec2 {
        return Vec2{ .x = v.y, .y = v.y };
    }
    pub fn yz(v: Vec3) Vec2 {
        return Vec2{ .x = v.y, .y = v.z };
    }
    pub fn zx(v: Vec3) Vec2 {
        return Vec2{ .x = v.z, .y = v.x };
    }
    pub fn zy(v: Vec3) Vec2 {
        return Vec2{ .x = v.z, .y = v.y };
    }
    pub fn zz(v: Vec3) Vec2 {
        return Vec2{ .x = v.z, .y = v.z };
    }
    pub fn xyz(v: Vec3) Vec3 {
        return v;
    }
    pub fn xzy(v: Vec3) Vec3 {
        return Vec3{ .x = v.x, .y = v.z, .z = v.y };
    }
    pub fn yxz(v: Vec3) Vec3 {
        return Vec3{ .x = v.y, .y = v.x, .z = v.z };
    }
    pub fn yzx(v: Vec3) Vec3 {
        return Vec3{ .x = v.y, .y = v.z, .z = v.x };
    }
    pub fn zxy(v: Vec3) Vec3 {
        return Vec3{ .x = v.z, .y = v.x, .z = v.y };
    }
    pub fn zyx(v: Vec3) Vec3 {
        return Vec3{ .x = v.z, .y = v.y, .z = v.x };
    }
};

/// Swizzle operations for Vec4
pub const Vec4Swizzles = struct {
    pub fn xy(v: Vec4) Vec2 {
        return Vec2{ .x = v.x, .y = v.y };
    }
    pub fn xz(v: Vec4) Vec2 {
        return Vec2{ .x = v.x, .y = v.z };
    }
    pub fn xw(v: Vec4) Vec2 {
        return Vec2{ .x = v.x, .y = v.w };
    }
    pub fn yz(v: Vec4) Vec2 {
        return Vec2{ .x = v.y, .y = v.z };
    }
    pub fn yw(v: Vec4) Vec2 {
        return Vec2{ .x = v.y, .y = v.w };
    }
    pub fn zw(v: Vec4) Vec2 {
        return Vec2{ .x = v.z, .y = v.w };
    }
    pub fn xyz(v: Vec4) Vec3 {
        return Vec3{ .x = v.x, .y = v.y, .z = v.z };
    }
    pub fn xyw(v: Vec4) Vec3 {
        return Vec3{ .x = v.x, .y = v.y, .z = v.w };
    }
    pub fn xzw(v: Vec4) Vec3 {
        return Vec3{ .x = v.x, .y = v.z, .z = v.w };
    }
    pub fn yzw(v: Vec4) Vec3 {
        return Vec3{ .x = v.y, .y = v.z, .z = v.w };
    }
    pub fn xyzw(v: Vec4) Vec4 {
        return v;
    }
    pub fn wzyx(v: Vec4) Vec4 {
        return Vec4{ .x = v.w, .y = v.z, .z = v.y, .w = v.x };
    }
};

// Export common swizzle operations
export fn vec2_swizzle_xx(v: Vec2) Vec2 {
    return Vec2Swizzles.xx(v);
}
export fn vec2_swizzle_yx(v: Vec2) Vec2 {
    return Vec2Swizzles.yx(v);
}
export fn vec3_swizzle_xy(v: Vec3) Vec2 {
    return Vec3Swizzles.xy(v);
}
export fn vec3_swizzle_xz(v: Vec3) Vec2 {
    return Vec3Swizzles.xz(v);
}
export fn vec3_swizzle_yz(v: Vec3) Vec2 {
    return Vec3Swizzles.yz(v);
}
export fn vec3_swizzle_zyx(v: Vec3) Vec3 {
    return Vec3Swizzles.zyx(v);
}
export fn vec4_swizzle_xy(v: Vec4) Vec2 {
    return Vec4Swizzles.xy(v);
}
export fn vec4_swizzle_xyz(v: Vec4) Vec3 {
    return Vec4Swizzles.xyz(v);
}
export fn vec4_swizzle_wzyx(v: Vec4) Vec4 {
    return Vec4Swizzles.wzyx(v);
}
