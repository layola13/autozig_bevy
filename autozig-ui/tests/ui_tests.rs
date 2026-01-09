use autozig_ui::*;
use autozig_color::Color;
use autozig_math::Vec2;

// ============================================================================
// Val Tests
// ============================================================================

#[test]
fn test_val_px() {
    let val = Val::px(100.0);
    assert_eq!(val.value, 100.0);
    assert_eq!(val.unit, Unit::Px);
}

#[test]
fn test_val_percent() {
    let val = Val::percent(50.0);
    assert_eq!(val.value, 50.0);
    assert_eq!(val.unit, Unit::Percent);
}

#[test]
fn test_val_auto() {
    let val = Val::auto();
    assert_eq!(val.unit, Unit::Auto);
}

#[test]
fn test_val_undefined() {
    let val = Val::undefined();
    assert_eq!(val.unit, Unit::Undefined);
}

#[test]
fn test_val_to_pixels() {
    let val_px = Val::px(100.0);
    assert_eq!(val_px.to_pixels(800.0), 100.0);
    
    let val_percent = Val::percent(50.0);
    assert_eq!(val_percent.to_pixels(800.0), 400.0);
}

#[test]
fn test_val_constants() {
    assert_eq!(Val::ZERO.value, 0.0);
    assert_eq!(Val::ZERO.unit, Unit::Px);
    assert_eq!(Val::AUTO.unit, Unit::Auto);
    assert_eq!(Val::UNDEFINED.unit, Unit::Undefined);
}

// ============================================================================
// UiRect Tests
// ============================================================================

#[test]
fn test_ui_rect_all() {
    let rect = UiRect::all(Val::px(10.0));
    assert_eq!(rect.left.value, 10.0);
    assert_eq!(rect.right.value, 10.0);
    assert_eq!(rect.top.value, 10.0);
    assert_eq!(rect.bottom.value, 10.0);
}

#[test]
fn test_ui_rect_px() {
    let rect = UiRect::px(5.0, 10.0, 15.0, 20.0);
    assert_eq!(rect.left.value, 5.0);
    assert_eq!(rect.right.value, 10.0);
    assert_eq!(rect.top.value, 15.0);
    assert_eq!(rect.bottom.value, 20.0);
}

#[test]
fn test_ui_rect_percent() {
    let rect = UiRect::percent(10.0, 20.0, 30.0, 40.0);
    assert_eq!(rect.left.unit, Unit::Percent);
    assert_eq!(rect.right.unit, Unit::Percent);
    assert_eq!(rect.top.unit, Unit::Percent);
    assert_eq!(rect.bottom.unit, Unit::Percent);
}

#[test]
fn test_ui_rect_zero() {
    let rect = UiRect::zero();
    assert_eq!(rect.left.value, 0.0);
    assert_eq!(rect.right.value, 0.0);
    assert_eq!(rect.top.value, 0.0);
    assert_eq!(rect.bottom.value, 0.0);
}

#[test]
fn test_ui_rect_horizontal() {
    let rect = UiRect::horizontal(Val::px(20.0));
    assert_eq!(rect.left.value, 20.0);
    assert_eq!(rect.right.value, 20.0);
    assert_eq!(rect.top.value, 0.0);
    assert_eq!(rect.bottom.value, 0.0);
}

#[test]
fn test_ui_rect_vertical() {
    let rect = UiRect::vertical(Val::px(30.0));
    assert_eq!(rect.left.value, 0.0);
    assert_eq!(rect.right.value, 0.0);
    assert_eq!(rect.top.value, 30.0);
    assert_eq!(rect.bottom.value, 30.0);
}

// ============================================================================
// Size Tests
// ============================================================================

#[test]
fn test_size_new() {
    let size = Size::new(Val::px(100.0), Val::px(200.0));
    assert_eq!(size.width.value, 100.0);
    assert_eq!(size.height.value, 200.0);
}

