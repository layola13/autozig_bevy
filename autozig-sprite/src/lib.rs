use autozig::include_zig;
use autozig_color::Color;
use autozig_math::{Rect, Vec2, Vec3};

/// Sprite component for 2D rendering
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
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
    /// Rectangle representing the region of the sprite's image to render
    pub rect: Option<Rect>,
    /// How the sprite's image will be scaled
    pub image_mode: SpriteImageMode,
}

/// Anchor point for sprite positioning
///
/// Normalized offset from the center of a 2D renderable entity.
/// The value is a Vec2 where:
/// - (0.0, 0.0) represents the center
/// - (-0.5, -0.5) represents bottom-left
/// - (0.5, 0.5) represents top-right
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Anchor(pub Vec2);

impl Anchor {
    pub const BOTTOM_LEFT: Self = Self(Vec2 { x: -0.5, y: -0.5 });
    pub const BOTTOM_CENTER: Self = Self(Vec2 { x: 0.0, y: -0.5 });
    pub const BOTTOM_RIGHT: Self = Self(Vec2 { x: 0.5, y: -0.5 });
    pub const CENTER_LEFT: Self = Self(Vec2 { x: -0.5, y: 0.0 });
    pub const CENTER: Self = Self(Vec2::ZERO);
    pub const CENTER_RIGHT: Self = Self(Vec2 { x: 0.5, y: 0.0 });
    pub const TOP_LEFT: Self = Self(Vec2 { x: -0.5, y: 0.5 });
    pub const TOP_CENTER: Self = Self(Vec2 { x: 0.0, y: 0.5 });
    pub const TOP_RIGHT: Self = Self(Vec2 { x: 0.5, y: 0.5 });

    pub fn as_vec(&self) -> Vec2 {
        self.0
    }

    pub fn custom(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Self::CENTER
    }
}

impl From<Vec2> for Anchor {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
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

/// Controls how the sprite's image is altered when scaled
#[derive(Debug, Clone, PartialEq)]
pub enum SpriteImageMode {
    /// The sprite will take on the size of the image by default,
    /// and will be stretched or shrunk if custom_size is set
    Auto,
    /// The texture will be scaled to fit the rect bounds
    Scale(SpriteScalingMode),
    /// The texture will be cut in 9 slices, keeping proportions on resize
    Sliced(TextureSlicer),
    /// The texture will be repeated if stretched beyond stretch_value
    Tiled {
        /// Should the image repeat horizontally
        tile_x: bool,
        /// Should the image repeat vertically
        tile_y: bool,
        /// The texture will repeat when the ratio between drawing dimensions
        /// and original texture size are above this value
        stretch_value: f32,
    },
}

impl Default for SpriteImageMode {
    fn default() -> Self {
        Self::Auto
    }
}

impl SpriteImageMode {
    /// Returns true if this mode uses slices internally
    pub fn uses_slices(&self) -> bool {
        matches!(self, Self::Sliced(..) | Self::Tiled { .. })
    }

    /// Returns SpriteScalingMode if scale is present
    pub fn scale(&self) -> Option<SpriteScalingMode> {
        if let Self::Scale(scale) = self {
            Some(*scale)
        } else {
            None
        }
    }
}

/// Represents various modes for proportional scaling of a texture
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpriteScalingMode {
    /// Scale uniformly to fill and center, maintaining aspect ratio
    FillCenter,
    /// Scale to fill, align overflow to start (left/top)
    FillStart,
    /// Scale to fill, align overflow to end (right/bottom)
    FillEnd,
    /// Scale to fit entirely inside, center aligned
    FitCenter,
    /// Scale to fit entirely inside, align to start (left/top)
    FitStart,
    /// Scale to fit entirely inside, align to end (right/bottom)
    FitEnd,
}

impl Default for SpriteScalingMode {
    fn default() -> Self {
        Self::FillCenter
    }
}

/// Border rectangle defining insets for 9-slice scaling
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderRect {
    /// Inset applied to the rectangle's minimum corner (left, bottom)
    pub min_inset: Vec2,
    /// Inset applied to the rectangle's maximum corner (right, top)
    pub max_inset: Vec2,
}

impl Default for BorderRect {
    fn default() -> Self {
        Self::ZERO
    }
}

impl BorderRect {
    /// An empty border with zero thickness along each edge
    pub const ZERO: Self = Self {
        min_inset: Vec2::ZERO,
        max_inset: Vec2::ZERO,
    };

    /// Creates a border with the same inset along each edge
    pub fn all(inset: f32) -> Self {
        Self {
            min_inset: Vec2::splat(inset),
            max_inset: Vec2::splat(inset),
        }
    }

