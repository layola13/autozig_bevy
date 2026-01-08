const std = @import("std");
const Color = @import("color.zig").Color;
const Hsla = @import("hsla.zig").Hsla;

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

// FFI exports
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
