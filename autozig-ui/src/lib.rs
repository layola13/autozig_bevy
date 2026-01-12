
use autozig::include_zig;
use autozig_color::Color;
use autozig_math::{Vec2, Vec3};

// ============================================================================
// Core UI Enums and Types
// ============================================================================

/// Display type for UI elements
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Display {
    /// Flex layout (default)
    Flex = 0,
    /// Element is hidden
    None = 1,
    /// Grid layout
    Grid = 2,
}

/// Direction for flex layout
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    /// Left to right
    Row = 0,
    /// Right to left
    RowReverse = 1,
    /// Top to bottom
    Column = 2,
    /// Bottom to top
    ColumnReverse = 3,
}

/// How flex items are aligned along the main axis
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyContent {
    /// Items are packed at the start
    FlexStart = 0,
    /// Items are packed at the end
    FlexEnd = 1,
    /// Items are centered
    Center = 2,
    /// Items are evenly distributed
    SpaceBetween = 3,
    /// Items have equal space around them
    SpaceAround = 4,
    /// Items have equal space around them including edges
    SpaceEvenly = 5,
}

/// How flex items are aligned along the cross axis
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignItems {
    /// Items are stretched to fill the container
    Stretch = 0,
    /// Items are packed at the start
    FlexStart = 1,
    /// Items are packed at the end
    FlexEnd = 2,
    /// Items are centered
    Center = 3,
    /// Items are aligned at their baseline
    Baseline = 4,
}

/// How a flex item is aligned along the cross axis (overrides AlignItems)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignSelf {
    /// Use the parent's AlignItems value
    Auto = 0,
    /// Stretch to fill the container
    Stretch = 1,
    /// Pack at the start
    FlexStart = 2,
    /// Pack at the end
    FlexEnd = 3,
    /// Center
    Center = 4,
    /// Align at baseline
    Baseline = 5,
}

/// How flex lines are aligned in a multi-line flex container
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignContent {
    /// Lines are stretched to fill the container
    Stretch = 0,
    /// Lines are packed at the start
    FlexStart = 1,
    /// Lines are packed at the end
    FlexEnd = 2,
    /// Lines are centered
    Center = 3,
    /// Lines are evenly distributed
    SpaceBetween = 4,
    /// Lines have equal space around them
    SpaceAround = 5,
}

/// Whether flex items wrap
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexWrap {
    /// Single line
    NoWrap = 0,
    /// Multi-line
    Wrap = 1,
    /// Multi-line reverse
    WrapReverse = 2,
}

/// Position type for UI elements
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionType {
    /// Normal flow
    Relative = 0,
    /// Removed from normal flow
    Absolute = 1,
}

/// Overflow behavior
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Overflow {
    pub x: OverflowAxis,
    pub y: OverflowAxis,
}

/// Unit type for CSS values
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    /// No value
    Undefined = 0,
    /// Pixel value
    Px = 1,
    /// Percentage value
    Percent = 2,
    /// Auto size
    Auto = 3,
    /// Viewport width percentage
    Vw = 4,
    /// Viewport height percentage
    Vh = 5,
    /// Viewport minimum percentage
    VMin = 6,
    /// Viewport maximum percentage
    VMax = 7,
}

/// CSS value with unit
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Val {
    pub value: f32,
    pub unit: Unit,
}

/// Two-dimensional value pair
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Val2 {
    pub x: Val,
    pub y: Val,
}

/// Rectangle of values (for margin, padding, border, etc.)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiRect {
    pub left: Val,
    pub right: Val,
    pub top: Val,
    pub bottom: Val,
}

/// Size specification
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: Val,
    pub height: Val,
}

/// Interaction state
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interaction {
    /// Not interacting
    None = 0,
    /// Mouse is hovering
    Hovered = 1,
    /// Mouse is pressed
    Pressed = 2,
}

/// Focus state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FocusState {
    pub is_focused: bool,
    pub tab_index: i32,
}

