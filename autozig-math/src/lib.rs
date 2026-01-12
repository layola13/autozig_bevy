pub mod vec3;
pub mod vec2;
pub mod vec4;
pub mod vec3a;
pub mod quat;
pub mod mat2;
pub mod mat3;
pub mod mat4;
pub mod mat3a;
pub mod ivec2;
pub mod ivec3;
pub mod ivec4;
pub mod uvec2;
pub mod uvec3;
pub mod uvec4;
pub mod bvec;
pub mod dvec2;
pub mod dvec3;
pub mod dvec4;
pub mod dmat2;
pub mod dmat3;
pub mod dmat4;
pub mod dquat;
pub mod rect;
pub mod ray;
pub mod rot2;
pub mod dir2;
pub mod dir3;
pub mod isometry2d;
pub mod isometry3d;
pub mod affine2;
pub mod affine3;
pub mod primitives;
pub mod bounding;
pub mod curve;
pub mod splines;
pub mod rects;
pub mod primitives_2d;
pub mod primitives_3d;
pub mod compass;
pub mod aspect_ratio;
pub mod easing;
pub mod enums;
pub mod float_ord;
pub mod swizzles;
pub mod ops;

// New extended modules for 130 API types
pub mod int_vectors;
pub mod affine_ext;
pub mod extended_types;
pub mod swizzles_ext;

pub use vec3::Vec3;
pub use vec2::Vec2;
pub use vec4::Vec4;
pub use vec3a::{Vec3A, Dir3A};
pub use quat::Quat;
pub use mat2::Mat2;
pub use mat3::Mat3;
pub use mat4::Mat4;
pub use mat3a::Mat3A;
pub use ivec2::IVec2;
pub use ivec3::IVec3;
pub use ivec4::IVec4;
pub use uvec2::UVec2;
pub use uvec3::UVec3;
pub use uvec4::UVec4;
pub use bvec::{BVec2, BVec3, BVec4};
pub use dvec2::DVec2;
pub use dvec3::DVec3;
pub use dvec4::DVec4;
pub use dmat2::DMat2;
pub use dmat3::DMat3;
pub use dmat4::DMat4;
pub use dquat::DQuat;
pub use rect::Rect;
pub use ray::{Ray2d, Ray3d};
pub use rot2::Rot2;
pub use dir2::Dir2;
pub use dir3::Dir3;
pub use isometry2d::Isometry2d;
pub use isometry3d::Isometry3d;
pub use affine2::Affine2;
pub use affine3::Affine3;
pub use primitives::{Circle, Sphere};
pub use bounding::{Aabb2d, Aabb3d};
pub use curve::CubicBezier3d;
pub use splines::{CubicHermite3d, CatmullRom3d, BSpline3d};
pub use rects::{IRect, URect};
pub use primitives_2d::{Triangle2d, Plane2d, Capsule2d};
pub use primitives_3d::{Cuboid, Cylinder, Capsule3d, Plane3d, InfinitePlane3d};
pub use compass::CompassOctant;
pub use aspect_ratio::AspectRatio;
pub use easing::EaseFunction;
pub use enums::{CompassQuadrant, EulerRot, InvalidDirectionError};
pub use float_ord::FloatOrd;
pub use swizzles::{Vec2Swizzles, Vec3Swizzles, Vec4Swizzles};
pub use ops::FloatPow;

// Extended integer vector types (18 new types)
pub use int_vectors::{
    I8Vec2, I8Vec3, I8Vec4,
    I16Vec2, I16Vec3, I16Vec4,
    I64Vec2, I64Vec3, I64Vec4,
    U8Vec2, U8Vec3, U8Vec4,
    U16Vec2, U16Vec3, U16Vec4,
    U64Vec2, U64Vec3, U64Vec4,
    BVec3A, BVec4A,
};

// Extended affine transform types (3 new types)
pub use affine_ext::{DAffine2, DAffine3, Affine3A};

// Extended shapes, curves, bounds, and other types (70+ new types)
pub use extended_types::{
    // Direction aliases (3 types)
    Direction2d, Direction3d, Direction3dA,
    
    // Curve types (14 types)
    CubicBezier2d, QuadraticBezier2d, QuadraticBezier3d,
    CubicBSpline, CubicCardinalSpline, CubicCurve, CubicSegment,
    CubicHermite, CubicNurbs, RationalCurve, RationalSegment,
    
    // 2D shapes (13 types)
    Arc2d, CircularSector, CircularSegment, Ellipse, Line2d,
    Rectangle, RegularPolygon, Rhombus, Segment2d,
    Polygon, BoxedPolygon, Polyline2d, BoxedPolyline2d,
    
    // 3D shapes (9 types)
    Line3d, Segment3d, Triangle3d, Tetrahedron,
    Cone, ConicalFrustum, Torus,
    Polyline3d, BoxedPolyline3d,
    
    // Bounding volumes (2 types)
    BoundingCircle, BoundingSphere,
    
    // Mesh and extrusion (5 types)
    Extruded, Extrusion, ExtrusionBuilder, NormalDisplacement, Gizmos,
    
    // Samplers (3 types)
    ChordLength, UniformMeshSampler, ShapeSample,
    
    // Enums (6 types)
    CubicNurbsError, IntersectionKind, Joinable, Rotation2d, WindingOrder, MeshingError,
    
    // Traits (11 types)
    Primitive2d, Primitive3d,
    Bounded2d, Bounded3d, BoundedExtrusion,
    Measured2d, Measured3d,
    IntersectsVolume, CubicGenerator, RationalGenerator, SampleCurve,
    BoundingVolume,
};

// Extended swizzle traits (12 new trait types)
pub use swizzles_ext::{
    BVec2Swizzles, BVec3Swizzles, BVec4Swizzles,
    DVec2Swizzles, DVec3Swizzles, DVec4Swizzles,
    IVec2Swizzles, IVec3Swizzles, IVec4Swizzles,
    UVec2Swizzles, UVec3Swizzles, UVec4Swizzles,
};
