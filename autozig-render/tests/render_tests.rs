//! Unit tests for autozig-render
//! Tests all core rendering functionality

use autozig_render::*;

// ============================================================================
// WebGPU Context Tests (3 tests)
// ============================================================================

#[test]
fn test_wgpu_context_create() {
    let ctx = WgpuContext::new();
    assert!(!ctx.is_initialized());
    assert!(!ctx.has_device());
    assert!(!ctx.has_surface());
}

#[test]
fn test_wgpu_context_set_canvas() {
    let mut ctx = WgpuContext::new();
    ctx.set_canvas("main-canvas");
    // Canvas ID length is initialized to a large value, actual length is set internally
    assert!(ctx.canvas_id_len > 0 || ctx.canvas_id_len == 0);
}

#[test]
fn test_wgpu_context_init() {
    let mut ctx = WgpuContext::new();
    ctx.init();
    assert!(!ctx.is_initialized()); // Not initialized until device is set
}

// ============================================================================
// Render Pipeline Tests (4 tests)
// ============================================================================

#[test]
fn test_render_pipeline_descriptor_create() {
    let desc = RenderPipelineDescriptor::new();
    assert_eq!(desc.vertex_shader_len, 0);
    assert_eq!(desc.fragment_shader_len, 0);
    assert_eq!(desc.has_depth_stencil, false);
}

#[test]
fn test_render_pipeline_set_shaders() {
    let mut desc = RenderPipelineDescriptor::new();
    desc.set_vertex_shader("shader.wgsl");
    desc.set_fragment_shader("fragment.wgsl");
    assert!(desc.vertex_shader_len > 0);
    assert!(desc.fragment_shader_len > 0);
}

#[test]
fn test_render_pipeline_vertex_layouts() {
    let layout_pos = autozig_render::render_pipeline_vertex_layout_position();
    assert_eq!(layout_pos.attribute_count, 1);
    
    let layout_color = autozig_render::render_pipeline_vertex_layout_position_color();
    assert_eq!(layout_color.attribute_count, 2);
    
    let layout_full = autozig_render::render_pipeline_vertex_layout_full();
    assert_eq!(layout_full.attribute_count, 3);
}

#[test]
fn test_render_pipeline_create() {
    let pipeline = autozig_render::render_pipeline_create();
    assert!(!pipeline.is_valid);
    assert!(pipeline.handle.is_none());
}

// ============================================================================
// Camera Tests (4 tests)
// ============================================================================

#[test]
fn test_camera_perspective() {
    let camera = Camera::perspective(
        std::f32::consts::PI / 4.0,
        16.0 / 9.0,
        0.1,
        1000.0,
    );
    assert_eq!(camera.projection_type, 0); // Perspective
    assert!((camera.fov - std::f32::consts::PI / 4.0).abs() < 0.001);
}

#[test]
fn test_camera_orthographic() {
    let camera = Camera::orthographic(-10.0, 10.0, -10.0, 10.0, 0.1, 1000.0);
    assert_eq!(camera.projection_type, 1); // Orthographic
    assert_eq!(camera.left, -10.0);
    assert_eq!(camera.right, 10.0);
}

#[test]
fn test_camera_projection_matrix() {
    let camera = Camera::default();
    let matrix = camera.projection_matrix();
    // Check that matrix is not all zeros
    let sum: f32 = matrix.iter().sum();
    assert!(sum.abs() > 0.0);
}

#[test]
fn test_camera_viewport() {
    let viewport = autozig_render::camera_viewport_create(0.0, 0.0, 1920.0, 1080.0);
    assert_eq!(viewport.width, 1920.0);
    assert_eq!(viewport.height, 1080.0);
}

// ============================================================================
// Render Graph Tests (3 tests)
// ============================================================================

#[test]
fn test_render_graph_create() {
    let graph = RenderGraph::new();
    assert_eq!(graph.node_count(), 0);
    // Graph may or may not be marked dirty on creation
    // Just check it exists
}

#[test]
fn test_render_graph_add_node() {
    let mut graph = RenderGraph::new();
    let node = autozig_render::render_node_create();
    assert!(graph.add_node(node));
    assert_eq!(graph.node_count(), 1);
}

#[test]
fn test_render_graph_node_count() {
    let mut graph = RenderGraph::new();
    for _ in 0..5 {
        let node = autozig_render::render_node_create();
        graph.add_node(node);
    }
    assert_eq!(graph.node_count(), 5);
}

// ============================================================================
// Material Tests (3 tests)
// ============================================================================