// ============================================================================
// Missing Enums (20 types)
// ============================================================================

/// Focus policy for input events
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPolicy {
    /// Block focus
    Block = 0,
    /// Pass focus through
    Pass = 1,
}

/// Box sizing model
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxSizing {
    /// Content box
    ContentBox = 0,
    /// Border box
    BorderBox = 1,
}

/// Grid auto flow direction
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridAutoFlow {
    Row = 0,
    Column = 1,
    RowDense = 2,
    ColumnDense = 3,
}

/// Grid track repetition
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridTrackRepetition {
    /// Count-based repetition
    Count = 0,
    /// Auto-fill
    AutoFill = 1,
    /// Auto-fit
    AutoFit = 2,
}

/// Justify items alignment
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifyItems {
    Default = 0,
    Start = 1,
    End = 2,
    Center = 3,
    Stretch = 4,
    Baseline = 5,
}

/// Justify self alignment
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JustifySelf {
    Auto = 0,
    Start = 1,
    End = 2,
    Center = 3,
    Stretch = 4,
    Baseline = 5,
}

/// Overflow behavior per axis
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowAxis {
    Visible = 0,
    Clip = 1,
    Hidden = 2,
    Scroll = 3,
}

/// Overflow clip box
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverflowClipBox {
    PaddingBox = 0,
    ContentBox = 1,
}

/// Interpolation color space for gradients
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpolationColorSpace {
    LinearRgb = 0,
    Srgb = 1,
    OkLab = 2,
    OkLch = 3,
}

/// Radial gradient shape
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadialGradientShape {
    Circle = 0,
    Ellipse = 1,
}

/// Node image rendering mode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeImageMode {
    Auto = 0,
    Stretch = 1,
    Tile = 2,
}

/// Minimum track sizing function
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinTrackSizingFunction {
    Fixed = 0,
    MinContent = 1,
    MaxContent = 2,
    Auto = 3,
}

/// Maximum track sizing function
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaxTrackSizingFunction {
    Fixed = 0,
    MinContent = 1,
    MaxContent = 2,
    FitContent = 3,
    Auto = 4,
    Fraction = 5,
}

/// Node measurement mode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeMeasure {
    Fixed = 0,
    Text = 1,
    Image = 2,
}

/// UI system set labels
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiSystems {
    Prepare = 0,
    Layout = 1,
    PostLayout = 2,
    Focus = 3,
    Stack = 4,
}

/// Gradient type
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gradient {
    Linear = 0,
    Radial = 1,
    Conic = 2,
}

/// Layout error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutError {
    InvalidNode = 0,
    InvalidStyle = 1,
    CircularDependency = 2,
}

/// Grid placement error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridPlacementError {
    InvalidSpan = 0,
    OutOfBounds = 1,
}

/// Val arithmetic error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValArithmeticError {
    IncompatibleUnits = 0,
    Overflow = 1,
}

/// Val parse error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValParseError {
    InvalidFormat = 0,
    InvalidUnit = 1,
    InvalidValue = 2,
}

// ============================================================================
// Missing Structs (64 types)
// ============================================================================

/// Button component marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Button;

/// Label component
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Label {
    pub text: *const u8,
    pub len: usize,
}

/// Text component
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Text {
    pub sections: *const u8,
    pub section_count: usize,
}

/// Checkable marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Checkable;

/// Checked state
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Checked {
    pub is_checked: bool,
}

/// Pressed state marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pressed;

/// Interaction disabled marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InteractionDisabled;

/// Color stop for gradients
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorStop {
    pub color: Color,
    pub position: f32,
}

/// Angular color stop for conic gradients
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngularColorStop {
    pub color: Color,
    pub angle: f32,
}

/// Linear gradient
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LinearGradient {
    pub angle: f32,
    pub stops: *const ColorStop,
    pub stop_count: usize,
    pub color_space: InterpolationColorSpace,
}

