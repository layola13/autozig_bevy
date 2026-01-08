//! Spot Light implementation for autozig-light
//! Provides spot light source with cone-shaped emission

const std = @import("std");
const math = std.math;

/// Spot light source with cone-shaped emission
pub const SpotLight = extern struct {
    color: [3]f32, // RGB color
    intensity: f32, // Light intensity in lumens
    range: f32, // Maximum light range
    inner_angle: f32, // Inner cone angle in radians
    outer_angle: f32, // Outer cone angle in radians
    direction: [3]f32, // Spotlight direction (normalized)
    shadows_enabled: u32, // 0 = disabled, 1 = enabled
    _padding: [3]f32, // Padding for 16-byte alignment

    /// Create a new spot light with default values
    pub fn init() SpotLight {
        return SpotLight{
            .color = [3]f32{ 1.0, 1.0, 1.0 },
            .intensity = 800.0, // Default: 800 lumens
            .range = 20.0,
            .inner_angle = 0.52, // ~30 degrees
            .outer_angle = 0.79, // ~45 degrees
            .direction = [3]f32{ 0.0, -1.0, 0.0 }, // Pointing down
            .shadows_enabled = 0,
            ._padding = [3]f32{ 0.0, 0.0, 0.0 },
        };
    }

    /// Create a new spot light with specified parameters
    pub fn new(color: [3]f32, intensity: f32, direction: [3]f32, inner: f32, outer: f32) SpotLight {
        var light = SpotLight{
            .color = color,
            .intensity = intensity,
            .range = 20.0,
            .inner_angle = inner,
            .outer_angle = outer,
            .direction = direction,
            .shadows_enabled = 0,
            ._padding = [3]f32{ 0.0, 0.0, 0.0 },
        };
        light.normalizeDirection();
        return light;
    }

    /// Normalize the light direction vector
    pub fn normalizeDirection(self: *SpotLight) void {
        const x = self.direction[0];
        const y = self.direction[1];
        const z = self.direction[2];

        const len = math.sqrt(x * x + y * y + z * z);

        if (len > 0.0001) {
            self.direction[0] = x / len;
            self.direction[1] = y / len;
            self.direction[2] = z / len;
        } else {
            // Fallback to default direction if zero vector
            self.direction[0] = 0.0;
            self.direction[1] = -1.0;
            self.direction[2] = 0.0;
        }
    }

    /// Calculate distance-based attenuation
    pub fn attenuation(self: *const SpotLight, distance: f32) f32 {
        if (distance >= self.range) return 0.0;
        if (distance <= 0.0) return 1.0;

        // Inverse square law with distance offset
        const distance_sq = distance * distance;
        const atten = 1.0 / (distance_sq + 1.0);

        // Smooth falloff at range boundary
        const falloff = 1.0 - (distance / self.range);
        const smooth_falloff = falloff * falloff;

        return atten * smooth_falloff;
    }

    /// Calculate spotlight cone factor
    /// light_dir should be the direction from light to surface (normalized)
    pub fn spotFactor(self: *const SpotLight, light_dir: [3]f32) f32 {
        // Calculate dot product between light direction and light-to-surface direction
        const dot = light_dir[0] * self.direction[0] +
            light_dir[1] * self.direction[1] +
            light_dir[2] * self.direction[2];

        const cos_outer = math.cos(self.outer_angle);
        const cos_inner = math.cos(self.inner_angle);

        // Outside outer cone
        if (dot < cos_outer) return 0.0;

        // Inside inner cone
        if (dot > cos_inner) return 1.0;

        // In transition zone - smooth interpolation
        const t = (dot - cos_outer) / (cos_inner - cos_outer);
        return t * t; // Quadratic smoothstep
    }

    /// Set light color (RGB values in range [0.0, 1.0])
    pub fn setColor(self: *SpotLight, r: f32, g: f32, b: f32) void {
        self.color[0] = r;
        self.color[1] = g;
        self.color[2] = b;
    }

    /// Set light intensity in lumens
    pub fn setIntensity(self: *SpotLight, lumens: f32) void {
        self.intensity = @max(0.0, lumens);
    }

    /// Set light range
    pub fn setRange(self: *SpotLight, new_range: f32) void {
        self.range = @max(0.0, new_range);
    }

    /// Set inner cone angle in radians
    pub fn setInnerAngle(self: *SpotLight, angle: f32) void {
        self.inner_angle = math.clamp(angle, 0.0, math.pi);
        // Ensure inner <= outer
        if (self.inner_angle > self.outer_angle) {
            self.outer_angle = self.inner_angle;
        }
    }

    /// Set outer cone angle in radians
    pub fn setOuterAngle(self: *SpotLight, angle: f32) void {
        self.outer_angle = math.clamp(angle, 0.0, math.pi);
        // Ensure inner <= outer
        if (self.inner_angle > self.outer_angle) {
            self.inner_angle = self.outer_angle;
        }
    }

    /// Set spotlight direction (will be normalized)
    pub fn setDirection(self: *SpotLight, x: f32, y: f32, z: f32) void {
        self.direction[0] = x;
        self.direction[1] = y;
        self.direction[2] = z;
        self.normalizeDirection();
    }

    /// Enable shadow casting
    pub fn enableShadows(self: *SpotLight) void {
        self.shadows_enabled = 1;
    }

    /// Disable shadow casting
    pub fn disableShadows(self: *SpotLight) void {
        self.shadows_enabled = 0;
    }

    /// Check if shadows are enabled
    pub fn hasShadows(self: *const SpotLight) bool {
        return self.shadows_enabled != 0;
    }
};

// C FFI exports
export fn spot_light_init() SpotLight {
    return SpotLight.init();
}

export fn spot_light_new(color: *const [3]f32, intensity: f32, direction: *const [3]f32, inner: f32, outer: f32) SpotLight {
    return SpotLight.new(color.*, intensity, direction.*, inner, outer);
}

export fn spot_light_normalize_direction(light: *SpotLight) void {
    light.normalizeDirection();
}

export fn spot_light_attenuation(light: *const SpotLight, distance: f32) f32 {
    return light.attenuation(distance);
}

export fn spot_light_spot_factor(light: *const SpotLight, light_dir: *const [3]f32) f32 {
    return light.spotFactor(light_dir.*);
}

export fn spot_light_set_color(light: *SpotLight, r: f32, g: f32, b: f32) void {
    light.setColor(r, g, b);
}

export fn spot_light_set_intensity(light: *SpotLight, lumens: f32) void {
    light.setIntensity(lumens);
}

export fn spot_light_set_range(light: *SpotLight, range: f32) void {
    light.setRange(range);
}

export fn spot_light_set_inner_angle(light: *SpotLight, angle: f32) void {
    light.setInnerAngle(angle);
}

export fn spot_light_set_outer_angle(light: *SpotLight, angle: f32) void {
    light.setOuterAngle(angle);
}

export fn spot_light_set_direction(light: *SpotLight, x: f32, y: f32, z: f32) void {
    light.setDirection(x, y, z);
}

export fn spot_light_enable_shadows(light: *SpotLight) void {
    light.enableShadows();
}

export fn spot_light_disable_shadows(light: *SpotLight) void {
    light.disableShadows();
}

export fn spot_light_has_shadows(light: *const SpotLight) bool {
    return light.hasShadows();
}
