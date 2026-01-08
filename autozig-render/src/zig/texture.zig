//! Texture and Sampler Management
//! Handles WebGPU textures and samplers

const std = @import("std");

/// Texture format
pub const TextureFormat = enum(u32) {
    RGBA8Unorm = 0,
    RGBA8UnormSrgb = 1,
    BGRA8Unorm = 2,
    BGRA8UnormSrgb = 3,
    Depth24Plus = 4,
    Depth32Float = 5,
    RGBA16Float = 6,
    RGBA32Float = 7,
};

/// Texture usage flags
pub const TextureUsage = enum(u32) {
    CopySrc = 1,
    CopyDst = 2,
    TextureBinding = 4,
    StorageBinding = 8,
    RenderAttachment = 16,
};

/// Texture dimension
pub const TextureDimension = enum(u32) {
    D1 = 0,
    D2 = 1,
    D3 = 2,
};

/// Texture descriptor
pub const TextureDescriptor = extern struct {
    width: u32,
    height: u32,
    depth: u32,
    mip_level_count: u32,
    sample_count: u32,
    dimension: u32, // TextureDimension
    format: u32, // TextureFormat
    usage: u32, // TextureUsage flags
};

/// Texture handle
pub const Texture = extern struct {
    handle: ?*anyopaque,
    width: u32,
    height: u32,
    depth: u32,
    format: u32,
    mip_levels: u32,
    is_valid: bool,
};

/// Texture view
pub const TextureView = extern struct {
    handle: ?*anyopaque,
    texture: ?*anyopaque, // Reference to parent texture
    is_valid: bool,
};

/// Address mode
pub const AddressMode = enum(u32) {
    ClampToEdge = 0,
    Repeat = 1,
    MirrorRepeat = 2,
};

/// Filter mode
pub const FilterMode = enum(u32) {
    Nearest = 0,
    Linear = 1,
};

/// Sampler descriptor
pub const SamplerDescriptor = extern struct {
    address_mode_u: u32, // AddressMode
    address_mode_v: u32,
    address_mode_w: u32,
    mag_filter: u32, // FilterMode
    min_filter: u32,
    mipmap_filter: u32,
    lod_min_clamp: f32,
    lod_max_clamp: f32,
    compare_function: u32, // 0 = None
};

/// Sampler handle
pub const Sampler = extern struct {
    handle: ?*anyopaque,
    is_valid: bool,
};

/// Create default texture descriptor
export fn texture_descriptor_create() TextureDescriptor {
    return TextureDescriptor{
        .width = 1,
        .height = 1,
        .depth = 1,
        .mip_level_count = 1,
        .sample_count = 1,
        .dimension = @intFromEnum(TextureDimension.D2),
        .format = @intFromEnum(TextureFormat.RGBA8Unorm),
        .usage = @intFromEnum(TextureUsage.TextureBinding) | @intFromEnum(TextureUsage.CopyDst),
    };
}

/// Create 2D texture descriptor
export fn texture_descriptor_2d(width: u32, height: u32, format: u32) TextureDescriptor {
    var desc = texture_descriptor_create();
    desc.width = width;
    desc.height = height;
    desc.format = format;
    return desc;
}

/// Create render target descriptor
export fn texture_descriptor_render_target(width: u32, height: u32, format: u32) TextureDescriptor {
    var desc = texture_descriptor_2d(width, height, format);
    desc.usage = @intFromEnum(TextureUsage.RenderAttachment) | @intFromEnum(TextureUsage.TextureBinding);
    return desc;
}

/// Create depth texture descriptor
export fn texture_descriptor_depth(width: u32, height: u32) TextureDescriptor {
    var desc = texture_descriptor_2d(width, height, @intFromEnum(TextureFormat.Depth24Plus));
    desc.usage = @intFromEnum(TextureUsage.RenderAttachment) | @intFromEnum(TextureUsage.TextureBinding);
    return desc;
}

/// Create empty texture
export fn texture_create() Texture {
    return Texture{
        .handle = null,
        .width = 0,
        .height = 0,
        .depth = 1,
        .format = @intFromEnum(TextureFormat.RGBA8Unorm),
        .mip_levels = 1,
        .is_valid = false,
    };
}

