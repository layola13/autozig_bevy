use autozig::include_zig;
use crate::{Vec2, Dir2};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle2d {
    pub v0: Vec2,
    pub v1: Vec2,
    pub v2: Vec2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane2d {
    pub normal: Dir2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Capsule2d {
    pub radius: f32,
    pub half_length: f32,
}

include_zig!("zig/primitives_2d.zig", {
    fn triangle2d_new(v0: Vec2, v1: Vec2, v2: Vec2) -> Triangle2d;
    fn plane2d_new(normal: Dir2) -> Plane2d;
    fn capsule2d_new(radius: f32, half_length: f32) -> Capsule2d;
});

impl Triangle2d {
    pub fn new(v0: Vec2, v1: Vec2, v2: Vec2) -> Self { triangle2d_new(v0, v1, v2) }
}

impl Plane2d {
    pub fn new(normal: Dir2) -> Self { plane2d_new(normal) }
}

impl Capsule2d {
    pub fn new(radius: f32, half_length: f32) -> Self { capsule2d_new(radius, half_length) }
}
