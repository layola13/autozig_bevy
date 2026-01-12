use autozig::include_zig;
use autozig_color::Color;
use autozig_math::Vec2;

// ============================================================================
// Core Enums
// ============================================================================

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

/// Font style variations
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontStyle {
    Normal = 0,
    Italic = 1,
    Oblique = 2,
}

/// Font weight from 100 (Thin) to 900 (Black)
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FontWeight {
    Thin = 100,
    ExtraLight = 200,
    Light = 300,
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

/// Font width/stretch property
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontWidth {
    UltraCondensed = 0,
    ExtraCondensed = 1,
    Condensed = 2,
    SemiCondensed = 3,
    Normal = 4,
    SemiExpanded = 5,
    Expanded = 6,
    ExtraExpanded = 7,
    UltraExpanded = 8,
}

/// Font hinting options
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontHinting {
    None = 0,
    Slight = 1,
    Normal = 2,
    Full = 3,
}

/// Font smoothing/antialiasing options
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSmoothing {
    None = 0,
    Grayscale = 1,
    SubpixelRgb = 2,
    SubpixelBgr = 3,
}

/// Text justification modes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Justify {
    Left = 0,
    Center = 1,
    Right = 2,
    Justified = 3,
}

/// Line breaking modes
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineBreak {
    WordBoundary = 0,
    AnyCharacter = 1,
    NoWrap = 2,
}

/// Line height specification
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineHeight {
    Pixels(f32),
    Relative(f32),
}

/// Font source specification
#[repr(C)]
#[derive(Debug, Clone)]
pub enum FontSource {
    /// Font data embedded in binary
    Binary {
        data_ptr: *const u8,
        data_len: usize,
    },
    /// Font file path
    Path {
        path_ptr: *const u8,
        path_len: usize,
    },
}

/// Text rendering errors
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextError {
    NoSuchFont = 0,
    FailedToAddGlyph = 1,
    FailedToGetGlyphImage = 2,
    ExceedMaxTextureSize = 3,
    InvalidFont = 4,
    InvalidGlyph = 5,
}

/// Font loader errors
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontLoaderError {
    Io = 0,
    InvalidFont = 1,
    UnsupportedFormat = 2,
}

// ============================================================================
// Font Types
// ============================================================================

/// Font handle for referencing loaded fonts
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontHandle {
    pub id: u32,
}

/// Font with complete styling information
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Font {
    pub handle: FontHandle,
    pub size: f32,
    pub style: FontStyle,
    pub weight: FontWeight,
    pub width: FontWidth,
}

/// Glyph identifier
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// Font face information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FontFaceInfo {
    pub family_name_ptr: *const u8,
    pub family_name_len: usize,
    pub style: FontStyle,
    pub weight: FontWeight,
    pub width: FontWidth,
}

/// OpenType font feature tag (4 ASCII characters)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontFeatureTag {
    pub tag: [u8; 4],
}

/// Font features collection
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FontFeatures {
    features_ptr: *mut FontFeatureTag,
    features_len: usize,
    features_cap: usize,
}

/// Builder for font features
#[repr(C)]
#[derive(Debug)]
pub struct FontFeaturesBuilder {
    features: FontFeatures,
}

/// Font loader for asset management
#[repr(C)]
#[derive(Debug)]
pub struct FontLoader {
    _private: u8,
}

/// Text font specification (combines handle and size)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextFont {
    pub font: FontHandle,
    pub font_size: f32,
}

// ============================================================================
// Glyph Atlas Types
// ============================================================================

/// Rectangle type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

/// Font atlas key for caching
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FontAtlasKey {
    pub font_id: u32,
    pub font_size: u32,
}

/// Font atlas for texture management
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FontAtlas {
    pub texture_id: u32,
    pub size: Vec2,
}

/// Font atlas set for multiple atlases
#[repr(C)]
#[derive(Debug)]
pub struct FontAtlasSet {
    atlases_ptr: *mut FontAtlas,
    atlases_len: usize,
    atlases_cap: usize,
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

/// Glyph atlas information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlyphAtlasInfo {
    pub atlas_index: u32,
    pub glyph_rect: Rect,
    pub uv_rect: Rect,
}

