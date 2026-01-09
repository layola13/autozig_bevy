// Unified text module for autozig-text
// Complete text rendering system for WebGPU/WASM platform
// Based on bevy_text design

const std = @import("std");

// ============================================================================
// Core Types (from dependencies)
// ============================================================================

pub const Color = extern struct {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
};

pub const Vec2 = extern struct {
    x: f32,
    y: f32,

    pub fn new(x: f32, y: f32) Vec2 {
        return .{ .x = x, .y = y };
    }

    pub fn zero() Vec2 {
        return .{ .x = 0.0, .y = 0.0 };
    }

    pub fn add(self: Vec2, other: Vec2) Vec2 {
        return .{ .x = self.x + other.x, .y = self.y + other.y };
    }

    pub fn scale(self: Vec2, s: f32) Vec2 {
        return .{ .x = self.x * s, .y = self.y * s };
    }
};

pub const Vec3 = extern struct {
    x: f32,
    y: f32,
    z: f32,

    pub fn new(x: f32, y: f32, z: f32) Vec3 {
        return .{ .x = x, .y = y, .z = z };
    }
};

pub const Rect = extern struct {
    min: Vec2,
    max: Vec2,

    pub fn new(min: Vec2, max: Vec2) Rect {
        return .{ .min = min, .max = max };
    }

    pub fn width(self: Rect) f32 {
        return self.max.x - self.min.x;
    }

    pub fn height(self: Rect) f32 {
        return self.max.y - self.min.y;
    }

    pub fn size(self: Rect) Vec2 {
        return Vec2.new(self.width(), self.height());
    }
};

// ============================================================================
// Text Alignment
// ============================================================================

pub const TextAlignment = enum(u8) {
    Left = 0,
    Center = 1,
    Right = 2,
    Justified = 3,

    pub fn getAlignmentOffset(self: TextAlignment, line_width: f32, max_width: f32) f32 {
        return switch (self) {
            .Left => 0.0,
            .Center => (max_width - line_width) * 0.5,
            .Right => max_width - line_width,
            .Justified => 0.0,
        };
    }
};

pub const VerticalAlignment = enum(u8) {
    Top = 0,
    Middle = 1,
    Bottom = 2,

    pub fn getVerticalOffset(self: VerticalAlignment, content_height: f32, max_height: f32) f32 {
        return switch (self) {
            .Top => 0.0,
            .Middle => (max_height - content_height) * 0.5,
            .Bottom => max_height - content_height,
        };
    }
};

// ============================================================================
// Font Types
// ============================================================================

pub const FontHandle = extern struct {
    id: u32,

    pub fn new(id: u32) FontHandle {
        return .{ .id = id };
    }

    pub fn isValid(self: FontHandle) bool {
        return self.id != 0;
    }
};

pub const GlyphId = extern struct {
    value: u32,

    pub fn new(value: u32) GlyphId {
        return .{ .value = value };
    }
};

pub const FontMetrics = extern struct {
    ascent: f32,
    descent: f32,
    line_gap: f32,
    units_per_em: f32,

    pub fn lineHeight(self: FontMetrics) f32 {
        return self.ascent - self.descent + self.line_gap;
    }

    pub fn scale(self: FontMetrics, font_size: f32) FontMetrics {
        const s = font_size / self.units_per_em;
        return FontMetrics{
            .ascent = self.ascent * s,
            .descent = self.descent * s,
            .line_gap = self.line_gap * s,
            .units_per_em = self.units_per_em,
        };
    }
};

pub const GlyphMetrics = extern struct {
    advance_width: f32,
    advance_height: f32,
    bearing_x: f32,
    bearing_y: f32,
    width: f32,
    height: f32,

    pub fn new() GlyphMetrics {
        return .{
            .advance_width = 0.0,
            .advance_height = 0.0,
            .bearing_x = 0.0,
            .bearing_y = 0.0,
            .width = 0.0,
            .height = 0.0,
        };
    }
};

