//! Shadow Map configuration for autozig-light
//! Provides shadow mapping and cascaded shadow map setup

const std = @import("std");
const math = std.math;

/// Shadow map configuration
pub const ShadowMap = extern struct {
    resolution: u32, // Shadow map resolution (512, 1024, 2048, 4096)
    cascade_count: u32, // Number of cascades (1-4)
    near_plane: f32,
    far_plane: f32,
    bias: f32, // Shadow bias to prevent acne
    _padding: [3]u32, // Padding for 16-byte alignment

    /// Create a new shadow map with default settings
    pub fn init() ShadowMap {
        return ShadowMap{
            .resolution = 1024,
            .cascade_count = 1,
            .near_plane = 0.1,
            .far_plane = 100.0,
            .bias = 0.005,
            ._padding = [3]u32{ 0, 0, 0 },
        };
    }

    /// Create a shadow map with specified resolution
    pub fn new(resolution: u32) ShadowMap {
        return ShadowMap{
            .resolution = resolution,
            .cascade_count = 1,
            .near_plane = 0.1,
            .far_plane = 100.0,
            .bias = 0.005,
            ._padding = [3]u32{ 0, 0, 0 },
        };
    }

    /// Set cascade count (1-4)
    pub fn setCascades(self: *ShadowMap, count: u32) void {
        self.cascade_count = math.clamp(count, 1, 4);
    }

    /// Set near and far planes
    pub fn setPlanes(self: *ShadowMap, near: f32, far: f32) void {
        self.near_plane = @max(0.001, near);
        self.far_plane = @max(self.near_plane + 0.1, far);
    }

    /// Set shadow bias
    pub fn setBias(self: *ShadowMap, bias: f32) void {
        self.bias = bias;
    }

    /// Set shadow map resolution
    pub fn setResolution(self: *ShadowMap, resolution: u32) void {
        // Clamp to valid power-of-two resolutions
        if (resolution <= 512) {
            self.resolution = 512;
        } else if (resolution <= 1024) {
            self.resolution = 1024;
        } else if (resolution <= 2048) {
            self.resolution = 2048;
        } else {
            self.resolution = 4096;
        }
    }
};

/// Cascaded shadow map configuration
pub const CascadeShadowMap = extern struct {
    splits: [4]f32, // Cascade split distances
    split_count: u32,
    _padding: [3]u32, // Padding for 16-byte alignment

    /// Create a new cascade shadow map
    pub fn init() CascadeShadowMap {
        return CascadeShadowMap{
            .splits = [4]f32{ 0.0, 0.0, 0.0, 0.0 },
            .split_count = 0,
            ._padding = [3]u32{ 0, 0, 0 },
        };
    }

    /// Calculate cascade splits using logarithmic distribution
    pub fn calculateSplits(near: f32, far: f32, count: u32) CascadeShadowMap {
        var csm = CascadeShadowMap.init();
        const actual_count = math.clamp(count, 1, 4);
        csm.split_count = actual_count;

        if (actual_count == 1) {
            csm.splits[0] = far;
            return csm;
        }

        // Logarithmic split scheme (better for perspective)
        const range = far - near;
        const ratio = far / near;

        var i: u32 = 0;
        while (i < actual_count) : (i += 1) {
            const p = @as(f32, @floatFromInt(i + 1)) / @as(f32, @floatFromInt(actual_count));

            // Logarithmic split
            const log_split = near * math.pow(f32, ratio, p);

            // Linear split
            const linear_split = near + range * p;

            // Blend between logarithmic and linear (60% log, 40% linear)
            csm.splits[i] = log_split * 0.6 + linear_split * 0.4;
        }

        return csm;
    }

    /// Calculate cascade splits using practical split scheme (PSSM)
    pub fn calculatePracticalSplits(near: f32, far: f32, count: u32, lambda: f32) CascadeShadowMap {
        var csm = CascadeShadowMap.init();
        const actual_count = math.clamp(count, 1, 4);
        csm.split_count = actual_count;

        if (actual_count == 1) {
            csm.splits[0] = far;
            return csm;
        }

        const range = far - near;
        const ratio = far / near;

        var i: u32 = 0;
        while (i < actual_count) : (i += 1) {
            const p = @as(f32, @floatFromInt(i + 1)) / @as(f32, @floatFromInt(actual_count));

            // Logarithmic split
            const log_split = near * math.pow(f32, ratio, p);

            // Linear split
            const linear_split = near + range * p;

            // Blend using lambda parameter
            csm.splits[i] = log_split * lambda + linear_split * (1.0 - lambda);
        }

        return csm;
    }

    /// Get split distance for a specific cascade
    pub fn getSplit(self: *const CascadeShadowMap, index: u32) f32 {
        if (index >= self.split_count) return 0.0;
        return self.splits[index];
    }
};

// C FFI exports
export fn shadow_map_init() ShadowMap {
    return ShadowMap.init();
}

export fn shadow_map_new(resolution: u32) ShadowMap {
    return ShadowMap.new(resolution);
}

export fn shadow_map_set_cascades(shadow_map: *ShadowMap, count: u32) void {
    shadow_map.setCascades(count);
}

export fn shadow_map_set_planes(shadow_map: *ShadowMap, near: f32, far: f32) void {
    shadow_map.setPlanes(near, far);
}

export fn shadow_map_set_bias(shadow_map: *ShadowMap, bias: f32) void {
    shadow_map.setBias(bias);
}

export fn shadow_map_set_resolution(shadow_map: *ShadowMap, resolution: u32) void {
    shadow_map.setResolution(resolution);
}

export fn cascade_shadow_map_init() CascadeShadowMap {
    return CascadeShadowMap.init();
}

export fn cascade_shadow_map_calculate_splits(near: f32, far: f32, count: u32) CascadeShadowMap {
    return CascadeShadowMap.calculateSplits(near, far, count);
}

export fn cascade_shadow_map_calculate_practical_splits(near: f32, far: f32, count: u32, lambda: f32) CascadeShadowMap {
    return CascadeShadowMap.calculatePracticalSplits(near, far, count, lambda);
}

export fn cascade_shadow_map_get_split(csm: *const CascadeShadowMap, index: u32) f32 {
    return csm.getSplit(index);
}
