//! WebGPU Context Management
//! Handles WebGPU instance, adapter, device, queue, and surface

const std = @import("std");

/// WebGPU Context with all core handles
pub const WgpuContext = extern struct {
    instance: ?*anyopaque = null,
    adapter: ?*anyopaque = null,
    device: ?*anyopaque = null,
    queue: ?*anyopaque = null,
    surface: ?*anyopaque = null,
    canvas_id: [128]u8 = [_]u8{0} ** 128,
    canvas_id_len: u32 = 0,
    is_initialized: bool = false,
};

/// Create a new uninitialized WebGPU context
export fn wgpu_context_create() WgpuContext {
    return WgpuContext{};
}

/// Initialize WebGPU context (to be called from JavaScript)
export fn wgpu_context_init(ctx: *WgpuContext) void {
    ctx.is_initialized = false;
    ctx.instance = null;
    ctx.adapter = null;
    ctx.device = null;
    ctx.queue = null;
    ctx.surface = null;
    ctx.canvas_id_len = 0;
}

/// Set canvas ID for surface creation
export fn wgpu_context_set_canvas(ctx: *WgpuContext, canvas_id: [*]const u8, len: u32) void {
    const copy_len = @min(len, 127);
    @memcpy(ctx.canvas_id[0..copy_len], canvas_id[0..copy_len]);
    ctx.canvas_id[copy_len] = 0;
    ctx.canvas_id_len = copy_len;
}

/// Set instance handle (from JavaScript)
export fn wgpu_context_set_instance(ctx: *WgpuContext, instance: ?*anyopaque) void {
    ctx.instance = instance;
}

/// Set adapter handle (from JavaScript)
export fn wgpu_context_set_adapter(ctx: *WgpuContext, adapter: ?*anyopaque) void {
    ctx.adapter = adapter;
}

/// Set device handle (from JavaScript)
export fn wgpu_context_set_device(ctx: *WgpuContext, device: ?*anyopaque) void {
    ctx.device = device;
}

/// Set queue handle (from JavaScript)
export fn wgpu_context_set_queue(ctx: *WgpuContext, queue: ?*anyopaque) void {
    ctx.queue = queue;
}

/// Set surface handle (from JavaScript)
export fn wgpu_context_set_surface(ctx: *WgpuContext, surface: ?*anyopaque) void {
    ctx.surface = surface;
}

/// Mark context as initialized
export fn wgpu_context_mark_initialized(ctx: *WgpuContext) void {
    ctx.is_initialized = true;
}

/// Check if context is initialized
export fn wgpu_context_is_initialized(ctx: *const WgpuContext) bool {
    return ctx.is_initialized;
}

/// Check if device is available
export fn wgpu_context_has_device(ctx: *const WgpuContext) bool {
    return ctx.device != null;
}

/// Check if surface is available
export fn wgpu_context_has_surface(ctx: *const WgpuContext) bool {
    return ctx.surface != null;
}

/// Get canvas ID
export fn wgpu_context_get_canvas_id(ctx: *const WgpuContext, out_buffer: [*]u8, buffer_size: u32) u32 {
    const copy_len = @min(ctx.canvas_id_len, buffer_size - 1);
    @memcpy(out_buffer[0..copy_len], ctx.canvas_id[0..copy_len]);
    out_buffer[copy_len] = 0;
    return copy_len;
}

/// Deinitialize context
export fn wgpu_context_deinit(ctx: *WgpuContext) void {
    ctx.is_initialized = false;
    ctx.instance = null;
    ctx.adapter = null;
    ctx.device = null;
    ctx.queue = null;
    ctx.surface = null;
    ctx.canvas_id_len = 0;
}
