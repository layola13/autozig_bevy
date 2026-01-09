use autozig_sprite::*;
use autozig_color::Color;
use autozig_math::{Vec2, Vec3};

#[test]
fn test_sprite_default() {
    let sprite = Sprite::default();
    assert_eq!(sprite.color, Color::WHITE);
    assert!(!sprite.flip_x);
    assert!(!sprite.flip_y);
    assert_eq!(sprite.custom_size, None);
}

#[test]
fn test_sprite_new() {
    let sprite = Sprite::new();
    assert_eq!(sprite.color, Color::WHITE);
    assert!(!sprite.flip_x);
    assert!(!sprite.flip_y);
}

#[test]
fn test_sprite_with_color() {
    let color = Color::rgba(1.0, 0.5, 0.25, 0.75);
    let sprite = Sprite::with_color(color);
    assert_eq!(sprite.color, color);
    assert!(!sprite.flip_x);
    assert!(!sprite.flip_y);
}

#[test]
fn test_sprite_flip_x() {
    let sprite = Sprite::new().flip_x();
    assert!(sprite.flip_x);
    assert!(!sprite.flip_y);
}

#[test]
fn test_sprite_flip_y() {
    let sprite = Sprite::new().flip_y();
    assert!(!sprite.flip_x);
    assert!(sprite.flip_y);
}

#[test]
fn test_sprite_flip_both() {
    let sprite = Sprite::new().flip_x().flip_y();
    assert!(sprite.flip_x);
    assert!(sprite.flip_y);
}

#[test]
fn test_sprite_with_custom_size() {
    let size = Vec2::new(100.0, 50.0);
    let sprite = Sprite::new().with_custom_size(size);
    assert_eq!(sprite.custom_size, Some(size));
}

#[test]
fn test_anchor_center() {
    let anchor = Anchor::CENTER;
    let vec = anchor.as_vec();
    assert_eq!(vec.x, 0.5);
    assert_eq!(vec.y, 0.5);
    assert!(!anchor.is_custom());
}

#[test]
fn test_anchor_bottom_left() {
    let anchor = Anchor::BOTTOM_LEFT;
    let vec = anchor.as_vec();
    assert_eq!(vec.x, 0.0);
    assert_eq!(vec.y, 0.0);
    assert!(!anchor.is_custom());
}

#[test]
fn test_anchor_top_right() {
    let anchor = Anchor::TOP_RIGHT;
    let vec = anchor.as_vec();
    assert_eq!(vec.x, 1.0);
    assert_eq!(vec.y, 1.0);
    assert!(!anchor.is_custom());
}

#[test]
fn test_anchor_bottom_center() {
    let anchor = Anchor::BOTTOM_CENTER;
    let vec = anchor.as_vec();
    assert_eq!(vec.x, 0.5);
    assert_eq!(vec.y, 0.0);
}

#[test]
fn test_anchor_top_center() {
    let anchor = Anchor::TOP_CENTER;
    let vec = anchor.as_vec();
    assert_eq!(vec.x, 0.5);
    assert_eq!(vec.y, 1.0);
}

#[test]
fn test_anchor_center_left() {
    let anchor = Anchor::CENTER_LEFT;
    let vec = anchor.as_vec();
    assert_eq!(vec.x, 0.0);
    assert_eq!(vec.y, 0.5);
}

#[test]
fn test_anchor_center_right() {
    let anchor = Anchor::CENTER_RIGHT;
    let vec = anchor.as_vec();
    assert_eq!(vec.x, 1.0);
    assert_eq!(vec.y, 0.5);
}

#[test]
fn test_anchor_custom() {
    let anchor = Anchor::custom(0.25, 0.75);
    let vec = anchor.as_vec();
    assert_eq!(vec.x, 0.25);
    assert_eq!(vec.y, 0.75);
    assert!(anchor.is_custom());
}

#[test]
fn test_texture_atlas_layout_new() {
    let layout = TextureAtlasLayout::new(Vec2::new(32.0, 32.0), 4, 4);
    assert_eq!(layout.tile_size, Vec2::new(32.0, 32.0));
    assert_eq!(layout.columns, 4);
    assert_eq!(layout.rows, 4);
    assert_eq!(layout.padding, None);
    assert_eq!(layout.offset, None);
}

#[test]
fn test_texture_atlas_layout_total_tiles() {
    let layout = TextureAtlasLayout::new(Vec2::new(32.0, 32.0), 4, 4);
    assert_eq!(layout.total_tiles(), 16);
}

