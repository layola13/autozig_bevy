const std = @import("std");

pub const Hwba = extern struct {
    hue: f32, // [0.0, 360.0]
    whiteness: f32, // [0.0, 1.0]
    blackness: f32, // [0.0, 1.0]
    alpha: f32,

    pub fn new(hue: f32, whiteness: f32, blackness: f32, alpha: f32) Hwba {
        return .{ .hue = hue, .whiteness = whiteness, .blackness = blackness, .alpha = alpha };
    }

    pub fn toSrgba(self: Hwba) @import("srgba.zig").Srgba {
        // HWB to HSV conversion
        const w = std.math.clamp(self.whiteness, 0.0, 1.0);
        const b = std.math.clamp(self.blackness, 0.0, 1.0);

        // Handle case where w + b >= 1
        const sum = w + b;
        const normalized_w = if (sum >= 1.0) w / sum else w;
        const normalized_b = if (sum >= 1.0) b / sum else b;

        const v = 1.0 - normalized_b;
        const s = if (v == 0.0) 0.0 else 1.0 - (normalized_w / v);

        // Convert HSV to RGB
        return hsvToSrgba(self.hue, s, v, self.alpha);
    }

    pub fn fromSrgba(color: @import("srgba.zig").Srgba) Hwba {
        const r = color.red;
        const g = color.green;
        const b = color.blue;

        const max_val = @max(@max(r, g), b);
        const min_val = @min(@min(r, g), b);

        const whiteness = min_val;
        const blackness = 1.0 - max_val;

        // Calculate hue
        var hue: f32 = 0.0;
        if (max_val != min_val) {
            const delta = max_val - min_val;
            if (max_val == r) {
                hue = 60.0 * (@mod((g - b) / delta, 6.0));
            } else if (max_val == g) {
                hue = 60.0 * (((b - r) / delta) + 2.0);
            } else {
                hue = 60.0 * (((r - g) / delta) + 4.0);
            }
        }

        if (hue < 0.0) hue += 360.0;

        return Hwba{
            .hue = hue,
            .whiteness = whiteness,
            .blackness = blackness,
            .alpha = color.alpha,
        };
    }

    fn hsvToSrgba(h: f32, s: f32, v: f32, a: f32) @import("srgba.zig").Srgba {
        const hue = @mod(h, 360.0) / 60.0;
        const sat = std.math.clamp(s, 0.0, 1.0);
        const val = std.math.clamp(v, 0.0, 1.0);

        const c = val * sat;
        const x = c * (1.0 - @abs(@mod(hue, 2.0) - 1.0));
        const m = val - c;

        var r: f32 = 0.0;
        var g: f32 = 0.0;
        var b: f32 = 0.0;

        if (hue < 1.0) {
            r = c;
            g = x;
            b = 0.0;
        } else if (hue < 2.0) {
            r = x;
            g = c;
            b = 0.0;
        } else if (hue < 3.0) {
            r = 0.0;
            g = c;
            b = x;
        } else if (hue < 4.0) {
            r = 0.0;
            g = x;
            b = c;
        } else if (hue < 5.0) {
            r = x;
            g = 0.0;
            b = c;
        } else {
            r = c;
            g = 0.0;
            b = x;
        }

        return @import("srgba.zig").Srgba{
            .red = r + m,
            .green = g + m,
            .blue = b + m,
            .alpha = a,
        };
    }
};

export fn hwba_new(hue: f32, whiteness: f32, blackness: f32, alpha: f32) Hwba {
    return Hwba.new(hue, whiteness, blackness, alpha);
}