/// Radial gradient
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RadialGradient {
    pub center: Vec2,
    pub radius: f32,
    pub shape: RadialGradientShape,
    pub stops: *const ColorStop,
    pub stop_count: usize,
    pub color_space: InterpolationColorSpace,
}

/// Conic gradient
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConicGradient {
    pub center: Vec2,
    pub rotation: f32,
    pub stops: *const AngularColorStop,
    pub stop_count: usize,
    pub color_space: InterpolationColorSpace,
}

/// Background gradient
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BackgroundGradient {
    pub gradient_type: Gradient,
    pub data: [u8; 64], // Union storage
}

/// Border gradient
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderGradient {
    pub gradient_type: Gradient,
    pub data: [u8; 64], // Union storage
}

/// Box shadow
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoxShadow {
    pub color: Color,
    pub offset: Vec2,
    pub blur_radius: f32,
    pub spread_radius: f32,
}

/// Text shadow
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextShadow {
    pub color: Color,
    pub offset: Vec2,
    pub blur_radius: f32,
}

/// Shadow style
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowStyle {
    pub shadows: *const BoxShadow,
    pub shadow_count: usize,
}

/// Outline
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Outline {
    pub width: Val,
    pub offset: Val,
    pub color: Color,
}

/// Overflow clip margin
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OverflowClipMargin {
    pub margin: f32,
    pub clip_box: OverflowClipBox,
}

/// Calculated clip rectangle
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CalculatedClip {
    pub clip: [f32; 4], // x, y, width, height
}

/// Override clip
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OverrideClip {
    pub clip: [f32; 4],
}

/// Resolved border radius after layout
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedBorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

/// Grid placement
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridPlacement {
    pub start: i32,
    pub end: i32,
    pub span: u16,
}

/// Grid track
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridTrack {
    pub min_sizing: MinTrackSizingFunction,
    pub max_sizing: MaxTrackSizingFunction,
}

/// Repeated grid track
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RepeatedGridTrack {
    pub repetition: GridTrackRepetition,
    pub tracks: *const GridTrack,
    pub track_count: usize,
}

/// Content size
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContentSize {
    pub size: Vec2,
}

/// Image node
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageNode {
    pub texture: usize, // Texture handle
    pub rect: Option<[f32; 4]>,
    pub mode: NodeImageMode,
}

/// Image node size
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageNodeSize {
    pub size: Vec2,
}

/// Text node flags
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextNodeFlags {
    pub needs_recompute: bool,
    pub linebreak_behavior: u8,
}

/// Ghost node (invisible but affects layout)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GhostNode;

/// Relative cursor position
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RelativeCursorPosition {
    pub normalized: Option<Vec2>,
}

/// Scroll position
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollPosition {
    pub offset: Vec2,
}

/// Ignore scroll marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgnoreScroll;

/// Z-index for layering
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZIndex {
    pub value: i32,
}

/// Global Z-index
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlobalZIndex {
    pub value: i32,
}

/// UI transform
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiTransform {
    pub translation: Vec3,
    pub rotation: f32,
    pub scale: Vec2,
}

/// UI global transform
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiGlobalTransform {
    pub translation: Vec3,
    pub rotation: f32,
    pub scale: Vec2,
}

/// UI position in screen space
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiPosition {
    pub position: Vec2,
}

/// UI scale factor
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiScale {
    pub scale: f32,
}

/// UI stack for rendering order
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiStack {
    pub index: usize,
}

/// Layout context
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutContext {
    pub scale_factor: f32,
    pub physical_size: Vec2,
    pub min_size: Vec2,
    pub max_size: Vec2,
}

/// Layout node
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutNode {
    pub position: Vec2,
    pub size: Vec2,
    pub parent: usize,
}

/// Layout configuration
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LayoutConfig {
    pub use_rounding: bool,
}

/// Measure arguments
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MeasureArgs {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub available_width: f32,
    pub available_height: f32,
}

/// Fixed measure
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedMeasure {
    pub size: Vec2,
}

/// Image measure
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageMeasure {
    pub size: Vec2,
}

