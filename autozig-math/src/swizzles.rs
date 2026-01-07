use autozig::include_zig;
use crate::{Vec2, Vec3, Vec4};

include_zig!("zig/swizzles.zig", {
    fn vec2_swizzle_xx(v: Vec2) -> Vec2;
    fn vec2_swizzle_yx(v: Vec2) -> Vec2;
    fn vec3_swizzle_xy(v: Vec3) -> Vec2;
    fn vec3_swizzle_xz(v: Vec3) -> Vec2;
    fn vec3_swizzle_yz(v: Vec3) -> Vec2;
    fn vec3_swizzle_zyx(v: Vec3) -> Vec3;
    fn vec4_swizzle_xy(v: Vec4) -> Vec2;
    fn vec4_swizzle_xyz(v: Vec4) -> Vec3;
    fn vec4_swizzle_wzyx(v: Vec4) -> Vec4;
});

/// Swizzle operations for Vec2
pub trait Vec2Swizzles {
    fn xx(self) -> Vec2;
    fn yx(self) -> Vec2;
}

/// Swizzle operations for Vec3
pub trait Vec3Swizzles {
    fn xy(self) -> Vec2;
    fn xz(self) -> Vec2;
    fn yz(self) -> Vec2;
    fn zyx(self) -> Vec3;
}

/// Swizzle operations for Vec4
pub trait Vec4Swizzles {
    fn xy(self) -> Vec2;
    fn xyz(self) -> Vec3;
    fn wzyx(self) -> Vec4;
}

impl Vec2Swizzles for Vec2 {
    fn xx(self) -> Vec2 {
        vec2_swizzle_xx(self)
    }
    fn yx(self) -> Vec2 {
        vec2_swizzle_yx(self)
    }
}

impl Vec3Swizzles for Vec3 {
    fn xy(self) -> Vec2 {
        vec3_swizzle_xy(self)
    }
    fn xz(self) -> Vec2 {
        vec3_swizzle_xz(self)
    }
    fn yz(self) -> Vec2 {
        vec3_swizzle_yz(self)
    }
    fn zyx(self) -> Vec3 {
        vec3_swizzle_zyx(self)
    }
}

impl Vec4Swizzles for Vec4 {
    fn xy(self) -> Vec2 {
        vec4_swizzle_xy(self)
    }
    fn xyz(self) -> Vec3 {
        vec4_swizzle_xyz(self)
    }
    fn wzyx(self) -> Vec4 {
        vec4_swizzle_wzyx(self)
    }
}
