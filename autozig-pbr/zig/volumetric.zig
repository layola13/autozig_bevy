//! Volumetric Effects - 体积雾和体积光效果

const std = @import("std");

pub const VolumetricFog = extern struct {
    density: f32,
    color: [3]f32,
    scattering: f32,
    absorption: f32,
    phase_function_g: f32,
    _padding: [2]f32,
};

pub const VolumetricLight = extern struct {
    density: f32,
    steps: u32,
    max_distance: f32,
    _padding: u32,
};

export fn volumetric_fog_init() VolumetricFog {
    return VolumetricFog{
        .density = 0.1,
        .color = [_]f32{ 0.5, 0.5, 0.5 },
        .scattering = 0.5,
        .absorption = 0.1,
        .phase_function_g = 0.0,
        ._padding = [_]f32{0.0} ** 2,
    };
}

export fn volumetric_fog_set_density(fog: *VolumetricFog, density: f32) void {
    fog.density = density;
}

export fn volumetric_fog_set_color(fog: *VolumetricFog, color: *const [3]f32) void {
    fog.color = color.*;
}

export fn volumetric_light_init() VolumetricLight {
    return VolumetricLight{
        .density = 1.0,
        .steps = 16,
        .max_distance = 100.0,
        ._padding = 0,
    };
}

export fn volumetric_light_set_steps(light: *VolumetricLight, steps: u32) void {
    light.steps = steps;
}