    /// Creates a border with horizontal and vertical insets
    pub fn axes(horizontal: f32, vertical: f32) -> Self {
        let insets = Vec2::new(horizontal, vertical);
        Self {
            min_inset: insets,
            max_inset: insets,
        }
    }
}

impl From<f32> for BorderRect {
    fn from(inset: f32) -> Self {
        Self::all(inset)
    }
}

impl From<[f32; 4]> for BorderRect {
    fn from([min_x, max_x, min_y, max_y]: [f32; 4]) -> Self {
        Self {
            min_inset: Vec2::new(min_x, min_y),
            max_inset: Vec2::new(max_x, max_y),
        }
    }
}

/// Defines how a texture slice scales when resized
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SliceScaleMode {
    /// The slice will be stretched to fit the area
    Stretch,
    /// The slice will be tiled to fit the area
    Tile {
        /// The slice will repeat when the ratio between drawing dimensions
        /// and original texture size are above this value
        stretch_value: f32,
    },
}

impl Default for SliceScaleMode {
    fn default() -> Self {
        Self::Stretch
    }
}

/// Single texture slice for 9-slice rendering
#[derive(Debug, Clone, PartialEq)]
pub struct TextureSlice {
    /// Texture area to draw
    pub texture_rect: Rect,
    /// Slice draw size
    pub draw_size: Vec2,
    /// Offset of the slice
    pub offset: Vec2,
}

/// Slices a texture using 9-slicing technique
#[derive(Debug, Clone, PartialEq)]
pub struct TextureSlicer {
    /// Border insets in pixels defining the nine slicing sections
    pub border: BorderRect,
    /// How the center part scales
    pub center_scale_mode: SliceScaleMode,
    /// How the side parts scale
    pub sides_scale_mode: SliceScaleMode,
    /// Maximum scale of corner slices (default 1.0)
    pub max_corner_scale: f32,
}

impl Default for TextureSlicer {
    fn default() -> Self {
        Self {
            border: BorderRect::default(),
            center_scale_mode: SliceScaleMode::default(),
            sides_scale_mode: SliceScaleMode::default(),
            max_corner_scale: 1.0,
        }
    }
}

impl TextureSlicer {
    /// Creates a new TextureSlicer with the given border
    pub fn new(border: BorderRect) -> Self {
        Self {
            border,
            ..Default::default()
        }
    }

    /// Sets the center scale mode
    pub fn with_center_scale_mode(mut self, mode: SliceScaleMode) -> Self {
        self.center_scale_mode = mode;
        self
    }

    /// Sets the sides scale mode
    pub fn with_sides_scale_mode(mut self, mode: SliceScaleMode) -> Self {
        self.sides_scale_mode = mode;
        self
    }

    /// Sets the maximum corner scale
    pub fn with_max_corner_scale(mut self, scale: f32) -> Self {
        self.max_corner_scale = scale;
        self
    }
}

/// 2D text component
#[repr(C)]
#[derive(Debug, Clone, PartialEq)]
pub struct Text2d {
    /// The text string to display
    pub text: String,
    /// Font size
    pub font_size: f32,
    /// Text color
    pub color: Color,
}

impl Text2d {
    /// Creates new 2D text
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: 12.0,
            color: Color::WHITE,
        }
    }

    /// Sets the font size
    pub fn with_font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Sets the color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Default for Text2d {
    fn default() -> Self {
        Self::new("")
    }
}

impl From<&str> for Text2d {
    fn from(text: &str) -> Self {
        Self::new(text)
    }
}

impl From<String> for Text2d {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

/// Shadow effect for 2D text
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Text2dShadow {
    /// Shadow displacement offset
    pub offset: Vec2,
    /// Shadow color
    pub color: Color,
}

impl Default for Text2dShadow {
    fn default() -> Self {
        Self {
            offset: Vec2::new(4.0, -4.0),
            color: Color::BLACK,
        }
    }
}

impl Text2dShadow {
    /// Creates a new text shadow with the given offset
    pub fn new(offset: Vec2) -> Self {
        Self {
            offset,
            color: Color::BLACK,
        }
    }

    /// Sets the shadow color
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

/// Sprite picking mode determining how transparent pixels are handled
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpritePickingMode {
    /// Consider the entire bounding box, ignoring transparency
    BoundingBox,
    /// Only consider pixels with alpha above the threshold (inclusive)
    AlphaThreshold(f32),
}

impl Default for SpritePickingMode {
    fn default() -> Self {
        Self::AlphaThreshold(0.1)
    }
}

/// Camera marker for sprite picking
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SpritePickingCamera;

/// Settings for sprite picking behavior
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpritePickingSettings {
    /// When true, only cameras with SpritePickingCamera component are used
    pub require_markers: bool,
    /// How transparent pixels are handled during picking
    pub picking_mode: SpritePickingMode,
}

impl Default for SpritePickingSettings {
    fn default() -> Self {
        Self {
            require_markers: false,
            picking_mode: SpritePickingMode::default(),
        }
    }
}

/// Plugin marker for sprite picking functionality
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SpritePickingPlugin;

/// Main sprite rendering plugin
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct SpritePlugin;

/// System set labels for sprite rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpriteSystems {
    /// Extract sprites for rendering
    ExtractSprites,
    /// Compute texture slices
    ComputeSlices,
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
    fn atlas_calculate_uv(atlas: TextureAtlas, texture_size: Vec2, out: *mut [f32; 4]) -> ();
    
    // Sprite vertex generation
    fn sprite_create_quad(sprite: Sprite, size: Vec2, anchor_offset: Vec2, out: *mut [SpriteVertex; 4]) -> ();
    fn sprite_create_quad_with_uv(sprite: Sprite, size: Vec2, anchor_offset: Vec2, uv_rect: *const [f32; 4], out: *mut [SpriteVertex; 4]) -> ();
    
    // Color packing for GPU
    fn pack_color(color: Color) -> u32;
    fn unpack_color(packed: u32) -> Color;
    
    // Batch operations
    fn batch_new(texture_id: u32, z_layer: f32) -> SpriteBatch;
    fn batch_sort_by_z(batches: *mut SpriteBatch, count: usize) -> ();
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


impl TextureAtlas {
    pub fn new(index: usize, layout: TextureAtlasLayout) -> Self {
        atlas_new(index, layout)
    }

    pub fn calculate_uv(&self, texture_size: Vec2) -> [f32; 4] {
        let mut out = [0.0f32; 4];
        atlas_calculate_uv(*self, texture_size, &mut out);
        out
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