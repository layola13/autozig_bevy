const std = @import("std");
const Vec3 = @import("vec3.zig").Vec3;
// Vec3 is available globally via autozig merge

pub const CubicBezier3d = extern struct {
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,

    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) CubicBezier3d {
        return .{ .p0 = p0, .p1 = p1, .p2 = p2, .p3 = p3 };
    }

    pub fn position(self: CubicBezier3d, t: f32) Vec3 {
        // Linear interpolation helper
        const mt = 1.0 - t;
        const mt2 = mt * mt;
        const mt3 = mt2 * mt;
        const t2 = t * t;
        const t3 = t2 * t;

        // B(t) = (1-t)^3 P0 + 3(1-t)^2 t P1 + 3(1-t) t^2 P2 + t^3 P3
        // No operator overloading for scalar mul on Vec3 in Zig yet, doing manually for safety/clarity or adding helper

        const term0 = self.p0.mul_scalar(mt3);
        const term1 = self.p1.mul_scalar(3.0 * mt2 * t);
        const term2 = self.p2.mul_scalar(3.0 * mt * t2);
        const term3 = self.p3.mul_scalar(t3);

        return term0.add(term1).add(term2).add(term3);
    }
};

export fn cubic_bezier3d_new(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3) CubicBezier3d {
    return CubicBezier3d.new(p0, p1, p2, p3);
}

export fn cubic_bezier3d_position(self: CubicBezier3d, t: f32) Vec3 {
    return self.position(t);
}