/// Glyph atlas location
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphAtlasLocation {
    pub atlas_index: u32,
    pub glyph_index: u32,
}

/// Positioned glyph for rendering
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PositionedGlyph {
    pub glyph_id: GlyphId,
    pub position: Vec2,
    pub size: Vec2,
    pub atlas_info: GlyphAtlasInfo,
}

/// Swash cache for glyph rasterization
#[repr(C)]
#[derive(Debug)]
pub struct SwashCache {
    _private: [u8; 64],
}

// ============================================================================
// Text Component & Styling
// ============================================================================

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

/// Text color component
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextColor {
    pub color: Color,
}

/// Text background color
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextBackgroundColor {
    pub color: Color,
}

/// Strikethrough styling
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Strikethrough {
    pub enabled: bool,
    pub offset: f32,
    pub thickness: f32,
}

/// Strikethrough color
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrikethroughColor {
    pub color: Color,
}

/// Underline styling
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Underline {
    pub enabled: bool,
    pub offset: f32,
    pub thickness: f32,
}

/// Underline color
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnderlineColor {
    pub color: Color,
}

// ============================================================================
// Text Layout & Measurement
// ============================================================================

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

/// Text layout detailed information
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextLayoutInfo {
    pub glyphs_ptr: *mut PositionedGlyph,
    pub glyphs_len: usize,
    pub glyphs_cap: usize,
    pub size: Vec2,
}

/// Text measurement information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextMeasureInfo {
    pub min: Vec2,
    pub max: Vec2,
}

/// Computed text block after layout
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ComputedTextBlock {
    pub entities_ptr: *mut u32,
    pub entities_len: usize,
    pub entities_cap: usize,
    pub size: Vec2,
}

/// Text run geometry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RunGeometry {
    pub offset: Vec2,
    pub size: Vec2,
    pub line_index: usize,
}

/// Text iteration scratch buffer
#[repr(C)]
#[derive(Debug)]
pub struct TextIterScratch {
    buffer_ptr: *mut u8,
    buffer_len: usize,
    buffer_cap: usize,
}

// ============================================================================
// Text Span (Rich Text Support)
// ============================================================================

/// Text span for rich text
#[repr(C)]
#[derive(Debug, Clone)]
pub struct TextSpan {
    text_ptr: *const u8,
    text_len: usize,
    pub font: Option<FontHandle>,
    pub font_size: Option<f32>,
    pub color: Option<Color>,
    pub style: FontStyle,
    pub weight: FontWeight,
}

/// Text span iterator
#[repr(C)]
#[derive(Debug)]
pub struct TextSpanIter {
    spans_ptr: *const TextSpan,
    spans_len: usize,
    current_index: usize,
}

/// Text entity reference
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextEntity {
    pub id: u32,
}

/// Text reader for traversing text tree
#[repr(C)]
#[derive(Debug)]
pub struct TextReader {
    _private: [u8; 32],
}

/// Text writer for modifying text tree
#[repr(C)]
#[derive(Debug)]
pub struct TextWriter {
    _private: [u8; 32],
}

// ============================================================================
// Cosmic Text Integration
// ============================================================================

/// Cosmic text buffer wrapper
#[repr(C)]
#[derive(Debug)]
pub struct CosmicBuffer {
    _private: [u8; 128],
}

/// Cosmic font system wrapper
#[repr(C)]
#[derive(Debug)]
pub struct CosmicFontSystem {
    _private: [u8; 256],
}

// ============================================================================
// Text Pipeline & Plugin
// ============================================================================

/// Text rendering pipeline
#[repr(C)]
#[derive(Debug)]
pub struct TextPipeline {
    font_atlas_set: FontAtlasSet,
    cosmic_font_system: CosmicFontSystem,
    cache_size: usize,
}

/// Text plugin for Bevy integration
#[repr(C)]
#[derive(Debug)]
pub struct TextPlugin {
    _private: u8,
}

/// Text 2D update systems
#[repr(C)]
#[derive(Debug)]
pub struct Text2dUpdateSystems {
    _private: u8,
}

// ============================================================================
// Text Rendering
// ============================================================================

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

// ============================================================================
// Traits
// ============================================================================

/// Trait for text root entities
pub trait TextRoot {
    fn text_root(&self) -> TextEntity;
}

