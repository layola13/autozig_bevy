const std = @import("std");

pub const Lcha = extern struct {
    lightness: f32, // L* [0.0, 100.0]
    chroma: f32, // C* [0.0, ~100.0+]
    hue: f32, // H* [0.0, 360.0]
    alpha: f32,

    pub fn new(lightness: f32, chroma: f32, hue: f32, alpha: f32) Lcha {
        return .{ .lightness = lightness, .chroma = chroma, .hue = hue, .alpha = alpha };
    }

    pub fn toLaba(self: Lcha) @import("laba.zig").Laba {
        const h_rad = self.hue * (std.math.pi / 180.0);
        const a = self.chroma * @cos(h_rad);
        const b = self.chroma * @sin(h_rad);

        return @import("laba.zig").Laba{
            .lightness = self.lightness,
            .a = a,
            .b = b,
            .alpha = self.alpha,
        };
    }

    pub fn fromLaba(lab: @import("laba.zig").Laba) Lcha {
        const c = std.math.sqrt(lab.a * lab.a + lab.b * lab.b);
        var h = std.math.atan2(lab.b, lab.a) * (180.0 / std.math.pi);
        if (h < 0.0) h += 360.0;

        return Lcha{
            .lightness = lab.lightness,
            .chroma = c,
            .hue = h,
            .alpha = lab.alpha,
        };
    }

    pub fn withLightness(self: Lcha, lightness: f32) Lcha {
        return Lcha{
            .lightness = lightness,
            .chroma = self.chroma,
            .hue = self.hue,
            .alpha = self.alpha,
        };
    }

    pub fn withChroma(self: Lcha, chroma: f32) Lcha {
        return Lcha{
            .lightness = self.lightness,
            .chroma = chroma,
            .hue = self.hue,
            .alpha = self.alpha,
        };
    }

    pub fn withHue(self: Lcha, hue: f32) Lcha {
        return Lcha{
            .lightness = self.lightness,
            .chroma = self.chroma,
            .hue = hue,
            .alpha = self.alpha,
        };
    }
};

export fn lcha_new(lightness: f32, chroma: f32, hue: f32, alpha: f32) Lcha {
    return Lcha.new(lightness, chroma, hue, alpha);
}
