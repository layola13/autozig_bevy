const std = @import("std");

pub const Ray2d = extern struct {
    origin: Vec2,
    direction: Dir2,

    pub fn new(origin: Vec2, direction: Dir2) Ray2d {
        return .{
            .origin = origin,
            .direction = direction,
        };
    }

    pub fn get_point(self: Ray2d, distance: f32) Vec2 {
        const dir_vec = self.direction.vec;
        return .{
            .x = self.origin.x + dir_vec.x * distance,
            .y = self.origin.y + dir_vec.y * distance,
        };
    }

    pub fn intersect_plane(self: Ray2d, plane_origin: Vec2, plane: Plane2d) f32 {
        const denominator = plane.normal.vec.dot(self.direction.vec);
        if (@abs(denominator) > std.math.floatEps(f32)) {
            const diff: Vec2 = .{ .x = plane_origin.x - self.origin.x, .y = plane_origin.y - self.origin.y };
            const distance = diff.dot(plane.normal.vec) / denominator;
            if (distance > std.math.floatEps(f32)) {
                return distance;
            }
        }
        return std.math.nan(f32);
    }
};

pub const Ray3d = extern struct {
    origin: Vec3,
    direction: Dir3,

    pub fn new(origin: Vec3, direction: Dir3) Ray3d {
        return .{
            .origin = origin,
            .direction = direction,
        };
    }

    pub fn get_point(self: Ray3d, distance: f32) Vec3 {
        const dir_vec = self.direction.vec;
        return .{
            .x = self.origin.x + dir_vec.x * distance,
            .y = self.origin.y + dir_vec.y * distance,
            .z = self.origin.z + dir_vec.z * distance,
        };
    }

    pub fn intersect_plane(self: Ray3d, plane_origin: Vec3, plane: InfinitePlane3d) f32 {
        const denominator = plane.normal.vec.dot(self.direction.vec);
        if (@abs(denominator) > std.math.floatEps(f32)) {
            const diff: Vec3 = .{ .x = plane_origin.x - self.origin.x, .y = plane_origin.y - self.origin.y, .z = plane_origin.z - self.origin.z };
            const distance = diff.dot(plane.normal.vec) / denominator;
            if (distance > std.math.floatEps(f32)) {
                return distance;
            }
        }
        return std.math.nan(f32);
    }
};

export fn ray2d_new(origin: Vec2, direction: Dir2) Ray2d {
    return Ray2d.new(origin, direction);
}

export fn ray2d_get_point(self: Ray2d, distance: f32) Vec2 {
    return self.get_point(distance);
}

export fn ray2d_intersect_plane(self: Ray2d, plane_origin: Vec2, plane: Plane2d) f32 {
    return self.intersect_plane(plane_origin, plane);
}

export fn ray3d_new(origin: Vec3, direction: Dir3) Ray3d {
    return Ray3d.new(origin, direction);
}

export fn ray3d_get_point(self: Ray3d, distance: f32) Vec3 {
    return self.get_point(distance);
}

export fn ray3d_intersect_plane(self: Ray3d, plane_origin: Vec3, plane: InfinitePlane3d) f32 {
    return self.intersect_plane(plane_origin, plane);
}