/// Text measure
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextMeasure {
    pub size: Vec2,
}

/// UI surface
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiSurface {
    pub camera_entity: usize,
}

/// Target camera for UI
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiTargetCamera {
    pub entity: usize,
}

/// Computed UI render target info
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComputedUiRenderTargetInfo {
    pub physical_size: Vec2,
    pub scale_factor: f32,
}

/// Computed UI target camera
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComputedUiTargetCamera {
    pub entity: usize,
}

/// Default UI camera marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DefaultUiCamera;

/// Is default UI camera marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IsDefaultUiCamera;

/// UI picking camera
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiPickingCamera {
    pub enabled: bool,
}

/// UI picking settings
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiPickingSettings {
    pub enabled: bool,
    pub should_ignore_scroll: bool,
}

/// UI picking plugin marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiPickingPlugin;

/// UI plugin marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiPlugin;

/// UI root nodes collection
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiRootNodes {
    pub nodes: *const usize,
    pub count: usize,
}

/// UI children collection
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiChildren {
    pub children: *const usize,
    pub count: usize,
}

/// UI children iterator
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiChildrenIter {
    pub children: *const usize,
    pub count: usize,
    pub index: usize,
}

/// Viewport node
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewportNode {
    pub viewport: [f32; 4], // x, y, width, height
}

/// Node query helper
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeQuery {
    pub entity: usize,
    pub node_type: u8,
}

/// State component for UI state management
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    pub value: u32,
}

// ============================================================================
// Missing Traits (3 types)
// ============================================================================

/// Trait for types that can be in a color space
pub trait InColorSpace {
    fn in_color_space(&self, space: InterpolationColorSpace) -> Self;
}

/// Trait for types that can measure content
pub trait Measure {
    fn measure(&self, args: MeasureArgs) -> Vec2;
}

/// Trait for numeric Val operations
pub trait ValNum {
    fn to_val(&self) -> Val;
    fn from_val(val: Val) -> Option<Self> where Self: Sized;
}

// ============================================================================
// Main Components (from original implementation)
// ============================================================================

/// Style component defining layout and appearance
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub display: Display,
    pub position_type: PositionType,
    pub overflow: Overflow,
    
    // Flex container properties
    pub flex_direction: FlexDirection,
    pub flex_wrap: FlexWrap,
    pub justify_content: JustifyContent,
    pub align_items: AlignItems,
    pub align_content: AlignContent,
    
    // Flex item properties
    pub align_self: AlignSelf,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Val,
    
    // Position
    pub left: Val,
    pub right: Val,
    pub top: Val,
    pub bottom: Val,
    
    // Size
    pub width: Val,
    pub height: Val,
    pub min_width: Val,
    pub min_height: Val,
    pub max_width: Val,
    pub max_height: Val,
    
    // Spacing
    pub margin: UiRect,
    pub padding: UiRect,
    pub border: UiRect,
    
    // Aspect ratio
    pub aspect_ratio: Option<f32>,
}

/// Node component representing a UI element
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Node {
    /// Calculated position in pixels
    pub position: Vec2,
    /// Calculated size in pixels
    pub size: Vec2,
    /// Z-index for layering
    pub z_index: i32,
    /// Whether the node is visible
    pub visible: bool,
}

/// Background color component
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BackgroundColor {
    pub color: Color,
}

/// Border color component
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderColor {
    pub color: Color,
}

/// Border radius for rounded corners
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderRadius {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

/// Computed layout result
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ComputedNode {
    pub position: Vec2,
    pub size: Vec2,
    pub content_size: Vec2,
    pub padding: UiRect,
    pub border: UiRect,
    pub margin: UiRect,
}

/// UI vertex for rendering
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UiVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub color: u32,
}

/// UI render batch
#[repr(C)]
#[derive(Debug, Clone)]
pub struct UiBatch {
    pub z_index: i32,
    pub clip_rect: Option<[f32; 4]>,
    pub vertices: Vec<UiVertex>,
    pub indices: Vec<u32>,
}