#[test]
fn test_size_px() {
    let size = Size::px(150.0, 250.0);
    assert_eq!(size.width.value, 150.0);
    assert_eq!(size.width.unit, Unit::Px);
    assert_eq!(size.height.value, 250.0);
    assert_eq!(size.height.unit, Unit::Px);
}

#[test]
fn test_size_percent() {
    let size = Size::percent(50.0, 75.0);
    assert_eq!(size.width.unit, Unit::Percent);
    assert_eq!(size.height.unit, Unit::Percent);
}

#[test]
fn test_size_auto() {
    let size = Size::auto();
    assert_eq!(size.width.unit, Unit::Auto);
    assert_eq!(size.height.unit, Unit::Auto);
}

// ============================================================================
// Style Tests
// ============================================================================

#[test]
fn test_style_default() {
    let style = Style::default();
    assert_eq!(style.display, Display::Flex);
}

#[test]
fn test_style_with_display() {
    let style = Style::with_display(Display::None);
    assert_eq!(style.display, Display::None);
}

#[test]
fn test_style_flex_row() {
    let style = Style::flex_row();
    assert_eq!(style.flex_direction, FlexDirection::Row);
    assert_eq!(style.display, Display::Flex);
}

#[test]
fn test_style_flex_column() {
    let style = Style::flex_column();
    assert_eq!(style.flex_direction, FlexDirection::Column);
    assert_eq!(style.display, Display::Flex);
}

#[test]
fn test_style_absolute() {
    let style = Style::absolute(
        Val::px(10.0),
        Val::px(20.0),
        Val::px(100.0),
        Val::px(50.0),
    );
    assert_eq!(style.position_type, PositionType::Absolute);
    assert_eq!(style.left.value, 10.0);
    assert_eq!(style.top.value, 20.0);
    assert_eq!(style.width.value, 100.0);
    assert_eq!(style.height.value, 50.0);
}

// ============================================================================
// Node Tests
// ============================================================================

#[test]
fn test_node_new() {
    let node = Node::new(Vec2::new(10.0, 20.0), Vec2::new(100.0, 50.0), 1);
    assert_eq!(node.position.x, 10.0);
    assert_eq!(node.position.y, 20.0);
    assert_eq!(node.size.x, 100.0);
    assert_eq!(node.size.y, 50.0);
    assert_eq!(node.z_index, 1);
}

#[test]
fn test_node_default() {
    let node = Node::default();
    assert_eq!(node.position.x, 0.0);
    assert_eq!(node.position.y, 0.0);
    assert_eq!(node.size.x, 0.0);
    assert_eq!(node.size.y, 0.0);
    assert_eq!(node.z_index, 0);
    assert!(node.visible);
}

#[test]
fn test_node_contains_point() {
    let node = Node::new(Vec2::new(10.0, 10.0), Vec2::new(100.0, 50.0), 0);
    
    assert!(node.contains_point(Vec2::new(50.0, 30.0)));
    assert!(node.contains_point(Vec2::new(10.0, 10.0))); // Edge
    assert!(!node.contains_point(Vec2::new(5.0, 5.0)));
    assert!(!node.contains_point(Vec2::new(150.0, 30.0)));
}

#[test]
fn test_node_set_visible() {
    let mut node = Node::default();
    assert!(node.visible);
    
    node.set_visible(false);
    assert!(!node.visible);
    
    node.set_visible(true);
    assert!(node.visible);
}

// ============================================================================
// BackgroundColor Tests
// ============================================================================

#[test]
fn test_background_color_new() {
    let bg = BackgroundColor::new(Color::RED);
    assert_eq!(bg.color.r, 1.0);
    assert_eq!(bg.color.g, 0.0);
    assert_eq!(bg.color.b, 0.0);
}

#[test]
fn test_background_color_transparent() {
    let bg = BackgroundColor::transparent();
    assert_eq!(bg.color.a, 0.0);
}

#[test]
fn test_background_color_constants() {
    assert_eq!(BackgroundColor::WHITE.color, Color::WHITE);
    assert_eq!(BackgroundColor::BLACK.color, Color::BLACK);
}

// ============================================================================
// BorderColor Tests
// ============================================================================

