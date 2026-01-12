const std = @import("std");

// Forward declaration of builtin - will be imported later in the file
// This avoids duplicate import since builtin is already imported at line 602
const builtin = @import("builtin");

// ============================================================================
// Platform-specific Allocator - 平台特定内存分配器
// ============================================================================
// For WASM targets, use wasm_allocator (no libc dependency)
// For native targets, use c_allocator (requires libc)
fn getDefaultAllocator() std.mem.Allocator {
    if (builtin.cpu.arch.isWasm()) {
        return std.heap.wasm_allocator;
    } else {
        return std.heap.c_allocator;
    }
}

// ============================================================================
// TextureFormat - 纹理格式
// ============================================================================
pub const TextureFormat = enum(u32) {
    R8 = 0,
    Rg8 = 1,
    Rgba8 = 2,
    Rgba16Float = 3,
    Rgba32Float = 4,

    pub fn bytesPerPixel(self: TextureFormat) u32 {
        return switch (self) {
            .R8 => 1,
            .Rg8 => 2,
            .Rgba8 => 4,
            .Rgba16Float => 8,
            .Rgba32Float => 16,
        };
    }

    pub fn componentCount(self: TextureFormat) u32 {
        return switch (self) {
            .R8 => 1,
            .Rg8 => 2,
            .Rgba8 => 4,
            .Rgba16Float => 4,
            .Rgba32Float => 4,
        };
    }

    pub fn isFloat(self: TextureFormat) bool {
        return switch (self) {
            .R8, .Rg8, .Rgba8 => false,
            .Rgba16Float, .Rgba32Float => true,
        };
    }

    pub fn bytesPerComponent(self: TextureFormat) u32 {
        return switch (self) {
            .R8, .Rg8, .Rgba8 => 1,
            .Rgba16Float => 2,
            .Rgba32Float => 4,
        };
    }
};

// ============================================================================
// Color - 颜色类型
// ============================================================================
pub const Color = extern struct {
    r: f32,
    g: f32,
    b: f32,
    a: f32,

    pub fn init(r: f32, g: f32, b: f32, a: f32) Color {
        return .{ .r = r, .g = g, .b = b, .a = a };
    }

    pub fn fromBytes(r: u8, g: u8, b: u8, a: u8) Color {
        return .{
            .r = @as(f32, @floatFromInt(r)) / 255.0,
            .g = @as(f32, @floatFromInt(g)) / 255.0,
            .b = @as(f32, @floatFromInt(b)) / 255.0,
            .a = @as(f32, @floatFromInt(a)) / 255.0,
        };
    }

    pub fn toBytes(self: Color) [4]u8 {
        return .{
            @intFromFloat(@min(@max(self.r, 0.0), 1.0) * 255.0),
            @intFromFloat(@min(@max(self.g, 0.0), 1.0) * 255.0),
            @intFromFloat(@min(@max(self.b, 0.0), 1.0) * 255.0),
            @intFromFloat(@min(@max(self.a, 0.0), 1.0) * 255.0),
        };
    }
};

