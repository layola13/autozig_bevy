const std = @import("std");

pub const Circle = extern struct {
    radius: f32,

    pub fn new(radius: f32) Circle {
        return .{ .radius = radius };
    }

    pub fn diameter(self: Circle) f32 {
        return self.radius * 2.0;
    }

    pub fn area(self: Circle) f32 {
        return std.math.pi * self.radius * self.radius;
    }

    pub fn perimeter(self: Circle) f32 {
        return 2.0 * std.math.pi * self.radius;
    }

    pub fn contains_point(self: Circle, point: Vec2) bool {
        return point.length_squared() <= self.radius * self.radius;
    }

    pub fn closest_point(self: Circle, point: Vec2) Vec2 {
        const len = point.length();
        if (len == 0) return Vec2{ .x = self.radius, .y = 0.0 };
        return point.mul_scalar(self.radius / len);
    }
};

pub const Sphere = extern struct {
    radius: f32,

    pub fn new(radius: f32) Sphere {
        return .{ .radius = radius };
    }

    pub fn diameter(self: Sphere) f32 {
        return self.radius * 2.0;
    }

    pub fn volume(self: Sphere) f32 {
        return (4.0 / 3.0) * std.math.pi * self.radius * self.radius * self.radius;
    }

    pub fn surface_area(self: Sphere) f32 {
        return 4.0 * std.math.pi * self.radius * self.radius;
    }

    pub fn contains_point(self: Sphere, point: Vec3) bool {
        return point.length_squared() <= self.radius * self.radius;
    }

    pub fn closest_point(self: Sphere, point: Vec3) Vec3 {
        const len = point.length();
        if (len == 0) return Vec3{ .x = self.radius, .y = 0.0, .z = 0.0 };
        return point.mul_scalar(self.radius / len);
    }
};

export fn circle_new(radius: f32) Circle {
    return Circle.new(radius);
}

export fn circle_area(self: Circle) f32 {
    return self.area();
}

export fn circle_perimeter(self: Circle) f32 {
    return self.perimeter();
}

export fn circle_contains_point(self: Circle, point: Vec2) bool {
    return self.contains_point(point);
}

export fn sphere_new(radius: f32) Sphere {
    return Sphere.new(radius);
}

export fn sphere_volume(self: Sphere) f32 {
    return self.volume();
}

export fn sphere_surface_area(self: Sphere) f32 {
    return self.surface_area();
}

export fn sphere_contains_point(self: Sphere, point: Vec3) bool {
    return self.contains_point(point);
}
