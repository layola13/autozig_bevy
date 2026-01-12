//! Extended math types for Bevy compatibility
//! Includes curves, additional shapes, directions, bounds, meshes, samplers, and traits

use autozig::include_zig;
use crate::{Vec2, Vec3};

// ============================================================================
// Direction type aliases
// ============================================================================

pub type Direction2d = crate::Dir2;
pub type Direction3d = crate::Dir3;
pub type Direction3dA = crate::Dir3A;

// ============================================================================
// Curve types
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicBezier2d {
    pub control_points: [Vec2; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadraticBezier2d {
    pub control_points: [Vec2; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadraticBezier3d {
    pub control_points: [Vec3; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CubicBSpline<P> {
    pub control_points: *const P,
    pub point_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CubicCardinalSpline<P> {
    pub tension: f32,
    pub control_points: *const P,
    pub point_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CubicCurve<P> {
    pub segments: *const CubicSegment<P>,
    pub segment_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CubicSegment<P> {
    pub coeff: [P; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CubicHermite<P> {
    pub control_points: *const P,
    pub tangents: *const P,
    pub point_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CubicNurbs<P> {
    pub control_points: *const P,
    pub point_count: usize,
    pub weights: *const f32,
    pub knots: *const f32,
    pub knot_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RationalCurve<P> {
    pub curve: CubicCurve<P>,
    pub weights: *const f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RationalSegment<P> {
    pub coeff: [P; 4],
    pub weight_coeff: [f32; 4],
}

// ============================================================================
// Additional 2D shapes
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Arc2d {
    pub radius: f32,
    pub half_angle: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircularSector {
    pub arc: Arc2d,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircularSegment {
    pub arc: Arc2d,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    pub half_size: Vec2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2d {
    pub direction: Vec2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub half_size: Vec2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegularPolygon {
    pub circumcircle: crate::Circle,
    pub sides: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rhombus {
    pub half_diagonals: Vec2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment2d {
    pub direction: Vec2,
    pub half_length: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Polygon<const N: usize> {
    pub vertices: [Vec2; N],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoxedPolygon {
    pub vertices: *const Vec2,
    pub vertex_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Polyline2d<const N: usize> {
    pub vertices: [Vec2; N],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoxedPolyline2d {
    pub vertices: *const Vec2,
    pub vertex_count: usize,
}

// ============================================================================
// Additional 3D shapes
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line3d {
    pub direction: Vec3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment3d {
    pub direction: Vec3,
    pub half_length: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle3d {
    pub vertices: [Vec3; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tetrahedron {
    pub vertices: [Vec3; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cone {
    pub radius: f32,
    pub height: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConicalFrustum {
    pub radius_top: f32,
    pub radius_bottom: f32,
    pub height: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Torus {
    pub minor_radius: f32,
    pub major_radius: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Polyline3d<const N: usize> {
    pub vertices: [Vec3; N],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BoxedPolyline3d {
    pub vertices: *const Vec3,
    pub vertex_count: usize,
}

// ============================================================================
// Bounding volumes
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingCircle {
    pub center: Vec2,
    pub radius: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
}

// ============================================================================
// Mesh and extrusion types
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Extruded<T> {
    pub base_shape: T,
    pub half_depth: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Extrusion<T> {
    pub base_shape: T,
    pub half_depth: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExtrusionBuilder<T> {
    pub base_shape: T,
    pub depth: f32,
    pub segments: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NormalDisplacement {
    pub distance: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Gizmos {
    // Placeholder for gizmo rendering data
    _data: u8,
}

// ============================================================================
// Sampler types
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ChordLength;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UniformMeshSampler {
    pub sample_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShapeSample {
    pub position: Vec3,
    pub normal: Vec3,
}

// ============================================================================
// Enumerations
// ============================================================================

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CubicNurbsError {
    NotEnoughControlPoints = 0,
    TooManyControlPoints = 1,
    NotEnoughKnots = 2,
    TooManyKnots = 3,
    WeightCountMismatch = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntersectionKind {
    None = 0,
    Point = 1,
    Line = 2,
    Plane = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Joinable {
    Yes = 0,
    No = 1,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation2d {
    None = 0,
    Clockwise90 = 1,
    Clockwise180 = 2,
    Clockwise270 = 3,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WindingOrder {
    Clockwise = 0,
    CounterClockwise = 1,
    Invalid = 2,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeshingError {
    InvalidInput = 0,
    TooManyVertices = 1,
    InsufficientMemory = 2,
}

// ============================================================================
// Trait definitions
// ============================================================================

/// Marker trait for 2D primitives
pub trait Primitive2d {}

/// Marker trait for 3D primitives
pub trait Primitive3d {}

/// Trait for shapes with 2D bounding volumes
pub trait Bounded2d {
    fn aabb_2d(&self) -> crate::Aabb2d;
    fn bounding_circle(&self) -> BoundingCircle;
}

/// Trait for shapes with 3D bounding volumes
pub trait Bounded3d {
    fn aabb_3d(&self) -> crate::Aabb3d;
    fn bounding_sphere(&self) -> BoundingSphere;
}

/// Trait for bounded extrusion operations
pub trait BoundedExtrusion: Bounded2d {
    fn extrusion_aabb_3d(&self, half_depth: f32) -> crate::Aabb3d;
}

/// Trait for 2D measurable shapes
pub trait Measured2d {
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
}

/// Trait for 3D measurable shapes
pub trait Measured3d {
    fn surface_area(&self) -> f32;
    fn volume(&self) -> f32;
}

/// Trait for volume intersection tests
pub trait IntersectsVolume<T> {
    fn intersects(&self, volume: &T) -> bool;
}

/// Trait for generating cubic curves
pub trait CubicGenerator<P: Copy> {
    fn to_curve(&self) -> CubicCurve<P>;
}

/// Trait for generating rational curves
pub trait RationalGenerator<P: Copy> {
    fn to_rational_curve(&self) -> RationalCurve<P>;
}

/// Trait for sampling curves
pub trait SampleCurve<P> {
    fn sample(&self, t: f32) -> P;
    fn sample_iter(&self, num_samples: usize) -> Vec<P>;
}

/// Trait for shapes that support boundary computation
pub trait BoundingVolume {
    type Output;
    fn merge(&self, other: &Self) -> Self::Output;
    fn contains(&self, point: Vec3) -> bool;
}

// ============================================================================
// Include Zig bindings
// ============================================================================

include_zig!("zig/extended_types.zig", {
    fn cubic_bezier2d_new(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) -> CubicBezier2d;
    fn quadratic_bezier2d_new(p0: Vec2, p1: Vec2, p2: Vec2) -> QuadraticBezier2d;
    fn quadratic_bezier3d_new(p0: Vec3, p1: Vec3, p2: Vec3) -> QuadraticBezier3d;
    fn arc2d_new(radius: f32, half_angle: f32) -> Arc2d;
    fn ellipse_new(half_size: Vec2) -> Ellipse;
    fn line2d_new(direction: Vec2) -> Line2d;
    fn rectangle_new(half_size: Vec2) -> Rectangle;
    fn rhombus_new(half_diagonals: Vec2) -> Rhombus;
    fn segment2d_new(direction: Vec2, half_length: f32) -> Segment2d;
    fn line3d_new(direction: Vec3) -> Line3d;
    fn segment3d_new(direction: Vec3, half_length: f32) -> Segment3d;
    fn triangle3d_new(v0: Vec3, v1: Vec3, v2: Vec3) -> Triangle3d;
    fn cone_new(radius: f32, height: f32) -> Cone;
    fn torus_new(minor_radius: f32, major_radius: f32) -> Torus;
    fn bounding_circle_new(center: Vec2, radius: f32) -> BoundingCircle;
    fn bounding_sphere_new(center: Vec3, radius: f32) -> BoundingSphere;
});

// ============================================================================
// Implementations
// ============================================================================

impl CubicBezier2d {
    pub fn new(control_points: [Vec2; 4]) -> Self {
        cubic_bezier2d_new(
            control_points[0],
            control_points[1],
            control_points[2],
            control_points[3],
        )
    }
}

impl QuadraticBezier2d {
    pub fn new(control_points: [Vec2; 3]) -> Self {
        quadratic_bezier2d_new(control_points[0], control_points[1], control_points[2])
    }
}

impl QuadraticBezier3d {
    pub fn new(control_points: [Vec3; 3]) -> Self {
        quadratic_bezier3d_new(control_points[0], control_points[1], control_points[2])
    }
}

impl Arc2d {
    pub fn new(radius: f32, half_angle: f32) -> Self {
        arc2d_new(radius, half_angle)
    }
}

impl Ellipse {
    pub fn new(half_size: Vec2) -> Self {
        ellipse_new(half_size)
    }
}

impl Line2d {
    pub fn new(direction: Vec2) -> Self {
        line2d_new(direction)
    }
}

impl Rectangle {
    pub fn new(half_size: Vec2) -> Self {
        rectangle_new(half_size)
    }
}

impl Rhombus {
    pub fn new(half_diagonals: Vec2) -> Self {
        rhombus_new(half_diagonals)
    }
}

impl Segment2d {
    pub fn new(direction: Vec2, half_length: f32) -> Self {
        segment2d_new(direction, half_length)
    }
}

impl Line3d {
    pub fn new(direction: Vec3) -> Self {
        line3d_new(direction)
    }
}

impl Segment3d {
    pub fn new(direction: Vec3, half_length: f32) -> Self {
        segment3d_new(direction, half_length)
    }
}

impl Triangle3d {
    pub fn new(vertices: [Vec3; 3]) -> Self {
        triangle3d_new(vertices[0], vertices[1], vertices[2])
    }
}

impl Cone {
    pub fn new(radius: f32, height: f32) -> Self {
        cone_new(radius, height)
    }
}

impl Torus {
    pub fn new(minor_radius: f32, major_radius: f32) -> Self {
        torus_new(minor_radius, major_radius)
    }
}

impl BoundingCircle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        bounding_circle_new(center, radius)
    }
}

impl BoundingSphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        bounding_sphere_new(center, radius)
    }
}

// Implement Primitive2d for 2D shapes
impl Primitive2d for crate::Circle {}
impl Primitive2d for Arc2d {}
impl Primitive2d for CircularSector {}
impl Primitive2d for CircularSegment {}
impl Primitive2d for Ellipse {}
impl Primitive2d for Line2d {}
impl Primitive2d for Rectangle {}
impl Primitive2d for RegularPolygon {}
impl Primitive2d for Rhombus {}
impl Primitive2d for Segment2d {}
impl Primitive2d for crate::Triangle2d {}
impl<const N: usize> Primitive2d for Polygon<N> {}
impl Primitive2d for BoxedPolygon {}

// Implement Primitive3d for 3D shapes
impl Primitive3d for crate::Sphere {}
impl Primitive3d for crate::Cuboid {}
impl Primitive3d for crate::Cylinder {}
impl Primitive3d for crate::Capsule3d {}
impl Primitive3d for Line3d {}
impl Primitive3d for Segment3d {}
impl Primitive3d for Triangle3d {}
impl Primitive3d for Tetrahedron {}
impl Primitive3d for Cone {}
impl Primitive3d for ConicalFrustum {}
impl Primitive3d for Torus {}