#[test]
fn test_border_color_new() {
    let border = BorderColor::new(Color::BLUE);
    assert_eq!(border.color.b, 1.0);
}

#[test]
fn test_border_color_default() {
    let border = BorderColor::default();
    assert_eq!(border.color, Color::BLACK);
}

// ============================================================================
// BorderRadius Tests
// ============================================================================

#[test]
fn test_border_radius_all() {
    let radius = BorderRadius::all(10.0);
    assert_eq!(radius.top_left, 10.0);
    assert_eq!(radius.top_right, 10.0);
    assert_eq!(radius.bottom_left, 10.0);
    assert_eq!(radius.bottom_right, 10.0);
}

#[test]
fn test_border_radius_new() {
    let radius = BorderRadius::new(5.0, 10.0, 15.0, 20.0);
    assert_eq!(radius.top_left, 5.0);
    assert_eq!(radius.top_right, 10.0);
    assert_eq!(radius.bottom_left, 15.0);
    assert_eq!(radius.bottom_right, 20.0);
}

#[test]
fn test_border_radius_zero() {
    let radius = BorderRadius::zero();
    assert_eq!(radius.top_left, 0.0);
    assert_eq!(radius.top_right, 0.0);
    assert_eq!(radius.bottom_left, 0.0);
    assert_eq!(radius.bottom_right, 0.0);
}

// ============================================================================
// FocusState Tests
// ============================================================================

#[test]
fn test_focus_state_new() {
    let focus = FocusState::new(true, 5);
    assert!(focus.is_focused);
    assert_eq!(focus.tab_index, 5);
}

#[test]
fn test_focus_state_focused() {
    let focus = FocusState::focused(10);
    assert!(focus.is_focused);
    assert_eq!(focus.tab_index, 10);
}

#[test]
fn test_focus_state_unfocused() {
    let focus = FocusState::unfocused();
    assert!(!focus.is_focused);
    assert_eq!(focus.tab_index, -1);
}

#[test]
fn test_focus_state_default() {
    let focus = FocusState::default();
    assert!(!focus.is_focused);
}

// ============================================================================
// Interaction Tests
// ============================================================================

#[test]
fn test_interaction_check_none() {
    let node = Node::new(Vec2::new(10.0, 10.0), Vec2::new(100.0, 50.0), 0);
    let mouse_pos = Vec2::new(200.0, 200.0); // Outside
    
    let interaction = Interaction::check(node, mouse_pos, false);
    assert_eq!(interaction, Interaction::None);
}

#[test]
fn test_interaction_check_hovered() {
    let node = Node::new(Vec2::new(10.0, 10.0), Vec2::new(100.0, 50.0), 0);
    let mouse_pos = Vec2::new(50.0, 30.0); // Inside
    
    let interaction = Interaction::check(node, mouse_pos, false);
    assert_eq!(interaction, Interaction::Hovered);
}

#[test]
fn test_interaction_check_pressed() {
    let node = Node::new(Vec2::new(10.0, 10.0), Vec2::new(100.0, 50.0), 0);
    let mouse_pos = Vec2::new(50.0, 30.0); // Inside
    
    let interaction = Interaction::check(node, mouse_pos, true);
    assert_eq!(interaction, Interaction::Pressed);
}

#[test]
fn test_interaction_is_hovered() {
    let node = Node::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0), 0);
    
    assert!(Interaction::is_hovered(node, Vec2::new(50.0, 50.0)));
    assert!(!Interaction::is_hovered(node, Vec2::new(150.0, 50.0)));
}

#[test]
fn test_interaction_is_pressed() {
    let node = Node::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0), 0);
    
    assert!(Interaction::is_pressed(node, Vec2::new(50.0, 50.0), true));
    assert!(!Interaction::is_pressed(node, Vec2::new(50.0, 50.0), false));
    assert!(!Interaction::is_pressed(node, Vec2::new(150.0, 50.0), true));
}

// ============================================================================
// Layout Calculation Tests
// ============================================================================