// ============================================================================
// Zig FFI Functions
// ============================================================================

include_zig!("zig/ui_all.zig", {
    // Val functions
    fn val_px(value: f32) -> Val;
    fn val_percent(value: f32) -> Val;
    fn val_vw(value: f32) -> Val;
    fn val_vh(value: f32) -> Val;
    fn val_vmin(value: f32) -> Val;
    fn val_vmax(value: f32) -> Val;
    fn val_auto() -> Val;
    fn val_undefined() -> Val;
    fn val_to_pixels(val: Val, reference: f32) -> f32;
    
    // UiRect functions
    fn ui_rect_all(val: Val) -> UiRect;
    fn ui_rect_px(left: f32, right: f32, top: f32, bottom: f32) -> UiRect;
    fn ui_rect_percent(left: f32, right: f32, top: f32, bottom: f32) -> UiRect;
    fn ui_rect_zero() -> UiRect;
    
    // Size functions
    fn size_new(width: Val, height: Val) -> Size;
    fn size_px(width: f32, height: f32) -> Size;
    fn size_percent(width: f32, height: f32) -> Size;
    fn size_auto() -> Size;
    
    // Style functions
    fn style_default() -> Style;
    fn style_with_display(display: Display) -> Style;
    fn style_flex_row() -> Style;
    fn style_flex_column() -> Style;
    fn style_absolute(left: Val, top: Val, width: Val, height: Val) -> Style;
    
    // Node functions
    fn node_new(position: Vec2, size: Vec2, z_index: i32) -> Node;
    fn node_default() -> Node;
    fn node_contains_point(node: Node, point: Vec2) -> bool;
    fn node_set_visible(node: *mut Node, visible: bool) -> ();
    
    // BackgroundColor functions
    fn background_color_new(color: Color) -> BackgroundColor;
    fn background_color_transparent() -> BackgroundColor;
    
    // BorderColor functions
    fn border_color_new(color: Color) -> BorderColor;
    
    // BorderRadius functions
    fn border_radius_all(radius: f32) -> BorderRadius;
    fn border_radius_new(top_left: f32, top_right: f32, bottom_left: f32, bottom_right: f32) -> BorderRadius;
    fn border_radius_zero() -> BorderRadius;
    
    // FocusState functions
    fn focus_state_new(is_focused: bool, tab_index: i32) -> FocusState;
    fn focus_state_default() -> FocusState;
    
    // Layout calculation
    fn calculate_layout(style: Style, parent_size: Vec2, available_space: Vec2) -> ComputedNode;
    fn calculate_flex_layout(style: Style, children_count: usize, parent_size: Vec2) -> ComputedNode;
    
    // Interaction
    fn check_interaction(node: Node, mouse_pos: Vec2, mouse_pressed: bool) -> Interaction;
    fn is_hovered(node: Node, mouse_pos: Vec2) -> bool;
    fn is_pressed(node: Node, mouse_pos: Vec2, mouse_pressed: bool) -> bool;
    
    // Rendering
    fn create_ui_quad(node: Node, color: u32, border_radius: BorderRadius, out: *mut [UiVertex; 4]) -> ();
    fn create_ui_border(node: Node, border: UiRect, color: u32, out: *mut [UiVertex; 8]) -> ();
    fn pack_color(color: Color) -> u32;
    fn unpack_color(packed: u32) -> Color;
    
    // Batch operations
    fn ui_batch_new(z_index: i32) -> UiBatch;
    fn ui_batch_add_quad(batch: *mut UiBatch, vertices: *const UiVertex, vertex_count: usize) -> ();
    fn ui_batch_set_clip(batch: *mut UiBatch, clip_rect: *const [f32; 4]) -> ();
    fn ui_batch_sort_by_z(batches: *mut UiBatch, count: usize) -> ();
});

// ============================================================================
// Rust API Implementations
// ============================================================================

