// Unified sprite module for autozig-sprite
// All functionality in a single file for include_zig! macro

const std = @import("std");

// ============================================================================
// Core Types
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
};

pub const Vec3 = extern struct {
    x: f32,
    y: f32,
    z: f32,

    pub fn new(x: f32, y: f32, z: f32) Vec3 {
        return .{ .x = x, .y = y, .z = z };
    }
};

// Anchor is now a simple struct with discriminant tag and optional custom values
// This avoids the union complexity that causes Zig compiler crashes
pub const Anchor = extern struct {
    tag: u8,
    custom_x: f32,
    custom_y: f32,

    pub const CENTER: u8 = 0;
    pub const BOTTOM_LEFT: u8 = 1;
    pub const BOTTOM_CENTER: u8 = 2;
    pub const BOTTOM_RIGHT: u8 = 3;
    pub const CENTER_LEFT: u8 = 4;
    pub const CENTER_RIGHT: u8 = 5;
    pub const TOP_LEFT: u8 = 6;
    pub const TOP_CENTER: u8 = 7;
    pub const TOP_RIGHT: u8 = 8;
    pub const CUSTOM: u8 = 9;

    pub fn center() Anchor {
        return .{ .tag = CENTER, .custom_x = 0.0, .custom_y = 0.0 };
    }

    pub fn custom(x: f32, y: f32) Anchor {
        return .{ .tag = CUSTOM, .custom_x = x, .custom_y = y };
    }

    pub fn asVec(self: Anchor) Vec2 {
        return switch (self.tag) {
            CENTER => Vec2.new(0.5, 0.5),
            BOTTOM_LEFT => Vec2.new(0.0, 0.0),
            BOTTOM_CENTER => Vec2.new(0.5, 0.0),
            BOTTOM_RIGHT => Vec2.new(1.0, 0.0),
            CENTER_LEFT => Vec2.new(0.0, 0.5),
            CENTER_RIGHT => Vec2.new(1.0, 0.5),
            TOP_LEFT => Vec2.new(0.0, 1.0),
            TOP_CENTER => Vec2.new(0.5, 1.0),
            TOP_RIGHT => Vec2.new(1.0, 1.0),
            CUSTOM => Vec2.new(self.custom_x, self.custom_y),
            else => Vec2.new(0.5, 0.5),
        };
    }

    pub fn isCustom(self: Anchor) bool {
        return self.tag == CUSTOM;
    }
};

pub const Sprite = extern struct {
    color: Color,
    flip_x: bool,
    flip_y: bool,
    custom_size: ?*const Vec2,
    anchor: Anchor,

    pub fn default() Sprite {
        return .{
            .color = Color{ .r = 1.0, .g = 1.0, .b = 1.0, .a = 1.0 },
            .flip_x = false,
            .flip_y = false,
            .custom_size = null,
            .anchor = Anchor.center(),
        };
    }

    pub fn new(color: Color, flip_x: bool, flip_y: bool) Sprite {
        return .{
            .color = color,
            .flip_x = flip_x,
            .flip_y = flip_y,
            .custom_size = null,
            .anchor = Anchor.center(),
        };
    }

    pub fn withAnchor(color: Color, anchor: Anchor) Sprite {
        return .{
            .color = color,
            .flip_x = false,
            .flip_y = false,
            .custom_size = null,
            .anchor = anchor,
        };
    }
};

pub const TextureAtlasLayout = extern struct {
    tile_size: Vec2,
    columns: usize,
    rows: usize,
    padding: ?*const Vec2,
    offset: ?*const Vec2,

    pub fn new(tile_size: Vec2, columns: usize, rows: usize) TextureAtlasLayout {
        return .{
            .tile_size = tile_size,
            .columns = columns,
            .rows = rows,
            .padding = null,
            .offset = null,
        };
    }
};