// ============================================================================
// Image - 图像数据结构
// ============================================================================
pub const Image = struct {
    data: []u8,
    width: u32,
    height: u32,
    format: TextureFormat,
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, width: u32, height: u32, format: TextureFormat) !*Image {
        const bytes_per_pixel = format.bytesPerPixel();
        const total_size = width * height * bytes_per_pixel;

        const data = try allocator.alloc(u8, total_size);
        @memset(data, 0);

        const image = try allocator.create(Image);
        image.* = .{
            .data = data,
            .width = width,
            .height = height,
            .format = format,
            .allocator = allocator,
        };

        return image;
    }

    pub fn fromRawData(allocator: std.mem.Allocator, data_ptr: [*]const u8, data_len: usize, width: u32, height: u32, format: TextureFormat) !*Image {
        const bytes_per_pixel = format.bytesPerPixel();
        const expected_size = width * height * bytes_per_pixel;

        if (data_len < expected_size) {
            return error.InvalidDataSize;
        }

        const data = try allocator.alloc(u8, expected_size);
        @memcpy(data, data_ptr[0..expected_size]);

        const image = try allocator.create(Image);
        image.* = .{
            .data = data,
            .width = width,
            .height = height,
            .format = format,
            .allocator = allocator,
        };

        return image;
    }

    pub fn deinit(self: *Image) void {
        self.allocator.free(self.data);
        self.allocator.destroy(self);
    }

    pub fn getPixel(self: *const Image, x: u32, y: u32) Color {
        if (x >= self.width or y >= self.height) {
            return Color.init(0.0, 0.0, 0.0, 0.0);
        }

        const bytes_per_pixel = self.format.bytesPerPixel();
        const offset = (y * self.width + x) * bytes_per_pixel;

        return switch (self.format) {
            .R8 => Color.fromBytes(self.data[offset], 0, 0, 255),
            .Rg8 => Color.fromBytes(self.data[offset], self.data[offset + 1], 0, 255),
            .Rgba8 => Color.fromBytes(
                self.data[offset],
                self.data[offset + 1],
                self.data[offset + 2],
                self.data[offset + 3],
            ),
            .Rgba16Float, .Rgba32Float => Color.init(0.0, 0.0, 0.0, 1.0),
        };
    }

    pub fn setPixel(self: *Image, x: u32, y: u32, color: Color) void {
        if (x >= self.width or y >= self.height) {
            return;
        }

        const bytes_per_pixel = self.format.bytesPerPixel();
        const offset = (y * self.width + x) * bytes_per_pixel;
        const bytes = color.toBytes();

        switch (self.format) {
            .R8 => {
                self.data[offset] = bytes[0];
            },
            .Rg8 => {
                self.data[offset] = bytes[0];
                self.data[offset + 1] = bytes[1];
            },
            .Rgba8 => {
                self.data[offset] = bytes[0];
                self.data[offset + 1] = bytes[1];
                self.data[offset + 2] = bytes[2];
                self.data[offset + 3] = bytes[3];
            },
            .Rgba16Float, .Rgba32Float => {},
        }
    }
};

// ============================================================================
// TextureDescriptor - 纹理描述符
// ============================================================================
pub const Extent3d = extern struct {
    width: u32,
    height: u32,
    depth_or_array_layers: u32,

    pub fn init2D(width: u32, height: u32) Extent3d {
        return .{ .width = width, .height = height, .depth_or_array_layers = 1 };
    }

    pub fn init3D(width: u32, height: u32, depth: u32) Extent3d {
        return .{ .width = width, .height = height, .depth_or_array_layers = depth };
    }
};

pub const TextureDimension = enum(u32) {
    D1 = 0,
    D2 = 1,
    D3 = 2,
};

pub const TextureUsage = extern struct {
    copy_src: bool,
    copy_dst: bool,
    texture_binding: bool,
    storage_binding: bool,
    render_attachment: bool,
    _padding: [27]u8,

    pub fn default() TextureUsage {
        return .{
            .copy_src = false,
            .copy_dst = true,
            .texture_binding = true,
            .storage_binding = false,
            .render_attachment = false,
            ._padding = [_]u8{0} ** 27,
        };
    }
};

pub const TextureDescriptor = extern struct {
    size: Extent3d,
    mip_level_count: u32,
    sample_count: u32,
    dimension: TextureDimension,
    format: TextureFormat,
    usage: TextureUsage,

    pub fn default2D(width: u32, height: u32) TextureDescriptor {
        return .{
            .size = Extent3d.init2D(width, height),
            .mip_level_count = 1,
            .sample_count = 1,
            .dimension = .D2,
            .format = .Rgba8,
            .usage = TextureUsage.default(),
        };
    }
};

