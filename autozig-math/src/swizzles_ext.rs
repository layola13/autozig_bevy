//! Extended swizzle traits for all vector types

use crate::{BVec2, BVec3, BVec4, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4};

// ============================================================================
// BVec Swizzle Traits
// ============================================================================

pub trait BVec2Swizzles {
    fn xx(self) -> BVec2;
    fn xy(self) -> BVec2;
    fn yx(self) -> BVec2;
    fn yy(self) -> BVec2;
}

pub trait BVec3Swizzles {
    fn xy(self) -> BVec2;
    fn xz(self) -> BVec2;
    fn yz(self) -> BVec2;
    fn xxx(self) -> BVec3;
    fn xyz(self) -> BVec3;
    fn zyx(self) -> BVec3;
}

pub trait BVec4Swizzles {
    fn xy(self) -> BVec2;
    fn xyz(self) -> BVec3;
    fn xyzw(self) -> BVec4;
    fn wzyx(self) -> BVec4;
}

// ============================================================================
// DVec Swizzle Traits
// ============================================================================

pub trait DVec2Swizzles {
    fn xx(self) -> DVec2;
    fn xy(self) -> DVec2;
    fn yx(self) -> DVec2;
    fn yy(self) -> DVec2;
}

pub trait DVec3Swizzles {
    fn xy(self) -> DVec2;
    fn xz(self) -> DVec2;
    fn yz(self) -> DVec2;
    fn xxx(self) -> DVec3;
    fn xyz(self) -> DVec3;
    fn zyx(self) -> DVec3;
}

pub trait DVec4Swizzles {
    fn xy(self) -> DVec2;
    fn xyz(self) -> DVec3;
    fn xyzw(self) -> DVec4;
    fn wzyx(self) -> DVec4;
}

// ============================================================================
// IVec Swizzle Traits
// ============================================================================

pub trait IVec2Swizzles {
    fn xx(self) -> IVec2;
    fn xy(self) -> IVec2;
    fn yx(self) -> IVec2;
    fn yy(self) -> IVec2;
}

pub trait IVec3Swizzles {
    fn xy(self) -> IVec2;
    fn xz(self) -> IVec2;
    fn yz(self) -> IVec2;
    fn xxx(self) -> IVec3;
    fn xyz(self) -> IVec3;
    fn zyx(self) -> IVec3;
}

pub trait IVec4Swizzles {
    fn xy(self) -> IVec2;
    fn xyz(self) -> IVec3;
    fn xyzw(self) -> IVec4;
    fn wzyx(self) -> IVec4;
}

// ============================================================================
// UVec Swizzle Traits
// ============================================================================

pub trait UVec2Swizzles {
    fn xx(self) -> UVec2;
    fn xy(self) -> UVec2;
    fn yx(self) -> UVec2;
    fn yy(self) -> UVec2;
}

pub trait UVec3Swizzles {
    fn xy(self) -> UVec2;
    fn xz(self) -> UVec2;
    fn yz(self) -> UVec2;
    fn xxx(self) -> UVec3;
    fn xyz(self) -> UVec3;
    fn zyx(self) -> UVec3;
}

pub trait UVec4Swizzles {
    fn xy(self) -> UVec2;
    fn xyz(self) -> UVec3;
    fn xyzw(self) -> UVec4;
    fn wzyx(self) -> UVec4;
}

// ============================================================================
// Default Implementations (using component access)
// ============================================================================

impl BVec2Swizzles for BVec2 {
    fn xx(self) -> BVec2 { BVec2 { x: self.x, y: self.x } }
    fn xy(self) -> BVec2 { self }
    fn yx(self) -> BVec2 { BVec2 { x: self.y, y: self.x } }
    fn yy(self) -> BVec2 { BVec2 { x: self.y, y: self.y } }
}

impl BVec3Swizzles for BVec3 {
    fn xy(self) -> BVec2 { BVec2 { x: self.x, y: self.y } }
    fn xz(self) -> BVec2 { BVec2 { x: self.x, y: self.z } }
    fn yz(self) -> BVec2 { BVec2 { x: self.y, y: self.z } }
    fn xxx(self) -> BVec3 { BVec3 { x: self.x, y: self.x, z: self.x } }
    fn xyz(self) -> BVec3 { self }
    fn zyx(self) -> BVec3 { BVec3 { x: self.z, y: self.y, z: self.x } }
}