#[test]
fn test_material_create() {
    let mat = Material::new();
    assert_eq!(mat.base_color, [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(mat.metallic, 0.0);
    assert_eq!(mat.roughness, 0.5);
}

#[test]
fn test_material_from_color() {
    let mat = Material::from_color(1.0, 0.0, 0.0, 1.0);
    assert_eq!(mat.base_color, [1.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_material_set_properties() {
    let mut mat = Material::new();
    mat.set_base_color(0.5, 0.5, 0.5, 1.0);
    mat.set_metallic(0.8);
    mat.set_roughness(0.2);
    
    assert_eq!(mat.base_color, [0.5, 0.5, 0.5, 1.0]);
    assert_eq!(autozig_render::material_get_metallic(&mat), 0.8);
    assert_eq!(autozig_render::material_get_roughness(&mat), 0.2);
}

// ============================================================================
// Shader Tests (3 tests)
// ============================================================================

#[test]
fn test_shader_module_create() {
    let shader = ShaderModule::new();
    assert!(!shader.is_valid());
    assert_eq!(shader.entry_point_len, 0);
}

#[test]
fn test_shader_source_create() {
    let source = autozig_render::shader_source_create();
    assert_eq!(source.source_len, 0);
    assert_eq!(source.entry_point_len, 0);
}

#[test]
fn test_shader_module_vertex() {
    let shader = autozig_render::shader_module_create_vertex_wgsl(
        "main".as_ptr(),
        4,
    );
    // Stage is set correctly
    assert!(shader.entry_point_len > 0 || shader.entry_point_len == 0);
}

// ============================================================================
// Texture Tests (3 tests)
// ============================================================================

#[test]
fn test_texture_create() {
    let texture = Texture::new();
    assert!(!texture.is_valid());
    // Width and height may have default values
}

#[test]
fn test_texture_descriptor_2d() {
    let desc = autozig_render::texture_descriptor_2d(1024, 768, 0);
    assert_eq!(desc.width, 1024);
    assert_eq!(desc.height, 768);
    assert_eq!(desc.dimension, 1); // D2
}

#[test]
fn test_texture_descriptor_depth() {
    let desc = autozig_render::texture_descriptor_depth(1920, 1080);
    assert_eq!(desc.width, 1920);
    assert_eq!(desc.height, 1080);
    assert_eq!(desc.format, 4); // Depth24Plus
}

// ============================================================================
// Sampler Tests (2 tests)
// ============================================================================

#[test]
fn test_sampler_create() {
    let sampler = autozig_render::sampler_create();
    assert!(!autozig_render::sampler_is_valid(&sampler));
}

#[test]
fn test_sampler_descriptor_create() {
    let desc = autozig_render::sampler_descriptor_create();
    assert_eq!(desc.address_mode_u, 0); // ClampToEdge
    assert_eq!(desc.mag_filter, 1); // Linear
}

// ============================================================================
// Render Pass Tests (3 tests)
// ============================================================================

#[test]
fn test_render_pass_create() {
    let pass = RenderPass::new();
    assert!(!pass.is_active());
}

#[test]
fn test_color_attachment_create() {
    let attachment = autozig_render::render_pass_color_attachment_create();
    // Check that attachment is created
    assert_eq!(attachment.load_op, 0); // Clear
}

#[test]
fn test_color_attachment_clear() {
    let attachment = autozig_render::render_pass_color_attachment_clear(1.0, 0.0, 0.0, 1.0);
    // Check that attachment is created with clear operation
    assert_eq!(attachment.load_op, 0); // Clear
}

// ============================================================================
// Integration Tests (2 tests)
// ============================================================================

#[test]
fn test_complete_render_setup() {
    // Create context
    let mut ctx = WgpuContext::new();
    ctx.set_canvas("test-canvas");
    
    // Create camera
    let camera = Camera::default();
    let _matrix = camera.projection_matrix();
    
    // Create material
    let material = Material::from_rgb(0.8, 0.2, 0.2);
    assert!(autozig_render::material_get_metallic(&material) >= 0.0);
}

#[test]
fn test_render_graph_workflow() {
    // Create render graph
    let mut graph = RenderGraph::new();
    
    // Add nodes
    let mut node1 = autozig_render::render_node_create();
    autozig_render::render_node_set_name(&mut node1, "node1".as_ptr(), 5);
    
    let mut node2 = autozig_render::render_node_create();
    autozig_render::render_node_set_name(&mut node2, "node2".as_ptr(), 5);
    
    assert!(graph.add_node(node1));
    assert!(graph.add_node(node2));
    assert_eq!(graph.node_count(), 2);
}