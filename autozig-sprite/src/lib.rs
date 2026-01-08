use autozig::include_zig;
use autozig_color::Color;
use autozig_math::{Vec2, Vec3};

/// Sprite component for 2D rendering
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sprite {
    /// Color tint for the sprite
    pub color: Color,
    /// Whether to flip the sprite horizontally
    pub flip_x: bool,
    /// Whether to flip the sprite vertically
    pub flip_y: bool,
    /// Custom size override (None = use texture size)
    pub custom_size: Option<Vec2>,
    /// Anchor point for sprite positioning
    pub anchor: Anchor,
}

/// Anchor point for sprite positioning
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Anchor {
    /// Center of the sprite
    Center,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-center
    BottomCenter,
    /// Bottom-right corner
    BottomRight,
    /// Center-left
    CenterLeft,
    /// Center-right
    CenterRight,
    /// Top-left corner
    TopLeft,
    /// Top-center
    TopCenter,
    /// Top-right corner
    TopRight,
    /// Custom anchor point (normalized coordinates: 0.0 = left/bottom, 1.0 = right/top)
    Custom(Vec2),
}

/// Texture atlas for sprite sheets and animations
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureAtlas {
    /// Index of the current sprite in the atlas
    pub index: usize,
    /// Layout information for the atlas
    pub layout: TextureAtlasLayout,
}

/// Layout information for a texture atlas
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureAtlasLayout {
    /// Size of each tile in the atlas
    pub tile_size: Vec2,
    /// Number of columns in the atlas
    pub columns: usize,
    /// Number of rows in the atlas
    pub rows: usize,
    /// Padding between tiles
    pub padding: Option<Vec2>,
    /// Offset from the top-left corner
    pub offset: Option<Vec2>,
}

/// Vertex data for sprite rendering
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpriteVertex {
    /// Position in 3D space (z for layer ordering)
    pub position: [f32; 3],
    /// UV coordinates for texture sampling
    pub uv: [f32; 2],
    /// Color tint (packed as RGBA8)
    pub color: u32,
}

/// Instance data for batch rendering
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpriteInstance {
    /// Transform matrix (position, rotation, scale)
    pub transform: [f32; 16],
    /// UV bounds for texture atlas
    pub uv_rect: [f32; 4],
    /// Color tint
    pub color: u32,
    /// Flip flags (x in bit 0, y in bit 1)
    pub flip_flags: u32,
}

/// Sprite batch for efficient rendering
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SpriteBatch {
    /// Texture handle (simplified for now)
    pub texture_id: u32,
    /// Z-layer for sorting
    pub z_layer: f32,
    /// Instances in this batch
    pub instances: Vec<SpriteInstance>,
}

// Include Zig sprite implementation
include_zig!("zig/sprite_all.zig", {
    // Sprite creation and manipulation
    fn sprite_default() -> Sprite;
    fn sprite_new(color: Color, flip_x: bool, flip_y: bool) -> Sprite;
    fn sprite_with_anchor(color: Color, anchor: Anchor) -> Sprite;
    
    // Anchor calculations
    fn anchor_as_vec(anchor: Anchor) -> Vec2;
    fn anchor_is_custom(anchor: Anchor) -> bool;
    
    // Texture atlas operations
    fn atlas_new(index: usize, layout: TextureAtlasLayout) -> TextureAtlas;
    fn atlas_layout_new(tile_size: Vec2, columns: usize, rows: usize) -> TextureAtlasLayout;
    fn atlas_calculate_uv(atlas: TextureAtlas, texture_size: Vec2) -> [f32; 4];
    
    // Sprite vertex generation
    fn sprite_create_quad(sprite: Sprite, size: Vec2, anchor_offset: Vec2) -> [SpriteVertex; 4];
    fn sprite_create_quad_with_uv(sprite: Sprite, size: Vec2, anchor_offset: Vec2, uv_rect: [f32; 4]) -> [SpriteVertex; 4];
    
    // Color packing for GPU
    fn pack_color(color: Color) -> u32;
    fn unpack_color(packed: u32) -> Color;
    
    // Batch operations
    fn batch_new(texture_id: u32, z_layer: f32) -> SpriteBatch;
    fn batch_sort_by_z(batches: *mut SpriteBatch, count: usize) void;
});

impl Default for Sprite {
    fn default() -> Self {
        sprite_default()
    }
}

impl Sprite {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_color(color: Color) -> Self {
        sprite_new(color, false, false)
    }

    pub fn with_anchor(anchor: Anchor) -> Self {
        sprite_with_anchor(Color::WHITE, anchor)
    }