#[test]
fn test_texture_atlas_layout_with_padding() {
    let layout = TextureAtlasLayout::new(Vec2::new(32.0, 32.0), 4, 4)
        .with_padding(Vec2::new(2.0, 2.0));
    assert_eq!(layout.padding, Some(Vec2::new(2.0, 2.0)));
}

#[test]
fn test_texture_atlas_layout_with_offset() {
    let layout = TextureAtlasLayout::new(Vec2::new(32.0, 32.0), 4, 4)
        .with_offset(Vec2::new(4.0, 4.0));
    assert_eq!(layout.offset, Some(Vec2::new(4.0, 4.0)));
}

#[test]
fn test_texture_atlas_new() {
    let layout = TextureAtlasLayout::new(Vec2::new(32.0, 32.0), 4, 4);
    let atlas = TextureAtlas::new(5, layout);
    assert_eq!(atlas.index, 5);
    assert_eq!(atlas.layout.columns, 4);
    assert_eq!(atlas.layout.rows, 4);
}

#[test]
fn test_texture_atlas_set_index() {
    let layout = TextureAtlasLayout::new(Vec2::new(32.0, 32.0), 4, 4);
    let mut atlas = TextureAtlas::new(0, layout);
    atlas.set_index(7);
    assert_eq!(atlas.index, 7);
}

#[test]
fn test_texture_atlas_calculate_uv() {
    let layout = TextureAtlasLayout::new(Vec2::new(32.0, 32.0), 4, 4);
    let atlas = TextureAtlas::new(0, layout);
    let texture_size = Vec2::new(128.0, 128.0);
    let uv = atlas.calculate_uv(texture_size);
    
    // First tile should be at (0, 0) to (32, 32) in a 128x128 texture
    assert_eq!(uv[0], 0.0); // u_min
    assert_eq!(uv[1], 0.0); // v_min
    assert_eq!(uv[2], 0.25); // u_max (32/128)
    assert_eq!(uv[3], 0.25); // v_max (32/128)
}

#[test]
fn test_texture_atlas_calculate_uv_with_index() {
    let layout = TextureAtlasLayout::new(Vec2::new(32.0, 32.0), 4, 4);
    let atlas = TextureAtlas::new(5, layout); // Column 1, Row 1
    let texture_size = Vec2::new(128.0, 128.0);
    let uv = atlas.calculate_uv(texture_size);
    
    // Index 5 is at column 1 (32px), row 1 (32px)
    assert_eq!(uv[0], 0.25); // u_min (32/128)
    assert_eq!(uv[1], 0.25); // v_min (32/128)
    assert_eq!(uv[2], 0.5);  // u_max (64/128)
    assert_eq!(uv[3], 0.5);  // v_max (64/128)
}

#[test]
fn test_color_packing() {
    let color = Color::rgba(1.0, 0.5, 0.25, 0.75);
    let packed = pack_sprite_color(color);
    let unpacked = unpack_sprite_color(packed);
    
    // Allow small floating point error (1/255 ≈ 0.004)
    assert!((unpacked.r - color.r).abs() < 0.01);
    assert!((unpacked.g - color.g).abs() < 0.01);
    assert!((unpacked.b - color.b).abs() < 0.01);
    assert!((unpacked.a - color.a).abs() < 0.01);
}

#[test]
fn test_color_packing_white() {
    let color = Color::WHITE;
    let packed = pack_sprite_color(color);
    assert_eq!(packed, 0xFFFFFFFF);
    let unpacked = unpack_sprite_color(packed);
    assert_eq!(unpacked.r, 1.0);
    assert_eq!(unpacked.g, 1.0);
    assert_eq!(unpacked.b, 1.0);
    assert_eq!(unpacked.a, 1.0);
}

#[test]
fn test_color_packing_black() {
    let color = Color::BLACK;
    let packed = pack_sprite_color(color);
    assert_eq!(packed, 0xFF000000);
    let unpacked = unpack_sprite_color(packed);
    assert_eq!(unpacked.r, 0.0);
    assert_eq!(unpacked.g, 0.0);
    assert_eq!(unpacked.b, 0.0);
    assert_eq!(unpacked.a, 1.0);
}

#[test]
fn test_color_packing_transparent() {
    let color = Color::TRANSPARENT;
    let packed = pack_sprite_color(color);
    assert_eq!(packed, 0x00000000);
    let unpacked = unpack_sprite_color(packed);
    assert_eq!(unpacked.r, 0.0);
    assert_eq!(unpacked.g, 0.0);
    assert_eq!(unpacked.b, 0.0);
    assert_eq!(unpacked.a, 0.0);
}

