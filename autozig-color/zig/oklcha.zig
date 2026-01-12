const std = @import("std");

pub const Oklcha = extern struct {
    lightness: f32, // L [0.0, 1.0]
    chroma: f32, // C [0.0, ~0.5]
    hue: f32, // H [0.0, 360.0]
    alpha: f32,

    pub fn new(lightness: f32, chroma: f32, hue: f32, alpha: f32) Oklcha {
        return .{ .lightness = lightness, .chroma = chroma, .hue = hue, .alpha = alpha };
    }

    pub fn toOklaba(self: Oklcha) @import("oklaba.zig").Oklaba {
        const h_rad = self.hue * (std.math.pi / 180.0);
        const a = self.chroma * @cos(h_rad);
        const b = self.chroma * @sin(h_rad);

        return @import("oklaba.zig").Oklaba{
            .lightness = self.lightness,
            .a = a,
            .b = b,
            .alpha = self.alpha,
        };
    }

    pub fn fromOklaba(oklab: @import("oklaba.zig").Oklaba) Oklcha {
        const c = std.math.sqrt(oklab.a * oklab.a + oklab.b * oklab.b);
        var h = std.math.atan2(oklab.b, oklab.a) * (180.0 / std.math.pi);
        if (h < 0.0) h += 360.0;

        return Oklcha{
            .lightness = oklab.lightness,
            .chroma = c,
            .hue = h,
            .alpha = oklab.alpha,
        };
    }

    pub fn withLightness(self: Oklcha, lightness: f32) Oklcha {
        return Oklcha{
            .lightness = lightness,
            .chroma = self.chroma,
            .hue = self.hue,
            .alpha = self.alpha,
        };
    }

    pub fn withChroma(self: Oklcha, chroma: f32) Oklcha {
        return Oklcha{
            .lightness = self.lightness,
            .chroma = chroma,
            .hue = self.hue,
            .alpha = self.alpha,
        };
    }

    pub fn withHue(self: Oklcha, hue: f32) Oklcha {
        return Oklcha{
            .lightness = self.lightness,
            .chroma = self.chroma,
            .hue = hue,
            .alpha = self.alpha,
        };
    }
};

export fn oklcha_new(lightness: f32, chroma: f32, hue: f32, alpha: f32) Oklcha {
    return Oklcha.new(lightness, chroma, hue, alpha);
}
