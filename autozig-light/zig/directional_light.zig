//! Directional Light implementation for autozig-light
//! Provides directional light source (like sunlight)

const std = @import("std");
const math = std.math;

/// Directional light source with parallel rays (like the sun)
pub const DirectionalLight = extern struct {
    color: [3]f32, // RGB color
    illuminance: f32, // Illuminance in lux
    direction: [3]f32, // Light direction (normalized)
    shadows_enabled: u32, // 0 = disabled, 1 = enabled
    shadow_depth: f32, // Shadow depth range
    shadow_normal_bias: f32, // Normal bias to prevent shadow acne
    _padding: [2]f32, // Padding for 16-byte alignment

    /// Create a new directional light with default values
    pub fn init() DirectionalLight {
        return DirectionalLight{
            .color = [3]f32{ 1.0, 1.0, 1.0 },
            .illuminance = 100000.0, // Default: 100,000 lux (bright sunlight)
            .direction = [3]f32{ 0.0, -1.0, 0.0 }, // Pointing down
            .shadows_enabled = 0,
            .shadow_depth = 100.0,
            .shadow_normal_bias = 0.1,
            ._padding = [2]f32{ 0.0, 0.0 },
        };
    }

    /// Create a new directional light with specified parameters
    pub fn new(color: [3]f32, illuminance: f32, direction: [3]f32) DirectionalLight {
        var light = DirectionalLight{
            .color = color,
            .illuminance = illuminance,
            .direction = direction,
            .shadows_enabled = 0,
            .shadow_depth = 100.0,
            .shadow_normal_bias = 0.1,
            ._padding = [2]f32{ 0.0, 0.0 },
        };
        light.normalizeDirection();
        return light;
    }

    /// Normalize the light direction vector
    pub fn normalizeDirection(self: *DirectionalLight) void {
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

    /// Set light color (RGB values in range [0.0, 1.0])
    pub fn setColor(self: *DirectionalLight, r: f32, g: f32, b: f32) void {
        self.color[0] = r;
        self.color[1] = g;
        self.color[2] = b;
    }

    /// Set light illuminance in lux
    pub fn setIlluminance(self: *DirectionalLight, lux: f32) void {
        self.illuminance = @max(0.0, lux);
    }

    /// Set light direction (will be normalized)
    pub fn setDirection(self: *DirectionalLight, x: f32, y: f32, z: f32) void {
        self.direction[0] = x;
        self.direction[1] = y;
        self.direction[2] = z;
        self.normalizeDirection();
    }

    /// Enable shadow casting
    pub fn enableShadows(self: *DirectionalLight) void {
        self.shadows_enabled = 1;
    }

    /// Disable shadow casting
    pub fn disableShadows(self: *DirectionalLight) void {
        self.shadows_enabled = 0;
    }

    /// Check if shadows are enabled
    pub fn hasShadows(self: *const DirectionalLight) bool {
        return self.shadows_enabled != 0;
    }

    /// Set shadow depth range
    pub fn setShadowDepth(self: *DirectionalLight, depth: f32) void {
        self.shadow_depth = @max(0.0, depth);
    }

    /// Set shadow normal bias
    pub fn setShadowNormalBias(self: *DirectionalLight, bias: f32) void {
        self.shadow_normal_bias = bias;
    }
};

// C FFI exports
export fn directional_light_init() DirectionalLight {
    return DirectionalLight.init();
}

export fn directional_light_new(color: *const [3]f32, illuminance: f32, direction: *const [3]f32) DirectionalLight {
    return DirectionalLight.new(color.*, illuminance, direction.*);
}

export fn directional_light_normalize_direction(light: *DirectionalLight) void {
    light.normalizeDirection();
}

export fn directional_light_set_color(light: *DirectionalLight, r: f32, g: f32, b: f32) void {
    light.setColor(r, g, b);
}

export fn directional_light_set_illuminance(light: *DirectionalLight, lux: f32) void {
    light.setIlluminance(lux);
}

export fn directional_light_set_direction(light: *DirectionalLight, x: f32, y: f32, z: f32) void {
    light.setDirection(x, y, z);
}

export fn directional_light_enable_shadows(light: *DirectionalLight) void {
    light.enableShadows();
}

export fn directional_light_disable_shadows(light: *DirectionalLight) void {
    light.disableShadows();
}

export fn directional_light_has_shadows(light: *const DirectionalLight) bool {
    return light.hasShadows();
}

export fn directional_light_set_shadow_depth(light: *DirectionalLight, depth: f32) void {
    light.setShadowDepth(depth);
}

export fn directional_light_set_shadow_normal_bias(light: *DirectionalLight, bias: f32) void {
    light.setShadowNormalBias(bias);
}
