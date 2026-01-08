//! Lighting Utilities for autozig-light
//! Provides common lighting calculation functions (BRDF, attenuation, etc.)

const std = @import("std");
const math = std.math;

/// Calculate distance attenuation using inverse square law with smooth falloff
pub fn calculateAttenuation(distance: f32, range: f32) f32 {
    if (distance >= range) return 0.0;
    if (distance <= 0.0) return 1.0;

    // Inverse square law with distance offset to prevent singularity
    const distance_sq = distance * distance;
    const atten = 1.0 / (distance_sq + 1.0);

    // Smooth falloff at range boundary
    const falloff = 1.0 - (distance / range);
    const smooth_falloff = falloff * falloff;

    return atten * smooth_falloff;
}

/// Calculate spotlight cone factor with smooth transition
pub fn calculateSpotFactor(light_dir: [3]f32, spot_dir: [3]f32, inner_angle: f32, outer_angle: f32) f32 {
    // Calculate dot product (cosine of angle between vectors)
    const dot = light_dir[0] * spot_dir[0] +
        light_dir[1] * spot_dir[1] +
        light_dir[2] * spot_dir[2];

    const cos_outer = math.cos(outer_angle);
    const cos_inner = math.cos(inner_angle);

    // Outside outer cone
    if (dot < cos_outer) return 0.0;

    // Inside inner cone
    if (dot > cos_inner) return 1.0;

    // In transition zone - smooth interpolation
    const t = (dot - cos_outer) / (cos_inner - cos_outer);
    return t * t; // Quadratic smoothstep
}

/// Lambertian diffuse reflection (N · L)
pub fn lambertian(normal: [3]f32, light_dir: [3]f32) f32 {
    const dot = normal[0] * light_dir[0] +
        normal[1] * light_dir[1] +
        normal[2] * light_dir[2];
    return @max(0.0, dot);
}

/// Blinn-Phong specular reflection
pub fn blinnPhong(normal: [3]f32, view_dir: [3]f32, light_dir: [3]f32, shininess: f32) f32 {
    // Calculate half vector
    const hx = view_dir[0] + light_dir[0];
    const hy = view_dir[1] + light_dir[1];
    const hz = view_dir[2] + light_dir[2];

    // Normalize half vector
    const h_len = math.sqrt(hx * hx + hy * hy + hz * hz);
    if (h_len < 0.0001) return 0.0;

    const half_x = hx / h_len;
    const half_y = hy / h_len;
    const half_z = hz / h_len;

    // N · H
    const n_dot_h = normal[0] * half_x +
        normal[1] * half_y +
        normal[2] * half_z;

    if (n_dot_h <= 0.0) return 0.0;

    // (N · H)^shininess
    return math.pow(f32, n_dot_h, shininess);
}

/// Simplified Cook-Torrance BRDF
/// This is a simplified version suitable for real-time rendering
pub fn cookTorrance(normal: [3]f32, view_dir: [3]f32, light_dir: [3]f32, roughness: f32, metallic: f32) f32 {
    // Calculate half vector
    const hx = view_dir[0] + light_dir[0];
    const hy = view_dir[1] + light_dir[1];
    const hz = view_dir[2] + light_dir[2];

    const h_len = math.sqrt(hx * hx + hy * hy + hz * hz);
    if (h_len < 0.0001) return 0.0;

    const half_x = hx / h_len;
    const half_y = hy / h_len;
    const half_z = hz / h_len;

    // N · H
    const n_dot_h = @max(0.0, normal[0] * half_x + normal[1] * half_y + normal[2] * half_z);

    // N · V
    const n_dot_v = @max(0.0, normal[0] * view_dir[0] + normal[1] * view_dir[1] + normal[2] * view_dir[2]);

    // N · L
    const n_dot_l = @max(0.0, normal[0] * light_dir[0] + normal[1] * light_dir[1] + normal[2] * light_dir[2]);

    if (n_dot_v < 0.0001 or n_dot_l < 0.0001) return 0.0;

    // Simplified GGX/Trowbridge-Reitz normal distribution
    const alpha = roughness * roughness;
    const alpha_sq = alpha * alpha;
    const n_dot_h_sq = n_dot_h * n_dot_h;
    const denom = n_dot_h_sq * (alpha_sq - 1.0) + 1.0;
    const D = alpha_sq / (math.pi * denom * denom);

    // Simplified geometry function (Smith)
    const k = alpha / 2.0;
    const G1_v = n_dot_v / (n_dot_v * (1.0 - k) + k);
    const G1_l = n_dot_l / (n_dot_l * (1.0 - k) + k);
    const G = G1_v * G1_l;

    // Simplified Fresnel (Schlick approximation)
    const v_dot_h = @max(0.0, view_dir[0] * half_x + view_dir[1] * half_y + view_dir[2] * half_z);
    const F0 = 0.04 + (1.0 - 0.04) * metallic; // Base reflectivity
    const one_minus_vh = 1.0 - v_dot_h;
    const one_minus_vh_5 = one_minus_vh * one_minus_vh * one_minus_vh * one_minus_vh * one_minus_vh;
    const F = F0 + (1.0 - F0) * one_minus_vh_5;

    // Cook-Torrance BRDF = (D * G * F) / (4 * N·V * N·L)
    const specular = (D * G * F) / (4.0 * n_dot_v * n_dot_l + 0.0001);

    return @min(specular, 10.0); // Clamp to prevent overflow
}

/// Vector dot product
fn dot3(a: [3]f32, b: [3]f32) f32 {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

/// Normalize a 3D vector
fn normalize3(v: [3]f32) [3]f32 {
    const len = math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    if (len < 0.0001) return [3]f32{ 0.0, 0.0, 1.0 };
    return [3]f32{ v[0] / len, v[1] / len, v[2] / len };
}

// C FFI exports
export fn lighting_calculate_attenuation(distance: f32, range: f32) f32 {
    return calculateAttenuation(distance, range);
}

export fn lighting_calculate_spot_factor(light_dir: *const [3]f32, spot_dir: *const [3]f32, inner_angle: f32, outer_angle: f32) f32 {
    return calculateSpotFactor(light_dir.*, spot_dir.*, inner_angle, outer_angle);
}

export fn lighting_lambertian(normal: *const [3]f32, light_dir: *const [3]f32) f32 {
    return lambertian(normal.*, light_dir.*);
}

export fn lighting_blinn_phong(normal: *const [3]f32, view_dir: *const [3]f32, light_dir: *const [3]f32, shininess: f32) f32 {
    return blinnPhong(normal.*, view_dir.*, light_dir.*, shininess);
}

export fn lighting_cook_torrance(normal: *const [3]f32, view_dir: *const [3]f32, light_dir: *const [3]f32, roughness: f32, metallic: f32) f32 {
    return cookTorrance(normal.*, view_dir.*, light_dir.*, roughness, metallic);
}