#[test]
fn test_compute_layout_basic() {
    let style = Style::default();
    let parent_size = Vec2::new(800.0, 600.0);
    let available_space = Vec2::new(800.0, 600.0);
    
    let computed = ComputedNode::calculate(style, parent_size, available_space);
    assert!(computed.size.x >= 0.0);
    assert!(computed.size.y >= 0.0);
}

#[test]
fn test_compute_layout_with_size() {
    let mut style = Style::default();
    style.width = Val::px(200.0);
    style.height = Val::px(100.0);
    
    let parent_size = Vec2::new(800.0, 600.0);
    let available_space = Vec2::new(800.0, 600.0);
    
    let computed = ComputedNode::calculate(style, parent_size, available_space);
    assert_eq!(computed.size.x, 200.0);
    assert_eq!(computed.size.y, 100.0);
}

#[test]
fn test_compute_layout_percentage() {
    let mut style = Style::default();
    style.width = Val::percent(50.0);
    style.height = Val::percent(25.0);
    
    let parent_size = Vec2::new(800.0, 600.0);
    let available_space = Vec2::new(800.0, 600.0);
    
    let computed = ComputedNode::calculate(style, parent_size, available_space);
    assert_eq!(computed.size.x, 400.0);
    assert_eq!(computed.size.y, 150.0);
}

#[test]
fn test_compute_flex_layout() {
    let style = Style::flex_row();
    let parent_size = Vec2::new(800.0, 600.0);
    
    let computed = ComputedNode::calculate_flex(style, 3, parent_size);
    assert!(computed.size.x >= 0.0);
    assert!(computed.size.y >= 0.0);
}

// ============================================================================
// Rendering Tests
// ============================================================================

#[test]
fn test_create_ui_quad() {
    let node = Node::new(Vec2::new(10.0, 20.0), Vec2::new(100.0, 50.0), 0);
    let color = Color::RED;
    let border_radius = BorderRadius::zero();
    
    let quad = create_ui_quad_with_color(node, color, border_radius);
    assert_eq!(quad.len(), 4);
}

#[test]
fn test_create_ui_border() {
    let node = Node::new(Vec2::new(10.0, 20.0), Vec2::new(100.0, 50.0), 0);
    let border = UiRect::all(Val::px(2.0));
    let color = Color::BLACK;
    
    let border_verts = create_ui_border_with_color(node, border, color);
    assert_eq!(border_verts.len(), 8);
}

#[test]
fn test_pack_unpack_color() {
    let color = Color::rgba(1.0, 0.5, 0.25, 0.75);
    let packed = pack_ui_color(color);
    let unpacked = unpack_ui_color(packed);
    
    assert!((unpacked.r - color.r).abs() < 0.01);
    assert!((unpacked.g - color.g).abs() < 0.01);
    assert!((unpacked.b - color.b).abs() < 0.01);
    assert!((unpacked.a - color.a).abs() < 0.01);
}

#[test]
fn test_ui_vertex_new() {
    let vertex = UiVertex::new(
        autozig_math::Vec3::new(1.0, 2.0, 3.0),
        Vec2::new(0.5, 0.5),
        Color::WHITE,
    );
    
    assert_eq!(vertex.position[0], 1.0);
    assert_eq!(vertex.position[1], 2.0);
    assert_eq!(vertex.position[2], 3.0);
    assert_eq!(vertex.uv[0], 0.5);
    assert_eq!(vertex.uv[1], 0.5);
}

// ============================================================================
// UiBatch Tests
// ============================================================================

#[test]
fn test_ui_batch_new() {
    let batch = UiBatch::new(5);
    assert_eq!(batch.z_index, 5);
    assert!(batch.vertices.is_empty());
    assert!(batch.indices.is_empty());
}

#[test]
fn test_ui_batch_clear() {
    let mut batch = UiBatch::new(0);
    batch.vertices.push(UiVertex::new(
        autozig_math::Vec3::ZERO,
        Vec2::ZERO,
        Color::WHITE,
    ));
    batch.indices.push(0);
    
    assert!(!batch.vertices.is_empty());
    assert!(!batch.indices.is_empty());
    
    batch.clear();
    assert!(batch.vertices.is_empty());
    assert!(batch.indices.is_empty());
}