pub const TextureAtlas = extern struct {
    index: usize,
    layout: TextureAtlasLayout,

    pub fn new(index: usize, layout: TextureAtlasLayout) TextureAtlas {
        return .{
            .index = index,
            .layout = layout,
        };
    }

    pub fn calculateUV(self: TextureAtlas, texture_size: Vec2) [4]f32 {
        const layout = self.layout;
        const col = self.index % layout.columns;
        const row = self.index / layout.columns;

        const padding = if (layout.padding) |p| p.* else Vec2.zero();
        const offset = if (layout.offset) |o| o.* else Vec2.zero();

        const x = offset.x + @as(f32, @floatFromInt(col)) * (layout.tile_size.x + padding.x);
        const y = offset.y + @as(f32, @floatFromInt(row)) * (layout.tile_size.y + padding.y);

        const u_min = x / texture_size.x;
        const v_min = y / texture_size.y;
        const u_max = (x + layout.tile_size.x) / texture_size.x;
        const v_max = (y + layout.tile_size.y) / texture_size.y;

        return [4]f32{ u_min, v_min, u_max, v_max };
    }
};

pub const SpriteVertex = extern struct {
    position: [3]f32,
    uv: [2]f32,
    color: u32,

    pub fn new(position: Vec3, uv: Vec2, color: u32) SpriteVertex {
        return .{
            .position = [3]f32{ position.x, position.y, position.z },
            .uv = [2]f32{ uv.x, uv.y },
            .color = color,
        };
    }
};

pub const SpriteInstance = extern struct {
    transform: [16]f32,
    uv_rect: [4]f32,
    color: u32,
    flip_flags: u32,
};

pub const SpriteBatch = extern struct {
    texture_id: u32,
    z_layer: f32,
    instances_ptr: ?[*]SpriteInstance,
    instances_len: usize,
    instances_cap: usize,

    pub fn new(texture_id: u32, z_layer: f32) SpriteBatch {
        return .{
            .texture_id = texture_id,
            .z_layer = z_layer,
            .instances_ptr = null,
            .instances_len = 0,
            .instances_cap = 0,
        };
    }
};

// ============================================================================
// Color Functions
// ============================================================================

pub fn packColor(color: Color) u32 {
    const r = @as(u32, @intFromFloat(@min(@max(color.r, 0.0), 1.0) * 255.0));
    const g = @as(u32, @intFromFloat(@min(@max(color.g, 0.0), 1.0) * 255.0));
    const b = @as(u32, @intFromFloat(@min(@max(color.b, 0.0), 1.0) * 255.0));
    const a = @as(u32, @intFromFloat(@min(@max(color.a, 0.0), 1.0) * 255.0));

    return (a << 24) | (b << 16) | (g << 8) | r;
}

fn unpack_color_impl(p: u32) Color {
    const r: f32 = @as(f32, @floatFromInt(p & 0xFF)) / 255.0;
    const g: f32 = @as(f32, @floatFromInt((p >> 8) & 0xFF)) / 255.0;
    const b: f32 = @as(f32, @floatFromInt((p >> 16) & 0xFF)) / 255.0;
    const a: f32 = @as(f32, @floatFromInt((p >> 24) & 0xFF)) / 255.0;
    return Color{ .r = r, .g = g, .b = b, .a = a };
}

pub fn unpackColor(p: u32) Color {
    return unpack_color_impl(p);
}

// ============================================================================
// Sprite Quad Generation
// ============================================================================

pub fn createSpriteQuad(sprite: Sprite, size: Vec2, anchor_offset: Vec2) [4]SpriteVertex {
    const uv_rect = [4]f32{ 0.0, 0.0, 1.0, 1.0 };
    return createSpriteQuadWithUV(sprite, size, anchor_offset, uv_rect);
}

