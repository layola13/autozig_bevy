use autozig_sprite::*;
use autozig_math::Vec2;

#[test]
fn test_texture_atlas_calculate_uv() {
    let layout = TextureAtlasLayout::new(Vec2::new(32.0, 32.0), 4, 4);
    let atlas = TextureAtlas::new(0, layout);
    let texture_size = Vec2::new(128.0, 128.0);
    
    let uv = atlas.calculate_uv(texture_size);
    
    // First tile (0,0) should have UV coordinates [0.0, 0.0, 0.25, 0.25]
    assert_eq!(uv[0], 0.0, "min_x should be 0.0");
    assert_eq!(uv[1], 0.0, "min_y should be 0.0");
    assert_eq!(uv[2], 0.25, "max_x should be 0.25");
    assert_eq!(uv[3], 0.25, "max_y should be 0.25");
}

#[test]
fn test_sprite_create_quad() {
    let sprite = Sprite::new();
    let size = Vec2::new(100.0, 100.0);
    let anchor_offset = Vec2::new(0.0, 0.0);
    
    let quad = sprite_create_quad(sprite, size, anchor_offset);
    
    // Should create 4 vertices
    assert_eq!(quad.len(), 4);
    
    // Check that vertices have valid positions (not NaN)
    for vertex in &quad {
        assert!(!vertex.position[0].is_nan(), "Vertex X should not be NaN");
        assert!(!vertex.position[1].is_nan(), "Vertex Y should not be NaN");
        assert!(!vertex.position[2].is_nan(), "Vertex Z should not be NaN");
    }
}