/// Trait for accessing text spans
pub trait TextSpanAccess {
    fn spans(&self) -> &[TextSpan];
    fn spans_mut(&mut self) -> &mut [TextSpan];
}

/// Trait for text span components
pub trait TextSpanComponent {
    fn as_text_span(&self) -> &TextSpan;
    fn as_text_span_mut(&mut self) -> &mut TextSpan;
}

// ============================================================================
// Zig FFI Bindings
// ============================================================================

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
    fn create_glyph_quad(instance: GlyphInstance, out: *mut [TextVertex; 4]) -> ();
    
    // Color packing functions
    fn pack_color(color: Color) -> u32;
    fn unpack_color(packed: u32) -> Color;
    
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

impl Font {
    pub fn new(handle: FontHandle, size: f32) -> Self {
        Self {
            handle,
            size,
            style: FontStyle::Normal,
            weight: FontWeight::Normal,
            width: FontWidth::Normal,
        }
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new(FontHandle::INVALID, 16.0)
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

impl FontFeatureTag {
    pub fn new(tag: [u8; 4]) -> Self {
        Self { tag }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() == 4 {
            let mut tag = [0u8; 4];
            tag.copy_from_slice(s.as_bytes());
            Some(Self { tag })
        } else {
            None
        }
    }
}

impl FontFeatures {
    pub fn new() -> Self {
        Self {
            features_ptr: std::ptr::null_mut(),
            features_len: 0,
            features_cap: 0,
        }
    }
    
    pub fn features(&self) -> &[FontFeatureTag] {
        if self.features_ptr.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.features_ptr, self.features_len) }
        }
    }
}

impl Default for FontFeatures {
    fn default() -> Self {
        Self::new()
    }
}

impl FontFeaturesBuilder {
    pub fn new() -> Self {
        Self {
            features: FontFeatures::new(),
        }
    }
    
    pub fn build(self) -> FontFeatures {
        self.features
    }
}

impl Default for FontFeaturesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TextFont {
    pub fn new(font: FontHandle, font_size: f32) -> Self {
        Self { font, font_size }
    }
}

impl Default for TextFont {
    fn default() -> Self {
        Self::new(FontHandle::INVALID, 16.0)
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

impl FontAtlasKey {
    pub fn new(font_id: u32, font_size: u32) -> Self {
        Self { font_id, font_size }
    }
}

impl FontAtlas {
    pub fn new(texture_id: u32, size: Vec2) -> Self {
        Self { texture_id, size }
    }
}

impl FontAtlasSet {
    pub fn new() -> Self {
        Self {
            atlases_ptr: std::ptr::null_mut(),
            atlases_len: 0,
            atlases_cap: 0,
        }
    }
    
    pub fn atlases(&self) -> &[FontAtlas] {
        if self.atlases_ptr.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.atlases_ptr, self.atlases_len) }
        }
    }
}

