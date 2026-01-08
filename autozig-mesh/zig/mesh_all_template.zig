const std = @import("std");

// ============================================================================
// Color type
// ============================================================================
pub const Color = extern struct {
    r: f32,
    g: f32,
    b: f32,
    a: f32,

    pub fn rgb(r: f32, g: f32, b: f32) Color {
        return .{ .r = r, .g = g, .b = b, .a = 1.0 };
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) Color {
        return .{ .r = r, .g = g, .b = b, .a = a };
    }

    pub fn hex(hex_str: []const u8) !Color {
        if (hex_str.len < 6) return error.InvalidHexString;

        const start: usize = if (hex_str[0] == '#') 1 else 0;

        if (hex_str.len - start < 6) return error.InvalidHexString;

        const r = try parseHexByte(hex_str[start .. start + 2]);
        const g = try parseHexByte(hex_str[start + 2 .. start + 4]);
        const b = try parseHexByte(hex_str[start + 4 .. start + 6]);

        const a: f32 = if (hex_str.len - start >= 8)
            @as(f32, @floatFromInt(try parseHexByte(hex_str[start + 6 .. start + 8]))) / 255.0
        else
            1.0;

        return Color{
            .r = @as(f32, @floatFromInt(r)) / 255.0,
            .g = @as(f32, @floatFromInt(g)) / 255.0,
            .b = @as(f32, @floatFromInt(b)) / 255.0,
            .a = a,
        };
    }

    fn parseHexByte(hex_bytes: []const u8) !u8 {
        if (hex_bytes.len != 2) return error.InvalidHexByte;

        const high = try parseHexDigit(hex_bytes[0]);
        const low = try parseHexDigit(hex_bytes[1]);

        return (high << 4) | low;
    }

    fn parseHexDigit(c: u8) !u8 {
        return switch (c) {
            '0'...'9' => c - '0',
            'a'...'f' => c - 'a' + 10,
            'A'...'F' => c - 'A' + 10,
            else => error.InvalidHexDigit,
        };
    }

    pub fn withAlpha(self: Color, alpha: f32) Color {
        return Color{
            .r = self.r,
            .g = self.g,
            .b = self.b,
            .a = alpha,
        };
    }

    pub fn lerp(self: Color, other: Color, t: f32) Color {
        return Color{
            .r = self.r + (other.r - self.r) * t,
            .g = self.g + (other.g - self.g) * t,
            .b = self.b + (other.b - self.b) * t,
            .a = self.a + (other.a - self.a) * t,
        };
    }

    pub fn mix(self: Color, other: Color, weight: f32) Color {
        return self.lerp(other, weight);
    }
};