// ============================================================================
// Sampler - 采样器
// ============================================================================
pub const AddressMode = enum(u32) {
    ClampToEdge = 0,
    Repeat = 1,
    MirrorRepeat = 2,
};

pub const FilterMode = enum(u32) {
    Nearest = 0,
    Linear = 1,
};

pub const SamplerDescriptor = extern struct {
    address_mode_u: AddressMode,
    address_mode_v: AddressMode,
    address_mode_w: AddressMode,
    mag_filter: FilterMode,
    min_filter: FilterMode,
    mipmap_filter: FilterMode,

    pub fn default() SamplerDescriptor {
        return .{
            .address_mode_u = .ClampToEdge,
            .address_mode_v = .ClampToEdge,
            .address_mode_w = .ClampToEdge,
            .mag_filter = .Linear,
            .min_filter = .Linear,
            .mipmap_filter = .Linear,
        };
    }

    pub fn repeat() SamplerDescriptor {
        return .{
            .address_mode_u = .Repeat,
            .address_mode_v = .Repeat,
            .address_mode_w = .Repeat,
            .mag_filter = .Linear,
            .min_filter = .Linear,
            .mipmap_filter = .Linear,
        };
    }
};

// ============================================================================
// Image Operations - 图像操作函数
// ============================================================================
pub fn solidColor(allocator: std.mem.Allocator, width: u32, height: u32, color: Color) !*Image {
    const image = try Image.init(allocator, width, height, .Rgba8);
    var y: u32 = 0;
    while (y < height) : (y += 1) {
        var x: u32 = 0;
        while (x < width) : (x += 1) {
            image.setPixel(x, y, color);
        }
    }
    return image;
}

pub fn resize(allocator: std.mem.Allocator, source: *const Image, new_width: u32, new_height: u32) !*Image {
    const result = try Image.init(allocator, new_width, new_height, source.format);

    var y: u32 = 0;
    while (y < new_height) : (y += 1) {
        var x: u32 = 0;
        while (x < new_width) : (x += 1) {
            const src_x = (x * source.width) / new_width;
            const src_y = (y * source.height) / new_height;
            const color = source.getPixel(src_x, src_y);
            result.setPixel(x, y, color);
        }
    }

    return result;
}

pub fn crop(allocator: std.mem.Allocator, source: *const Image, start_x: u32, start_y: u32, crop_width: u32, crop_height: u32) !*Image {
    if (start_x + crop_width > source.width or start_y + crop_height > source.height) {
        return error.CropOutOfBounds;
    }

    const result = try Image.init(allocator, crop_width, crop_height, source.format);

    var y: u32 = 0;
    while (y < crop_height) : (y += 1) {
        var x: u32 = 0;
        while (x < crop_width) : (x += 1) {
            const color = source.getPixel(start_x + x, start_y + y);
            result.setPixel(x, y, color);
        }
    }

    return result;
}

pub fn flipVertical(image: *Image) void {
    const bytes_per_pixel = image.format.bytesPerPixel();
    const row_size = image.width * bytes_per_pixel;

    var y: u32 = 0;
    while (y < image.height / 2) : (y += 1) {
        const top_offset = y * row_size;
        const bottom_offset = (image.height - 1 - y) * row_size;

        var x: u32 = 0;
        while (x < row_size) : (x += 1) {
            const temp = image.data[top_offset + x];
            image.data[top_offset + x] = image.data[bottom_offset + x];
            image.data[bottom_offset + x] = temp;
        }
    }
}

pub fn flipHorizontal(image: *Image) void {
    const bytes_per_pixel = image.format.bytesPerPixel();

    var y: u32 = 0;
    while (y < image.height) : (y += 1) {
        var x: u32 = 0;
        while (x < image.width / 2) : (x += 1) {
            const left_offset = (y * image.width + x) * bytes_per_pixel;
            const right_offset = (y * image.width + (image.width - 1 - x)) * bytes_per_pixel;

            var b: u32 = 0;
            while (b < bytes_per_pixel) : (b += 1) {
                const temp = image.data[left_offset + b];
                image.data[left_offset + b] = image.data[right_offset + b];
                image.data[right_offset + b] = temp;
            }
        }
    }
}

