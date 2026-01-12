const std = @import("std");

pub const LinearRgba = extern struct {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,

    pub fn init(r: f32, g: f32, b: f32, a: f32) LinearRgba {
        return .{ .red = r, .green = g, .blue = b, .alpha = a };
    }

    pub fn fromSrgba(color: @import("srgba.zig").Srgba) LinearRgba {
        return LinearRgba{
            .red = srgbToLinear(color.red),
            .green = srgbToLinear(color.green),
            .blue = srgbToLinear(color.blue),
            .alpha = color.alpha,
        };
    }

    pub fn toSrgba(self: LinearRgba) @import("srgba.zig").Srgba {
        return @import("srgba.zig").Srgba{
            .red = linearToSrgb(self.red),
            .green = linearToSrgb(self.green),
            .blue = linearToSrgb(self.blue),
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

    fn linearToSrgb(value: f32) f32 {
        if (value <= 0.0031308) {
            return value * 12.92;
        } else {
            return 1.055 * std.math.pow(f32, value, 1.0 / 2.4) - 0.055;
        }
    }

    pub fn lerp(self: LinearRgba, other: LinearRgba, t: f32) LinearRgba {
        return LinearRgba{
            .red = self.red + (other.red - self.red) * t,
            .green = self.green + (other.green - self.green) * t,
            .blue = self.blue + (other.blue - self.blue) * t,
            .alpha = self.alpha + (other.alpha - self.alpha) * t,
        };
    }

    pub fn toOklaba(self: LinearRgba) @import("oklaba.zig").Oklaba {
        return @import("oklaba.zig").Oklaba.fromLinearRgba(self);
    }

    pub fn fromOklaba(oklab: @import("oklaba.zig").Oklaba) LinearRgba {
        return oklab.toLinearRgba();
    }

    pub fn toXyza(self: LinearRgba) @import("xyza.zig").Xyza {
        return @import("xyza.zig").Xyza.fromLinearRgba(self);
    }

    pub fn fromXyza(xyz: @import("xyza.zig").Xyza) LinearRgba {
        return xyz.toLinearRgba();
    }
};

export fn linear_rgba_init(r: f32, g: f32, b: f32, a: f32) LinearRgba {
    return LinearRgba.init(r, g, b, a);
}

export fn linear_rgba_lerp(a: LinearRgba, b: LinearRgba, t: f32) LinearRgba {
    return a.lerp(b, t);
}
