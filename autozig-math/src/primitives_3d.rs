use autozig::include_zig;
use crate::{Vec3, Dir3};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cuboid {
    pub half_size: Vec3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cylinder {
    pub radius: f32,
    pub half_height: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Capsule3d {
    pub radius: f32,
    pub half_length: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plane3d {
    pub normal: Dir3,
    pub d: f32,
}

include_zig!("zig/primitives_3d.zig", {
    fn cuboid_new(half_size: Vec3) -> Cuboid;
    fn cylinder_new(radius: f32, half_height: f32) -> Cylinder;
    fn capsule3d_new(radius: f32, half_length: f32) -> Capsule3d;
    fn plane3d_new(normal: Dir3, d: f32) -> Plane3d;
    fn infinite_plane3d_new(normal: Dir3) -> InfinitePlane3d;
});

impl Cuboid {
    pub fn new(half_size: Vec3) -> Self { cuboid_new(half_size) }
}

impl Cylinder {
    pub fn new(radius: f32, half_height: f32) -> Self { cylinder_new(radius, half_height) }
}

impl Capsule3d {
    pub fn new(radius: f32, half_length: f32) -> Self { capsule3d_new(radius, half_length) }
}

impl Plane3d {
    pub fn new(normal: Dir3, d: f32) -> Self { plane3d_new(normal, d) }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InfinitePlane3d {
    pub normal: Dir3,
}

impl InfinitePlane3d {
    pub fn new(normal: Dir3) -> Self { infinite_plane3d_new(normal) }
}