#[test]
fn test_sprite_batch_new() {
    let batch = SpriteBatch::new(1, 0.5);
    assert_eq!(batch.texture_id, 1);
    assert_eq!(batch.z_layer, 0.5);
    assert!(batch.is_empty());
    assert_eq!(batch.len(), 0);
}

#[test]
fn test_sprite_batch_add_instance() {
    let mut batch = SpriteBatch::new(1, 0.5);
    let instance = SpriteInstance {
        transform: [1.0; 16],
        uv_rect: [0.0, 0.0, 1.0, 1.0],
        color: 0xFFFFFFFF,
        flip_flags: 0,
    };
    
    batch.add_instance(instance);
    assert_eq!(batch.len(), 1);
    assert!(!batch.is_empty());
}

#[test]
fn test_sprite_batch_clear() {
    let mut batch = SpriteBatch::new(1, 0.5);
    let instance = SpriteInstance {
        transform: [1.0; 16],
        uv_rect: [0.0, 0.0, 1.0, 1.0],
        color: 0xFFFFFFFF,
        flip_flags: 0,
    };
    
    batch.add_instance(instance);
    assert_eq!(batch.len(), 1);
    
    batch.clear();
    assert!(batch.is_empty());
    assert_eq!(batch.len(), 0);
}

#[test]
fn test_sort_batches_by_z() {
    let mut batches = vec![
        SpriteBatch::new(1, 0.8),
        SpriteBatch::new(2, 0.2),
        SpriteBatch::new(3, 0.5),
        SpriteBatch::new(4, 0.1),
    ];
    
    sort_batches_by_z(&mut batches);
    
    assert_eq!(batches[0].z_layer, 0.1);
    assert_eq!(batches[1].z_layer, 0.2);
    assert_eq!(batches[2].z_layer, 0.5);
    assert_eq!(batches[3].z_layer, 0.8);
}

#[test]
fn test_sort_batches_empty() {
    let mut batches: Vec<SpriteBatch> = vec![];
    sort_batches_by_z(&mut batches);
    assert_eq!(batches.len(), 0);
}

#[test]
fn test_sort_batches_single() {
    let mut batches = vec![SpriteBatch::new(1, 0.5)];
    sort_batches_by_z(&mut batches);
    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].z_layer, 0.5);
}

#[test]
fn test_sprite_vertex_new() {
    let position = Vec3::new(1.0, 2.0, 3.0);
    let uv = Vec2::new(0.5, 0.5);
    let color = Color::RED;
    
    let vertex = SpriteVertex::new(position, uv, color);
    assert_eq!(vertex.position[0], 1.0);
    assert_eq!(vertex.position[1], 2.0);
    assert_eq!(vertex.position[2], 3.0);
    assert_eq!(vertex.uv[0], 0.5);
    assert_eq!(vertex.uv[1], 0.5);
}

// Comparison tests with bevy_sprite behavior
#[test]
fn test_bevy_sprite_comparison_default_anchor() {
    // In bevy_sprite, default anchor is Center
    let sprite = Sprite::default();
    let anchor_vec = sprite.anchor.as_vec();
    assert_eq!(anchor_vec, Vec2::new(0.5, 0.5));
}

#[test]
fn test_bevy_sprite_comparison_flip_independent() {
    // In bevy_sprite, flip_x and flip_y are independent
    let sprite1 = Sprite::new().flip_x();
    assert!(sprite1.flip_x);
    assert!(!sprite1.flip_y);
    
    let sprite2 = Sprite::new().flip_y();
    assert!(!sprite2.flip_x);
    assert!(sprite2.flip_y);
}

#[test]
fn test_bevy_sprite_comparison_atlas_grid() {
    // In bevy_sprite, TextureAtlas uses grid-based indexing
    let layout = TextureAtlasLayout::new(Vec2::new(16.0, 16.0), 8, 8);
    assert_eq!(layout.total_tiles(), 64);
    
    // Index 0 should be top-left
    let atlas0 = TextureAtlas::new(0, layout);
    let uv0 = atlas0.calculate_uv(Vec2::new(128.0, 128.0));
    assert_eq!(uv0[0], 0.0); // u_min
    assert_eq!(uv0[1], 0.0); // v_min
}

#[test]
fn test_bevy_sprite_comparison_color_tint() {
    // In bevy_sprite, sprites can be tinted with any color
    let red_sprite = Sprite::with_color(Color::RED);
    assert_eq!(red_sprite.color, Color::RED);
    
    let semi_transparent = Sprite::with_color(Color::rgba(1.0, 1.0, 1.0, 0.5));
    assert_eq!(semi_transparent.color.a, 0.5);
}