pub fn convertFormat(allocator: std.mem.Allocator, source: *const Image, target_format: TextureFormat) !*Image {
    if (source.format == target_format) {
        const result = try Image.init(allocator, source.width, source.height, target_format);
        @memcpy(result.data, source.data);
        return result;
    }

    const result = try Image.init(allocator, source.width, source.height, target_format);

    var y: u32 = 0;
    while (y < source.height) : (y += 1) {
        var x: u32 = 0;
        while (x < source.width) : (x += 1) {
            const color = source.getPixel(x, y);
            result.setPixel(x, y, color);
        }
    }

    return result;
}

// ============================================================================
// FFI Exports - 所有导出函数
// ============================================================================

// Image exports
export fn image_create(width: u32, height: u32, format: TextureFormat) ?*Image {
    const allocator = getDefaultAllocator();
    return Image.init(allocator, width, height, format) catch null;
}

export fn image_from_raw_data(data_ptr: [*]const u8, data_len: usize, width: u32, height: u32, format: TextureFormat) ?*Image {
    const allocator = getDefaultAllocator();
    return Image.fromRawData(allocator, data_ptr, data_len, width, height, format) catch null;
}

export fn image_destroy(image: *Image) void {
    image.deinit();
}

export fn image_width(image: *const Image) u32 {
    return image.width;
}

export fn image_height(image: *const Image) u32 {
    return image.height;
}

export fn image_format(image: *const Image) TextureFormat {
    return image.format;
}

export fn image_data_ptr(image: *const Image) [*]const u8 {
    return image.data.ptr;
}

export fn image_data_len(image: *const Image) usize {
    return image.data.len;
}

export fn image_get_pixel(image: *const Image, x: u32, y: u32) Color {
    return image.getPixel(x, y);
}

export fn image_set_pixel(image: *Image, x: u32, y: u32, color: Color) void {
    image.setPixel(x, y, color);
}

// Image operations exports
export fn image_solid_color(width: u32, height: u32, color: Color) ?*Image {
    const allocator = getDefaultAllocator();
    return solidColor(allocator, width, height, color) catch null;
}

export fn image_resize(image: *const Image, new_width: u32, new_height: u32) ?*Image {
    const allocator = getDefaultAllocator();
    return resize(allocator, image, new_width, new_height) catch null;
}

export fn image_crop(image: *const Image, x: u32, y: u32, width: u32, height: u32) ?*Image {
    const allocator = getDefaultAllocator();
    return crop(allocator, image, x, y, width, height) catch null;
}

export fn image_flip_vertical(image: *Image) void {
    flipVertical(image);
}

export fn image_flip_horizontal(image: *Image) void {
    flipHorizontal(image);
}

export fn image_convert_format(image: *const Image, target_format: TextureFormat) ?*Image {
    const allocator = getDefaultAllocator();
    return convertFormat(allocator, image, target_format) catch null;
}

// Extent3d exports
export fn extent3d_init_2d(width: u32, height: u32) Extent3d {
    return Extent3d.init2D(width, height);
}

export fn extent3d_init_3d(width: u32, height: u32, depth: u32) Extent3d {
    return Extent3d.init3D(width, height, depth);
}

// TextureUsage exports
export fn texture_usage_default() TextureUsage {
    return TextureUsage.default();
}

// SamplerDescriptor exports
export fn sampler_descriptor_default() SamplerDescriptor {
    return SamplerDescriptor.default();
}

export fn sampler_descriptor_repeat() SamplerDescriptor {
    return SamplerDescriptor.repeat();
}

// TextureFormat exports
export fn texture_format_bytes_per_pixel(format: TextureFormat) u32 {
    return format.bytesPerPixel();
}

