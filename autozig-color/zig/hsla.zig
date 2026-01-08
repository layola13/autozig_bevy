const std = @import("std");

pub const Hsla = extern struct {
    h: f32, // [0.0, 360.0]
    s: f32, // [0.0, 1.0]
    l: f32, // [0.0, 1.0]
    a: f32,

    pub fn init(h: f32, s: f32, l: f32, a: f32) Hsla {
        return .{ .h = h, .s = s, .l = l, .a = a };
    }

    pub fn toRgba(self: Hsla) @import("color.zig").Color {
        const h = @mod(self.h, 360.0) / 60.0;
        const s = std.math.clamp(self.s, 0.0, 1.0);
        const l = std.math.clamp(self.l, 0.0, 1.0);

        const c = (1.0 - @abs(2.0 * l - 1.0)) * s;
        const x = c * (1.0 - @abs(@mod(h, 2.0) - 1.0));
        const m = l - c / 2.0;

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

        return @import("color.zig").Color{
            .r = r + m,
            .g = g + m,
            .b = b + m,
            .a = self.a,
        };
    }

    pub fn fromRgba(color: @import("color.zig").Color) Hsla {
        const r = color.r;
        const g = color.g;
        const b = color.b;

        const max_val = @max(@max(r, g), b);
        const min_val = @min(@min(r, g), b);
        const delta = max_val - min_val;

        const l = (max_val + min_val) / 2.0;

        if (delta == 0.0) {
            return @import("color.zig").Hsla{
                .h = 0.0,
                .s = 0.0,
                .l = l,
                .a = color.a,
            };
        }

        const s = if (l < 0.5)
            delta / (max_val + min_val)
        else
            delta / (2.0 - max_val - min_val);

        var h: f32 = 0.0;
        if (max_val == r) {
            h = 60.0 * (@mod((g - b) / delta, 6.0));
        } else if (max_val == g) {
            h = 60.0 * (((b - r) / delta) + 2.0);
        } else {
            h = 60.0 * (((r - g) / delta) + 4.0);
        }

        if (h < 0.0) h += 360.0;

        return @import("color.zig").Hsla{
            .h = h,
            .s = s,
            .l = l,
            .a = color.a,
        };
    }
};

// FFI exports
export fn hsla_init(h: f32, s: f32, l: f32, a: f32) Hsla {
    return Hsla.init(h, s, l, a);
}

export fn hsla_to_rgba(hsla: Hsla) @import("color.zig").Color {
    return hsla.toRgba();
}

export fn hsla_from_rgba(color: @import("color.zig").Color) Hsla {
    return Hsla.fromRgba(color);
}
