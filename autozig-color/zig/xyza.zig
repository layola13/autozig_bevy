const std = @import("std");

pub const Xyza = extern struct {
    x: f32,
    y: f32,
    z: f32,
    alpha: f32,

    // D65 standard illuminant
    const D65_X: f32 = 0.95047;
    const D65_Y: f32 = 1.00000;
    const D65_Z: f32 = 1.08883;

    pub fn new(x: f32, y: f32, z: f32, alpha: f32) Xyza {
        return .{ .x = x, .y = y, .z = z, .alpha = alpha };
    }

    pub fn toLinearRgba(self: Xyza) @import("linear_rgba.zig").LinearRgba {
        // XYZ to Linear RGB conversion matrix (D65)
        const r = 3.2404542 * self.x - 1.5371385 * self.y - 0.4985314 * self.z;
        const g = -0.9692660 * self.x + 1.8760108 * self.y + 0.0415560 * self.z;
        const b = 0.0556434 * self.x - 0.2040259 * self.y + 1.0572252 * self.z;

        return @import("linear_rgba.zig").LinearRgba{
            .red = r,
            .green = g,
            .blue = b,
            .alpha = self.alpha,
        };
    }

    pub fn fromLinearRgba(color: @import("linear_rgba.zig").LinearRgba) Xyza {
        // Linear RGB to XYZ conversion matrix (D65)
        const x = 0.4124564 * color.red + 0.3575761 * color.green + 0.1804375 * color.blue;
        const y = 0.2126729 * color.red + 0.7151522 * color.green + 0.0721750 * color.blue;
        const z = 0.0193339 * color.red + 0.1191920 * color.green + 0.9503041 * color.blue;

        return Xyza{
            .x = x,
            .y = y,
            .z = z,
            .alpha = color.alpha,
        };
    }

    pub fn toLaba(self: Xyza) @import("laba.zig").Laba {
        const xr = self.x / D65_X;
        const yr = self.y / D65_Y;
        const zr = self.z / D65_Z;

        const fx = labF(xr);
        const fy = labF(yr);
        const fz = labF(zr);

        const l = 116.0 * fy - 16.0;
        const a = 500.0 * (fx - fy);
        const b = 200.0 * (fy - fz);

        return @import("laba.zig").Laba{
            .lightness = l,
            .a = a,
            .b = b,
            .alpha = self.alpha,
        };
    }

    fn labF(t: f32) f32 {
        const delta: f32 = 6.0 / 29.0;
        const delta_cubed = delta * delta * delta;

        if (t > delta_cubed) {
            return std.math.pow(f32, t, 1.0 / 3.0);
        } else {
            return t / (3.0 * delta * delta) + 4.0 / 29.0;
        }
    }
};

export fn xyza_new(x: f32, y: f32, z: f32, alpha: f32) Xyza {
    return Xyza.new(x, y, z, alpha);
}
