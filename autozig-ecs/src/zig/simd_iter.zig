/// SIMD-optimized iteration kernels for ECS hot loops
/// Uses native SIMD vectors for maximum performance
const std = @import("std");

/// 8-wide float vector for AVX-256 compatible SIMD
const Vec8 = @Vector(8, f32);

/// 4-wide float vector for SSE compatible SIMD
const Vec4 = @Vector(4, f32);

/// Process movement for entities using 8-wide SIMD (AVX)
/// Each entity has Position{x,y}, total 2 floats per position
/// We process 4 entities per SIMD operation (8 floats = 4 entities × 2 position components)
pub export fn movement_update_simd8(
    positions: [*]f32,
    velocities: [*]const f32,
    count: usize,
    dt: f32,
) void {
    const dt_vec: Vec8 = @splat(dt);
    const floats_per_entity: usize = 2; // x, y
    const total_floats = count * floats_per_entity;
    var i: usize = 0;

    // Main SIMD loop (8 floats at a time = 4 entities)
    while (i + 8 <= total_floats) : (i += 8) {
        const pos_slice = positions[i..][0..8];
        const vel_slice = velocities[i..][0..8];

        const pos: Vec8 = pos_slice.*;
        const vel: Vec8 = vel_slice.*;
        const new_pos = pos + vel * dt_vec;

        pos_slice.* = new_pos;
    }

    // 4-wide remainder (SSE)
    while (i + 4 <= total_floats) : (i += 4) {
        const pos_slice = positions[i..][0..4];
        const vel_slice = velocities[i..][0..4];

        const pos: Vec4 = pos_slice.*;
        const vel: Vec4 = vel_slice.*;
        const dt_vec4: Vec4 = @splat(dt);
        const new_pos = pos + vel * dt_vec4;

        pos_slice.* = new_pos;
    }

    // Scalar remainder
    while (i < total_floats) : (i += 1) {
        positions[i] += velocities[i] * dt;
    }
}

/// Alias for compatibility (prefetch version just uses same code for now)
pub export fn movement_update_simd8_prefetch(
    positions: [*]f32,
    velocities: [*]const f32,
    count: usize,
    dt: f32,
) void {
    movement_update_simd8(positions, velocities, count, dt);
}