// ============================================================================
// Glyph Atlas
// ============================================================================

pub const GlyphAtlasEntry = extern struct {
    glyph_id: GlyphId,
    uv_rect: Rect,
    metrics: GlyphMetrics,
    texture_index: u32,

    pub fn new(glyph_id: GlyphId, uv_rect: Rect, metrics: GlyphMetrics, texture_index: u32) GlyphAtlasEntry {
        return .{
            .glyph_id = glyph_id,
            .uv_rect = uv_rect,
            .metrics = metrics,
            .texture_index = texture_index,
        };
    }
};

pub const GlyphAtlas = extern struct {
    texture_size: Vec2,
    current_x: f32,
    current_y: f32,
    row_height: f32,
    padding: f32,

    pub fn new(texture_size: Vec2, padding: f32) GlyphAtlas {
        return .{
            .texture_size = texture_size,
            .current_x = padding,
            .current_y = padding,
            .row_height = 0.0,
            .padding = padding,
        };
    }

    pub fn allocate(self: *GlyphAtlas, width: f32, height: f32) ?Rect {
        const padded_width = width + self.padding * 2.0;
        const padded_height = height + self.padding * 2.0;

        // Check if we need to move to next row
        if (self.current_x + padded_width > self.texture_size.x) {
            self.current_x = self.padding;
            self.current_y += self.row_height + self.padding;
            self.row_height = 0.0;
        }

        // Check if we have space in atlas
        if (self.current_y + padded_height > self.texture_size.y) {
            return null; // Atlas is full
        }

        const min = Vec2.new(self.current_x, self.current_y);
        const max = Vec2.new(self.current_x + width, self.current_y + height);
        const rect = Rect.new(min, max);

        self.current_x += padded_width;
        self.row_height = @max(self.row_height, padded_height);

        return rect;
    }

    pub fn reset(self: *GlyphAtlas) void {
        self.current_x = self.padding;
        self.current_y = self.padding;
        self.row_height = 0.0;
    }

    pub fn uvRect(self: GlyphAtlas, pixel_rect: Rect) Rect {
        return Rect.new(
            Vec2.new(pixel_rect.min.x / self.texture_size.x, pixel_rect.min.y / self.texture_size.y),
            Vec2.new(pixel_rect.max.x / self.texture_size.x, pixel_rect.max.y / self.texture_size.y),
        );
    }
};

// ============================================================================
// Text Component
// ============================================================================

pub const Text = extern struct {
    content_ptr: ?[*]const u8,
    content_len: usize,
    font: FontHandle,
    font_size: f32,
    color: Color,
    alignment: TextAlignment,
    vertical_alignment: VerticalAlignment,
    line_height_factor: f32,
    letter_spacing: f32,
    word_spacing: f32,

    pub fn new(content_ptr: ?[*]const u8, content_len: usize, font: FontHandle, font_size: f32, color: Color) Text {
        return .{
            .content_ptr = content_ptr,
            .content_len = content_len,
            .font = font,
            .font_size = font_size,
            .color = color,
            .alignment = .Left,
            .vertical_alignment = .Top,
            .line_height_factor = 1.2,
            .letter_spacing = 0.0,
            .word_spacing = 0.0,
        };
    }

    pub fn withAlignment(self: Text, alignment: TextAlignment) Text {
        var result = self;
        result.alignment = alignment;
        return result;
    }

    pub fn withVerticalAlignment(self: Text, vertical_alignment: VerticalAlignment) Text {
        var result = self;
        result.vertical_alignment = vertical_alignment;
        return result;
    }

    pub fn withLineHeight(self: Text, line_height_factor: f32) Text {
        var result = self;
        result.line_height_factor = line_height_factor;
        return result;
    }

    pub fn getContent(self: Text) []const u8 {
        if (self.content_ptr) |ptr| {
            return ptr[0..self.content_len];
        }
        return &[_]u8{};
    }
};

