//! Shadow System - 阴影系统占位符实现

const std = @import("std");

pub const CascadeShadowConfig = extern struct {
    minimum_distance: f32,
    maximum_distance: f32,
    bounds: [2]f32,
    overlap_proportion: f32,
};

export fn cascade_shadow_config_init() CascadeShadowConfig {
    return CascadeShadowConfig{
        .minimum_distance = 0.1,
        .maximum_distance = 1000.0,
        .bounds = [_]f32{ 0.0, 1.0 },
        .overlap_proportion = 0.3,
    };
}

export fn cascade_shadow_config_set_distances(config: *CascadeShadowConfig, min: f32, max: f32) void {
    config.minimum_distance = min;
    config.maximum_distance = max;
}

export fn cascade_shadow_config_set_bounds(config: *CascadeShadowConfig, near: f32, far: f32) void {
    config.bounds[0] = near;
    config.bounds[1] = far;
}
