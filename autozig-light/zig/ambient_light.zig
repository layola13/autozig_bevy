//! Ambient Light implementation for autozig-light
//! Provides uniform ambient illumination

const std = @import("std");
const math = std.math;

/// Ambient light providing uniform scene illumination
pub const AmbientLight = extern struct {
    color: [3]f32, // RGB color
    brightness: f32, // Brightness/intensity factor

    /// Create a new ambient light with default values
    pub fn init() AmbientLight {
        return AmbientLight{
            .color = [3]f32{ 1.0, 1.0, 1.0 },
            .brightness = 0.1, // Default: 10% ambient lighting
        };
    }

    /// Create a new ambient light with specified parameters
    pub fn new(color: [3]f32, brightness: f32) AmbientLight {
        return AmbientLight{
            .color = color,
            .brightness = @max(0.0, brightness),
        };
    }

    /// Set light color (RGB values in range [0.0, 1.0])
    pub fn setColor(self: *AmbientLight, r: f32, g: f32, b: f32) void {
        self.color[0] = r;
        self.color[1] = g;
        self.color[2] = b;
    }

    /// Set light brightness
    pub fn setBrightness(self: *AmbientLight, brightness: f32) void {
        self.brightness = @max(0.0, brightness);
    }

    /// Get the effective color (color * brightness)
    pub fn getEffectiveColor(self: *const AmbientLight) [3]f32 {
        return [3]f32{
            self.color[0] * self.brightness,
            self.color[1] * self.brightness,
            self.color[2] * self.brightness,
        };
    }
};

// C FFI exports
export fn ambient_light_init(out: *AmbientLight) void {
    out.* = AmbientLight.init();
}

export fn ambient_light_new(out: *AmbientLight, color: *const [3]f32, brightness: f32) void {
    out.* = AmbientLight.new(color.*, brightness);
}

export fn ambient_light_set_color(light: *AmbientLight, r: f32, g: f32, b: f32) void {
    light.setColor(r, g, b);
}

export fn ambient_light_set_brightness(light: *AmbientLight, brightness: f32) void {
    light.setBrightness(brightness);
}

export fn ambient_light_get_effective_color(light: *const AmbientLight, out: *[3]f32) void {
    out.* = light.getEffectiveColor();
}