export fn texture_format_component_count(format: TextureFormat) u32 {
    return format.componentCount();
}

export fn texture_format_is_float(format: TextureFormat) bool {
    return format.isFloat();
}

export fn texture_format_bytes_per_component(format: TextureFormat) u32 {
    return format.bytesPerComponent();
}

// Additional Image exports
export fn image_new(width: u32, height: u32, format: TextureFormat) ?*Image {
    const allocator = getDefaultAllocator();
    return Image.init(allocator, width, height, format) catch null;
}

// TextureDescriptor exports
export fn texture_descriptor_default_2d(width: u32, height: u32) TextureDescriptor {
    return TextureDescriptor.default2D(width, height);
}

export fn texture_descriptor_with_mip_levels(desc: TextureDescriptor, mip_levels: u32) TextureDescriptor {
    var result = desc;
    result.mip_level_count = mip_levels;
    return result;
}

export fn texture_descriptor_render_target_2d(width: u32, height: u32, format: TextureFormat) TextureDescriptor {
    var desc = TextureDescriptor.default2D(width, height);
    desc.format = format;
    desc.usage.render_attachment = true;
    desc.usage.texture_binding = true;
    return desc;
}

// TextureUsage exports
export fn texture_usage_render_target() TextureUsage {
    return TextureUsage{
        .copy_src = false,
        .copy_dst = false,
        .texture_binding = true,
        .storage_binding = false,
        .render_attachment = true,
        ._padding = [_]u8{0} ** 27,
    };
}

// SamplerDescriptor additional exports
export fn sampler_descriptor_nearest() SamplerDescriptor {
    return SamplerDescriptor{
        .address_mode_u = .ClampToEdge,
        .address_mode_v = .ClampToEdge,
        .address_mode_w = .ClampToEdge,
        .mag_filter = .Nearest,
        .min_filter = .Nearest,
        .mipmap_filter = .Nearest,
    };
}

export fn sampler_descriptor_linear() SamplerDescriptor {
    return SamplerDescriptor.default();
}

export fn sampler_descriptor_with_address_mode(desc: SamplerDescriptor, mode: AddressMode) SamplerDescriptor {
    var result = desc;
    result.address_mode_u = mode;
    result.address_mode_v = mode;
    result.address_mode_w = mode;
    return result;
}

export fn sampler_descriptor_with_filter(desc: SamplerDescriptor, filter: FilterMode) SamplerDescriptor {
    var result = desc;
    result.mag_filter = filter;
    result.min_filter = filter;
    result.mipmap_filter = filter;
    return result;
}

// ============================================================================
// Tests - Only compile for native targets, not WASM
// ============================================================================
// builtin is already imported at the top of the file

// Tests require std.testing.allocator which uses c_allocator
// Skip all tests for WASM targets to avoid libc dependency
test "image creation" {
    if (builtin.cpu.arch.isWasm()) return error.SkipZigTest;
    const allocator = std.testing.allocator;
    const img = try Image.init(allocator, 100, 100, .Rgba8);
    defer img.deinit();

    try std.testing.expectEqual(@as(u32, 100), img.width);
    try std.testing.expectEqual(@as(u32, 100), img.height);
    try std.testing.expectEqual(TextureFormat.Rgba8, img.format);
}

test "texture format bytes per pixel" {
    if (builtin.cpu.arch.isWasm()) return error.SkipZigTest;
    try std.testing.expectEqual(@as(u32, 1), TextureFormat.R8.bytesPerPixel());
    try std.testing.expectEqual(@as(u32, 2), TextureFormat.Rg8.bytesPerPixel());
    try std.testing.expectEqual(@as(u32, 4), TextureFormat.Rgba8.bytesPerPixel());
    try std.testing.expectEqual(@as(u32, 8), TextureFormat.Rgba16Float.bytesPerPixel());
    try std.testing.expectEqual(@as(u32, 16), TextureFormat.Rgba32Float.bytesPerPixel());
}

