const std = @import("std");
const sprite = @import("sprite.zig");

pub const Color = sprite.Color;
pub const Vec2 = sprite.Vec2;
pub const Sprite = sprite.Sprite;
pub const SpriteVertex = sprite.SpriteVertex;

// SpriteInstance for batch rendering
pub const SpriteInstance = extern struct {
    transform: [16]f32,
    uv_rect: [4]f32,
    color: u32,
    flip_flags: u32,
};

// SpriteBatch for efficient rendering
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

// Sort batches by Z-layer (bubble sort for simplicity, stable)
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

// FFI exports
export fn batch_new(texture_id: u32, z_layer: f32) SpriteBatch {
    return SpriteBatch.new(texture_id, z_layer);
}

export fn batch_sort_by_z(batches: [*]SpriteBatch, count: usize) void {
    sortBatchesByZ(batches, count);
}
