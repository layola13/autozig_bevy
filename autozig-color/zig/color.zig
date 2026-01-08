const std = @import("std");

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

// FFI exports
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