// ============================================================================
// Text Layout
// ============================================================================

pub const LineInfo = extern struct {
    start_index: usize,
    end_index: usize,
    width: f32,
    y_offset: f32,

    pub fn new(start_index: usize, end_index: usize, width: f32, y_offset: f32) LineInfo {
        return .{
            .start_index = start_index,
            .end_index = end_index,
            .width = width,
            .y_offset = y_offset,
        };
    }
};

pub const TextLayout = extern struct {
    lines_ptr: ?[*]LineInfo,
    lines_len: usize,
    lines_cap: usize,
    total_width: f32,
    total_height: f32,
    font_metrics: FontMetrics,

    pub fn new(font_metrics: FontMetrics) TextLayout {
        return .{
            .lines_ptr = null,
            .lines_len = 0,
            .lines_cap = 0,
            .total_width = 0.0,
            .total_height = 0.0,
            .font_metrics = font_metrics,
        };
    }

    pub fn getLines(self: TextLayout) []LineInfo {
        if (self.lines_ptr) |ptr| {
            return ptr[0..self.lines_len];
        }
        return &[_]LineInfo{};
    }
};

// ============================================================================
// Text Vertex (for rendering)
// ============================================================================

pub const TextVertex = extern struct {
    position: [3]f32,
    uv: [2]f32,
    color: u32,

    pub fn new(position: Vec3, uv: Vec2, color: u32) TextVertex {
        return .{
            .position = [3]f32{ position.x, position.y, position.z },
            .uv = [2]f32{ uv.x, uv.y },
            .color = color,
        };
    }
};

pub const GlyphInstance = extern struct {
    position: Vec2,
    size: Vec2,
    uv_rect: Rect,
    color: u32,

    pub fn new(position: Vec2, size: Vec2, uv_rect: Rect, color: u32) GlyphInstance {
        return .{
            .position = position,
            .size = size,
            .uv_rect = uv_rect,
            .color = color,
        };
    }
};

// ============================================================================
// Text Measurement
// ============================================================================

pub const TextBounds = extern struct {
    width: f32,
    height: f32,
    line_count: usize,

    pub fn new(width: f32, height: f32, line_count: usize) TextBounds {
        return .{
            .width = width,
            .height = height,
            .line_count = line_count,
        };
    }
};

pub fn measureText(text: []const u8, font_size: f32, font_metrics: FontMetrics, max_width: ?f32) TextBounds {
    const scaled_metrics = font_metrics.scale(font_size);
    const line_height = scaled_metrics.lineHeight();

    var max_line_width: f32 = 0.0;
    var current_width: f32 = 0.0;
    var line_count: usize = 1;

    for (text) |c| {
        if (c == '\n') {
            max_line_width = @max(max_line_width, current_width);
            current_width = 0.0;
            line_count += 1;
            continue;
        }

        // Simplified: assume each character is 0.6 * font_size wide
        const char_width = font_size * 0.6;
        current_width += char_width;

        if (max_width) |max_w| {
            if (current_width > max_w) {
                max_line_width = @max(max_line_width, max_w);
                current_width = char_width;
                line_count += 1;
            }
        }
    }

    max_line_width = @max(max_line_width, current_width);
    const total_height = @as(f32, @floatFromInt(line_count)) * line_height;

    return TextBounds.new(max_line_width, total_height, line_count);
}

// ============================================================================
// Color Packing
// ============================================================================

pub fn packColor(color: Color) u32 {
    const r = @as(u32, @intFromFloat(@min(@max(color.r, 0.0), 1.0) * 255.0));
    const g = @as(u32, @intFromFloat(@min(@max(color.g, 0.0), 1.0) * 255.0));
    const b = @as(u32, @intFromFloat(@min(@max(color.b, 0.0), 1.0) * 255.0));
    const a = @as(u32, @intFromFloat(@min(@max(color.a, 0.0), 1.0) * 255.0));
    return (a << 24) | (b << 16) | (g << 8) | r;
}

