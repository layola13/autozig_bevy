//! GPU Light Data structures for autozig-light
//! Provides GPU-compatible light data structures with proper alignment

const std = @import("std");
const light_scene = @import("light_scene.zig");
const LightScene = light_scene.LightScene;

/// GPU-compatible point light (16-byte aligned)
pub const GpuPointLight = extern struct {
    position: [4]f32, // xyz + padding
    color: [4]f32, // rgb + intensity
    range_radius: [2]f32, // range, radius
    shadows: u32, // shadows_enabled
    _padding: u32, // Padding to 48 bytes (3 * 16)

    /// Create from point light and position
    pub fn fromLight(light: anytype, position: [3]f32) GpuPointLight {
        return GpuPointLight{
            .position = [4]f32{ position[0], position[1], position[2], 0.0 },
            .color = [4]f32{ light.color[0], light.color[1], light.color[2], light.intensity },
            .range_radius = [2]f32{ light.range, light.radius },
            .shadows = light.shadows_enabled,
            ._padding = 0,
        };
    }
};

/// GPU-compatible directional light (16-byte aligned)
pub const GpuDirectionalLight = extern struct {
    direction: [4]f32, // xyz + padding
    color: [4]f32, // rgb + illuminance

    /// Create from directional light
    pub fn fromLight(light: anytype) GpuDirectionalLight {
        return GpuDirectionalLight{
            .direction = [4]f32{ light.direction[0], light.direction[1], light.direction[2], 0.0 },
            .color = [4]f32{ light.color[0], light.color[1], light.color[2], light.illuminance },
        };
    }
};

/// GPU-compatible spot light (16-byte aligned)
pub const GpuSpotLight = extern struct {
    position: [4]f32, // xyz + padding
    direction: [4]f32, // xyz + padding
    color: [4]f32, // rgb + intensity
    range_angles: [4]f32, // range, inner_angle, outer_angle, shadows
    _padding: [4]f32, // Padding for alignment

    /// Create from spot light and position
    pub fn fromLight(light: anytype, position: [3]f32) GpuSpotLight {
        return GpuSpotLight{
            .position = [4]f32{ position[0], position[1], position[2], 0.0 },
            .direction = [4]f32{ light.direction[0], light.direction[1], light.direction[2], 0.0 },
            .color = [4]f32{ light.color[0], light.color[1], light.color[2], light.intensity },
            .range_angles = [4]f32{ light.range, light.inner_angle, light.outer_angle, @floatFromInt(light.shadows_enabled) },
            ._padding = [4]f32{ 0.0, 0.0, 0.0, 0.0 },
        };
    }
};

/// GPU light buffer containing all scene lights (16-byte aligned)
pub const GpuLightBuffer = extern struct {
    ambient_color: [4]f32, // rgb + brightness
    point_light_count: u32,
    directional_light_count: u32,
    spot_light_count: u32,
    _padding1: u32, // Padding for alignment
    point_lights: [32]GpuPointLight, // 32 point lights
    directional_lights: [4]GpuDirectionalLight, // 4 directional lights
    spot_lights: [16]GpuSpotLight, // 16 spot lights

    /// Create from light scene
    /// Note: This function doesn't handle positions - caller must set them separately
    pub fn fromScene(scene: *const LightScene) GpuLightBuffer {
        var buffer: GpuLightBuffer = undefined;

        // Set ambient light
        buffer.ambient_color = [4]f32{
            scene.ambient.color[0],
            scene.ambient.color[1],
            scene.ambient.color[2],
            scene.ambient.brightness,
        };

        // Set light counts
        buffer.point_light_count = scene.point_light_count;
        buffer.directional_light_count = scene.directional_light_count;
        buffer.spot_light_count = scene.spot_light_count;
        buffer._padding1 = 0;

        // Initialize all lights to zero
        var i: usize = 0;
        while (i < 32) : (i += 1) {
            buffer.point_lights[i] = std.mem.zeroes(GpuPointLight);
        }
        i = 0;
        while (i < 4) : (i += 1) {
            buffer.directional_lights[i] = std.mem.zeroes(GpuDirectionalLight);
        }
        i = 0;
        while (i < 16) : (i += 1) {
            buffer.spot_lights[i] = std.mem.zeroes(GpuSpotLight);
        }

        // Copy directional lights (these don't need position)
        i = 0;
        while (i < scene.directional_light_count) : (i += 1) {
            buffer.directional_lights[i] = GpuDirectionalLight.fromLight(scene.directional_lights[i]);
        }

        return buffer;
    }

    /// Set a point light with position
    pub fn setPointLight(self: *GpuLightBuffer, index: u32, light: anytype, position: [3]f32) void {
        if (index >= 32) return;
        self.point_lights[index] = GpuPointLight.fromLight(light, position);
    }

    /// Set a spot light with position
    pub fn setSpotLight(self: *GpuLightBuffer, index: u32, light: anytype, position: [3]f32) void {
        if (index >= 16) return;
        self.spot_lights[index] = GpuSpotLight.fromLight(light, position);
    }

    /// Get buffer size in bytes
    pub fn getSize() usize {
        return @sizeOf(GpuLightBuffer);
    }

    /// Check if buffer is properly aligned for GPU usage
    pub fn checkAlignment() bool {
        // Check 16-byte alignment for all structures
        const ambient_aligned = @alignOf([4]f32) >= 16;
        const point_aligned = @alignOf(GpuPointLight) >= 16;
        const directional_aligned = @alignOf(GpuDirectionalLight) >= 16;
        const spot_aligned = @alignOf(GpuSpotLight) >= 16;

        return ambient_aligned and point_aligned and directional_aligned and spot_aligned;
    }
};

// C FFI exports
// Note: Generic functions cannot be exported, these are internal only
// Rust side will call gpu_light_buffer_from_scene instead

export fn gpu_light_buffer_from_scene(scene: *const LightScene) GpuLightBuffer {
    return GpuLightBuffer.fromScene(scene);
}

// Note: Generic functions cannot be exported
// These are for internal use only within Zig

export fn gpu_light_buffer_get_size() usize {
    return GpuLightBuffer.getSize();
}

export fn gpu_light_buffer_check_alignment() bool {
    return GpuLightBuffer.checkAlignment();
}
