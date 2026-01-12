const std = @import("std");

pub const Oklaba = extern struct {
    lightness: f32, // L [0.0, 1.0]
    a: f32, // a (green-red)
    b: f32, // b (blue-yellow)
    alpha: f32,

    pub fn new(lightness: f32, a: f32, b: f32, alpha: f32) Oklaba {
        return .{ .lightness = lightness, .a = a, .b = b, .alpha = alpha };
    }

    pub fn toLinearRgba(self: Oklaba) @import("linear_rgba.zig").LinearRgba {
        // Oklab to Linear sRGB conversion
        const l_ = self.lightness + 0.3963377774 * self.a + 0.2158037573 * self.b;
        const m_ = self.lightness - 0.1055613458 * self.a - 0.0638541728 * self.b;
        const s_ = self.lightness - 0.0894841775 * self.a - 1.2914855480 * self.b;

        const l = l_ * l_ * l_;
        const m = m_ * m_ * m_;
        const s = s_ * s_ * s_;

        const r = 4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s;
        const g = -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s;
        const b = -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s;

        return @import("linear_rgba.zig").LinearRgba{
            .red = r,
            .green = g,
            .blue = b,
            .alpha = self.alpha,
        };
    }

    pub fn fromLinearRgba(color: @import("linear_rgba.zig").LinearRgba) Oklaba {
        // Linear sRGB to Oklab conversion
        const l = 0.4122214708 * color.red + 0.5363325363 * color.green + 0.0514459929 * color.blue;
        const m = 0.2119034982 * color.red + 0.6806995451 * color.green + 0.1073969566 * color.blue;
        const s = 0.0883024619 * color.red + 0.2817188376 * color.green + 0.6299787005 * color.blue;

        const l_ = std.math.cbrt(l);
        const m_ = std.math.cbrt(m);
        const s_ = std.math.cbrt(s);

        const lightness = 0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_;
        const a = 1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_;
        const b = 0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_;

        return Oklaba{
            .lightness = lightness,
            .a = a,
            .b = b,
            .alpha = color.alpha,
        };
    }

    pub fn toOklcha(self: Oklaba) @import("oklcha.zig").Oklcha {
        const c = std.math.sqrt(self.a * self.a + self.b * self.b);
        var h = std.math.atan2(self.b, self.a) * (180.0 / std.math.pi);
        if (h < 0.0) h += 360.0;

        return @import("oklcha.zig").Oklcha{
            .lightness = self.lightness,
            .chroma = c,
            .hue = h,
            .alpha = self.alpha,
        };
    }

    pub fn fromOklcha(oklch: @import("oklcha.zig").Oklcha) Oklaba {
        const h_rad = oklch.hue * (std.math.pi / 180.0);
        const a = oklch.chroma * @cos(h_rad);
        const b = oklch.chroma * @sin(h_rad);

        return Oklaba{
            .lightness = oklch.lightness,
            .a = a,
            .b = b,
            .alpha = oklch.alpha,
        };
    }
};

export fn oklaba_new(lightness: f32, a: f32, b: f32, alpha: f32) Oklaba {
    return Oklaba.new(lightness, a, b, alpha);
}
