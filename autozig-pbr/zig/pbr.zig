//! AutoZig-PBR 主模块
//!
//! 实现 PBR (Physically Based Rendering) 材质系统
//! 专注于 WebGPU/WASM 平台的高性能渲染

const std = @import("std");
const pbr_material = @import("pbr_material.zig");
const pbr_lighting = @import("pbr_lighting.zig");
const pbr_texture = @import("pbr_texture.zig");

// 导出子模块
pub const Material = pbr_material.Material;
pub const MaterialHandle = pbr_material.MaterialHandle;
pub const Lighting = pbr_lighting;
pub const Texture = pbr_texture;
pub const LightData = pbr_lighting.LightData;

// ============================================================================
// FFI 导出函数 - PBR 批量光照计算
// ============================================================================

/// SIMD 批量计算 PBR 光照
export fn pbr_lighting_calculate_batch_simd(
    materials: [*]const MaterialHandle,
    positions: [*]const f32,
    normals: [*]const f32,
    view_dirs: [*]const f32,
    lights: [*]const LightData,
    num_vertices: u32,
    num_lights: u32,
    out_colors: [*]f32,
) void {
    pbr_lighting.calculateBatchSimd(
        materials,
        positions,
        normals,
        view_dirs,
        lights,
        num_vertices,
        num_lights,
        out_colors,
    );
}

// ============================================================================
// 测试
// ============================================================================

test "PBR module imports" {
    const testing = std.testing;

    // 验证所有子模块都正确导入
    _ = Material;
    _ = MaterialHandle;
    _ = Lighting;
    _ = Texture;

    try testing.expect(true);
}
