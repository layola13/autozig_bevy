use autozig::include_zig;
use autozig_color::Color;
use autozig_math::Vec2;

/// Text alignment options
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left = 0,
    Center = 1,
    Right = 2,
    Justified = 3,
}

/// Vertical alignment options
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    Top = 0,
    Middle = 1,
    Bottom = 2,
}

/// Word wrap mode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordWrapMode {
    NoWrap = 0,
    WordWrap = 1,
    CharacterWrap = 2,
}

/// Font handle for referencing loaded fonts
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FontHandle {
    pub id: u32,
}

/// Glyph identifier
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlyphId {
    pub value: u32,
}

/// Font metrics information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FontMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
    pub units_per_em: f32,
}

/// Glyph metrics information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlyphMetrics {
    pub advance_width: f32,
    pub advance_height: f32,
    pub bearing_x: f32,
    pub bearing_y: f32,
    pub width: f32,
    pub height: f32,
}

/// Rectangle type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

/// Glyph atlas entry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlyphAtlasEntry {
    pub glyph_id: GlyphId,
    pub uv_rect: Rect,
    pub metrics: GlyphMetrics,
    pub texture_index: u32,
}

/// Glyph atlas for managing glyph textures
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlyphAtlas {
    pub texture_size: Vec2,
    pub current_x: f32,
    pub current_y: f32,
    pub row_height: f32,
    pub padding: f32,
}

/// Text component
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Text {
    content_ptr: *const u8,
    content_len: usize,
    pub font: FontHandle,
    pub font_size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub vertical_alignment: VerticalAlignment,
    pub line_height_factor: f32,
    pub letter_spacing: f32,
    pub word_spacing: f32,
}

/// Line information for text layout
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LineInfo {
    pub start_index: usize,
    pub end_index: usize,
    pub width: f32,
    pub y_offset: f32,
}

/// Text layout information
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextLayout {
    lines_ptr: *mut LineInfo,
    lines_len: usize,
    lines_cap: usize,
    pub total_width: f32,
    pub total_height: f32,
    pub font_metrics: FontMetrics,
}

/// Text vertex for rendering
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub color: u32,
}

/// Glyph instance for rendering
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlyphInstance {
    pub position: Vec2,
    pub size: Vec2,
    pub uv_rect: Rect,
    pub color: u32,
}

/// Text bounds measurement result
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextBounds {
    pub width: f32,
    pub height: f32,
    pub line_count: usize,
}

/// SDF (Signed Distance Field) parameters
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SDFParams {
    pub spread: f32,
    pub smoothness: f32,
    pub threshold: f32,
}

