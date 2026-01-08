const std = @import("std");
const Color = @import("color.zig").Color;

pub const Hsva = extern struct {
    h: f32, // [0.0, 360.0]
    s: f32, // [0.0, 1.0]
    v: f32, // [0.0, 1.0]
    a: f32,

    pub fn init(h: f32, s: f32, v: f32, a: f32) Hsva {
        return .{ .h = h, .s = s, .v = v, .a = a };
    }

    pub fn toRgba(self: Hsva) Color {
        const h = @mod(self.h, 360.0) / 60.0;
        const s = std.math.clamp(self.s, 0.0, 1.0);
        const v = std.math.clamp(self.v, 0.0, 1.0);

        const c = v * s;
        const x = c * (1.0 - @abs(@mod(h, 2.0) - 1.0));
        const m = v - c;

        var r: f32 = 0.0;
        var g: f32 = 0.0;
        var b: f32 = 0.0;

        if (h < 1.0) {
            r = c;
            g = x;
            b = 0.0;
        } else if (h < 2.0) {
            r = x;
            g = c;
            b = 0.0;
        } else if (h < 3.0) {
            r = 0.0;
            g = c;
            b = x;
        } else if (h < 4.0) {
            r = 0.0;
            g = x;
            b = c;
        } else if (h < 5.0) {
            r = x;
            g = 0.0;
            b = c;
        } else {
            r = c;
            g = 0.0;
            b = x;
        }

        return Color{
            .r = r + m,
            .g = g + m,
            .b = b + m,
            .a = self.a,
        };
    }

    pub fn fromRgba(color: Color) Hsva {
        const r = color.r;
        const g = color.g;
        const b = color.b;

        const max_val = @max(@max(r, g), b);
        const min_val = @min(@min(r, g), b);
        const delta = max_val - min_val;

        const v = max_val;

        if (delta == 0.0) {
            return Hsva{
                .h = 0.0,
                .s = 0.0,
                .v = v,
                .a = color.a,
            };
        }

        const s = delta / max_val;

        var h: f32 = 0.0;
        if (max_val == r) {
            h = 60.0 * (@mod((g - b) / delta, 6.0));
        } else if (max_val == g) {
            h = 60.0 * (((b - r) / delta) + 2.0);
        } else {
            h = 60.0 * (((r - g) / delta) + 4.0);
        }

        if (h < 0.0) h += 360.0;

        return Hsva{
            .h = h,
            .s = s,
            .v = v,
            .a = color.a,
        };
    }
};

// FFI exports
export fn hsva_init(h: f32, s: f32, v: f32, a: f32) Hsva {
    return Hsva.init(h, s, v, a);
}

export fn hsva_to_rgba(hsva: Hsva) Color {
    return hsva.toRgba();
}

export fn hsva_from_rgba(color: Color) Hsva {
    return Hsva.fromRgba(color);
}
