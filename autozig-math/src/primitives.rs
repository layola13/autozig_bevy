use autozig::include_zig;
use crate::{Vec2, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub radius: f32,
}

include_zig!("zig/primitives.zig", {
    fn circle_new(radius: f32) -> Circle;
    fn circle_area(self_: Circle) -> f32;
    fn circle_perimeter(self_: Circle) -> f32;
    fn circle_contains_point(self_: Circle, point: Vec2) -> bool;
    fn sphere_new(radius: f32) -> Sphere;
    fn sphere_volume(self_: Sphere) -> f32;
    fn sphere_surface_area(self_: Sphere) -> f32;
    fn sphere_contains_point(self_: Sphere, point: Vec3) -> bool;
});

impl Circle {
    pub fn new(radius: f32) -> Self {
        circle_new(radius)
    }

    pub fn area(&self) -> f32 {
        circle_area(*self)
    }

    pub fn perimeter(&self) -> f32 {
        circle_perimeter(*self)
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        circle_contains_point(*self, point)
    }
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        sphere_new(radius)
    }

    pub fn volume(&self) -> f32 {
        sphere_volume(*self)
    }

    pub fn surface_area(&self) -> f32 {
        sphere_surface_area(*self)
    }

    pub fn contains_point(&self, point: Vec3) -> bool {
        sphere_contains_point(*self, point)
    }
}
