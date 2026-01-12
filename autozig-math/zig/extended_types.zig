//! Extended types implementation for curves, shapes, bounds, etc.

const std = @import("std");

const Vec2 = extern struct { x: f32, y: f32 };
const Vec3 = extern struct { x: f32, y: f32, z: f32 };

// ============================================================================
// Curve types
// ============================================================================

pub const CubicBezier2d = extern struct {
    control_points: [4]Vec2,
};

pub const QuadraticBezier2d = extern struct {
    control_points: [3]Vec2,
};

pub const QuadraticBezier3d = extern struct {
    control_points: [3]Vec3,
};

// ============================================================================
// Shape types
// ============================================================================

pub const Arc2d = extern struct {
    radius: f32,
    half_angle: f32,
};

pub const Ellipse = extern struct {
    half_size: Vec2,
};

pub const Line2d = extern struct {
    direction: Vec2,
};

pub const Rectangle = extern struct {
    half_size: Vec2,
};

pub const Rhombus = extern struct {
    half_diagonals: Vec2,
};

pub const Segment2d = extern struct {
    direction: Vec2,
    half_length: f32,
};

pub const Line3d = extern struct {
    direction: Vec3,
};

pub const Segment3d = extern struct {
    direction: Vec3,
    half_length: f32,
};

pub const Triangle3d = extern struct {
    vertices: [3]Vec3,
};

pub const Cone = extern struct {
    radius: f32,
    height: f32,
};

pub const Torus = extern struct {
    minor_radius: f32,
    major_radius: f32,
};

// ============================================================================
// Bounding volumes
// ============================================================================

pub const BoundingCircle = extern struct {
    center: Vec2,
    radius: f32,
};

pub const BoundingSphere = extern struct {
    center: Vec3,
    radius: f32,
};

// ============================================================================
// FFI Exports
// ============================================================================

export fn cubic_bezier2d_new(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2) CubicBezier2d {
    return CubicBezier2d{
        .control_points = [4]Vec2{ p0, p1, p2, p3 },
    };
}

export fn quadratic_bezier2d_new(p0: Vec2, p1: Vec2, p2: Vec2) QuadraticBezier2d {
    return QuadraticBezier2d{
        .control_points = [3]Vec2{ p0, p1, p2 },
    };
}

export fn quadratic_bezier3d_new(p0: Vec3, p1: Vec3, p2: Vec3) QuadraticBezier3d {
    return QuadraticBezier3d{
        .control_points = [3]Vec3{ p0, p1, p2 },
    };
}

export fn arc2d_new(radius: f32, half_angle: f32) Arc2d {
    return Arc2d{
        .radius = radius,
        .half_angle = half_angle,
    };
}

export fn ellipse_new(half_size: Vec2) Ellipse {
    return Ellipse{
        .half_size = half_size,
    };
}

export fn line2d_new(direction: Vec2) Line2d {
    return Line2d{
        .direction = direction,
    };
}

export fn rectangle_new(half_size: Vec2) Rectangle {
    return Rectangle{
        .half_size = half_size,
    };
}

export fn rhombus_new(half_diagonals: Vec2) Rhombus {
    return Rhombus{
        .half_diagonals = half_diagonals,
    };
}

export fn segment2d_new(direction: Vec2, half_length: f32) Segment2d {
    return Segment2d{
        .direction = direction,
        .half_length = half_length,
    };
}

export fn line3d_new(direction: Vec3) Line3d {
    return Line3d{
        .direction = direction,
    };
}

export fn segment3d_new(direction: Vec3, half_length: f32) Segment3d {
    return Segment3d{
        .direction = direction,
        .half_length = half_length,
    };
}

export fn triangle3d_new(v0: Vec3, v1: Vec3, v2: Vec3) Triangle3d {
    return Triangle3d{
        .vertices = [3]Vec3{ v0, v1, v2 },
    };
}

export fn cone_new(radius: f32, height: f32) Cone {
    return Cone{
        .radius = radius,
        .height = height,
    };
}

export fn torus_new(minor_radius: f32, major_radius: f32) Torus {
    return Torus{
        .minor_radius = minor_radius,
        .major_radius = major_radius,
    };
}

export fn bounding_circle_new(center: Vec2, radius: f32) BoundingCircle {
    return BoundingCircle{
        .center = center,
        .radius = radius,
    };
}

export fn bounding_sphere_new(center: Vec3, radius: f32) BoundingSphere {
    return BoundingSphere{
        .center = center,
        .radius = radius,
    };
}
