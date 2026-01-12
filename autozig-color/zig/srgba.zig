const std = @import("std");

pub const Srgba = extern struct {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,

    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) Srgba {
        return .{ .red = red, .green = green, .blue = blue, .alpha = alpha };
    }

    pub fn rgb(red: f32, green: f32, blue: f32) Srgba {
        return new(red, green, blue, 1.0);
    }

    pub fn toLinear(self: Srgba) @import("linear_rgba.zig").LinearRgba {
        return @import("linear_rgba.zig").LinearRgba{
            .red = srgbToLinear(self.red),
            .green = srgbToLinear(self.green),
            .blue = srgbToLinear(self.blue),
            .alpha = self.alpha,
        };
    }

    fn srgbToLinear(value: f32) f32 {
        if (value <= 0.04045) {
            return value / 12.92;
        } else {
            return std.math.pow(f32, (value + 0.055) / 1.055, 2.4);
        }
    }

    pub fn parseHex(hex_str: []const u8) !Srgba {
        if (hex_str.len < 6) return error.InvalidHexString;

        const start: usize = if (hex_str[0] == '#') 1 else 0;
        if (hex_str.len - start < 6) return error.InvalidHexString;

        const r = try parseHexByte(hex_str[start .. start + 2]);
        const g = try parseHexByte(hex_str[start + 2 .. start + 4]);
        const b = try parseHexByte(hex_str[start + 4 .. start + 6]);

        const a: u8 = if (hex_str.len - start >= 8)
            try parseHexByte(hex_str[start + 6 .. start + 8])
        else
            255;

        return Srgba{
            .red = @as(f32, @floatFromInt(r)) / 255.0,
            .green = @as(f32, @floatFromInt(g)) / 255.0,
            .blue = @as(f32, @floatFromInt(b)) / 255.0,
            .alpha = @as(f32, @floatFromInt(a)) / 255.0,
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
};

export fn srgba_new(red: f32, green: f32, blue: f32, alpha: f32) Srgba {
    return Srgba.new(red, green, blue, alpha);
}

export fn srgba_rgb(red: f32, green: f32, blue: f32) Srgba {
    return Srgba.rgb(red, green, blue);
}
