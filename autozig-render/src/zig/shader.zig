//! Shader Module Management
//! Handles WebGPU shader modules

const std = @import("std");

/// Shader stage
pub const ShaderStage = enum(u32) {
    Vertex = 1,
    Fragment = 2,
    Compute = 4,
};

/// Shader module
pub const ShaderModule = extern struct {
    handle: ?*anyopaque,
    entry_point: [64]u8,
    entry_point_len: u32,
    stage: u32, // ShaderStage
    is_valid: bool,
};

/// Shader source descriptor
pub const ShaderSource = extern struct {
    source: [4096]u8,
    source_len: u32,
    entry_point: [64]u8,
    entry_point_len: u32,
    stage: u32,
};

/// Create empty shader module
export fn shader_module_create() ShaderModule {
    return ShaderModule{
        .handle = null,
        .entry_point = [_]u8{0} ** 64,
        .entry_point_len = 0,
        .stage = @intFromEnum(ShaderStage.Vertex),
        .is_valid = false,
    };
}

/// Create shader source descriptor
export fn shader_source_create() ShaderSource {
    return ShaderSource{
        .source = [_]u8{0} ** 4096,
        .source_len = 0,
        .entry_point = [_]u8{0} ** 64,
        .entry_point_len = 0,
        .stage = @intFromEnum(ShaderStage.Vertex),
    };
}

/// Set shader source
export fn shader_source_set_source(desc: *ShaderSource, source: [*]const u8, len: u32) bool {
    if (len >= 4096) return false;
    @memcpy(desc.source[0..len], source[0..len]);
    desc.source[len] = 0;
    desc.source_len = len;
    return true;
}

/// Set shader entry point
export fn shader_source_set_entry_point(desc: *ShaderSource, entry: [*]const u8, len: u32) void {
    const copy_len = @min(len, 63);
    @memcpy(desc.entry_point[0..copy_len], entry[0..copy_len]);
    desc.entry_point[copy_len] = 0;
    desc.entry_point_len = copy_len;
}

/// Set shader stage
export fn shader_source_set_stage(desc: *ShaderSource, stage: u32) void {
    desc.stage = stage;
}

/// Set shader module handle (from JavaScript)
export fn shader_module_set_handle(module: *ShaderModule, handle: ?*anyopaque) void {
    module.handle = handle;
    module.is_valid = handle != null;
}

/// Set shader entry point
export fn shader_module_set_entry_point(module: *ShaderModule, entry: [*]const u8, len: u32) void {
    const copy_len = @min(len, 63);
    @memcpy(module.entry_point[0..copy_len], entry[0..copy_len]);
    module.entry_point[copy_len] = 0;
    module.entry_point_len = copy_len;
}

/// Set shader stage
export fn shader_module_set_stage(module: *ShaderModule, stage: u32) void {
    module.stage = stage;
}

/// Get entry point
export fn shader_module_get_entry_point(module: *const ShaderModule, out_buffer: [*]u8, buffer_size: u32) u32 {
    const copy_len = @min(module.entry_point_len, buffer_size - 1);
    @memcpy(out_buffer[0..copy_len], module.entry_point[0..copy_len]);
    out_buffer[copy_len] = 0;
    return copy_len;
}

/// Check if shader module is valid
export fn shader_module_is_valid(module: *const ShaderModule) bool {
    return module.is_valid and module.handle != null;
}

/// Get shader stage
export fn shader_module_get_stage(module: *const ShaderModule) u32 {
    return module.stage;
}

/// Destroy shader module
export fn shader_module_destroy(module: *ShaderModule) void {
    module.handle = null;
    module.is_valid = false;
    module.entry_point_len = 0;
}

/// Create vertex shader from WGSL
export fn shader_module_create_vertex_wgsl(entry: [*]const u8, entry_len: u32) ShaderModule {
    var module = shader_module_create();
    module.stage = @intFromEnum(ShaderStage.Vertex);
    shader_module_set_entry_point(&module, entry, entry_len);
    return module;
}

/// Create fragment shader from WGSL
export fn shader_module_create_fragment_wgsl(entry: [*]const u8, entry_len: u32) ShaderModule {
    var module = shader_module_create();
    module.stage = @intFromEnum(ShaderStage.Fragment);
    shader_module_set_entry_point(&module, entry, entry_len);
    return module;
}

/// Create compute shader from WGSL
export fn shader_module_create_compute_wgsl(entry: [*]const u8, entry_len: u32) ShaderModule {
    var module = shader_module_create();
    module.stage = @intFromEnum(ShaderStage.Compute);
    shader_module_set_entry_point(&module, entry, entry_len);
    return module;
}

/// Copy shader module
export fn shader_module_copy(dest: *ShaderModule, src: *const ShaderModule) void {
    dest.handle = src.handle;
    dest.entry_point = src.entry_point;
    dest.entry_point_len = src.entry_point_len;
    dest.stage = src.stage;
    dest.is_valid = src.is_valid;
}
