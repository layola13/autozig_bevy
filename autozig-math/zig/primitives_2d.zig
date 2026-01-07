const std = @import("std");

pub const Triangle2d = extern struct {
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
    pub fn new(v0: Vec2, v1: Vec2, v2: Vec2) Triangle2d {
        return .{ .v0 = v0, .v1 = v1, .v2 = v2 };
    }
};

pub const Plane2d = extern struct {
    normal: Dir2,
    pub fn new(normal: Dir2) Plane2d {
        return .{ .normal = normal };
    }
};

pub const Capsule2d = extern struct {
    radius: f32,
    half_length: f32,
    pub fn new(radius: f32, half_length: f32) Capsule2d {
        return .{ .radius = radius, .half_length = half_length };
    }
};

export fn triangle2d_new(v0: Vec2, v1: Vec2, v2: Vec2) Triangle2d {
    return Triangle2d.new(v0, v1, v2);
}
export fn plane2d_new(normal: Dir2) Plane2d {
    return Plane2d.new(normal);
}
export fn capsule2d_new(radius: f32, half_length: f32) Capsule2d {
    return Capsule2d.new(radius, half_length);
}
