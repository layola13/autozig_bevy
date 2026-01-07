use autozig::include_zig;
use crate::{Vec2, Vec3, Dir2, Dir3, Plane2d, InfinitePlane3d};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray2d {
    pub origin: Vec2,
    pub direction: Dir2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray3d {
    pub origin: Vec3,
    pub direction: Dir3,
}

include_zig!("zig/ray.zig", {
    fn ray2d_new(origin: Vec2, direction: Dir2) -> Ray2d;
    fn ray2d_get_point(self_: Ray2d, distance: f32) -> Vec2;
    fn ray2d_intersect_plane(self_: Ray2d, plane_origin: Vec2, plane: Plane2d) -> f32;

    fn ray3d_new(origin: Vec3, direction: Dir3) -> Ray3d;
    fn ray3d_get_point(self_: Ray3d, distance: f32) -> Vec3;
    fn ray3d_intersect_plane(self_: Ray3d, plane_origin: Vec3, plane: InfinitePlane3d) -> f32;
});

impl Ray2d {
    pub fn new(origin: Vec2, direction: Dir2) -> Self { ray2d_new(origin, direction) }
    pub fn get_point(&self, distance: f32) -> Vec2 { ray2d_get_point(*self, distance) }
    pub fn intersect_plane(&self, plane_origin: Vec2, plane: Plane2d) -> Option<f32> {
        let dist = ray2d_intersect_plane(*self, plane_origin, plane);
        if dist.is_nan() { None } else { Some(dist) }
    }
}

impl Ray3d {
    pub fn new(origin: Vec3, direction: Dir3) -> Self { ray3d_new(origin, direction) }
    pub fn get_point(&self, distance: f32) -> Vec3 { ray3d_get_point(*self, distance) }
    pub fn intersect_plane(&self, plane_origin: Vec3, plane: InfinitePlane3d) -> Option<f32> {
        let dist = ray3d_intersect_plane(*self, plane_origin, plane);
        if dist.is_nan() { None } else { Some(dist) }
    }
}
