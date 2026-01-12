//! Lighting System - 光照系统占位符实现
//! 注意：完整光照实现在autozig-light模块中，这里仅提供占位符

const std = @import("std");

// 占位符类型定义，实际实现在lib.rs中使用autozig-light
pub const AmbientLight = extern struct {
    color: [3]f32,
    brightness: f32,
};

pub const DirectionalLight = extern struct {
    color: [3]f32,
    illuminance: f32,
    direction: [3]f32,
    shadows_enabled: u32,
    shadow_depth_bias: f32,
    shadow_normal_bias: f32,
    _padding: [2]f32,
};

pub const PointLight = extern struct {
    color: [3]f32,
    intensity: f32,
    range: f32,
    radius: f32,
    shadows_enabled: u32,
    shadow_depth_bias: f32,
    shadow_normal_bias: f32,
    _padding: [3]f32,
};

pub const SpotLight = extern struct {
    color: [3]f32,
    intensity: f32,
    range: f32,
    radius: f32,
    shadows_enabled: u32,
    shadow_depth_bias: f32,
    shadow_normal_bias: f32,
    inner_angle: f32,
    outer_angle: f32,
    direction: [3]f32,
    _padding: f32,
};

export fn ambient_light_init() AmbientLight {
    return AmbientLight{
        .color = [_]f32{ 1.0, 1.0, 1.0 },
        .brightness = 0.02,
    };
}

export fn directional_light_init() DirectionalLight {
    return DirectionalLight{
        .color = [_]f32{ 1.0, 1.0, 1.0 },
        .illuminance = 100000.0,
        .direction = [_]f32{ 0.0, -1.0, 0.0 },
        .shadows_enabled = 0,
        .shadow_depth_bias = 0.02,
        .shadow_normal_bias = 0.6,
        ._padding = [_]f32{0.0} ** 2,
    };
}

export fn point_light_init() PointLight {
    return PointLight{
        .color = [_]f32{ 1.0, 1.0, 1.0 },
        .intensity = 800.0,
        .range = 20.0,
        .radius = 0.0,
        .shadows_enabled = 0,
        .shadow_depth_bias = 0.02,
        .shadow_normal_bias = 0.6,
        ._padding = [_]f32{0.0} ** 3,
    };
}

export fn spot_light_init() SpotLight {
    return SpotLight{
        .color = [_]f32{ 1.0, 1.0, 1.0 },
        .intensity = 800.0,
        .range = 20.0,
        .radius = 0.0,
        .shadows_enabled = 0,
        .shadow_depth_bias = 0.02,
        .shadow_normal_bias = 0.6,
        .inner_angle = 0.0,
        .outer_angle = 0.785398, // π/4
        .direction = [_]f32{ 0.0, -1.0, 0.0 },
        ._padding = 0.0,
    };
}

export fn ambient_light_set_color(light: *AmbientLight, color: *const [3]f32) void {
    light.color = color.*;
}

export fn directional_light_set_direction(light: *DirectionalLight, direction: *const [3]f32) void {
    light.direction = direction.*;
}

export fn point_light_set_intensity(light: *PointLight, intensity: f32) void {
    light.intensity = intensity;
}

export fn spot_light_set_angles(light: *SpotLight, inner: f32, outer: f32) void {
    light.inner_angle = inner;
    light.outer_angle = outer;
}