impl Val {
    pub const ZERO: Self = Self { value: 0.0, unit: Unit::Px };
    pub const AUTO: Self = Self { value: 0.0, unit: Unit::Auto };
    pub const UNDEFINED: Self = Self { value: 0.0, unit: Unit::Undefined };
    
    pub fn px(value: f32) -> Self {
        val_px(value)
    }
    
    pub fn percent(value: f32) -> Self {
        val_percent(value)
    }
    
    pub fn vw(value: f32) -> Self {
        val_vw(value)
    }
    
    pub fn vh(value: f32) -> Self {
        val_vh(value)
    }
    
    pub fn vmin(value: f32) -> Self {
        val_vmin(value)
    }
    
    pub fn vmax(value: f32) -> Self {
        val_vmax(value)
    }
    
    pub fn auto() -> Self {
        val_auto()
    }
    
    pub fn undefined() -> Self {
        val_undefined()
    }
    
    pub fn to_pixels(&self, reference: f32) -> f32 {
        val_to_pixels(*self, reference)
    }
}

impl Default for Val {
    fn default() -> Self {
        Self::UNDEFINED
    }
}

impl UiRect {
    pub const ZERO: Self = Self {
        left: Val::ZERO,
        right: Val::ZERO,
        top: Val::ZERO,
        bottom: Val::ZERO,
    };
    
    pub fn all(val: Val) -> Self {
        ui_rect_all(val)
    }
    
    pub fn px(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        ui_rect_px(left, right, top, bottom)
    }
    
    pub fn percent(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        ui_rect_percent(left, right, top, bottom)
    }
    
    pub fn zero() -> Self {
        ui_rect_zero()
    }
    
    pub fn horizontal(val: Val) -> Self {
        Self {
            left: val,
            right: val,
            top: Val::ZERO,
            bottom: Val::ZERO,
        }
    }
    
    pub fn vertical(val: Val) -> Self {
        Self {
            left: Val::ZERO,
            right: Val::ZERO,
            top: val,
            bottom: val,
        }
    }
}

impl Default for UiRect {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Size {
    pub const AUTO: Self = Self {
        width: Val::AUTO,
        height: Val::AUTO,
    };
    
    pub fn new(width: Val, height: Val) -> Self {
        size_new(width, height)
    }
    
    pub fn px(width: f32, height: f32) -> Self {
        size_px(width, height)
    }
    
    pub fn percent(width: f32, height: f32) -> Self {
        size_percent(width, height)
    }
    
    pub fn auto() -> Self {
        size_auto()
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::AUTO
    }
}

impl Style {
    pub fn default() -> Self {
        style_default()
    }
    
    pub fn with_display(display: Display) -> Self {
        style_with_display(display)
    }
    
    pub fn flex_row() -> Self {
        style_flex_row()
    }
    
    pub fn flex_column() -> Self {
        style_flex_column()
    }
    
    pub fn absolute(left: Val, top: Val, width: Val, height: Val) -> Self {
        style_absolute(left, top, width, height)
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::default()
    }
}

impl Node {
    pub fn new(position: Vec2, size: Vec2, z_index: i32) -> Self {
        node_new(position, size, z_index)
    }
    
    pub fn default() -> Self {
        node_default()
    }
    
    pub fn contains_point(&self, point: Vec2) -> bool {
        node_contains_point(*self, point)
    }
    
    pub fn set_visible(&mut self, visible: bool) {
        node_set_visible(self, visible);
    }
}

impl Default for Node {
    fn default() -> Self {
        Self::default()
    }
}

impl BackgroundColor {
    pub fn new(color: Color) -> Self {
        background_color_new(color)
    }
    
    pub fn transparent() -> Self {
        background_color_transparent()
    }
    
    pub const WHITE: Self = Self { color: Color::WHITE };
    pub const BLACK: Self = Self { color: Color::BLACK };
}

impl Default for BackgroundColor {
    fn default() -> Self {
        Self::transparent()
    }
}

impl BorderColor {
    pub fn new(color: Color) -> Self {
        border_color_new(color)
    }
    
