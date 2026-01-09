//! AutoZig Shader - Complete module export
//! This file aggregates and re-exports all shader-related modules

const std = @import("std");

// ============================================================================
// Module Imports
// ============================================================================

pub const shader_module = @import("shader_module.zig");
pub const uniform_layout = @import("uniform_layout.zig");
pub const builtin_shaders = @import("builtin_shaders.zig");

// ============================================================================
// Re-export Core Types from shader_module
// ============================================================================

// WebGPU Types
pub const WGPUDevice = shader_module.WGPUDevice;
pub const WGPUShaderModule = shader_module.WGPUShaderModule;
pub const WGPUChainedStruct = shader_module.WGPUChainedStruct;
pub const WGPUShaderModuleWGSLDescriptor = shader_module.WGPUShaderModuleWGSLDescriptor;
pub const WGPUShaderModuleDescriptor = shader_module.WGPUShaderModuleDescriptor;

// Shader Module Handle
pub const ShaderModuleHandle = shader_module.ShaderModuleHandle;

// Shader Module Functions
pub const shader_module_create = shader_module.shader_module_create;
pub const shader_module_destroy = shader_module.shader_module_destroy;
pub const shader_module_is_valid = shader_module.shader_module_is_valid;
pub const shader_module_get_compilation_info = shader_module.shader_module_get_compilation_info;
pub const shader_module_get_count = shader_module.shader_module_get_count;
pub const shader_module_clear_all = shader_module.shader_module_clear_all;

// ============================================================================
// Re-export Core Types from uniform_layout
// ============================================================================

// Layout Types
pub const UniformField = uniform_layout.UniformField;
pub const BindGroupLayoutEntry = uniform_layout.BindGroupLayoutEntry;
pub const WgslType = uniform_layout.WgslType;
pub const LayoutBuilder = uniform_layout.LayoutBuilder;

// Layout Functions
pub const uniform_layout_get_std140_alignment = uniform_layout.uniform_layout_get_std140_alignment;
pub const uniform_layout_get_std430_alignment = uniform_layout.uniform_layout_get_std430_alignment;
pub const uniform_layout_align_offset = uniform_layout.uniform_layout_align_offset;
pub const uniform_layout_calculate_size = uniform_layout.uniform_layout_calculate_size;
pub const uniform_layout_calculate_alignment = uniform_layout.uniform_layout_calculate_alignment;
pub const uniform_layout_create_bind_group_layout = uniform_layout.uniform_layout_create_bind_group_layout;
pub const uniform_layout_calculate_padding = uniform_layout.uniform_layout_calculate_padding;
pub const uniform_layout_is_aligned = uniform_layout.uniform_layout_is_aligned;
pub const uniform_layout_next_field_offset = uniform_layout.uniform_layout_next_field_offset;
pub const uniform_field_create = uniform_layout.uniform_field_create;

// Type Helper Functions
pub const getTypeSize = uniform_layout.getTypeSize;
pub const getTypeAlignment = uniform_layout.getTypeAlignment;

// ============================================================================
// Re-export Built-in Shader Functions
// ============================================================================

pub const builtin_shader_get_pbr_vertex = builtin_shaders.builtin_shader_get_pbr_vertex;
pub const builtin_shader_get_pbr_fragment = builtin_shaders.builtin_shader_get_pbr_fragment;
pub const builtin_shader_get_sprite_vertex = builtin_shaders.builtin_shader_get_sprite_vertex;
pub const builtin_shader_get_sprite_fragment = builtin_shaders.builtin_shader_get_sprite_fragment;
pub const builtin_shader_get_ui_vertex = builtin_shaders.builtin_shader_get_ui_vertex;
pub const builtin_shader_get_ui_fragment = builtin_shaders.builtin_shader_get_ui_fragment;
pub const builtin_shader_get_fullscreen_vertex = builtin_shaders.builtin_shader_get_fullscreen_vertex;

// ============================================================================
// Convenience Functions
// ============================================================================

/// Create a shader module from WGSL source (convenience wrapper)
pub fn createShaderModule(
    device_id: u64,
    code: []const u8,
    label: []const u8,
) ShaderModuleHandle {
    return shader_module_create(
        device_id,
        code.ptr,
        code.len,
        label.ptr,
        label.len,
    );
}

/// Get shader source as string slice (helper for builtin shaders)
pub fn getBuiltinShaderSource(
    getter_fn: fn (*?[*]const u8, *usize) void,
) []const u8 {
    var ptr: ?[*]const u8 = null;
    var len: usize = 0;
    getter_fn(&ptr, &len);

    if (ptr) |p| {
        return p[0..len];
    }
    return &[_]u8{};
}

/// Calculate aligned uniform buffer size with padding
pub fn calculateUniformBufferSize(fields: []const UniformField) u64 {
    return uniform_layout_calculate_size(fields.ptr, fields.len);
}

/// Create a uniform field with automatic alignment
pub fn createUniformField(offset: u32, wgsl_type: WgslType) UniformField {
    const size = getTypeSize(wgsl_type);
    const alignment = getTypeAlignment(wgsl_type);
    return uniform_field_create(offset, size, alignment);
}

// ============================================================================
// Module Metadata
// ============================================================================

pub const VERSION = "0.1.0";
pub const MODULE_NAME = "autozig-shader";
pub const DESCRIPTION = "Bevy-style shader management for WebGPU/WASM";

// ============================================================================
// Tests
// ============================================================================

test "shader_all_module_imports" {
    // Verify all modules are accessible
    const sm = shader_module;
    const ul = uniform_layout;
    const bs = builtin_shaders;

    _ = sm;
    _ = ul;
    _ = bs;
}

test "shader_all_type_exports" {
    // Test that key types are accessible
    const handle = ShaderModuleHandle{ .id = 123 };
    try std.testing.expect(handle.id == 123);

    const field = UniformField{
        .offset = 0,
        .size = 16,
        .alignment = 16,
        ._padding = 0,
    };
    try std.testing.expect(field.size == 16);
}

test "shader_all_function_exports" {
    // Test that key functions are accessible
    const alignment = uniform_layout_get_std140_alignment(16);
    try std.testing.expect(alignment == 16);

    const invalid_handle = ShaderModuleHandle{ .id = 0 };
    const valid = shader_module_is_valid(invalid_handle);
    try std.testing.expect(!valid);
}

test "shader_all_builtin_shader_exports" {
    // Test builtin shader functions are accessible
    var ptr: ?[*]const u8 = null;
    var len: usize = 0;

    builtin_shader_get_pbr_vertex(&ptr, &len);
    try std.testing.expect(len > 0);
    try std.testing.expect(ptr != null);
}

test "shader_all_convenience_functions" {
    // Test convenience functions
    const source = getBuiltinShaderSource(builtin_shader_get_sprite_vertex);
    try std.testing.expect(source.len > 0);

    const fields = [_]UniformField{
        .{ .offset = 0, .size = 64, .alignment = 16, ._padding = 0 },
        .{ .offset = 64, .size = 16, .alignment = 16, ._padding = 0 },
    };

    const size = calculateUniformBufferSize(&fields);
    try std.testing.expect(size >= 80);
    try std.testing.expect(size % 16 == 0);
}

test "shader_all_create_uniform_field" {
    const field = createUniformField(0, .vec4f);
    try std.testing.expect(field.size == 16);
    try std.testing.expect(field.alignment == 16);
}