impl Default for FontAtlasSet {
    fn default() -> Self {
        Self::new()
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

impl TextColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Default for TextColor {
    fn default() -> Self {
        Self::new(Color::WHITE)
    }
}

impl TextBackgroundColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Strikethrough {
    pub fn new() -> Self {
        Self {
            enabled: true,
            offset: 0.0,
            thickness: 1.0,
        }
    }
}

impl Default for Strikethrough {
    fn default() -> Self {
        Self::new()
    }
}

impl StrikethroughColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Underline {
    pub fn new() -> Self {
        Self {
            enabled: true,
            offset: -2.0,
            thickness: 1.0,
        }
    }
}

impl Default for Underline {
    fn default() -> Self {
        Self::new()
    }
}

impl UnderlineColor {
    pub fn new(color: Color) -> Self {
        Self { color }
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

impl GlyphAtlasInfo {
    pub fn new(atlas_index: u32, glyph_rect: Rect, uv_rect: Rect) -> Self {
        Self {
            atlas_index,
            glyph_rect,
            uv_rect,
        }
    }
}

impl GlyphAtlasLocation {
    pub fn new(atlas_index: u32, glyph_index: u32) -> Self {
        Self {
            atlas_index,
            glyph_index,
        }
    }
}

impl PositionedGlyph {
    pub fn new(glyph_id: GlyphId, position: Vec2, size: Vec2, atlas_info: GlyphAtlasInfo) -> Self {
        Self {
            glyph_id,
            position,
            size,
            atlas_info,
        }
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

impl TextLayoutInfo {
    pub fn new() -> Self {
        Self {
            glyphs_ptr: std::ptr::null_mut(),
            glyphs_len: 0,
            glyphs_cap: 0,
            size: Vec2::splat(0.0),
        }
    }
    
    pub fn glyphs(&self) -> &[PositionedGlyph] {
        if self.glyphs_ptr.is_null() {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(self.glyphs_ptr, self.glyphs_len) }
        }
    }
}

impl Default for TextLayoutInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl TextMeasureInfo {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
}

impl ComputedTextBlock {
    pub fn new() -> Self {
        Self {
            entities_ptr: std::ptr::null_mut(),
            entities_len: 0,
            entities_cap: 0,
            size: Vec2::splat(0.0),
        }
    }
}

impl Default for ComputedTextBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl RunGeometry {
    pub fn new(offset: Vec2, size: Vec2, line_index: usize) -> Self {
        Self {
            offset,
            size,
            line_index,
        }
    }
}

impl TextSpan {
    pub fn new(text: &str) -> Self {
        Self {
            text_ptr: text.as_ptr(),
            text_len: text.len(),
            font: None,
            font_size: None,
            color: None,
            style: FontStyle::Normal,
            weight: FontWeight::Normal,
        }
    }
    
    pub fn text(&self) -> &str {
        if self.text_ptr.is_null() {
            ""
        } else {
            let slice = unsafe { std::slice::from_raw_parts(self.text_ptr, self.text_len) };
            std::str::from_utf8(slice).unwrap_or("")
        }
    }
}

impl TextEntity {
    pub fn new(id: u32) -> Self {
        Self { id }
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
        let mut out = [TextVertex { position: [0.0; 3], uv: [0.0; 2], color: 0 }; 4];
        create_glyph_quad(*self, &mut out);
        out
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

impl Default for LineHeight {
    fn default() -> Self {
        LineHeight::Relative(1.2)
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
    fn test_all_48_types_exist() {
        // 验证所有48个类型都已定义
        let _: ComputedTextBlock;
        let _: CosmicBuffer;
        let _: CosmicFontSystem;
        let _: FontAtlas;
        let _: FontAtlasKey;
        let _: FontAtlasSet;
        let _: FontFaceInfo;
        let _: FontFeatureTag;
        let _: FontFeatures;
        let _: FontFeaturesBuilder;
        let _: FontLoader;
        let _: Font;
        let _: FontWeight;
        let _: FontWidth;
        let _: GlyphAtlasInfo;
        let _: GlyphAtlasLocation;
        let _: PositionedGlyph;
        let _: RunGeometry;
        let _: Strikethrough;
        let _: StrikethroughColor;
        let _: SwashCache;
        let _: Text2dUpdateSystems;
        let _: TextBackgroundColor;
        let _: TextColor;
        let _: TextEntity;
        let _: TextFont;
        let _: TextIterScratch;
        let _: TextLayoutInfo;
        let _: TextMeasureInfo;
        let _: TextPipeline;
        let _: TextPlugin;
        let _: TextReader;
        let _: TextSpan;
        let _: TextSpanIter;
        let _: TextWriter;
        let _: Underline;
        let _: UnderlineColor;
        
        // Enums
        let _: FontHinting;
        let _: FontLoaderError;
        let _: FontSmoothing;
        let _: FontSource;
        let _: FontStyle;
        let _: Justify;
        let _: LineBreak;
        let _: LineHeight;
        let _: TextError;
    }

    #[test]
    fn test_font_handle_creation() {
        let handle = FontHandle::new(1);
        assert_eq!(handle.id, 1);
        assert!(handle.is_valid());
    }

    #[test]
    fn test_font_weight_ordering() {
        assert!(FontWeight::Thin < FontWeight::Normal);
        assert!(FontWeight::Normal < FontWeight::Bold);
        assert!(FontWeight::Bold < FontWeight::Black);
    }

    #[test]
    fn test_font_feature_tag() {
        let tag = FontFeatureTag::new([b'l', b'i', b'g', b'a']);
        assert_eq!(tag.tag, [b'l', b'i', b'g', b'a']);
    }
} 