// Include Zig text implementation
include_zig!("zig/text_all.zig", {
    // Text Component functions
    fn text_new(content_ptr: *const u8, content_len: usize, font: FontHandle, font_size: f32, color: Color) -> Text;
    fn text_with_alignment(text: Text, alignment: TextAlignment) -> Text;
    fn text_with_vertical_alignment(text: Text, vertical_alignment: VerticalAlignment) -> Text;
    fn text_with_line_height(text: Text, line_height_factor: f32) -> Text;
    
    // Font functions
    fn font_handle_new(id: u32) -> FontHandle;
    fn font_handle_is_valid(handle: FontHandle) -> bool;
    fn glyph_id_new(value: u32) -> GlyphId;
    fn font_metrics_line_height(metrics: FontMetrics) -> f32;
    fn font_metrics_scale(metrics: FontMetrics, font_size: f32) -> FontMetrics;
    fn glyph_metrics_new() -> GlyphMetrics;
    
    // Glyph Atlas functions
    fn glyph_atlas_new(texture_size: Vec2, padding: f32) -> GlyphAtlas;
    fn glyph_atlas_allocate(atlas: *mut GlyphAtlas, width: f32, height: f32, out_rect: *mut Rect) -> bool;
    fn glyph_atlas_reset(atlas: *mut GlyphAtlas) -> ();
    fn glyph_atlas_uv_rect(atlas: GlyphAtlas, pixel_rect: Rect) -> Rect;
    fn glyph_atlas_entry_new(glyph_id: GlyphId, uv_rect: Rect, metrics: GlyphMetrics, texture_index: u32) -> GlyphAtlasEntry;
    
    // Text Layout functions
    fn text_layout_new(font_metrics: FontMetrics) -> TextLayout;
    fn line_info_new(start_index: usize, end_index: usize, width: f32, y_offset: f32) -> LineInfo;
    
    // Text measurement functions
    fn text_measure(text_ptr: *const u8, text_len: usize, font_size: f32, font_metrics: FontMetrics, max_width_ptr: *const f32) -> TextBounds;
    fn text_bounds_new(width: f32, height: f32, line_count: usize) -> TextBounds;
    
    // Text vertex functions
    fn text_vertex_new(position: *const [f32; 3], uv: Vec2, color: u32) -> TextVertex;
    fn glyph_instance_new(position: Vec2, size: Vec2, uv_rect: Rect, color: u32) -> GlyphInstance;
    fn create_glyph_quad(instance: GlyphInstance) -> [TextVertex; 4];
    
    // Color packing functions
    fn pack_color(color: Color) -> u32;
    fn unpack_color(packed: u32) -> Color;
    
    // Note: rect_new, rect_width, rect_height, rect_size, vec2_new, vec2_zero,
    // vec2_add, vec2_scale are provided by autozig-math
    
    // Alignment functions
    fn text_alignment_get_offset(alignment: TextAlignment, line_width: f32, max_width: f32) -> f32;
    fn vertical_alignment_get_offset(alignment: VerticalAlignment, content_height: f32, max_height: f32) -> f32;
    
    // SDF functions
    fn sdf_params_default() -> SDFParams;
    fn calculate_sdf_value(distance: f32, params: SDFParams) -> f32;
    
    // Word wrap functions
    fn wrap_text(text_ptr: *const u8, text_len: usize, max_width: f32, font_size: f32, mode: WordWrapMode) -> usize;
});

// ============================================================================
// Rust API Implementations
// ============================================================================

impl FontHandle {
    pub fn new(id: u32) -> Self {
        font_handle_new(id)
    }

    pub fn is_valid(&self) -> bool {
        font_handle_is_valid(*self)
    }

    pub const INVALID: Self = Self { id: 0 };
}

impl Default for FontHandle {
    fn default() -> Self {
        Self::INVALID
    }
}

impl GlyphId {
    pub fn new(value: u32) -> Self {
        glyph_id_new(value)
    }
}

impl FontMetrics {
    pub fn line_height(&self) -> f32 {
        font_metrics_line_height(*self)
    }

    pub fn scale(&self, font_size: f32) -> Self {
        font_metrics_scale(*self, font_size)
    }
}

impl Default for GlyphMetrics {
    fn default() -> Self {
        glyph_metrics_new()
    }
}

impl Rect {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }

    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        let half_size = Vec2::new(size.x * 0.5, size.y * 0.5);
        Self::new(
            Vec2::new(center.x - half_size.x, center.y - half_size.y),
            Vec2::new(center.x + half_size.x, center.y + half_size.y),
        )
    }
}

impl Text {
    pub fn new(content: &str, font: FontHandle, font_size: f32, color: Color) -> Self {
        text_new(
            content.as_ptr(),
            content.len(),
            font,
            font_size,
            color,
        )
    }

    pub fn with_alignment(self, alignment: TextAlignment) -> Self {
        text_with_alignment(self, alignment)
    }

    pub fn with_vertical_alignment(self, vertical_alignment: VerticalAlignment) -> Self {
        text_with_vertical_alignment(self, vertical_alignment)
    }

    pub fn with_line_height(self, line_height_factor: f32) -> Self {
        text_with_line_height(self, line_height_factor)
    }

