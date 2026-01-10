const std = @import("std");
const Vec3 = @import("vec3.zig").Vec3;
const Dir3 = @import("dir3.zig").Dir3;

pub const Cuboid = extern struct {
    half_size: Vec3,
    pub fn new(half_size: Vec3) Cuboid {
        return .{ .half_size = half_size };
    }
};

pub const Cylinder = extern struct {
    radius: f32,
    half_height: f32,
    pub fn new(radius: f32, half_height: f32) Cylinder {
        return .{ .radius = radius, .half_height = half_height };
    }
};

pub const Capsule3d = extern struct {
    radius: f32,
    half_length: f32,
    pub fn new(radius: f32, half_length: f32) Capsule3d {
        return .{ .radius = radius, .half_length = half_length };
    }
};

pub const Plane3d = extern struct {
    normal: Dir3,
    d: f32,
    pub fn new(normal: Dir3, d: f32) Plane3d {
        return .{ .normal = normal, .d = d };
    }
};

export fn cuboid_new(half_size: Vec3) Cuboid {
    return Cuboid.new(half_size);
}
export fn cylinder_new(radius: f32, half_height: f32) Cylinder {
    return Cylinder.new(radius, half_height);
}
export fn capsule3d_new(radius: f32, half_length: f32) Capsule3d {
    return Capsule3d.new(radius, half_length);
}
export fn plane3d_new(normal: Dir3, d: f32) Plane3d {
    return Plane3d.new(normal, d);
}

pub const InfinitePlane3d = extern struct {
    normal: Dir3,
    pub fn new(normal: Dir3) InfinitePlane3d {
        return .{ .normal = normal };
    }
};

export fn infinite_plane3d_new(normal: Dir3) InfinitePlane3d {
    return InfinitePlane3d.new(normal);
}