// ============================================================================
// Hsla type
// ============================================================================
pub const Hsla = extern struct {
    h: f32, // [0.0, 360.0]
    s: f32, // [0.0, 1.0]
    l: f32, // [0.0, 1.0]
    a: f32,

    pub fn init(h: f32, s: f32, l: f32, a: f32) Hsla {
        return .{ .h = h, .s = s, .l = l, .a = a };
    }

    pub fn toRgba(self: Hsla) Color {
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

        return Color{
            .r = r + m,
            .g = g + m,
            .b = b + m,
            .a = self.a,
        };
    }

    pub fn fromRgba(color: Color) Hsla {
        const r = color.r;
        const g = color.g;
        const b = color.b;

        const max_val = @max(@max(r, g), b);
        const min_val = @min(@min(r, g), b);
        const delta = max_val - min_val;

        const l = (max_val + min_val) / 2.0;

        if (delta == 0.0) {
            return Hsla{
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

        return Hsla{
            .h = h,
            .s = s,
            .l = l,
            .a = color.a,
        };
    }
};

// ============================================================================
// Hsva type
// ============================================================================
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

// ============================================================================
// LinearRgba type
// ============================================================================
pub const LinearRgba = extern struct {
    r: f32,
    g: f32,
    b: f32,
    a: f32,

    pub fn init(r: f32, g: f32, b: f32, a: f32) LinearRgba {
        return .{ .r = r, .g = g, .b = b, .a = a };
    }

    pub fn fromRgba(color: Color) LinearRgba {
        return LinearRgba{
            .r = srgbToLinear(color.r),
            .g = srgbToLinear(color.g),
            .b = srgbToLinear(color.b),
            .a = color.a,
        };
    }

    pub fn toRgba(self: LinearRgba) Color {
        return Color{
            .r = linearToSrgb(self.r),
            .g = linearToSrgb(self.g),
            .b = linearToSrgb(self.b),
            .a = self.a,
        };
    }

    fn srgbToLinear(value: f32) f32 {
        if (value <= 0.04045) {
            return value / 12.92;
        } else {
            return std.math.pow(f32, (value + 0.055) / 1.055, 2.4);
        }
    }

    fn linearToSrgb(value: f32) f32 {
        if (value <= 0.0031308) {
            return value * 12.92;
        } else {
            return 1.055 * std.math.pow(f32, value, 1.0 / 2.4) - 0.055;
        }
    }

    pub fn lerp(self: LinearRgba, other: LinearRgba, t: f32) LinearRgba {
        return LinearRgba{
            .r = self.r + (other.r - self.r) * t,
            .g = self.g + (other.g - self.g) * t,
            .b = self.b + (other.b - self.b) * t,
            .a = self.a + (other.a - self.a) * t,
        };
    }
};

// ============================================================================
// Color operations
// ============================================================================
pub fn lighten(color: Color, amount: f32) Color {
    const hsl = Hsla.fromRgba(color);
    const new_l = std.math.clamp(hsl.l + amount, 0.0, 1.0);
    const new_hsl = Hsla{ .h = hsl.h, .s = hsl.s, .l = new_l, .a = hsl.a };
    return new_hsl.toRgba();
}

pub fn darken(color: Color, amount: f32) Color {
    return lighten(color, -amount);
}

pub fn saturate(color: Color, amount: f32) Color {
    const hsl = Hsla.fromRgba(color);
    const new_s = std.math.clamp(hsl.s + amount, 0.0, 1.0);
    const new_hsl = Hsla{ .h = hsl.h, .s = new_s, .l = hsl.l, .a = hsl.a };
    return new_hsl.toRgba();
}

pub fn desaturate(color: Color, amount: f32) Color {
    return saturate(color, -amount);
}

// ============================================================================
// Standard colors
// ============================================================================
pub const WHITE = Color.rgb(1.0, 1.0, 1.0);
pub const BLACK = Color.rgb(0.0, 0.0, 0.0);
pub const RED = Color.rgb(1.0, 0.0, 0.0);
pub const GREEN = Color.rgb(0.0, 1.0, 0.0);
pub const BLUE = Color.rgb(0.0, 0.0, 1.0);
pub const YELLOW = Color.rgb(1.0, 1.0, 0.0);
pub const CYAN = Color.rgb(0.0, 1.0, 1.0);
pub const MAGENTA = Color.rgb(1.0, 0.0, 1.0);
pub const GRAY = Color.rgb(0.5, 0.5, 0.5);
pub const DARK_GRAY = Color.rgb(0.25, 0.25, 0.25);
pub const LIGHT_GRAY = Color.rgb(0.75, 0.75, 0.75);
pub const ORANGE = Color.rgb(1.0, 0.5, 0.0);
pub const PURPLE = Color.rgb(0.5, 0.0, 0.5);
pub const PINK = Color.rgb(1.0, 0.75, 0.8);
pub const BROWN = Color.rgb(0.6, 0.4, 0.2);
pub const TRANSPARENT = Color.rgba(0.0, 0.0, 0.0, 0.0);

// ============================================================================
// FFI exports
// ============================================================================

// Color exports
export fn color_rgb(r: f32, g: f32, b: f32) Color {
    return Color.rgb(r, g, b);
}

export fn color_rgba(r: f32, g: f32, b: f32, a: f32) Color {
    return Color.rgba(r, g, b, a);
}

export fn color_with_alpha(color: Color, alpha: f32) Color {
    return color.withAlpha(alpha);
}

export fn color_lerp(a: Color, b: Color, t: f32) Color {
    return a.lerp(b, t);
}

export fn color_mix(a: Color, b: Color, weight: f32) Color {
    return a.mix(b, weight);
}

// Hsla exports
export fn hsla_init(h: f32, s: f32, l: f32, a: f32) Hsla {
    return Hsla.init(h, s, l, a);
}

export fn hsla_to_rgba(hsla: Hsla) Color {
    return hsla.toRgba();
}

export fn hsla_from_rgba(color: Color) Hsla {
    return Hsla.fromRgba(color);
}

// Hsva exports
export fn hsva_init(h: f32, s: f32, v: f32, a: f32) Hsva {
    return Hsva.init(h, s, v, a);
}

export fn hsva_to_rgba(hsva: Hsva) Color {
    return hsva.toRgba();
}

export fn hsva_from_rgba(color: Color) Hsva {
    return Hsva.fromRgba(color);
}

// LinearRgba exports
export fn linear_rgba_init(r: f32, g: f32, b: f32, a: f32) LinearRgba {
    return LinearRgba.init(r, g, b, a);
}

export fn linear_rgba_from_rgba(color: Color) LinearRgba {
    return LinearRgba.fromRgba(color);
}

export fn linear_rgba_to_rgba(linear: LinearRgba) Color {
    return linear.toRgba();
}

export fn linear_rgba_lerp(a: LinearRgba, b: LinearRgba, t: f32) LinearRgba {
    return a.lerp(b, t);
}

// Color operations exports
export fn color_lighten(color: Color, amount: f32) Color {
    return lighten(color, amount);
}

export fn color_darken(color: Color, amount: f32) Color {
    return darken(color, amount);
}

export fn color_saturate(color: Color, amount: f32) Color {
    return saturate(color, amount);
}

export fn color_desaturate(color: Color, amount: f32) Color {
    return desaturate(color, amount);
}

// Standard colors exports
export fn color_white() Color {
    return WHITE;
}

export fn color_black() Color {
    return BLACK;
}

export fn color_red() Color {
    return RED;
}

export fn color_green() Color {
    return GREEN;
}

export fn color_blue() Color {
    return BLUE;
}

export fn color_yellow() Color {
    return YELLOW;
}

export fn color_cyan() Color {
    return CYAN;
}

export fn color_magenta() Color {
    return MAGENTA;
}

export fn color_gray() Color {
    return GRAY;
}

export fn color_orange() Color {
    return ORANGE;
}

export fn color_purple() Color {
    return PURPLE;
}

export fn color_pink() Color {
    return PINK;
}

export fn color_transparent() Color {
    return TRANSPARENT;
}
