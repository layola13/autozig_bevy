const std = @import("std");

// ============================================================================
// Srgba - Standard RGB with Alpha
// ============================================================================
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

    pub fn toLinear(self: Srgba) LinearRgba {
        return LinearRgba{
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
};

// ============================================================================
// LinearRgba - Linear RGB with Alpha
// ============================================================================
pub const LinearRgba = extern struct {
    red: f32,
    green: f32,
    blue: f32,
    alpha: f32,

    pub fn init(r: f32, g: f32, b: f32, a: f32) LinearRgba {
        return .{ .red = r, .green = g, .blue = b, .alpha = a };
    }

    pub fn fromRgba(color: Srgba) LinearRgba {
        return LinearRgba{
            .red = srgbToLinear(color.red),
            .green = srgbToLinear(color.green),
            .blue = srgbToLinear(color.blue),
            .alpha = color.alpha,
        };
    }

    pub fn toRgba(self: LinearRgba) Srgba {
        return Srgba{
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
};

// ============================================================================
// Hsla - HSL with Alpha
// ============================================================================
pub const Hsla = extern struct {
    hue: f32,
    saturation: f32,
    lightness: f32,
    alpha: f32,

    pub fn init(h: f32, s: f32, l: f32, a: f32) Hsla {
        return .{ .hue = h, .saturation = s, .lightness = l, .alpha = a };
    }

    pub fn toSrgba(self: Hsla) Srgba {
        const h = @mod(self.hue, 360.0) / 60.0;
        const s = std.math.clamp(self.saturation, 0.0, 1.0);
        const l = std.math.clamp(self.lightness, 0.0, 1.0);

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

        return Srgba{
            .red = r + m,
            .green = g + m,
            .blue = b + m,
            .alpha = self.alpha,
        };
    }

    pub fn fromSrgba(color: Srgba) Hsla {
        const r = color.red;
        const g = color.green;
        const b = color.blue;

        const max_val = @max(@max(r, g), b);
        const min_val = @min(@min(r, g), b);
        const delta = max_val - min_val;

        const l = (max_val + min_val) / 2.0;

        if (delta == 0.0) {
            return Hsla{
                .hue = 0.0,
                .saturation = 0.0,
                .lightness = l,
                .alpha = color.alpha,
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
            .hue = h,
            .saturation = s,
            .lightness = l,
            .alpha = color.alpha,
        };
    }
};

// ============================================================================
// Hsva - HSV with Alpha
// ============================================================================
pub const Hsva = extern struct {
    hue: f32,
    saturation: f32,
    value: f32,
    alpha: f32,

    pub fn init(h: f32, s: f32, v: f32, a: f32) Hsva {
        return .{ .hue = h, .saturation = s, .value = v, .alpha = a };
    }

    pub fn toSrgba(self: Hsva) Srgba {
        const h = @mod(self.hue, 360.0) / 60.0;
        const s = std.math.clamp(self.saturation, 0.0, 1.0);
        const v = std.math.clamp(self.value, 0.0, 1.0);

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

        return Srgba{
            .red = r + m,
            .green = g + m,
            .blue = b + m,
            .alpha = self.alpha,
        };
    }

    pub fn fromSrgba(color: Srgba) Hsva {
        const r = color.red;
        const g = color.green;
        const b = color.blue;

        const max_val = @max(@max(r, g), b);
        const min_val = @min(@min(r, g), b);
        const delta = max_val - min_val;

        const v = max_val;

        if (delta == 0.0) {
            return Hsva{
                .hue = 0.0,
                .saturation = 0.0,
                .value = v,
                .alpha = color.alpha,
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
            .hue = h,
            .saturation = s,
            .value = v,
            .alpha = color.alpha,
        };
    }
};

// ============================================================================
// Hwba - HWB with Alpha
// ============================================================================
pub const Hwba = extern struct {
    hue: f32,
    whiteness: f32,
    blackness: f32,
    alpha: f32,

    pub fn new(hue: f32, whiteness: f32, blackness: f32, alpha: f32) Hwba {
        return .{ .hue = hue, .whiteness = whiteness, .blackness = blackness, .alpha = alpha };
    }
};

// ============================================================================
// Laba - CIE Lab with Alpha
// ============================================================================
pub const Laba = extern struct {
    lightness: f32,
    a: f32,
    b: f32,
    alpha: f32,

    pub fn new(lightness: f32, a: f32, b: f32, alpha: f32) Laba {
        return .{ .lightness = lightness, .a = a, .b = b, .alpha = alpha };
    }

    pub fn deltaE(self: Laba, other: Laba) f32 {
        const dl = self.lightness - other.lightness;
        const da = self.a - other.a;
        const db = self.b - other.b;
        return std.math.sqrt(dl * dl + da * da + db * db);
    }
};

// ============================================================================
// Lcha - LCH with Alpha
// ============================================================================
pub const Lcha = extern struct {
    lightness: f32,
    chroma: f32,
    hue: f32,
    alpha: f32,

    pub fn new(lightness: f32, chroma: f32, hue: f32, alpha: f32) Lcha {
        return .{ .lightness = lightness, .chroma = chroma, .hue = hue, .alpha = alpha };
    }
};

// ============================================================================
// Oklaba - Oklab with Alpha
// ============================================================================
pub const Oklaba = extern struct {
    lightness: f32,
    a: f32,
    b: f32,
    alpha: f32,

    pub fn new(lightness: f32, a: f32, b: f32, alpha: f32) Oklaba {
        return .{ .lightness = lightness, .a = a, .b = b, .alpha = alpha };
    }
};

// ============================================================================
// Oklcha - Oklch with Alpha
// ============================================================================
pub const Oklcha = extern struct {
    lightness: f32,
    chroma: f32,
    hue: f32,
    alpha: f32,

    pub fn new(lightness: f32, chroma: f32, hue: f32, alpha: f32) Oklcha {
        return .{ .lightness = lightness, .chroma = chroma, .hue = hue, .alpha = alpha };
    }
};

// ============================================================================
// Xyza - CIE XYZ with Alpha
// ============================================================================
pub const Xyza = extern struct {
    x: f32,
    y: f32,
    z: f32,
    alpha: f32,

    pub fn new(x: f32, y: f32, z: f32, alpha: f32) Xyza {
        return .{ .x = x, .y = y, .z = z, .alpha = alpha };
    }
};

// ============================================================================
// FFI Exports
// ============================================================================

// Srgba exports
export fn srgba_new(red: f32, green: f32, blue: f32, alpha: f32) Srgba {
    return Srgba.new(red, green, blue, alpha);
}

export fn srgba_rgb(red: f32, green: f32, blue: f32) Srgba {
    return Srgba.rgb(red, green, blue);
}

// LinearRgba exports
export fn linear_rgba_init(r: f32, g: f32, b: f32, a: f32) LinearRgba {
    return LinearRgba.init(r, g, b, a);
}

export fn linear_rgba_from_rgba(color: Srgba) LinearRgba {
    return LinearRgba.fromRgba(color);
}

export fn linear_rgba_to_rgba(linear: LinearRgba) Srgba {
    return linear.toRgba();
}

export fn linear_rgba_lerp(a: LinearRgba, b: LinearRgba, t: f32) LinearRgba {
    return a.lerp(b, t);
}

// Hsla exports
export fn hsla_init(h: f32, s: f32, l: f32, a: f32) Hsla {
    return Hsla.init(h, s, l, a);
}

export fn hsla_to_srgba(hsla: Hsla) Srgba {
    return hsla.toSrgba();
}

export fn hsla_from_srgba(color: Srgba) Hsla {
    return Hsla.fromSrgba(color);
}

// Hsva exports
export fn hsva_init(h: f32, s: f32, v: f32, a: f32) Hsva {
    return Hsva.init(h, s, v, a);
}

export fn hsva_to_srgba(hsva: Hsva) Srgba {
    return hsva.toSrgba();
}

export fn hsva_from_srgba(color: Srgba) Hsva {
    return Hsva.fromSrgba(color);
}

// Hwba exports
export fn hwba_new(hue: f32, whiteness: f32, blackness: f32, alpha: f32) Hwba {
    return Hwba.new(hue, whiteness, blackness, alpha);
}

// Laba exports
export fn laba_new(lightness: f32, a: f32, b: f32, alpha: f32) Laba {
    return Laba.new(lightness, a, b, alpha);
}

export fn laba_delta_e(a: Laba, b: Laba) f32 {
    return a.deltaE(b);
}

// Lcha exports
export fn lcha_new(lightness: f32, chroma: f32, hue: f32, alpha: f32) Lcha {
    return Lcha.new(lightness, chroma, hue, alpha);
}

// Oklaba exports
export fn oklaba_new(lightness: f32, a: f32, b: f32, alpha: f32) Oklaba {
    return Oklaba.new(lightness, a, b, alpha);
}

// Oklcha exports
export fn oklcha_new(lightness: f32, chroma: f32, hue: f32, alpha: f32) Oklcha {
    return Oklcha.new(lightness, chroma, hue, alpha);
}

// Xyza exports
export fn xyza_new(x: f32, y: f32, z: f32, alpha: f32) Xyza {
    return Xyza.new(x, y, z, alpha);
}