    pub const WHITE: Self = Self { color: Color::WHITE };
    pub const BLACK: Self = Self { color: Color::BLACK };
}

impl Default for BorderColor {
    fn default() -> Self {
        Self::new(Color::BLACK)
    }
}

impl BorderRadius {
    pub fn all(radius: f32) -> Self {
        border_radius_all(radius)
    }
    
    pub fn new(top_left: f32, top_right: f32, bottom_left: f32, bottom_right: f32) -> Self {
        border_radius_new(top_left, top_right, bottom_left, bottom_right)
    }
    
    pub fn zero() -> Self {
        border_radius_zero()
    }
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self::zero()
    }
}

impl FocusState {
    pub fn new(is_focused: bool, tab_index: i32) -> Self {
        focus_state_new(is_focused, tab_index)
    }
    
    pub fn focused(tab_index: i32) -> Self {
        Self::new(true, tab_index)
    }
    
    pub fn unfocused() -> Self {
        Self::new(false, -1)
    }
}

impl Default for FocusState {
    fn default() -> Self {
        focus_state_default()
    }
}

impl ComputedNode {
    pub fn calculate(style: Style, parent_size: Vec2, available_space: Vec2) -> Self {
        calculate_layout(style, parent_size, available_space)
    }
    
    pub fn calculate_flex(style: Style, children_count: usize, parent_size: Vec2) -> Self {
        calculate_flex_layout(style, children_count, parent_size)
    }
}

impl Interaction {
    pub fn check(node: Node, mouse_pos: Vec2, mouse_pressed: bool) -> Self {
        check_interaction(node, mouse_pos, mouse_pressed)
    }
    
    pub fn is_hovered(node: Node, mouse_pos: Vec2) -> bool {
        is_hovered(node, mouse_pos)
    }
    
    pub fn is_pressed(node: Node, mouse_pos: Vec2, mouse_pressed: bool) -> bool {
        is_pressed(node, mouse_pos, mouse_pressed)
    }
}

impl UiVertex {
    pub fn new(position: Vec3, uv: Vec2, color: Color) -> Self {
        Self {
            position: [position.x, position.y, position.z],
            uv: [uv.x, uv.y],
            color: pack_color(color),
        }
    }
}

impl UiBatch {
    pub fn new(z_index: i32) -> Self {
        ui_batch_new(z_index)
    }
    
    pub fn add_quad(&mut self, vertices: &[UiVertex]) {
        if vertices.len() >= 4 {
            ui_batch_add_quad(self, vertices.as_ptr(), vertices.len());
        }
    }
    
    pub fn set_clip_rect(&mut self, clip_rect: [f32; 4]) {
        ui_batch_set_clip(self, &clip_rect);
    }
    
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}

/// Create a UI quad for a node with background color
pub fn create_ui_quad_with_color(node: Node, color: Color, border_radius: BorderRadius) -> [UiVertex; 4] {
    let mut result = [UiVertex { position: [0.0; 3], uv: [0.0; 2], color: 0 }; 4];
    create_ui_quad(node, pack_color(color), border_radius, &mut result);
    result
}

/// Create a UI border for a node
pub fn create_ui_border_with_color(node: Node, border: UiRect, color: Color) -> [UiVertex; 8] {
    let mut result = [UiVertex { position: [0.0; 3], uv: [0.0; 2], color: 0 }; 8];
    create_ui_border(node, border, pack_color(color), &mut result);
    result
}

/// Sort UI batches by Z-index
pub fn sort_ui_batches(batches: &mut [UiBatch]) {
    if batches.is_empty() {
        return;
    }
    ui_batch_sort_by_z(batches.as_mut_ptr(), batches.len());
}

/// Pack color into u32 for GPU
pub fn pack_ui_color(color: Color) -> u32 {
    pack_color(color)
}

/// Unpack u32 color from GPU
pub fn unpack_ui_color(packed: u32) -> Color {
    unpack_color(packed)
}