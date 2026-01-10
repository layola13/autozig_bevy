const std = @import("std");
const Vec3 = @import("vec3.zig").Vec3;

/// Hermite spline (cubic Hermite interpolation)
pub const CubicHermite3d = extern struct {
    p0: Vec3, // Start point
    p1: Vec3, // End point
    t0: Vec3, // Start tangent
    t1: Vec3, // End tangent

    pub fn new(p0: Vec3, p1: Vec3, t0: Vec3, t1: Vec3) CubicHermite3d {
        return .{ .p0 = p0, .p1 = p1, .t0 = t0, .t1 = t1 };
    }

    pub fn position(self: CubicHermite3d, t: f32) Vec3 {
        const t2 = t * t;
        const t3 = t2 * t;

        // Hermite basis functions
        const h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
        const h10 = t3 - 2.0 * t2 + t;
        const h01 = -2.0 * t3 + 3.0 * t2;
        const h11 = t3 - t2;

        return Vec3{
            .x = h00 * self.p0.x + h10 * self.t0.x + h01 * self.p1.x + h11 * self.t1.x,
            .y = h00 * self.p0.y + h10 * self.t0.y + h01 * self.p1.y + h11 * self.t1.y,
            .z = h00 * self.p0.z + h10 * self.t0.z + h01 * self.p1.z + h11 * self.t1.z,
        };
    }

    pub fn velocity(self: CubicHermite3d, t: f32) Vec3 {
        const t2 = t * t;

        // Derivatives of Hermite basis functions
        const h00 = 6.0 * t2 - 6.0 * t;
        const h10 = 3.0 * t2 - 4.0 * t + 1.0;
        const h01 = -6.0 * t2 + 6.0 * t;
        const h11 = 3.0 * t2 - 2.0 * t;

        return Vec3{
            .x = h00 * self.p0.x + h10 * self.t0.x + h01 * self.p1.x + h11 * self.t1.x,
            .y = h00 * self.p0.y + h10 * self.t0.y + h01 * self.p1.y + h11 * self.t1.y,
            .z = h00 * self.p0.z + h10 * self.t0.z + h01 * self.p1.z + h11 * self.t1.z,
        };
    }
};

/// Catmull-Rom spline (passing through all control points)
pub const CatmullRom3d = extern struct {
    p0: Vec3, // Previous point
    p1: Vec3, // Start point
    p2: Vec3, // End point
    p3: Vec3, // Next point

    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) CatmullRom3d {
        return .{ .p0 = p0, .p1 = p1, .p2 = p2, .p3 = p3 };
    }

    pub fn position(self: CatmullRom3d, t: f32) Vec3 {
        const t2 = t * t;
        const t3 = t2 * t;

        // Catmull-Rom basis functions
        const b0 = -0.5 * t3 + t2 - 0.5 * t;
        const b1 = 1.5 * t3 - 2.5 * t2 + 1.0;
        const b2 = -1.5 * t3 + 2.0 * t2 + 0.5 * t;
        const b3 = 0.5 * t3 - 0.5 * t2;

        return Vec3{
            .x = b0 * self.p0.x + b1 * self.p1.x + b2 * self.p2.x + b3 * self.p3.x,
            .y = b0 * self.p0.y + b1 * self.p1.y + b2 * self.p2.y + b3 * self.p3.y,
            .z = b0 * self.p0.z + b1 * self.p1.z + b2 * self.p2.z + b3 * self.p3.z,
        };
    }
};

/// B-Spline segment (uniform cubic B-spline)
pub const BSpline3d = extern struct {
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,

    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) BSpline3d {
        return .{ .p0 = p0, .p1 = p1, .p2 = p2, .p3 = p3 };
    }

    pub fn position(self: BSpline3d, t: f32) Vec3 {
        const t2 = t * t;
        const t3 = t2 * t;

        // B-spline basis functions (uniform cubic)
        const inv6: f32 = 1.0 / 6.0;
        const b0 = inv6 * (-t3 + 3.0 * t2 - 3.0 * t + 1.0);
        const b1 = inv6 * (3.0 * t3 - 6.0 * t2 + 4.0);
        const b2 = inv6 * (-3.0 * t3 + 3.0 * t2 + 3.0 * t + 1.0);
        const b3 = inv6 * t3;

        return Vec3{
            .x = b0 * self.p0.x + b1 * self.p1.x + b2 * self.p2.x + b3 * self.p3.x,
            .y = b0 * self.p0.y + b1 * self.p1.y + b2 * self.p2.y + b3 * self.p3.y,
            .z = b0 * self.p0.z + b1 * self.p1.z + b2 * self.p2.z + b3 * self.p3.z,
        };
    }
};

export fn cubic_hermite3d_new(p0: Vec3, p1: Vec3, t0: Vec3, t1: Vec3) CubicHermite3d {
    return CubicHermite3d.new(p0, p1, t0, t1);
}

export fn cubic_hermite3d_position(self: CubicHermite3d, t: f32) Vec3 {
    return self.position(t);
}

export fn cubic_hermite3d_velocity(self: CubicHermite3d, t: f32) Vec3 {
    return self.velocity(t);
}

export fn catmull_rom3d_new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) CatmullRom3d {
    return CatmullRom3d.new(p0, p1, p2, p3);
}

export fn catmull_rom3d_position(self: CatmullRom3d, t: f32) Vec3 {
    return self.position(t);
}

export fn bspline3d_new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) BSpline3d {
    return BSpline3d.new(p0, p1, p2, p3);
}

export fn bspline3d_position(self: BSpline3d, t: f32) Vec3 {
    return self.position(t);
}