pub fn unpackColor(packed_value: u32) Color {
    const r = @as(f32, @floatFromInt(packed_value & 0xFF)) / 255.0;
    const g = @as(f32, @floatFromInt((packed_value >> 8) & 0xFF)) / 255.0;
    const b = @as(f32, @floatFromInt((packed_value >> 16) & 0xFF)) / 255.0;
    const a = @as(f32, @floatFromInt((packed_value >> 24) & 0xFF)) / 255.0;
    return Color{ .r = r, .g = g, .b = b, .a = a };
}

// ============================================================================
// Glyph Quad Generation
// ============================================================================

pub fn createGlyphQuad(instance: GlyphInstance) [4]TextVertex {
    const pos = instance.position;
    const size = instance.size;
    const uv = instance.uv_rect;

    return [4]TextVertex{
        // Bottom-left
        TextVertex.new(
            Vec3.new(pos.x, pos.y, 0.0),
            Vec2.new(uv.min.x, uv.max.y),
            instance.color,
        ),
        // Bottom-right
        TextVertex.new(
            Vec3.new(pos.x + size.x, pos.y, 0.0),
            Vec2.new(uv.max.x, uv.max.y),
            instance.color,
        ),
        // Top-right
        TextVertex.new(
            Vec3.new(pos.x + size.x, pos.y + size.y, 0.0),
            Vec2.new(uv.max.x, uv.min.y),
            instance.color,
        ),
        // Top-left
        TextVertex.new(
            Vec3.new(pos.x, pos.y + size.y, 0.0),
            Vec2.new(uv.min.x, uv.min.y),
            instance.color,
        ),
    };
}

// ============================================================================
// Text Wrapping
// ============================================================================

pub const WordWrapMode = enum(u8) {
    NoWrap = 0,
    WordWrap = 1,
    CharacterWrap = 2,
};

pub fn wrapText(text: []const u8, max_width: f32, font_size: f32, mode: WordWrapMode) []const u8 {
    // Simplified implementation - in real scenario, would need proper word breaking
    _ = max_width;
    _ = font_size;
    _ = mode;
    return text;
}

// ============================================================================
// SDF (Signed Distance Field) Support
// ============================================================================

pub const SDFParams = extern struct {
    spread: f32,
    smoothness: f32,
    threshold: f32,

    pub fn default() SDFParams {
        return .{
            .spread = 4.0,
            .smoothness = 0.25,
            .threshold = 0.5,
        };
    }
};

pub fn calculateSDFValue(distance: f32, params: SDFParams) f32 {
    const normalized = distance / params.spread;
    const smoothed = std.math.clamp(
        (normalized + params.threshold) / params.smoothness,
        0.0,
        1.0,
    );
    return smoothed;
}

// ============================================================================
// FFI Exports
// ============================================================================

// Text Component exports
export fn text_new(content_ptr: ?[*]const u8, content_len: usize, font: FontHandle, font_size: f32, color: Color) Text {
    return Text.new(content_ptr, content_len, font, font_size, color);
}

export fn text_with_alignment(text: Text, alignment: TextAlignment) Text {
    return text.withAlignment(alignment);
}

export fn text_with_vertical_alignment(text: Text, vertical_alignment: VerticalAlignment) Text {
    return text.withVerticalAlignment(vertical_alignment);
}

export fn text_with_line_height(text: Text, line_height_factor: f32) Text {
    return text.withLineHeight(line_height_factor);
}

// Font exports
export fn font_handle_new(id: u32) FontHandle {
    return FontHandle.new(id);
}

export fn font_handle_is_valid(handle: FontHandle) bool {
    return handle.isValid();
}

export fn glyph_id_new(value: u32) GlyphId {
    return GlyphId.new(value);
}

export fn font_metrics_line_height(metrics: FontMetrics) f32 {
    return metrics.lineHeight();
}

