const std = @import("std");
const Vec2 = @import("vec2.zig").Vec2;
const Vec3 = @import("vec3.zig").Vec3;

pub const Aabb2d = extern struct {
    min: Vec2,
    max: Vec2,

    pub fn new(min: Vec2, max: Vec2) Aabb2d {
        return .{ .min = min, .max = max };
    }

    pub fn from_center_half_size(ctr: Vec2, half: Vec2) Aabb2d {
        return .{
            .min = ctr.sub(half),
            .max = ctr.add(half),
        };
    }

    pub fn center(self: Aabb2d) Vec2 {
        return Vec2{
            .x = (self.min.x + self.max.x) * 0.5,
            .y = (self.min.y + self.max.y) * 0.5,
        };
    }

    pub fn half_size(self: Aabb2d) Vec2 {
        return Vec2{
            .x = (self.max.x - self.min.x) * 0.5,
            .y = (self.max.y - self.min.y) * 0.5,
        };
    }

    pub fn size(self: Aabb2d) Vec2 {
        return self.max.sub(self.min);
    }

    pub fn contains_point(self: Aabb2d, point: Vec2) bool {
        return point.x >= self.min.x and point.x <= self.max.x and
            point.y >= self.min.y and point.y <= self.max.y;
    }

    pub fn intersects(self: Aabb2d, other: Aabb2d) bool {
        return self.min.x <= other.max.x and self.max.x >= other.min.x and
            self.min.y <= other.max.y and self.max.y >= other.min.y;
    }

    pub fn merge(self: Aabb2d, other: Aabb2d) Aabb2d {
        return .{
            .min = self.min.min(other.min),
            .max = self.max.max(other.max),
        };
    }

    pub fn expand(self: Aabb2d, point: Vec2) Aabb2d {
        return .{
            .min = self.min.min(point),
            .max = self.max.max(point),
        };
    }
};

pub const Aabb3d = extern struct {
    min: Vec3,
    max: Vec3,

    pub fn new(min: Vec3, max: Vec3) Aabb3d {
        return .{ .min = min, .max = max };
    }

    pub fn from_center_half_size(ctr: Vec3, half: Vec3) Aabb3d {
        return .{
            .min = ctr.sub(half),
            .max = ctr.add(half),
        };
    }

    pub fn center(self: Aabb3d) Vec3 {
        return Vec3{
            .x = (self.min.x + self.max.x) * 0.5,
            .y = (self.min.y + self.max.y) * 0.5,
            .z = (self.min.z + self.max.z) * 0.5,
        };
    }

    pub fn half_size(self: Aabb3d) Vec3 {
        return Vec3{
            .x = (self.max.x - self.min.x) * 0.5,
            .y = (self.max.y - self.min.y) * 0.5,
            .z = (self.max.z - self.min.z) * 0.5,
        };
    }

    pub fn size(self: Aabb3d) Vec3 {
        return self.max.sub(self.min);
    }

    pub fn contains_point(self: Aabb3d, point: Vec3) bool {
        return point.x >= self.min.x and point.x <= self.max.x and
            point.y >= self.min.y and point.y <= self.max.y and
            point.z >= self.min.z and point.z <= self.max.z;
    }

    pub fn intersects(self: Aabb3d, other: Aabb3d) bool {
        return self.min.x <= other.max.x and self.max.x >= other.min.x and
            self.min.y <= other.max.y and self.max.y >= other.min.y and
            self.min.z <= other.max.z and self.max.z >= other.min.z;
    }

    pub fn merge(self: Aabb3d, other: Aabb3d) Aabb3d {
        return .{
            .min = self.min.min(other.min),
            .max = self.max.max(other.max),
        };
    }

    pub fn expand(self: Aabb3d, point: Vec3) Aabb3d {
        return .{
            .min = self.min.min(point),
            .max = self.max.max(point),
        };
    }
};

export fn aabb2d_new(min: Vec2, max: Vec2) Aabb2d {
    return Aabb2d.new(min, max);
}

export fn aabb2d_center(self: Aabb2d) Vec2 {
    return self.center();
}

export fn aabb2d_half_size(self: Aabb2d) Vec2 {
    return self.half_size();
}

export fn aabb2d_size(self: Aabb2d) Vec2 {
    return self.size();
}

export fn aabb2d_contains_point(self: Aabb2d, point: Vec2) bool {
    return self.contains_point(point);
}

export fn aabb2d_intersects(self: Aabb2d, other: Aabb2d) bool {
    return self.intersects(other);
}

export fn aabb2d_merge(self: Aabb2d, other: Aabb2d) Aabb2d {
    return self.merge(other);
}

export fn aabb3d_new(min: Vec3, max: Vec3) Aabb3d {
    return Aabb3d.new(min, max);
}

export fn aabb3d_center(self: Aabb3d) Vec3 {
    return self.center();
}

export fn aabb3d_half_size(self: Aabb3d) Vec3 {
    return self.half_size();
}

export fn aabb3d_size(self: Aabb3d) Vec3 {
    return self.size();
}

export fn aabb3d_contains_point(self: Aabb3d, point: Vec3) bool {
    return self.contains_point(point);
}

export fn aabb3d_intersects(self: Aabb3d, other: Aabb3d) bool {
    return self.intersects(other);
}

export fn aabb3d_merge(self: Aabb3d, other: Aabb3d) Aabb3d {
    return self.merge(other);
}
