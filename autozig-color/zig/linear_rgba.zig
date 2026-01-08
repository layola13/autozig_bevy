const std = @import("std");
const Color = @import("color.zig").Color;

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

// FFI exports
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