export fn font_metrics_scale(metrics: FontMetrics, font_size: f32) FontMetrics {
    return metrics.scale(font_size);
}

export fn glyph_metrics_new() GlyphMetrics {
    return GlyphMetrics.new();
}

// Glyph Atlas exports
export fn glyph_atlas_new(texture_size: Vec2, padding: f32) GlyphAtlas {
    return GlyphAtlas.new(texture_size, padding);
}

export fn glyph_atlas_allocate(atlas: *GlyphAtlas, width: f32, height: f32, out_rect: *Rect) bool {
    if (atlas.allocate(width, height)) |rect| {
        out_rect.* = rect;
        return true;
    }
    return false;
}

export fn glyph_atlas_reset(atlas: *GlyphAtlas) void {
    atlas.reset();
}

export fn glyph_atlas_uv_rect(atlas: GlyphAtlas, pixel_rect: Rect) Rect {
    return atlas.uvRect(pixel_rect);
}

export fn glyph_atlas_entry_new(glyph_id: GlyphId, uv_rect: Rect, metrics: GlyphMetrics, texture_index: u32) GlyphAtlasEntry {
    return GlyphAtlasEntry.new(glyph_id, uv_rect, metrics, texture_index);
}

// Text Layout exports
export fn text_layout_new(font_metrics: FontMetrics) TextLayout {
    return TextLayout.new(font_metrics);
}

export fn line_info_new(start_index: usize, end_index: usize, width: f32, y_offset: f32) LineInfo {
    return LineInfo.new(start_index, end_index, width, y_offset);
}

// Text measurement exports
export fn text_measure(text_ptr: [*]const u8, text_len: usize, font_size: f32, font_metrics: FontMetrics, max_width_ptr: ?*const f32) TextBounds {
    const text_slice = text_ptr[0..text_len];
    const max_width = if (max_width_ptr) |ptr| ptr.* else null;
    return measureText(text_slice, font_size, font_metrics, max_width);
}

export fn text_bounds_new(width: f32, height: f32, line_count: usize) TextBounds {
    return TextBounds.new(width, height, line_count);
}

// Text vertex exports
export fn text_vertex_new(position: *const [3]f32, uv: Vec2, color: u32) TextVertex {
    const pos = Vec3.new(position[0], position[1], position[2]);
    return TextVertex.new(pos, uv, color);
}

export fn glyph_instance_new(position: Vec2, size: Vec2, uv_rect: Rect, color: u32) GlyphInstance {
    return GlyphInstance.new(position, size, uv_rect, color);
}

export fn create_glyph_quad(instance: GlyphInstance) [4]TextVertex {
    return createGlyphQuad(instance);
}

// Color packing exports
export fn pack_color(color: Color) u32 {
    return packColor(color);
}

export fn unpack_color(packed_value: u32) Color {
    return unpackColor(packed_value);
}

// Note: rect_new, rect_width, rect_height, vec2_new, vec2_add, vec2_scale
// are provided by autozig-math, so we don't re-export them here

// Alignment exports
export fn text_alignment_get_offset(alignment: TextAlignment, line_width: f32, max_width: f32) f32 {
    return alignment.getAlignmentOffset(line_width, max_width);
}

export fn vertical_alignment_get_offset(alignment: VerticalAlignment, content_height: f32, max_height: f32) f32 {
    return alignment.getVerticalOffset(content_height, max_height);
}

// SDF exports
export fn sdf_params_default() SDFParams {
    return SDFParams.default();
}

export fn calculate_sdf_value(distance: f32, params: SDFParams) f32 {
    return calculateSDFValue(distance, params);
}

// Word wrap exports
export fn wrap_text(text_ptr: [*]const u8, text_len: usize, max_width: f32, font_size: f32, mode: WordWrapMode) usize {
    const text_slice = text_ptr[0..text_len];
    const wrapped = wrapText(text_slice, max_width, font_size, mode);
    return wrapped.len;
}
