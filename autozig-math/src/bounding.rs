use autozig::include_zig;
use crate::{Vec2, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb2d {
    pub min: Vec2,
    pub max: Vec2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb3d {
    pub min: Vec3,
    pub max: Vec3,
}

include_zig!("zig/bounding.zig", {
    fn aabb2d_new(min: Vec2, max: Vec2) -> Aabb2d;
    fn aabb2d_center(self_: Aabb2d) -> Vec2;
    fn aabb2d_half_size(self_: Aabb2d) -> Vec2;
    fn aabb2d_size(self_: Aabb2d) -> Vec2;
    fn aabb2d_contains_point(self_: Aabb2d, point: Vec2) -> bool;
    fn aabb2d_intersects(self_: Aabb2d, other: Aabb2d) -> bool;
    fn aabb2d_merge(self_: Aabb2d, other: Aabb2d) -> Aabb2d;
    fn aabb3d_new(min: Vec3, max: Vec3) -> Aabb3d;
    fn aabb3d_center(self_: Aabb3d) -> Vec3;
    fn aabb3d_half_size(self_: Aabb3d) -> Vec3;
    fn aabb3d_size(self_: Aabb3d) -> Vec3;
    fn aabb3d_contains_point(self_: Aabb3d, point: Vec3) -> bool;
    fn aabb3d_intersects(self_: Aabb3d, other: Aabb3d) -> bool;
    fn aabb3d_merge(self_: Aabb3d, other: Aabb3d) -> Aabb3d;
});

impl Aabb2d {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        aabb2d_new(min, max)
    }

    pub fn center(self) -> Vec2 {
        aabb2d_center(self)
    }

    pub fn half_size(self) -> Vec2 {
        aabb2d_half_size(self)
    }

    pub fn size(self) -> Vec2 {
        aabb2d_size(self)
    }

    pub fn contains_point(self, point: Vec2) -> bool {
        aabb2d_contains_point(self, point)
    }

    pub fn intersects(self, other: Self) -> bool {
        aabb2d_intersects(self, other)
    }

    pub fn merge(self, other: Self) -> Self {
        aabb2d_merge(self, other)
    }
}

impl Aabb3d {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        aabb3d_new(min, max)
    }

    pub fn center(self) -> Vec3 {
        aabb3d_center(self)
    }

    pub fn half_size(self) -> Vec3 {
        aabb3d_half_size(self)
    }

    pub fn size(self) -> Vec3 {
        aabb3d_size(self)
    }

    pub fn contains_point(self, point: Vec3) -> bool {
        aabb3d_contains_point(self, point)
    }

    pub fn intersects(self, other: Self) -> bool {
        aabb3d_intersects(self, other)
    }

    pub fn merge(self, other: Self) -> Self {
        aabb3d_merge(self, other)
    }
}