pub fn createSpriteQuadWithUV(sprite: Sprite, size: Vec2, anchor_offset: Vec2, uv_rect: [4]f32) [4]SpriteVertex {
    const anchor_vec = sprite.anchor.asVec();
    const offset_x = -size.x * anchor_vec.x + anchor_offset.x;
    const offset_y = -size.y * anchor_vec.y + anchor_offset.y;

    const packed_color = packColor(sprite.color);

    var u_min = uv_rect[0];
    var v_min = uv_rect[1];
    var u_max = uv_rect[2];
    var v_max = uv_rect[3];

    if (sprite.flip_x) {
        const temp = u_min;
        u_min = u_max;
        u_max = temp;
    }
    if (sprite.flip_y) {
        const temp = v_min;
        v_min = v_max;
        v_max = temp;
    }

    return [4]SpriteVertex{
        SpriteVertex.new(
            Vec3.new(offset_x, offset_y, 0.0),
            Vec2.new(u_min, v_max),
            packed_color,
        ),
        SpriteVertex.new(
            Vec3.new(offset_x + size.x, offset_y, 0.0),
            Vec2.new(u_max, v_max),
            packed_color,
        ),
        SpriteVertex.new(
            Vec3.new(offset_x + size.x, offset_y + size.y, 0.0),
            Vec2.new(u_max, v_min),
            packed_color,
        ),
        SpriteVertex.new(
            Vec3.new(offset_x, offset_y + size.y, 0.0),
            Vec2.new(u_min, v_min),
            packed_color,
        ),
    };
}

// ============================================================================
// Batch Functions
// ============================================================================

pub fn sortBatchesByZ(batches: [*]SpriteBatch, count: usize) void {
    if (count <= 1) return;

    var i: usize = 0;
    while (i < count - 1) : (i += 1) {
        var j: usize = 0;
        while (j < count - i - 1) : (j += 1) {
            if (batches[j].z_layer > batches[j + 1].z_layer) {
                const temp = batches[j];
                batches[j] = batches[j + 1];
                batches[j + 1] = temp;
            }
        }
    }
}

// ============================================================================
// FFI Exports
// ============================================================================

export fn sprite_default() Sprite {
    return Sprite.default();
}

export fn sprite_new(color: Color, flip_x: bool, flip_y: bool) Sprite {
    return Sprite.new(color, flip_x, flip_y);
}

export fn sprite_with_anchor(color: Color, anchor: Anchor) Sprite {
    return Sprite.withAnchor(color, anchor);
}

export fn anchor_as_vec(anchor: Anchor) Vec2 {
    return anchor.asVec();
}

export fn anchor_is_custom(anchor: Anchor) bool {
    return anchor.isCustom();
}

export fn atlas_new(index: usize, layout: TextureAtlasLayout) TextureAtlas {
    return TextureAtlas.new(index, layout);
}

export fn atlas_layout_new(tile_size: Vec2, columns: usize, rows: usize) TextureAtlasLayout {
    return TextureAtlasLayout.new(tile_size, columns, rows);
}

export fn atlas_calculate_uv(atlas: TextureAtlas, texture_size: Vec2, out: *[4]f32) void {
    out.* = atlas.calculateUV(texture_size);
}

export fn sprite_create_quad(sprite: Sprite, size: Vec2, anchor_offset: Vec2, out: *[4]SpriteVertex) void {
    out.* = createSpriteQuad(sprite, size, anchor_offset);
}

export fn sprite_create_quad_with_uv(sprite: Sprite, size: Vec2, anchor_offset: Vec2, uv_rect: *const [4]f32, out: *[4]SpriteVertex) void {
    out.* = createSpriteQuadWithUV(sprite, size, anchor_offset, uv_rect.*);
}

export fn pack_color(color: Color) u32 {
    return packColor(color);
}

export fn unpack_color(packed_value: u32) Color {
    return unpackColor(packed_value);
}

export fn batch_new(texture_id: u32, z_layer: f32) SpriteBatch {
    return SpriteBatch.new(texture_id, z_layer);
}

export fn batch_sort_by_z(batches: [*]SpriteBatch, count: usize) void {
    sortBatchesByZ(batches, count);
}