test "get set pixel" {
    if (builtin.cpu.arch.isWasm()) return error.SkipZigTest;
    const allocator = std.testing.allocator;
    const img = try Image.init(allocator, 10, 10, .Rgba8);
    defer img.deinit();

    const test_color = Color{ .r = 1.0, .g = 0.5, .b = 0.25, .a = 1.0 };
    img.setPixel(5, 5, test_color);
    const retrieved = img.getPixel(5, 5);

    try std.testing.expect(@abs(retrieved.r - test_color.r) < 0.01);
    try std.testing.expect(@abs(retrieved.g - test_color.g) < 0.01);
    try std.testing.expect(@abs(retrieved.b - test_color.b) < 0.01);
}

test "solid color image" {
    if (builtin.cpu.arch.isWasm()) return error.SkipZigTest;
    const allocator = std.testing.allocator;
    const color = Color{ .r = 1.0, .g = 0.0, .b = 0.0, .a = 1.0 };
    const img = try solidColor(allocator, 10, 10, color);
    defer img.deinit();

    const pixel = img.getPixel(5, 5);
    try std.testing.expect(@abs(pixel.r - 1.0) < 0.01);
    try std.testing.expect(@abs(pixel.g - 0.0) < 0.01);
}

test "image resize" {
    if (builtin.cpu.arch.isWasm()) return error.SkipZigTest;
    const allocator = std.testing.allocator;
    const original = try Image.init(allocator, 100, 100, .Rgba8);
    defer original.deinit();

    const resized = try resize(allocator, original, 50, 50);
    defer resized.deinit();

    try std.testing.expectEqual(@as(u32, 50), resized.width);
    try std.testing.expectEqual(@as(u32, 50), resized.height);
}

test "image crop" {
    if (builtin.cpu.arch.isWasm()) return error.SkipZigTest;
    const allocator = std.testing.allocator;
    const original = try Image.init(allocator, 100, 100, .Rgba8);
    defer original.deinit();

    const cropped = try crop(allocator, original, 10, 10, 50, 50);
    defer cropped.deinit();

    try std.testing.expectEqual(@as(u32, 50), cropped.width);
    try std.testing.expectEqual(@as(u32, 50), cropped.height);
}

test "image flip vertical" {
    if (builtin.cpu.arch.isWasm()) return error.SkipZigTest;
    const allocator = std.testing.allocator;
    const img = try Image.init(allocator, 10, 10, .Rgba8);
    defer img.deinit();

    const top_color = Color{ .r = 1.0, .g = 0.0, .b = 0.0, .a = 1.0 };
    const bottom_color = Color{ .r = 0.0, .g = 1.0, .b = 0.0, .a = 1.0 };

    img.setPixel(5, 0, top_color);
    img.setPixel(5, 9, bottom_color);

    flipVertical(img);

    const flipped_top = img.getPixel(5, 0);
    const flipped_bottom = img.getPixel(5, 9);

    try std.testing.expect(@abs(flipped_top.g - 1.0) < 0.01);
    try std.testing.expect(@abs(flipped_bottom.r - 1.0) < 0.01);
}

test "texture descriptor" {
    if (builtin.cpu.arch.isWasm()) return error.SkipZigTest;
    const desc = TextureDescriptor.default2D(1024, 768);
    try std.testing.expectEqual(@as(u32, 1024), desc.size.width);
    try std.testing.expectEqual(@as(u32, 768), desc.size.height);
    try std.testing.expectEqual(@as(u32, 1), desc.mip_level_count);
}

test "sampler descriptor" {
    if (builtin.cpu.arch.isWasm()) return error.SkipZigTest;
    const sampler_desc = SamplerDescriptor.default();
    try std.testing.expectEqual(AddressMode.ClampToEdge, sampler_desc.address_mode_u);
    try std.testing.expectEqual(FilterMode.Linear, sampler_desc.mag_filter);
}