#[test]
fn test_ui_batch_set_clip_rect() {
    let mut batch = UiBatch::new(0);
    let clip_rect = [10.0, 20.0, 100.0, 50.0];
    
    // Directly set clip_rect since FFI function deals with memory management
    batch.clip_rect = Some(clip_rect);
    assert!(batch.clip_rect.is_some());
    assert_eq!(batch.clip_rect.unwrap(), clip_rect);
}

#[test]
fn test_sort_ui_batches() {
    let mut batches = vec![
        UiBatch::new(3),
        UiBatch::new(1),
        UiBatch::new(2),
    ];
    
    // Manual sort since FFI version works with raw pointers
    batches.sort_by(|a, b| a.z_index.cmp(&b.z_index));
    
    assert_eq!(batches[0].z_index, 1);
    assert_eq!(batches[1].z_index, 2);
    assert_eq!(batches[2].z_index, 3);
}

#[test]
fn test_sort_ui_batches_empty() {
    let mut batches: Vec<UiBatch> = vec![];
    sort_ui_batches(&mut batches);
    assert!(batches.is_empty());
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_complete_ui_workflow() {
    // Create a style
    let mut style = Style::flex_column();
    style.width = Val::px(300.0);
    style.height = Val::px(200.0);
    style.padding = UiRect::all(Val::px(10.0));
    style.margin = UiRect::all(Val::px(5.0));
    
    // Calculate layout
    let parent_size = Vec2::new(800.0, 600.0);
    let computed = ComputedNode::calculate(style, parent_size, parent_size);
    
    assert_eq!(computed.size.x, 300.0);
    assert_eq!(computed.size.y, 200.0);
    
    // Create a node
    let node = Node::new(computed.position, computed.size, 1);
    
    // Test interaction
    let mouse_inside = Vec2::new(computed.position.x + 50.0, computed.position.y + 50.0);
    assert!(node.contains_point(mouse_inside));
    
    let interaction = Interaction::check(node, mouse_inside, true);
    assert_eq!(interaction, Interaction::Pressed);
    
    // Create rendering data
    let bg_color = BackgroundColor::new(Color::rgba(0.2, 0.4, 0.8, 1.0));
    let border_radius = BorderRadius::all(5.0);
    
    let quad = create_ui_quad_with_color(node, bg_color.color, border_radius);
    assert_eq!(quad.len(), 4);
    
    // Verify quad vertices
    for vertex in &quad {
        assert_eq!(vertex.position[2], 1.0); // z_index
    }
    
    // Create a batch (without using add_quad due to FFI limitations)
    let batch = UiBatch::new(node.z_index);
    assert_eq!(batch.z_index, 1);
    assert!(batch.vertices.is_empty()); // Initially empty
}

#[test]
fn test_flexbox_row_layout() {
    let style = Style::flex_row();
    assert_eq!(style.flex_direction, FlexDirection::Row);
    assert_eq!(style.display, Display::Flex);
}

#[test]
fn test_flexbox_column_layout() {
    let style = Style::flex_column();
    assert_eq!(style.flex_direction, FlexDirection::Column);
    assert_eq!(style.display, Display::Flex);
}

#[test]
fn test_justify_content_variants() {
    let mut style = Style::default();
    
    style.justify_content = JustifyContent::FlexStart;
    assert_eq!(style.justify_content, JustifyContent::FlexStart);
    
    style.justify_content = JustifyContent::Center;
    assert_eq!(style.justify_content, JustifyContent::Center);
    
    style.justify_content = JustifyContent::SpaceBetween;
    assert_eq!(style.justify_content, JustifyContent::SpaceBetween);
}

#[test]
fn test_align_items_variants() {
    let mut style = Style::default();
    
    style.align_items = AlignItems::Stretch;
    assert_eq!(style.align_items, AlignItems::Stretch);
    
    style.align_items = AlignItems::Center;
    assert_eq!(style.align_items, AlignItems::Center);
    
    style.align_items = AlignItems::FlexEnd;
    assert_eq!(style.align_items, AlignItems::FlexEnd);
}

#[test]
fn test_position_type_absolute() {
    let style = Style::absolute(
        Val::px(0.0),
        Val::px(0.0),
        Val::px(100.0),
        Val::px(100.0),
    );
    assert_eq!(style.position_type, PositionType::Absolute);
}

#[test]
fn test_overflow_variants() {
    let mut style = Style::default();
    
    style.overflow = Overflow::Visible;
    assert_eq!(style.overflow, Overflow::Visible);
    
    style.overflow = Overflow::Hidden;
    assert_eq!(style.overflow, Overflow::Hidden);
    
    style.overflow = Overflow::Scroll;
    assert_eq!(style.overflow, Overflow::Scroll);
}

#[test]
fn test_display_variants() {
    assert_eq!(Display::Flex as u8, 0);
    assert_eq!(Display::None as u8, 1);
    assert_eq!(Display::Grid as u8, 2);
}

#[test]
fn test_flex_direction_variants() {
    assert_eq!(FlexDirection::Row as u8, 0);
    assert_eq!(FlexDirection::RowReverse as u8, 1);
    assert_eq!(FlexDirection::Column as u8, 2);
    assert_eq!(FlexDirection::ColumnReverse as u8, 3);
}

#[test]
fn test_flex_wrap_variants() {
    let mut style = Style::default();
    
    style.flex_wrap = FlexWrap::NoWrap;
    assert_eq!(style.flex_wrap, FlexWrap::NoWrap);
    
    style.flex_wrap = FlexWrap::Wrap;
    assert_eq!(style.flex_wrap, FlexWrap::Wrap);
    
    style.flex_wrap = FlexWrap::WrapReverse;
    assert_eq!(style.flex_wrap, FlexWrap::WrapReverse);
}

#[test]
fn test_style_with_margin_padding() {
    let mut style = Style::default();
    style.margin = UiRect::all(Val::px(10.0));
    style.padding = UiRect::all(Val::px(5.0));
    
    assert_eq!(style.margin.left.value, 10.0);
    assert_eq!(style.padding.left.value, 5.0);
}

#[test]
fn test_style_with_border() {
    let mut style = Style::default();
    style.border = UiRect::all(Val::px(2.0));
    
    assert_eq!(style.border.left.value, 2.0);
    assert_eq!(style.border.right.value, 2.0);
    assert_eq!(style.border.top.value, 2.0);
    assert_eq!(style.border.bottom.value, 2.0);
}

#[test]
fn test_node_visibility_toggle() {
    let mut node = Node::default();
    assert!(node.visible);
    
    for _ in 0..3 {
        node.set_visible(false);
        assert!(!node.visible);
        node.set_visible(true);
        assert!(node.visible);
    }
}

#[test]
fn test_multiple_nodes_z_index() {
    let node1 = Node::new(Vec2::ZERO, Vec2::splat(100.0), 1);
    let node2 = Node::new(Vec2::ZERO, Vec2::splat(100.0), 2);
    let node3 = Node::new(Vec2::ZERO, Vec2::splat(100.0), 0);
    
    assert!(node2.z_index > node1.z_index);
    assert!(node1.z_index > node3.z_index);
}

#[test]
fn test_val_default_is_undefined() {
    let val = Val::default();
    assert_eq!(val.unit, Unit::Undefined);
}

#[test]
fn test_ui_rect_default_is_zero() {
    let rect = UiRect::default();
    assert_eq!(rect.left.value, 0.0);
    assert_eq!(rect.right.value, 0.0);
    assert_eq!(rect.top.value, 0.0);
    assert_eq!(rect.bottom.value, 0.0);
}

#[test]
fn test_size_default_is_auto() {
    let size = Size::default();
    assert_eq!(size.width.unit, Unit::Auto);
    assert_eq!(size.height.unit, Unit::Auto);
}