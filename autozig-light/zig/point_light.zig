//! Point Light implementation for autozig-light
//! Provides point light source with distance-based attenuation

const std = @import("std");
const math = std.math;

/// Point light source with omnidirectional emission
pub const PointLight = extern struct {
    color: [3]f32, // RGB color
    intensity: f32, // Light intensity in lumens
    range: f32, // Maximum light range
    radius: f32, // Light source radius (for soft shadows)
    shadows_enabled: u32, // 0 = disabled, 1 = enabled
    _padding: [3]f32, // Padding for 16-byte alignment

    /// Create a new point light with default values
    pub fn init() PointLight {
        return PointLight{
            .color = [3]f32{ 1.0, 1.0, 1.0 },
            .intensity = 800.0, // Default: 800 lumens (roughly 60W bulb)
            .range = 20.0,
            .radius = 0.0,
            .shadows_enabled = 0,
            ._padding = [3]f32{ 0.0, 0.0, 0.0 },
        };
    }

    /// Create a new point light with specified parameters
    pub fn new(color: [3]f32, intensity: f32, range: f32) PointLight {
        return PointLight{
            .color = color,
            .intensity = intensity,
            .range = range,
            .radius = 0.0,
            .shadows_enabled = 0,
            ._padding = [3]f32{ 0.0, 0.0, 0.0 },
        };
    }

    /// Calculate distance-based attenuation using inverse square law with smoothstep falloff
    /// Returns attenuation factor in range [0.0, 1.0]
    pub fn attenuation(self: *const PointLight, distance: f32) f32 {
        if (distance >= self.range) return 0.0;
        if (distance <= 0.0) return 1.0;

        // Inverse square law with distance offset to prevent singularity
        const distance_sq = distance * distance;
        const atten = 1.0 / (distance_sq + 1.0);

        // Smooth falloff at range boundary
        const falloff = 1.0 - (distance / self.range);
        const smooth_falloff = falloff * falloff;

        return atten * smooth_falloff;
    }

    /// Set light color (RGB values in range [0.0, 1.0])
    pub fn setColor(self: *PointLight, r: f32, g: f32, b: f32) void {
        self.color[0] = r;
        self.color[1] = g;
        self.color[2] = b;
    }

    /// Set light intensity in lumens
    pub fn setIntensity(self: *PointLight, lumens: f32) void {
        self.intensity = lumens;
    }

    /// Set light range
    pub fn setRange(self: *PointLight, new_range: f32) void {
        self.range = @max(0.0, new_range);
    }

    /// Set light radius for soft shadows
    pub fn setRadius(self: *PointLight, new_radius: f32) void {
        self.radius = @max(0.0, new_radius);
    }

    /// Enable shadow casting
    pub fn enableShadows(self: *PointLight) void {
        self.shadows_enabled = 1;
    }

    /// Disable shadow casting
    pub fn disableShadows(self: *PointLight) void {
        self.shadows_enabled = 0;
    }

    /// Check if shadows are enabled
    pub fn hasShadows(self: *const PointLight) bool {
        return self.shadows_enabled != 0;
    }
};

// C FFI exports
export fn point_light_init() PointLight {
    return PointLight.init();
}

export fn point_light_new(color: *const [3]f32, intensity: f32, range: f32) PointLight {
    return PointLight.new(color.*, intensity, range);
}

export fn point_light_attenuation(light: *const PointLight, distance: f32) f32 {
    return light.attenuation(distance);
}

export fn point_light_set_color(light: *PointLight, r: f32, g: f32, b: f32) void {
    light.setColor(r, g, b);
}

export fn point_light_set_intensity(light: *PointLight, lumens: f32) void {
    light.setIntensity(lumens);
}

export fn point_light_set_range(light: *PointLight, range: f32) void {
    light.setRange(range);
}

export fn point_light_set_radius(light: *PointLight, radius: f32) void {
    light.setRadius(radius);
}

export fn point_light_enable_shadows(light: *PointLight) void {
    light.enableShadows();
}

export fn point_light_disable_shadows(light: *PointLight) void {
    light.disableShadows();
}

export fn point_light_has_shadows(light: *const PointLight) bool {
    return light.hasShadows();
}