    pub fn content(&self) -> &str {
        if self.content_ptr.is_null() {
            return "";
        }
        let slice = unsafe { core::slice::from_raw_parts(self.content_ptr, self.content_len) };
        core::str::from_utf8(slice).unwrap_or("")
    }

    pub fn set_content(&mut self, content: &str) {
        self.content_ptr = content.as_ptr();
        self.content_len = content.len();
    }
}

impl GlyphAtlas {
    pub fn new(texture_size: Vec2, padding: f32) -> Self {
        glyph_atlas_new(texture_size, padding)
    }

    pub fn allocate(&mut self, width: f32, height: f32) -> Option<Rect> {
        let mut rect = Rect::new(Vec2::splat(0.0), Vec2::splat(0.0));
        if glyph_atlas_allocate(self, width, height, &mut rect) {
            Some(rect)
        } else {
            None
        }
    }

    pub fn reset(&mut self) {
        glyph_atlas_reset(self);
    }

    pub fn uv_rect(&self, pixel_rect: Rect) -> Rect {
        glyph_atlas_uv_rect(*self, pixel_rect)
    }
}

impl GlyphAtlasEntry {
    pub fn new(glyph_id: GlyphId, uv_rect: Rect, metrics: GlyphMetrics, texture_index: u32) -> Self {
        glyph_atlas_entry_new(glyph_id, uv_rect, metrics, texture_index)
    }
}

impl TextLayout {
    pub fn new(font_metrics: FontMetrics) -> Self {
        text_layout_new(font_metrics)
    }

    pub fn lines(&self) -> &[LineInfo] {
        if self.lines_ptr.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.lines_ptr, self.lines_len) }
        }
    }
}

impl LineInfo {
    pub fn new(start_index: usize, end_index: usize, width: f32, y_offset: f32) -> Self {
        line_info_new(start_index, end_index, width, y_offset)
    }
}

impl TextBounds {
    pub fn new(width: f32, height: f32, line_count: usize) -> Self {
        text_bounds_new(width, height, line_count)
    }

    pub fn measure(text: &str, font_size: f32, font_metrics: FontMetrics, max_width: Option<f32>) -> Self {
        match max_width {
            Some(w) => text_measure(text.as_ptr(), text.len(), font_size, font_metrics, &w),
            None => text_measure(text.as_ptr(), text.len(), font_size, font_metrics, core::ptr::null()),
        }
    }
}

impl TextVertex {
    pub fn new(position: [f32; 3], uv: Vec2, color: u32) -> Self {
        text_vertex_new(&position, uv, color)
    }
}

impl GlyphInstance {
    pub fn new(position: Vec2, size: Vec2, uv_rect: Rect, color: u32) -> Self {
        glyph_instance_new(position, size, uv_rect, color)
    }

    pub fn create_quad(&self) -> [TextVertex; 4] {
        create_glyph_quad(*self)
    }
}

impl TextAlignment {
    pub fn get_offset(&self, line_width: f32, max_width: f32) -> f32 {
        text_alignment_get_offset(*self, line_width, max_width)
    }
}

impl VerticalAlignment {
    pub fn get_offset(&self, content_height: f32, max_height: f32) -> f32 {
        vertical_alignment_get_offset(*self, content_height, max_height)
    }
}

impl SDFParams {
    pub fn default() -> Self {
        sdf_params_default()
    }

    pub fn calculate_value(&self, distance: f32) -> f32 {
        calculate_sdf_value(distance, *self)
    }
}

impl Default for SDFParams {
    fn default() -> Self {
        Self::default()
    }
}

/// Pack color into u32 for GPU (RGBA8)
pub fn pack_text_color(color: Color) -> u32 {
    pack_color(color)
}

/// Unpack u32 color from GPU to Color
pub fn unpack_text_color(packed: u32) -> Color {
    unpack_color(packed)
}

