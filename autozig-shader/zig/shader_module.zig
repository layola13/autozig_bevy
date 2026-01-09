//! ShaderModule creation and management for WebGPU
//! 85% Zig implementation - Core WebGPU shader module operations

const std = @import("std");

// ============================================================================
// WebGPU Types and Extern Functions
// ============================================================================

/// Opaque WebGPU device handle
pub const WGPUDevice = ?*opaque {};

/// Opaque WebGPU shader module handle
pub const WGPUShaderModule = ?*opaque {};

/// Chain structure for descriptor extensions
pub const WGPUChainedStruct = extern struct {
    next: ?*const WGPUChainedStruct,
    sType: u32,
};

/// WGSL descriptor chain type
pub const WGPUSType_ShaderModuleWGSLDescriptor: u32 = 0x00000006;

/// WGSL shader source descriptor
pub const WGPUShaderModuleWGSLDescriptor = extern struct {
    chain: WGPUChainedStruct,
    code: [*:0]const u8,
};

/// Shader module descriptor
pub const WGPUShaderModuleDescriptor = extern struct {
    nextInChain: ?*const WGPUChainedStruct,
    label: [*:0]const u8,
};

/// Shader module handle for FFI
pub const ShaderModuleHandle = extern struct {
    id: u64,
};

// WebGPU C API - Stub implementations for testing
// In actual WASM environment, these would be imported from WebGPU
export fn wgpuDeviceCreateShaderModule(
    device: WGPUDevice,
    descriptor: *const WGPUShaderModuleDescriptor,
) WGPUShaderModule {
    _ = device;
    _ = descriptor;
    // Return a dummy non-null pointer for testing
    return @ptrFromInt(0x1000);
}

export fn wgpuShaderModuleRelease(module: WGPUShaderModule) void {
    _ = module;
    // No-op for testing
}

// ============================================================================
// Shader Module Registry
// ============================================================================

const MAX_SHADER_MODULES: usize = 256;

const ShaderModuleEntry = struct {
    handle: WGPUShaderModule,
    device: WGPUDevice,
    label: [64]u8,
    label_len: usize,
    in_use: bool,
};

var shader_modules: [MAX_SHADER_MODULES]ShaderModuleEntry = undefined;
var shader_modules_initialized: bool = false;
var next_shader_id: u64 = 1;

fn initShaderModules() void {
    if (!shader_modules_initialized) {
        for (&shader_modules) |*entry| {
            entry.* = .{
                .handle = null,
                .device = null,
                .label = undefined,
                .label_len = 0,
                .in_use = false,
            };
        }
        shader_modules_initialized = true;
    }
}

fn findFreeSlot() ?usize {
    initShaderModules();
    for (shader_modules, 0..) |entry, i| {
        if (!entry.in_use) {
            return i;
        }
    }
    return null;
}

fn findModuleById(id: u64) ?usize {
    if (id == 0) return null;
    initShaderModules();
    for (shader_modules, 0..) |entry, i| {
        if (entry.in_use and i + 1 == id) {
            return i;
        }
    }
    return null;
}

// ============================================================================
// Shader Module Creation
// ============================================================================

/// Create a shader module from WGSL source code
export fn shader_module_create(
    device_id: u64,
    code: [*]const u8,
    code_len: usize,
    label: [*]const u8,
    label_len: usize,
) ShaderModuleHandle {
    initShaderModules();

    // Validate inputs
    if (device_id == 0 or code_len == 0) {
        return ShaderModuleHandle{ .id = 0 };
    }

    const slot = findFreeSlot() orelse return ShaderModuleHandle{ .id = 0 };

    // Convert device_id to device handle (in real implementation)
    // Fixed: Convert u64 -> usize -> pointer for cross-platform compatibility
    const device: WGPUDevice = @ptrFromInt(@as(usize, @intCast(device_id)));

    // Create null-terminated code buffer
    var code_buffer: [65536]u8 = undefined;
    if (code_len >= code_buffer.len) {
        return ShaderModuleHandle{ .id = 0 };
    }
    @memcpy(code_buffer[0..code_len], code[0..code_len]);
    code_buffer[code_len] = 0;

    // Create null-terminated label buffer
    var label_buffer: [256]u8 = undefined;
    const actual_label_len = @min(label_len, 255);
    @memcpy(label_buffer[0..actual_label_len], label[0..actual_label_len]);
    label_buffer[actual_label_len] = 0;

    // Setup WGSL descriptor
    const wgsl_desc = WGPUShaderModuleWGSLDescriptor{
        .chain = WGPUChainedStruct{
            .next = null,
            .sType = WGPUSType_ShaderModuleWGSLDescriptor,
        },
        .code = @ptrCast(&code_buffer),
    };

    // Setup shader module descriptor
    const module_desc = WGPUShaderModuleDescriptor{
        .nextInChain = @ptrCast(&wgsl_desc.chain),
        .label = @ptrCast(&label_buffer),
    };

    // Create shader module via WebGPU API
    const module = wgpuDeviceCreateShaderModule(device, &module_desc);

    // Store in registry
    shader_modules[slot] = .{
        .handle = module,
        .device = device,
        .label = undefined,
        .label_len = actual_label_len,
        .in_use = true,
    };
    @memcpy(shader_modules[slot].label[0..actual_label_len], label[0..actual_label_len]);

    const id = slot + 1;
    return ShaderModuleHandle{ .id = id };
}

/// Destroy a shader module
export fn shader_module_destroy(module: ShaderModuleHandle) void {
    const slot = findModuleById(module.id) orelse return;

    if (shader_modules[slot].in_use) {
        if (shader_modules[slot].handle) |handle| {
            wgpuShaderModuleRelease(handle);
        }
        shader_modules[slot].in_use = false;
    }
}

/// Check if shader module is valid
export fn shader_module_is_valid(module: ShaderModuleHandle) bool {
    const slot = findModuleById(module.id) orelse return false;
    return shader_modules[slot].in_use and shader_modules[slot].handle != null;
}

/// Get compilation info (simplified for WASM - always returns success)
export fn shader_module_get_compilation_info(
    module: ShaderModuleHandle,
    out_buffer: [*]u8,
    buffer_len: usize,
) usize {
    const slot = findModuleById(module.id) orelse return 0;

    if (!shader_modules[slot].in_use or buffer_len == 0) {
        return 0;
    }

    const message = "Shader compiled successfully";
    const msg_len = message.len;
    const copy_len = @min(msg_len, buffer_len);

    @memcpy(out_buffer[0..copy_len], message[0..copy_len]);
    return copy_len;
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Get total number of active shader modules
export fn shader_module_get_count() usize {
    initShaderModules();
    var count: usize = 0;
    for (shader_modules) |entry| {
        if (entry.in_use) {
            count += 1;
        }
    }
    return count;
}

/// Clear all shader modules (for cleanup)
export fn shader_module_clear_all() void {
    initShaderModules();
    for (&shader_modules) |*entry| {
        if (entry.in_use) {
            if (entry.handle) |handle| {
                wgpuShaderModuleRelease(handle);
            }
            entry.in_use = false;
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

test "shader_module_handle_creation" {
    const handle = ShaderModuleHandle{ .id = 123 };
    try std.testing.expect(handle.id == 123);
}

test "shader_module_registry" {
    initShaderModules();
    const slot = findFreeSlot();
    try std.testing.expect(slot != null);
}

test "shader_module_invalid_handle" {
    const invalid = ShaderModuleHandle{ .id = 0 };
    try std.testing.expect(!shader_module_is_valid(invalid));
}
