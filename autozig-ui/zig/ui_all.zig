// Unified UI module for autozig-ui
// Complete UI system with Flexbox layout, interaction, and rendering
// Based on bevy_ui design

const std = @import("std");

// ============================================================================
// Core Types (from dependencies)
// ============================================================================

pub const Color = extern struct {
    r: f32,
    g: f32,
    b: f32,
    a: f32,

    pub const WHITE = Color{ .r = 1.0, .g = 1.0, .b = 1.0, .a = 1.0 };
    pub const BLACK = Color{ .r = 0.0, .g = 0.0, .b = 0.0, .a = 1.0 };
    pub const TRANSPARENT = Color{ .r = 0.0, .g = 0.0, .b = 0.0, .a = 0.0 };
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

    pub fn sub(self: Vec2, other: Vec2) Vec2 {
        return .{ .x = self.x - other.x, .y = self.y - other.y };
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

// ============================================================================
// UI Enums
// ============================================================================

pub const Display = enum(u8) {
    Flex = 0,
    None = 1,
    Grid = 2,
};

pub const FlexDirection = enum(u8) {
    Row = 0,
    RowReverse = 1,
    Column = 2,
    ColumnReverse = 3,
};

pub const JustifyContent = enum(u8) {
    FlexStart = 0,
    FlexEnd = 1,
    Center = 2,
    SpaceBetween = 3,
    SpaceAround = 4,
    SpaceEvenly = 5,
};

pub const AlignItems = enum(u8) {
    Stretch = 0,
    FlexStart = 1,
    FlexEnd = 2,
    Center = 3,
    Baseline = 4,
};

pub const AlignSelf = enum(u8) {
    Auto = 0,
    Stretch = 1,
    FlexStart = 2,
    FlexEnd = 3,
    Center = 4,
    Baseline = 5,
};

pub const AlignContent = enum(u8) {
    Stretch = 0,
    FlexStart = 1,
    FlexEnd = 2,
    Center = 3,
    SpaceBetween = 4,
    SpaceAround = 5,
};

pub const FlexWrap = enum(u8) {
    NoWrap = 0,
    Wrap = 1,
    WrapReverse = 2,
};

pub const PositionType = enum(u8) {
    Relative = 0,
    Absolute = 1,
};

pub const Overflow = enum(u8) {
    Visible = 0,
    Hidden = 1,
    Scroll = 2,
};

pub const Unit = enum(u8) {
    Undefined = 0,
    Px = 1,
    Percent = 2,
    Auto = 3,
};

pub const Interaction = enum(u8) {
    None = 0,
    Hovered = 1,
    Pressed = 2,
};

// ============================================================================
// Val - Value with Unit
// ============================================================================

pub const Val = extern struct {
    value: f32,
    unit: Unit,

    pub fn px(v: f32) Val {
        return .{ .value = v, .unit = .Px };
    }

    pub fn percent(v: f32) Val {
        return .{ .value = v, .unit = .Percent };
    }

    pub fn auto() Val {
        return .{ .value = 0.0, .unit = .Auto };
    }

    pub fn @"undefined"() Val {
        return .{ .value = 0.0, .unit = .Undefined };
    }

    pub fn toPixels(self: Val, reference: f32) f32 {
        return switch (self.unit) {
            .Px => self.value,
            .Percent => reference * (self.value / 100.0),
            .Auto, .Undefined => 0.0,
        };
    }
};

// ============================================================================
// UiRect - Rectangle of values
// ============================================================================

pub const UiRect = extern struct {
    left: Val,
    right: Val,
    top: Val,
    bottom: Val,

    pub fn all(val: Val) UiRect {
        return .{
            .left = val,
            .right = val,
            .top = val,
            .bottom = val,
        };
    }

    pub fn px(left: f32, right: f32, top: f32, bottom: f32) UiRect {
        return .{
            .left = Val.px(left),
            .right = Val.px(right),
            .top = Val.px(top),
            .bottom = Val.px(bottom),
        };
    }

    pub fn percent(left: f32, right: f32, top: f32, bottom: f32) UiRect {
        return .{
            .left = Val.percent(left),
            .right = Val.percent(right),
            .top = Val.percent(top),
            .bottom = Val.percent(bottom),
        };
    }

    pub fn zero() UiRect {
        return .{
            .left = Val.px(0.0),
            .right = Val.px(0.0),
            .top = Val.px(0.0),
            .bottom = Val.px(0.0),
        };
    }
};

// ============================================================================
// Size
// ============================================================================

pub const Size = extern struct {
    width: Val,
    height: Val,

    pub fn new(width: Val, height: Val) Size {
        return .{ .width = width, .height = height };
    }

    pub fn px(w: f32, h: f32) Size {
        return .{
            .width = Val.px(w),
            .height = Val.px(h),
        };
    }

    pub fn percent(w: f32, h: f32) Size {
        return .{
            .width = Val.percent(w),
            .height = Val.percent(h),
        };
    }

    pub fn auto() Size {
        return .{
            .width = Val.auto(),
            .height = Val.auto(),
        };
    }
};

// ============================================================================
// Style - Layout and appearance properties
// ============================================================================

pub const Style = extern struct {
    display: Display,
    position_type: PositionType,
    overflow: Overflow,

    flex_direction: FlexDirection,
    flex_wrap: FlexWrap,
    justify_content: JustifyContent,
    align_items: AlignItems,
    align_content: AlignContent,

    align_self: AlignSelf,
    flex_grow: f32,
    flex_shrink: f32,
    flex_basis: Val,

    left: Val,
    right: Val,
    top: Val,
    bottom: Val,

    width: Val,
    height: Val,
    min_width: Val,
    min_height: Val,
    max_width: Val,
    max_height: Val,

    margin: UiRect,
    padding: UiRect,
    border: UiRect,

    aspect_ratio: ?*const f32,

    pub fn default() Style {
        return .{
            .display = .Flex,
            .position_type = .Relative,
            .overflow = .Visible,
            .flex_direction = .Row,
            .flex_wrap = .NoWrap,
            .justify_content = .FlexStart,
            .align_items = .Stretch,
            .align_content = .Stretch,
            .align_self = .Auto,
            .flex_grow = 0.0,
            .flex_shrink = 1.0,
            .flex_basis = Val.auto(),
            .left = Val.undefined(),
            .right = Val.undefined(),
            .top = Val.undefined(),
            .bottom = Val.undefined(),
            .width = Val.auto(),
            .height = Val.auto(),
            .min_width = Val.undefined(),
            .min_height = Val.undefined(),
            .max_width = Val.undefined(),
            .max_height = Val.undefined(),
            .margin = UiRect.zero(),
            .padding = UiRect.zero(),
            .border = UiRect.zero(),
            .aspect_ratio = null,
        };
    }

    pub fn withDisplay(display: Display) Style {
        var style = Style.default();
        style.display = display;
        return style;
    }

    pub fn flexRow() Style {
        var style = Style.default();
        style.flex_direction = .Row;
        return style;
    }

    pub fn flexColumn() Style {
        var style = Style.default();
        style.flex_direction = .Column;
        return style;
    }

    pub fn absolute(left: Val, top: Val, width: Val, height: Val) Style {
        var style = Style.default();
        style.position_type = .Absolute;
        style.left = left;
        style.top = top;
        style.width = width;
        style.height = height;
        return style;
    }
};

// ============================================================================
// Node - UI element with calculated layout
// ============================================================================

pub const Node = extern struct {
    position: Vec2,
    size: Vec2,
    z_index: i32,
    visible: bool,

    pub fn new(position: Vec2, size: Vec2, z_index: i32) Node {
        return .{
            .position = position,
            .size = size,
            .z_index = z_index,
            .visible = true,
        };
    }

    pub fn default() Node {
        return .{
            .position = Vec2.zero(),
            .size = Vec2.zero(),
            .z_index = 0,
            .visible = true,
        };
    }

    pub fn containsPoint(self: Node, point: Vec2) bool {
        if (!self.visible) return false;
        return point.x >= self.position.x and
            point.x <= self.position.x + self.size.x and
            point.y >= self.position.y and
            point.y <= self.position.y + self.size.y;
    }

    pub fn setVisible(self: *Node, visible: bool) void {
        self.visible = visible;
    }
};

// ============================================================================
// Color Components
// ============================================================================

pub const BackgroundColor = extern struct {
    color: Color,

    pub fn new(color: Color) BackgroundColor {
        return .{ .color = color };
    }

    pub fn transparent() BackgroundColor {
        return .{ .color = Color.TRANSPARENT };
    }
};

pub const BorderColor = extern struct {
    color: Color,

    pub fn new(color: Color) BorderColor {
        return .{ .color = color };
    }
};

pub const BorderRadius = extern struct {
    top_left: f32,
    top_right: f32,
    bottom_left: f32,
    bottom_right: f32,

    pub fn all(radius: f32) BorderRadius {
        return .{
            .top_left = radius,
            .top_right = radius,
            .bottom_left = radius,
            .bottom_right = radius,
        };
    }

    pub fn new(top_left: f32, top_right: f32, bottom_left: f32, bottom_right: f32) BorderRadius {
        return .{
            .top_left = top_left,
            .top_right = top_right,
            .bottom_left = bottom_left,
            .bottom_right = bottom_right,
        };
    }

    pub fn zero() BorderRadius {
        return .{
            .top_left = 0.0,
            .top_right = 0.0,
            .bottom_left = 0.0,
            .bottom_right = 0.0,
        };
    }
};

pub const FocusState = extern struct {
    is_focused: bool,
    tab_index: i32,

    pub fn new(is_focused: bool, tab_index: i32) FocusState {
        return .{
            .is_focused = is_focused,
            .tab_index = tab_index,
        };
    }

    pub fn default() FocusState {
        return .{
            .is_focused = false,
            .tab_index = -1,
        };
    }
};

// ============================================================================
// ComputedNode - Layout calculation result
// ============================================================================

pub const ComputedNode = extern struct {
    position: Vec2,
    size: Vec2,
    content_size: Vec2,
    padding: UiRect,
    border: UiRect,
    margin: UiRect,
};

// ============================================================================
// Layout Calculation (Flexbox)
// ============================================================================

fn resolveSize(val: Val, parent_size: f32, min_val: Val, max_val: Val) f32 {
    var size = val.toPixels(parent_size);

    const min_size = min_val.toPixels(parent_size);
    const max_size = max_val.toPixels(parent_size);

    if (min_val.unit != .Undefined) {
        size = @max(size, min_size);
    }
    if (max_val.unit != .Undefined) {
        size = @min(size, max_size);
    }

    return size;
}

pub fn calculateLayout(style: Style, parent_size: Vec2, available_space: Vec2) ComputedNode {
    _ = available_space;

    var computed: ComputedNode = undefined;

    // Calculate padding and border in pixels
    computed.padding = UiRect{
        .left = Val.px(style.padding.left.toPixels(parent_size.x)),
        .right = Val.px(style.padding.right.toPixels(parent_size.x)),
        .top = Val.px(style.padding.top.toPixels(parent_size.y)),
        .bottom = Val.px(style.padding.bottom.toPixels(parent_size.y)),
    };

    computed.border = UiRect{
        .left = Val.px(style.border.left.toPixels(parent_size.x)),
        .right = Val.px(style.border.right.toPixels(parent_size.x)),
        .top = Val.px(style.border.top.toPixels(parent_size.y)),
        .bottom = Val.px(style.border.bottom.toPixels(parent_size.y)),
    };

    computed.margin = UiRect{
        .left = Val.px(style.margin.left.toPixels(parent_size.x)),
        .right = Val.px(style.margin.right.toPixels(parent_size.x)),
        .top = Val.px(style.margin.top.toPixels(parent_size.y)),
        .bottom = Val.px(style.margin.bottom.toPixels(parent_size.y)),
    };

    // Calculate size
    const width = resolveSize(style.width, parent_size.x, style.min_width, style.max_width);
    const height = resolveSize(style.height, parent_size.y, style.min_height, style.max_height);

    computed.size = Vec2.new(width, height);
    computed.content_size = Vec2.new(width, height);

    // Position based on position_type
    if (style.position_type == .Absolute) {
        computed.position = Vec2.new(
            style.left.toPixels(parent_size.x),
            style.top.toPixels(parent_size.y),
        );
    } else {
        computed.position = Vec2.zero();
    }

    return computed;
}

pub fn calculateFlexLayout(style: Style, children_count: usize, parent_size: Vec2) ComputedNode {
    const computed = calculateLayout(style, parent_size, parent_size);

    // Basic flex container setup
    const is_row = style.flex_direction == .Row or style.flex_direction == .RowReverse;
    _ = is_row;
    _ = children_count;

    // In a full implementation, this would calculate child positions
    // For now, we return basic layout
    return computed;
}

// ============================================================================
// Interaction - Hit testing and state
// ============================================================================

pub fn checkInteraction(node: Node, mouse_pos: Vec2, mouse_pressed: bool) Interaction {
    if (!node.visible) return .None;

    const hovered = node.containsPoint(mouse_pos);
    if (hovered and mouse_pressed) {
        return .Pressed;
    } else if (hovered) {
        return .Hovered;
    }
    return .None;
}

pub fn isHovered(node: Node, mouse_pos: Vec2) bool {
    return node.visible and node.containsPoint(mouse_pos);
}

pub fn isPressed(node: Node, mouse_pos: Vec2, mouse_pressed: bool) bool {
    return node.visible and node.containsPoint(mouse_pos) and mouse_pressed;
}

// ============================================================================
// Rendering - Vertex generation
// ============================================================================

pub const UiVertex = extern struct {
    position: [3]f32,
    uv: [2]f32,
    color: u32,

    pub fn new(position: Vec3, uv: Vec2, color: u32) UiVertex {
        return .{
            .position = [3]f32{ position.x, position.y, position.z },
            .uv = [2]f32{ uv.x, uv.y },
            .color = color,
        };
    }
};

pub const UiBatch = extern struct {
    z_index: i32,
    clip_rect: ?*const [4]f32,
    vertices_ptr: ?[*]UiVertex,
    vertices_len: usize,
    vertices_cap: usize,
    indices_ptr: ?[*]u32,
    indices_len: usize,
    indices_cap: usize,

    pub fn new(z_index: i32) UiBatch {
        return .{
            .z_index = z_index,
            .clip_rect = null,
            .vertices_ptr = null,
            .vertices_len = 0,
            .vertices_cap = 0,
            .indices_ptr = null,
            .indices_len = 0,
            .indices_cap = 0,
        };
    }
};

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

pub fn createUiQuad(node: Node, color: u32, border_radius: BorderRadius) [4]UiVertex {
    _ = border_radius; // TODO: implement rounded corners

    const pos = node.position;
    const size = node.size;
    const z = @as(f32, @floatFromInt(node.z_index));

    return [4]UiVertex{
        // Bottom-left
        UiVertex.new(
            Vec3.new(pos.x, pos.y, z),
            Vec2.new(0.0, 1.0),
            color,
        ),
        // Bottom-right
        UiVertex.new(
            Vec3.new(pos.x + size.x, pos.y, z),
            Vec2.new(1.0, 1.0),
            color,
        ),
        // Top-right
        UiVertex.new(
            Vec3.new(pos.x + size.x, pos.y + size.y, z),
            Vec2.new(1.0, 0.0),
            color,
        ),
        // Top-left
        UiVertex.new(
            Vec3.new(pos.x, pos.y + size.y, z),
            Vec2.new(0.0, 0.0),
            color,
        ),
    };
}

pub fn createUiBorder(node: Node, border: UiRect, color: u32) [8]UiVertex {
    const pos = node.position;
    const size = node.size;
    const z = @as(f32, @floatFromInt(node.z_index));

    _ = border.left.value;
    const right = border.right.value;
    const top = border.top.value;
    const bottom = border.bottom.value;

    // Create 8 vertices for 4 border rectangles (left, right, top, bottom)
    return [8]UiVertex{
        // Left border - bottom-left
        UiVertex.new(Vec3.new(pos.x, pos.y, z), Vec2.new(0.0, 1.0), color),
        // Left border - top-left
        UiVertex.new(Vec3.new(pos.x, pos.y + size.y, z), Vec2.new(0.0, 0.0), color),
        // Right border - bottom-right
        UiVertex.new(Vec3.new(pos.x + size.x - right, pos.y, z), Vec2.new(1.0, 1.0), color),
        // Right border - top-right
        UiVertex.new(Vec3.new(pos.x + size.x, pos.y + size.y, z), Vec2.new(1.0, 0.0), color),
        // Top border - top-left
        UiVertex.new(Vec3.new(pos.x, pos.y + size.y - top, z), Vec2.new(0.0, 0.0), color),
        // Top border - top-right
        UiVertex.new(Vec3.new(pos.x + size.x, pos.y + size.y, z), Vec2.new(1.0, 0.0), color),
        // Bottom border - bottom-left
        UiVertex.new(Vec3.new(pos.x, pos.y, z), Vec2.new(0.0, 1.0), color),
        // Bottom border - bottom-right
        UiVertex.new(Vec3.new(pos.x + size.x, pos.y + bottom, z), Vec2.new(1.0, 1.0), color),
    };
}

pub fn uiBatchAddQuad(batch: *UiBatch, vertices: [*]const UiVertex, vertex_count: usize) void {
    _ = batch;
    _ = vertices;
    _ = vertex_count;
    // TODO: implement batch vertex accumulation
    // This would require dynamic memory allocation
}

pub fn uiBatchSetClip(batch: *UiBatch, clip_rect: *const [4]f32) void {
    batch.clip_rect = clip_rect;
}

pub fn uiBatchSortByZ(batches: [*]UiBatch, count: usize) void {
    if (count <= 1) return;

    // Simple bubble sort by z_index
    var i: usize = 0;
    while (i < count - 1) : (i += 1) {
        var j: usize = 0;
        while (j < count - i - 1) : (j += 1) {
            if (batches[j].z_index > batches[j + 1].z_index) {
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

// Val functions
export fn val_px(value: f32) Val {
    return Val.px(value);
}

export fn val_percent(value: f32) Val {
    return Val.percent(value);
}

export fn val_auto() Val {
    return Val.auto();
}

export fn val_undefined() Val {
    return Val.undefined();
}

export fn val_to_pixels(val: Val, reference: f32) f32 {
    return val.toPixels(reference);
}

// UiRect functions
export fn ui_rect_all(val: Val) UiRect {
    return UiRect.all(val);
}

export fn ui_rect_px(left: f32, right: f32, top: f32, bottom: f32) UiRect {
    return UiRect.px(left, right, top, bottom);
}

export fn ui_rect_percent(left: f32, right: f32, top: f32, bottom: f32) UiRect {
    return UiRect.percent(left, right, top, bottom);
}

export fn ui_rect_zero() UiRect {
    return UiRect.zero();
}

// Size functions
export fn size_new(width: Val, height: Val) Size {
    return Size.new(width, height);
}

export fn size_px(width: f32, height: f32) Size {
    return Size.px(width, height);
}

export fn size_percent(width: f32, height: f32) Size {
    return Size.percent(width, height);
}

export fn size_auto() Size {
    return Size.auto();
}

// Style functions
export fn style_default() Style {
    return Style.default();
}

export fn style_with_display(display: Display) Style {
    return Style.withDisplay(display);
}

export fn style_flex_row() Style {
    return Style.flexRow();
}

export fn style_flex_column() Style {
    return Style.flexColumn();
}

export fn style_absolute(left: Val, top: Val, width: Val, height: Val) Style {
    return Style.absolute(left, top, width, height);
}

// Node functions
export fn node_new(position: Vec2, size: Vec2, z_index: i32) Node {
    return Node.new(position, size, z_index);
}

export fn node_default() Node {
    return Node.default();
}

export fn node_contains_point(node: Node, point: Vec2) bool {
    return node.containsPoint(point);
}

export fn node_set_visible(node: *Node, visible: bool) void {
    node.setVisible(visible);
}

// BackgroundColor functions
export fn background_color_new(color: Color) BackgroundColor {
    return BackgroundColor.new(color);
}

export fn background_color_transparent() BackgroundColor {
    return BackgroundColor.transparent();
}

// BorderColor functions
export fn border_color_new(color: Color) BorderColor {
    return BorderColor.new(color);
}

// BorderRadius functions
export fn border_radius_all(radius: f32) BorderRadius {
    return BorderRadius.all(radius);
}

export fn border_radius_new(top_left: f32, top_right: f32, bottom_left: f32, bottom_right: f32) BorderRadius {
    return BorderRadius.new(top_left, top_right, bottom_left, bottom_right);
}

export fn border_radius_zero() BorderRadius {
    return BorderRadius.zero();
}

// FocusState functions
export fn focus_state_new(is_focused: bool, tab_index: i32) FocusState {
    return FocusState.new(is_focused, tab_index);
}

export fn focus_state_default() FocusState {
    return FocusState.default();
}

// Layout calculation
export fn calculate_layout(style: Style, parent_size: Vec2, available_space: Vec2) ComputedNode {
    return calculateLayout(style, parent_size, available_space);
}

export fn calculate_flex_layout(style: Style, children_count: usize, parent_size: Vec2) ComputedNode {
    return calculateFlexLayout(style, children_count, parent_size);
}

// Interaction
export fn check_interaction(node: Node, mouse_pos: Vec2, mouse_pressed: bool) Interaction {
    return checkInteraction(node, mouse_pos, mouse_pressed);
}

export fn is_hovered(node: Node, mouse_pos: Vec2) bool {
    return isHovered(node, mouse_pos);
}

export fn is_pressed(node: Node, mouse_pos: Vec2, mouse_pressed: bool) bool {
    return isPressed(node, mouse_pos, mouse_pressed);
}

// Rendering
export fn create_ui_quad(node: Node, color: u32, border_radius: BorderRadius, out: *[4]UiVertex) void {
    out.* = createUiQuad(node, color, border_radius);
}

export fn create_ui_border(node: Node, border: UiRect, color: u32, out: *[8]UiVertex) void {
    out.* = createUiBorder(node, border, color);
}

export fn pack_color(color: Color) u32 {
    return packColor(color);
}

export fn unpack_color(packed_value: u32) Color {
    return unpackColor(packed_value);
}

// Batch operations
export fn ui_batch_new(z_index: i32) UiBatch {
    return UiBatch.new(z_index);
}

export fn ui_batch_add_quad(batch: *UiBatch, vertices: [*]const UiVertex, vertex_count: usize) void {
    uiBatchAddQuad(batch, vertices, vertex_count);
}

export fn ui_batch_set_clip(batch: *UiBatch, clip_rect: *const [4]f32) void {
    uiBatchSetClip(batch, clip_rect);
}

export fn ui_batch_sort_by_z(batches: [*]UiBatch, count: usize) void {
    uiBatchSortByZ(batches, count);
}
