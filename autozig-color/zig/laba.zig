const std = @import("std");

pub const Laba = extern struct {
    lightness: f32, // L* [0.0, 100.0]
    a: f32, // a* (green-red)
    b: f32, // b* (blue-yellow)
    alpha: f32,

    pub fn new(lightness: f32, a: f32, b: f32, alpha: f32) Laba {
        return .{ .lightness = lightness, .a = a, .b = b, .alpha = alpha };
    }

    pub fn toXyza(self: Laba) @import("xyza.zig").Xyza {
        const fy = (self.lightness + 16.0) / 116.0;
        const fx = self.a / 500.0 + fy;
        const fz = fy - self.b / 200.0;

        const xr = labFInv(fx);
        const yr = labFInv(fy);
        const zr = labFInv(fz);

        // D65 standard illuminant
        const D65_X: f32 = 0.95047;
        const D65_Y: f32 = 1.00000;
        const D65_Z: f32 = 1.08883;

        return @import("xyza.zig").Xyza{
            .x = xr * D65_X,
            .y = yr * D65_Y,
            .z = zr * D65_Z,
            .alpha = self.alpha,
        };
    }

    pub fn fromXyza(xyz: @import("xyza.zig").Xyza) Laba {
        return xyz.toLaba();
    }

    pub fn toLcha(self: Laba) @import("lcha.zig").Lcha {
        const c = std.math.sqrt(self.a * self.a + self.b * self.b);
        var h = std.math.atan2(self.b, self.a) * (180.0 / std.math.pi);
        if (h < 0.0) h += 360.0;

        return @import("lcha.zig").Lcha{
            .lightness = self.lightness,
            .chroma = c,
            .hue = h,
            .alpha = self.alpha,
        };
    }

    pub fn fromLcha(lch: @import("lcha.zig").Lcha) Laba {
        const h_rad = lch.hue * (std.math.pi / 180.0);
        const a = lch.chroma * @cos(h_rad);
        const b = lch.chroma * @sin(h_rad);

        return Laba{
            .lightness = lch.lightness,
            .a = a,
            .b = b,
            .alpha = lch.alpha,
        };
    }

    fn labFInv(t: f32) f32 {
        const delta: f32 = 6.0 / 29.0;

        if (t > delta) {
            return t * t * t;
        } else {
            return 3.0 * delta * delta * (t - 4.0 / 29.0);
        }
    }

    // Delta E 2000 color difference
    pub fn deltaE(self: Laba, other: Laba) f32 {
        // Simplified Delta E (Euclidean distance in Lab space)
        const dl = self.lightness - other.lightness;
        const da = self.a - other.a;
        const db = self.b - other.b;
        return std.math.sqrt(dl * dl + da * da + db * db);
    }
};

export fn laba_new(lightness: f32, a: f32, b: f32, alpha: f32) Laba {
    return Laba.new(lightness, a, b, alpha);
}

export fn laba_delta_e(a: Laba, b: Laba) f32 {
    return a.deltaE(b);
}