impl BVec4Swizzles for BVec4 {
    fn xy(self) -> BVec2 { BVec2 { x: self.x, y: self.y } }
    fn xyz(self) -> BVec3 { BVec3 { x: self.x, y: self.y, z: self.z } }
    fn xyzw(self) -> BVec4 { self }
    fn wzyx(self) -> BVec4 { BVec4 { x: self.w, y: self.z, z: self.y, w: self.x } }
}

impl DVec2Swizzles for DVec2 {
    fn xx(self) -> DVec2 { DVec2::new(self.x, self.x) }
    fn xy(self) -> DVec2 { self }
    fn yx(self) -> DVec2 { DVec2::new(self.y, self.x) }
    fn yy(self) -> DVec2 { DVec2::new(self.y, self.y) }
}

impl DVec3Swizzles for DVec3 {
    fn xy(self) -> DVec2 { DVec2::new(self.x, self.y) }
    fn xz(self) -> DVec2 { DVec2::new(self.x, self.z) }
    fn yz(self) -> DVec2 { DVec2::new(self.y, self.z) }
    fn xxx(self) -> DVec3 { DVec3::new(self.x, self.x, self.x) }
    fn xyz(self) -> DVec3 { self }
    fn zyx(self) -> DVec3 { DVec3::new(self.z, self.y, self.x) }
}

impl DVec4Swizzles for DVec4 {
    fn xy(self) -> DVec2 { DVec2::new(self.x, self.y) }
    fn xyz(self) -> DVec3 { DVec3::new(self.x, self.y, self.z) }
    fn xyzw(self) -> DVec4 { self }
    fn wzyx(self) -> DVec4 { DVec4::new(self.w, self.z, self.y, self.x) }
}

impl IVec2Swizzles for IVec2 {
    fn xx(self) -> IVec2 { IVec2::new(self.x, self.x) }
    fn xy(self) -> IVec2 { self }
    fn yx(self) -> IVec2 { IVec2::new(self.y, self.x) }
    fn yy(self) -> IVec2 { IVec2::new(self.y, self.y) }
}

impl IVec3Swizzles for IVec3 {
    fn xy(self) -> IVec2 { IVec2::new(self.x, self.y) }
    fn xz(self) -> IVec2 { IVec2::new(self.x, self.z) }
    fn yz(self) -> IVec2 { IVec2::new(self.y, self.z) }
    fn xxx(self) -> IVec3 { IVec3::new(self.x, self.x, self.x) }
    fn xyz(self) -> IVec3 { self }
    fn zyx(self) -> IVec3 { IVec3::new(self.z, self.y, self.x) }
}

impl IVec4Swizzles for IVec4 {
    fn xy(self) -> IVec2 { IVec2::new(self.x, self.y) }
    fn xyz(self) -> IVec3 { IVec3::new(self.x, self.y, self.z) }
    fn xyzw(self) -> IVec4 { self }
    fn wzyx(self) -> IVec4 { IVec4::new(self.w, self.z, self.y, self.x) }
}

impl UVec2Swizzles for UVec2 {
    fn xx(self) -> UVec2 { UVec2::new(self.x, self.x) }
    fn xy(self) -> UVec2 { self }
    fn yx(self) -> UVec2 { UVec2::new(self.y, self.x) }
    fn yy(self) -> UVec2 { UVec2::new(self.y, self.y) }
}

impl UVec3Swizzles for UVec3 {
    fn xy(self) -> UVec2 { UVec2::new(self.x, self.y) }
    fn xz(self) -> UVec2 { UVec2::new(self.x, self.z) }
    fn yz(self) -> UVec2 { UVec2::new(self.y, self.z) }
    fn xxx(self) -> UVec3 { UVec3::new(self.x, self.x, self.x) }
    fn xyz(self) -> UVec3 { self }
    fn zyx(self) -> UVec3 { UVec3::new(self.z, self.y, self.x) }
}

impl UVec4Swizzles for UVec4 {
    fn xy(self) -> UVec2 { UVec2::new(self.x, self.y) }
    fn xyz(self) -> UVec3 { UVec3::new(self.x, self.y, self.z) }
    fn xyzw(self) -> UVec4 { self }
    fn wzyx(self) -> UVec4 { UVec4::new(self.w, self.z, self.y, self.x) }
}