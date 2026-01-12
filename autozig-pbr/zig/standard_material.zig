//! Standard Material - PBR材质系统核心实现

const std = @import("std");

// AlphaMode枚举对应Rust定义
pub const AlphaMode = enum(c_int) {
    Opaque = 0,
    Mask = 1,
    Blend = 2,
    Premultiplied = 3,
    Add = 4,
    Multiply = 5,
};

pub const ParallaxMappingMethod = enum(c_int) {
    Parallax = 0,
    Relief = 1,
    ReliefRaymarching = 2,
};

pub const OpaqueRendererMethod = enum(c_int) {
    Forward = 0,
    Deferred = 1,
    Auto = 2,
};

pub const FaceCullMode = enum(c_int) {
    None = 0,
    Front = 1,
    Back = 2,
};

// StandardMaterial结构体（必须与Rust #[repr(C)]布局完全匹配）
pub const StandardMaterial = extern struct {
    base_color: [4]f32,
    emissive: [4]f32,
    perceptual_roughness: f32,
    metallic: f32,
    reflectance: f32,
    diffuse_transmission: f32,
    specular_transmission: f32,
    thickness: f32,
    ior: f32,
    attenuation_distance: f32,
    attenuation_color: [3]f32,
    alpha_mode: AlphaMode,
    alpha_cutoff: f32,
    parallax_depth_scale: f32,
    parallax_mapping_method: ParallaxMappingMethod,
    max_parallax_layer_count: f32,
    lightmap_exposure: f32,
    opaque_render_method: OpaqueRendererMethod,
    deferred_lighting_pass_id: u8,
    double_sided: bool,
    cull_mode: FaceCullMode,
    unlit: bool,
    fog_enabled: bool,
    depth_bias: f32,
    flip_normal_map_y: bool,
    _padding: [3]u8,
};

export fn standard_material_init() StandardMaterial {
    return StandardMaterial{
        .base_color = [_]f32{ 1.0, 1.0, 1.0, 1.0 },
        .emissive = [_]f32{ 0.0, 0.0, 0.0, 0.0 },
        .perceptual_roughness = 0.5,
        .metallic = 0.0,
        .reflectance = 0.5,
        .diffuse_transmission = 0.0,
        .specular_transmission = 0.0,
        .thickness = 0.0,
        .ior = 1.5,
        .attenuation_distance = std.math.inf(f32),
        .attenuation_color = [_]f32{ 1.0, 1.0, 1.0 },
        .alpha_mode = .Opaque,
        .alpha_cutoff = 0.5,
        .parallax_depth_scale = 0.1,
        .parallax_mapping_method = .Parallax,
        .max_parallax_layer_count = 16.0,
        .lightmap_exposure = 1.0,
        .opaque_render_method = .Forward,
        .deferred_lighting_pass_id = 1,
        .double_sided = false,
        .cull_mode = .Back,
        .unlit = false,
        .fog_enabled = true,
        .depth_bias = 0.0,
        .flip_normal_map_y = false,
        ._padding = [_]u8{0} ** 3,
    };
}

export fn standard_material_new(base_color: *const [4]f32) StandardMaterial {
    var mat = standard_material_init();
    mat.base_color = base_color.*;
    return mat;
}

export fn standard_material_set_base_color(mat: *StandardMaterial, color: *const [4]f32) void {
    mat.base_color = color.*;
}

export fn standard_material_set_metallic_roughness(mat: *StandardMaterial, metallic: f32, roughness: f32) void {
    mat.metallic = metallic;
    mat.perceptual_roughness = roughness;
}

export fn standard_material_set_emissive(mat: *StandardMaterial, emissive: *const [4]f32) void {
    mat.emissive = emissive.*;
}

export fn standard_material_set_alpha_mode(mat: *StandardMaterial, mode: AlphaMode) void {
    mat.alpha_mode = mode;
}

export fn standard_material_set_double_sided(mat: *StandardMaterial, enabled: bool) void {
    mat.double_sided = enabled;
}

export fn standard_material_set_unlit(mat: *StandardMaterial, enabled: bool) void {
    mat.unlit = enabled;
}