    pub fn flip_x(mut self) -> Self {
        self.flip_x = !self.flip_x;
        self
    }

    pub fn flip_y(mut self) -> Self {
        self.flip_y = !self.flip_y;
        self
    }

    pub fn with_custom_size(mut self, size: Vec2) -> Self {
        self.custom_size = Some(size);
        self
    }
}

impl Anchor {
    pub const CENTER: Self = Self::Center;
    pub const BOTTOM_LEFT: Self = Self::BottomLeft;
    pub const BOTTOM_CENTER: Self = Self::BottomCenter;
    pub const BOTTOM_RIGHT: Self = Self::BottomRight;
    pub const CENTER_LEFT: Self = Self::CenterLeft;
    pub const CENTER_RIGHT: Self = Self::CenterRight;
    pub const TOP_LEFT: Self = Self::TopLeft;
    pub const TOP_CENTER: Self = Self::TopCenter;
    pub const TOP_RIGHT: Self = Self::TopRight;

    pub fn as_vec(&self) -> Vec2 {
        anchor_as_vec(*self)
    }

    pub fn is_custom(&self) -> bool {
        anchor_is_custom(*self)
    }

    pub fn custom(x: f32, y: f32) -> Self {
        Self::Custom(Vec2::new(x, y))
    }
}

impl TextureAtlas {
    pub fn new(index: usize, layout: TextureAtlasLayout) -> Self {
        atlas_new(index, layout)
    }

    pub fn calculate_uv(&self, texture_size: Vec2) -> [f32; 4] {
        atlas_calculate_uv(*self, texture_size)
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl TextureAtlasLayout {
    pub fn new(tile_size: Vec2, columns: usize, rows: usize) -> Self {
        atlas_layout_new(tile_size, columns, rows)
    }

    pub fn with_padding(mut self, padding: Vec2) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn total_tiles(&self) -> usize {
        self.columns * self.rows
    }
}

impl SpriteVertex {
    pub fn new(position: Vec3, uv: Vec2, color: Color) -> Self {
        Self {
            position: [position.x, position.y, position.z],
            uv: [uv.x, uv.y],
            color: pack_color(color),
        }
    }
}

impl SpriteBatch {
    pub fn new(texture_id: u32, z_layer: f32) -> Self {
        batch_new(texture_id, z_layer)
    }

    pub fn add_instance(&mut self, instance: SpriteInstance) {
        self.instances.push(instance);
    }

    pub fn clear(&mut self) {
        self.instances.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }
}

/// Sort batches by Z-layer for proper rendering order
pub fn sort_batches_by_z(batches: &mut [SpriteBatch]) {
    if batches.is_empty() {
        return;
    }
    batch_sort_by_z(batches.as_mut_ptr(), batches.len());
}

/// Pack color into u32 for GPU (RGBA8)
pub fn pack_sprite_color(color: Color) -> u32 {
    pack_color(color)
}

/// Unpack u32 color from GPU to Color
pub fn unpack_sprite_color(packed: u32) -> Color {
    unpack_color(packed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sprite_creation() {
        let sprite = Sprite::new();
        assert_eq!(sprite.color, Color::WHITE);
        assert!(!sprite.flip_x);
        assert!(!sprite.flip_y);
    }

    #[test]
    fn test_sprite_flip() {
        let sprite = Sprite::new().flip_x().flip_y();
        assert!(sprite.flip_x);
        assert!(sprite.flip_y);
    }

    #[test]
    fn test_anchor_positions() {
        assert_eq!(Anchor::CENTER.as_vec(), Vec2::new(0.5, 0.5));
        assert_eq!(Anchor::BOTTOM_LEFT.as_vec(), Vec2::new(0.0, 0.0));
        assert_eq!(Anchor::TOP_RIGHT.as_vec(), Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_texture_atlas() {
        let layout = TextureAtlasLayout::new(Vec2::new(32.0, 32.0), 4, 4);
        assert_eq!(layout.total_tiles(), 16);
        
        let atlas = TextureAtlas::new(0, layout);
        assert_eq!(atlas.index, 0);
    }

    #[test]
    fn test_color_packing() {
        let color = Color::rgba(1.0, 0.5, 0.25, 0.75);
        let packed = pack_sprite_color(color);
        let unpacked = unpack_sprite_color(packed);
        
        // Allow small floating point error
        assert!((unpacked.r - color.r).abs() < 0.01);
        assert!((unpacked.g - color.g).abs() < 0.01);
        assert!((unpacked.b - color.b).abs() < 0.01);
        assert!((unpacked.a - color.a).abs() < 0.01);
    }
}