/// Set texture handle (from JavaScript)
export fn texture_set_handle(texture: *Texture, handle: ?*anyopaque) void {
    texture.handle = handle;
    texture.is_valid = handle != null;
}

/// Set texture dimensions
export fn texture_set_dimensions(texture: *Texture, width: u32, height: u32, depth: u32) void {
    texture.width = width;
    texture.height = height;
    texture.depth = depth;
}

/// Set texture format
export fn texture_set_format(texture: *Texture, format: u32) void {
    texture.format = format;
}

/// Set texture mip levels
export fn texture_set_mip_levels(texture: *Texture, mip_levels: u32) void {
    texture.mip_levels = mip_levels;
}

/// Check if texture is valid
export fn texture_is_valid(texture: *const Texture) bool {
    return texture.is_valid and texture.handle != null;
}

/// Get texture width
export fn texture_get_width(texture: *const Texture) u32 {
    return texture.width;
}

/// Get texture height
export fn texture_get_height(texture: *const Texture) u32 {
    return texture.height;
}

/// Get texture format
export fn texture_get_format(texture: *const Texture) u32 {
    return texture.format;
}

/// Destroy texture
export fn texture_destroy(texture: *Texture) void {
    texture.handle = null;
    texture.is_valid = false;
}

/// Create empty texture view
export fn texture_view_create() TextureView {
    return TextureView{
        .handle = null,
        .texture = null,
        .is_valid = false,
    };
}

/// Set texture view handle (from JavaScript)
export fn texture_view_set_handle(view: *TextureView, handle: ?*anyopaque, texture: ?*anyopaque) void {
    view.handle = handle;
    view.texture = texture;
    view.is_valid = handle != null;
}

/// Check if texture view is valid
export fn texture_view_is_valid(view: *const TextureView) bool {
    return view.is_valid and view.handle != null;
}

/// Destroy texture view
export fn texture_view_destroy(view: *TextureView) void {
    view.handle = null;
    view.texture = null;
    view.is_valid = false;
}

/// Create default sampler descriptor
export fn sampler_descriptor_create() SamplerDescriptor {
    return SamplerDescriptor{
        .address_mode_u = @intFromEnum(AddressMode.ClampToEdge),
        .address_mode_v = @intFromEnum(AddressMode.ClampToEdge),
        .address_mode_w = @intFromEnum(AddressMode.ClampToEdge),
        .mag_filter = @intFromEnum(FilterMode.Linear),
        .min_filter = @intFromEnum(FilterMode.Linear),
        .mipmap_filter = @intFromEnum(FilterMode.Linear),
        .lod_min_clamp = 0.0,
        .lod_max_clamp = 32.0,
        .compare_function = 0,
    };
}

/// Create nearest sampler descriptor
export fn sampler_descriptor_nearest() SamplerDescriptor {
    var desc = sampler_descriptor_create();
    desc.mag_filter = @intFromEnum(FilterMode.Nearest);
    desc.min_filter = @intFromEnum(FilterMode.Nearest);
    desc.mipmap_filter = @intFromEnum(FilterMode.Nearest);
    return desc;
}

/// Create repeat sampler descriptor
export fn sampler_descriptor_repeat() SamplerDescriptor {
    var desc = sampler_descriptor_create();
    desc.address_mode_u = @intFromEnum(AddressMode.Repeat);
    desc.address_mode_v = @intFromEnum(AddressMode.Repeat);
    desc.address_mode_w = @intFromEnum(AddressMode.Repeat);
    return desc;
}

/// Create empty sampler
export fn sampler_create() Sampler {
    return Sampler{
        .handle = null,
        .is_valid = false,
    };
}

/// Set sampler handle (from JavaScript)
export fn sampler_set_handle(sampler: *Sampler, handle: ?*anyopaque) void {
    sampler.handle = handle;
    sampler.is_valid = handle != null;
}

/// Check if sampler is valid
export fn sampler_is_valid(sampler: *const Sampler) bool {
    return sampler.is_valid and sampler.handle != null;
}

/// Destroy sampler
export fn sampler_destroy(sampler: *Sampler) void {
    sampler.handle = null;
    sampler.is_valid = false;
}