/// Wrap text according to the specified mode
pub fn wrap_text_content(text: &str, max_width: f32, font_size: f32, mode: WordWrapMode) -> usize {
    wrap_text(text.as_ptr(), text.len(), max_width, font_size, mode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_handle_creation() {
        let handle = FontHandle::new(1);
        assert_eq!(handle.id, 1);
        assert!(handle.is_valid());
    }

    #[test]
    fn test_font_handle_invalid() {
        let handle = FontHandle::INVALID;
        assert_eq!(handle.id, 0);
        assert!(!handle.is_valid());
    }

    #[test]
    fn test_glyph_id_creation() {
        let glyph_id = GlyphId::new(42);
        assert_eq!(glyph_id.value, 42);
    }

    #[test]
    fn test_rect_creation() {
        let rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 50.0));
        assert_eq!(rect.width(), 100.0);
        assert_eq!(rect.height(), 50.0);
    }

    #[test]
    fn test_rect_from_center_size() {
        let rect = Rect::from_center_size(Vec2::new(50.0, 25.0), Vec2::new(100.0, 50.0));
        assert_eq!(rect.width(), 100.0);
        assert_eq!(rect.height(), 50.0);
        assert_eq!(rect.min.x, 0.0);
        assert_eq!(rect.min.y, 0.0);
    }

    #[test]
    fn test_text_creation() {
        let font = FontHandle::new(1);
        let text = Text::new("Hello, World!", font, 16.0, Color::WHITE);
        assert_eq!(text.content(), "Hello, World!");
        assert_eq!(text.font_size, 16.0);
    }

    #[test]
    fn test_text_alignment() {
        let font = FontHandle::new(1);
        let text = Text::new("Test", font, 16.0, Color::WHITE)
            .with_alignment(TextAlignment::Center);
        assert_eq!(text.alignment, TextAlignment::Center);
    }

    #[test]
    fn test_text_vertical_alignment() {
        let font = FontHandle::new(1);
        let text = Text::new("Test", font, 16.0, Color::WHITE)
            .with_vertical_alignment(VerticalAlignment::Middle);
        assert_eq!(text.vertical_alignment, VerticalAlignment::Middle);
    }

    #[test]
    fn test_text_line_height() {
        let font = FontHandle::new(1);
        let text = Text::new("Test", font, 16.0, Color::WHITE)
            .with_line_height(1.5);
        assert_eq!(text.line_height_factor, 1.5);
    }

    #[test]
    fn test_glyph_atlas_creation() {
        let atlas = GlyphAtlas::new(Vec2::new(512.0, 512.0), 2.0);
        assert_eq!(atlas.texture_size.x, 512.0);
        assert_eq!(atlas.texture_size.y, 512.0);
        assert_eq!(atlas.padding, 2.0);
    }

    #[test]
    fn test_glyph_atlas_allocation() {
        let mut atlas = GlyphAtlas::new(Vec2::new(512.0, 512.0), 2.0);
        let rect = atlas.allocate(32.0, 32.0);
        assert!(rect.is_some());
        let rect = rect.unwrap();
        assert_eq!(rect.width(), 32.0);
        assert_eq!(rect.height(), 32.0);
    }

    #[test]
    fn test_glyph_atlas_uv_rect() {
        let atlas = GlyphAtlas::new(Vec2::new(512.0, 512.0), 2.0);
        let pixel_rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(32.0, 32.0));
        let uv_rect = atlas.uv_rect(pixel_rect);
        assert_eq!(uv_rect.min.x, 0.0);
        assert_eq!(uv_rect.min.y, 0.0);
        assert!((uv_rect.max.x - 32.0 / 512.0).abs() < 0.001);
        assert!((uv_rect.max.y - 32.0 / 512.0).abs() < 0.001);
    }

    #[test]
    fn test_font_metrics() {
        let metrics = FontMetrics {
            ascent: 800.0,
            descent: -200.0,
            line_gap: 100.0,
            units_per_em: 1000.0,
        };
        let line_height = metrics.line_height();
        assert_eq!(line_height, 1100.0); // 800 - (-200) + 100
    }

    #[test]
    fn test_font_metrics_scale() {
        let metrics = FontMetrics {
            ascent: 800.0,
            descent: -200.0,
            line_gap: 100.0,
            units_per_em: 1000.0,
        };
        let scaled = metrics.scale(16.0);
        assert!((scaled.ascent - 12.8).abs() < 0.01);
        assert!((scaled.descent - (-3.2)).abs() < 0.01);
    }

    #[test]
    fn test_text_bounds_measure() {
        let metrics = FontMetrics {
            ascent: 800.0,
            descent: -200.0,
            line_gap: 100.0,
            units_per_em: 1000.0,
        };
        let bounds = TextBounds::measure("Hello", 16.0, metrics, None);
        assert!(bounds.width > 0.0);
        assert!(bounds.height > 0.0);
        assert_eq!(bounds.line_count, 1);
    }

    #[test]
    fn test_text_bounds_multiline() {
        let metrics = FontMetrics {
            ascent: 800.0,
            descent: -200.0,
            line_gap: 100.0,
            units_per_em: 1000.0,
        };
        let bounds = TextBounds::measure("Hello\nWorld", 16.0, metrics, None);
        assert!(bounds.width > 0.0);
        assert!(bounds.height > 0.0);
        assert_eq!(bounds.line_count, 2);
    }

    #[test]
    fn test_color_packing() {
        let color = Color::rgba(1.0, 0.5, 0.25, 0.75);
        let packed = pack_text_color(color);
        let unpacked = unpack_text_color(packed);
        
        assert!((unpacked.r - color.r).abs() < 0.01);
        assert!((unpacked.g - color.g).abs() < 0.01);
        assert!((unpacked.b - color.b).abs() < 0.01);
        assert!((unpacked.a - color.a).abs() < 0.01);
    }

    #[test]
    fn test_text_vertex_creation() {
        let vertex = TextVertex::new([0.0, 0.0, 0.0], Vec2::new(0.0, 0.0), 0xFFFFFFFF);
        assert_eq!(vertex.position, [0.0, 0.0, 0.0]);
        assert_eq!(vertex.uv, [0.0, 0.0]);
        assert_eq!(vertex.color, 0xFFFFFFFF);
    }

    #[test]
    fn test_glyph_instance_creation() {
        let instance = GlyphInstance::new(
            Vec2::new(10.0, 20.0),
            Vec2::new(32.0, 32.0),
            Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)),
            0xFFFFFFFF,
        );
        assert_eq!(instance.position.x, 10.0);
        assert_eq!(instance.position.y, 20.0);
        assert_eq!(instance.size.x, 32.0);
        assert_eq!(instance.size.y, 32.0);
    }

    #[test]
    fn test_glyph_instance_create_quad() {
        let instance = GlyphInstance::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(32.0, 32.0),
            Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)),
            0xFFFFFFFF,
        );
        let quad = instance.create_quad();
        assert_eq!(quad.len(), 4);
    }

    #[test]
    fn test_line_info_creation() {
        let line = LineInfo::new(0, 10, 100.0, 0.0);
        assert_eq!(line.start_index, 0);
        assert_eq!(line.end_index, 10);
        assert_eq!(line.width, 100.0);
        assert_eq!(line.y_offset, 0.0);
    }

    #[test]
    fn test_text_layout_creation() {
        let metrics = FontMetrics {
            ascent: 800.0,
            descent: -200.0,
            line_gap: 100.0,
            units_per_em: 1000.0,
        };
        let layout = TextLayout::new(metrics);
        assert_eq!(layout.lines().len(), 0);
    }

    #[test]
    fn test_text_alignment_offset() {
        let offset = TextAlignment::Center.get_offset(50.0, 100.0);
        assert_eq!(offset, 25.0);
        
        let offset = TextAlignment::Right.get_offset(50.0, 100.0);
        assert_eq!(offset, 50.0);
        
        let offset = TextAlignment::Left.get_offset(50.0, 100.0);
        assert_eq!(offset, 0.0);
    }

    #[test]
    fn test_vertical_alignment_offset() {
        let offset = VerticalAlignment::Middle.get_offset(50.0, 100.0);
        assert_eq!(offset, 25.0);
        
        let offset = VerticalAlignment::Bottom.get_offset(50.0, 100.0);
        assert_eq!(offset, 50.0);
        
        let offset = VerticalAlignment::Top.get_offset(50.0, 100.0);
        assert_eq!(offset, 0.0);
    }

    #[test]
    fn test_sdf_params_default() {
        let params = SDFParams::default();
        assert_eq!(params.spread, 4.0);
        assert_eq!(params.smoothness, 0.25);
        assert_eq!(params.threshold, 0.5);
    }

    #[test]
    fn test_sdf_calculate_value() {
        let params = SDFParams::default();
        let value = params.calculate_value(0.0);
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn test_glyph_atlas_entry_creation() {
        let glyph_id = GlyphId::new(42);
        let uv_rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
        let metrics = GlyphMetrics::default();
        let entry = GlyphAtlasEntry::new(glyph_id, uv_rect, metrics, 0);
        assert_eq!(entry.glyph_id.value, 42);
        assert_eq!(entry.texture_index, 0);
    }

    #[test]
    fn test_glyph_metrics_default() {
        let metrics = GlyphMetrics::default();
        assert_eq!(metrics.advance_width, 0.0);
        assert_eq!(metrics.advance_height, 0.0);
    }

    #[test]
    fn test_text_content_mutation() {
        let font = FontHandle::new(1);
        let mut text = Text::new("Hello", font, 16.0, Color::WHITE);
        assert_eq!(text.content(), "Hello");
        
        text.set_content("World");
        assert_eq!(text.content(), "World");
    }

    #[test]
    fn test_text_builder_pattern() {
        let font = FontHandle::new(1);
        let text = Text::new("Test", font, 16.0, Color::WHITE)
            .with_alignment(TextAlignment::Center)
            .with_vertical_alignment(VerticalAlignment::Middle)
            .with_line_height(1.5);
        
        assert_eq!(text.alignment, TextAlignment::Center);
        assert_eq!(text.vertical_alignment, VerticalAlignment::Middle);
        assert_eq!(text.line_height_factor, 1.5);
    }

    #[test]
    fn test_rect_size() {
        let rect = Rect::new(Vec2::new(10.0, 20.0), Vec2::new(50.0, 60.0));
        let size = rect.size();
        assert_eq!(size.x, 40.0);
        assert_eq!(size.y, 40.0);
    }

    #[test]
    fn test_glyph_atlas_reset() {
        let mut atlas = GlyphAtlas::new(Vec2::new(512.0, 512.0), 2.0);
        atlas.allocate(32.0, 32.0);
        atlas.reset();
        assert_eq!(atlas.current_x, 2.0); // Reset to padding
        assert_eq!(atlas.current_y, 2.0);
    }

    #[test]
    fn test_multiple_glyph_allocations() {
        let mut atlas = GlyphAtlas::new(Vec2::new(512.0, 512.0), 2.0);
        
        let rect1 = atlas.allocate(32.0, 32.0);
        assert!(rect1.is_some());
        
        let rect2 = atlas.allocate(32.0, 32.0);
        assert!(rect2.is_some());
        
        // They should have different positions
        let r1 = rect1.unwrap();
        let r2 = rect2.unwrap();
        assert!(r1.min.x != r2.min.x || r1.min.y != r2.min.y);
    }

    #[test]
    fn test_text_with_empty_content() {
        let font = FontHandle::new(1);
        let text = Text::new("", font, 16.0, Color::WHITE);
        assert_eq!(text.content(), "");
    }

    #[test]
    fn test_text_with_unicode() {
        let font = FontHandle::new(1);
        let text = Text::new("你好世界🌍", font, 16.0, Color::WHITE);
        assert_eq!(text.content(), "你好世界🌍");
    }
}