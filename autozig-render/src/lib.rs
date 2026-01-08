
//! AutoZig Render - Bevy render system for WebGPU/WASM platforms
//! 
//! This crate provides WebGPU rendering capabilities using Zig for
//! high-performance graphics operations.

use autozig::include_zig;

// ============================================================================
// WebGPU Context
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WgpuContext {
    pub instance: Option<*mut std::ffi::c_void>,
    pub adapter: Option<*mut std::ffi::c_void>,
    pub device: Option<*mut std::ffi::c_void>,
    pub queue: Option<*mut std::ffi::c_void>,
    pub surface: Option<*mut std::ffi::c_void>,
    pub canvas_id: [u8; 128],
    pub canvas_id_len: u32,
    pub is_initialized: bool,
}

include_zig!("src/zig/wgpu_context.zig", {
    fn wgpu_context_create() -> WgpuContext;
    fn wgpu_context_init(ctx: *mut WgpuContext);
    fn wgpu_context_set_canvas(ctx: *mut WgpuContext, canvas_id: *const u8, len: u32);
    fn wgpu_context_set_instance(ctx: *mut WgpuContext, instance: Option<*mut std::ffi::c_void>);
    fn wgpu_context_set_adapter(ctx: *mut WgpuContext, adapter: Option<*mut std::ffi::c_void>);
    fn wgpu_context_set_device(ctx: *mut WgpuContext, device: Option<*mut std::ffi::c_void>);
    fn wgpu_context_set_queue(ctx: *mut WgpuContext, queue: Option<*mut std::ffi::c_void>);
    fn wgpu_context_set_surface(ctx: *mut WgpuContext, surface: Option<*mut std::ffi::c_void>);
    fn wgpu_context_mark_initialized(ctx: *mut WgpuContext);
    fn wgpu_context_is_initialized(ctx: *const WgpuContext) -> bool;
    fn wgpu_context_has_device(ctx: *const WgpuContext) -> bool;
    fn wgpu_context_has_surface(ctx: *const WgpuContext) -> bool;
    fn wgpu_context_get_canvas_id(ctx: *const WgpuContext, out_buffer: *mut u8, buffer_size: u32) -> u32;
    fn wgpu_context_deinit(ctx: *mut WgpuContext);
});

impl WgpuContext {
    pub fn new() -> Self {
        wgpu_context_create()
    }

    pub fn init(&mut self) {
        wgpu_context_init(self);
    }

    pub fn set_canvas(&mut self, canvas_id: &str) {
        wgpu_context_set_canvas(self, canvas_id.as_ptr(), canvas_id.len() as u32);
    }

    pub fn is_initialized(&self) -> bool {
        wgpu_context_is_initialized(self)
    }

    pub fn has_device(&self) -> bool {
        wgpu_context_has_device(self)
    }

    pub fn has_surface(&self) -> bool {
        wgpu_context_has_surface(self)
    }
}

impl Default for WgpuContext {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Render Pipeline
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VertexAttribute {
    pub format: u32,
    pub offset: u32,
    pub shader_location: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VertexLayout {
    pub attributes: [VertexAttribute; 8],
    pub attribute_count: u32,
    pub array_stride: u32,
    pub step_mode: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DepthStencilState {
    pub format: u32,
    pub depth_write_enabled: bool,
    pub depth_compare: u32,
    pub stencil_read_mask: u32,
    pub stencil_write_mask: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderPipelineDescriptor {
    pub vertex_shader: [u8; 128],
    pub fragment_shader: [u8; 128],
    pub vertex_shader_len: u32,
    pub fragment_shader_len: u32,
    pub vertex_layout: VertexLayout,
    pub primitive_topology: u32,
    pub has_depth_stencil: bool,
    pub depth_stencil: DepthStencilState,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderPipeline {
    pub handle: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
}

include_zig!("src/zig/render_pipeline.zig", {
    fn render_pipeline_default_vertex_layout() -> VertexLayout;
    fn render_pipeline_vertex_layout_position() -> VertexLayout;
    fn render_pipeline_vertex_layout_position_color() -> VertexLayout;
    fn render_pipeline_vertex_layout_full() -> VertexLayout;
    fn render_pipeline_descriptor_create() -> RenderPipelineDescriptor;
    fn render_pipeline_descriptor_set_vertex_shader(desc: *mut RenderPipelineDescriptor, path: *const u8, len: u32);
    fn render_pipeline_descriptor_set_fragment_shader(desc: *mut RenderPipelineDescriptor, path: *const u8, len: u32);
    fn render_pipeline_descriptor_set_vertex_layout(desc: *mut RenderPipelineDescriptor, layout: VertexLayout);
    fn render_pipeline_descriptor_set_topology(desc: *mut RenderPipelineDescriptor, topology: u32);
    fn render_pipeline_descriptor_enable_depth(desc: *mut RenderPipelineDescriptor, format: u32, write_enabled: bool);
    fn render_pipeline_create() -> RenderPipeline;
    fn render_pipeline_set_handle(pipeline: *mut RenderPipeline, handle: Option<*mut std::ffi::c_void>);
    fn render_pipeline_is_valid(pipeline: *const RenderPipeline) -> bool;
    fn render_pipeline_destroy(pipeline: *mut RenderPipeline);
});

impl RenderPipelineDescriptor {
    pub fn new() -> Self {
        render_pipeline_descriptor_create()
    }

    pub fn set_vertex_shader(&mut self, path: &str) {
        render_pipeline_descriptor_set_vertex_shader(self, path.as_ptr(), path.len() as u32);
    }

    pub fn set_fragment_shader(&mut self, path: &str) {
        render_pipeline_descriptor_set_fragment_shader(self, path.as_ptr(), path.len() as u32);
    }
}

impl Default for RenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Camera
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub projection_type: u32,
    pub viewport: Viewport,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
    pub projection_matrix: [f32; 16],
    pub matrix_dirty: bool,
}

include_zig!("src/zig/camera.zig", {
    fn camera_default_viewport() -> Viewport;
    fn camera_viewport_create(x: f32, y: f32, width: f32, height: f32) -> Viewport;
    fn camera_perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Camera;
    fn camera_orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Camera;
    fn camera_default_perspective() -> Camera;
    fn camera_default_orthographic() -> Camera;
    fn camera_update_projection_matrix(camera: *mut Camera);
    fn camera_get_projection_matrix(camera: *const Camera, out_matrix: *mut [f32; 16]);
    fn camera_set_viewport(camera: *mut Camera, viewport: Viewport);
    fn camera_set_perspective(camera: *mut Camera, fov: f32, aspect: f32, near: f32, far: f32);
    fn camera_set_orthographic(camera: *mut Camera, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32);
    fn camera_is_matrix_dirty(camera: *const Camera) -> bool;
    fn camera_get_aspect_ratio(camera: *const Camera) -> f32;
});

impl Camera {
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        camera_perspective(fov, aspect, near, far)
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        camera_orthographic(left, right, bottom, top, near, far)
    }

    pub fn projection_matrix(&self) -> [f32; 16] {
        let mut matrix = [0.0f32; 16];
        camera_get_projection_matrix(self, &mut matrix);
        matrix
    }

    pub fn update_projection_matrix(&mut self) {
        camera_update_projection_matrix(self);
    }
}

impl Default for Camera {
    fn default() -> Self {
        camera_default_perspective()
    }
}

// ============================================================================
// Render Graph
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderNode {
    pub name: [u8; 64],
    pub name_len: u32,
    pub inputs: [u32; 8],
    pub input_count: u32,
    pub outputs: [u32; 8],
    pub output_count: u32,
    pub execute_fn: Option<extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>,
    pub user_data: Option<*mut std::ffi::c_void>,
    pub is_enabled: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderGraph {
    pub nodes: [RenderNode; 32],
    pub node_count: u32,
    pub execution_order: [u32; 32],
    pub is_dirty: bool,
}

include_zig!("src/zig/render_graph.zig", {
    fn render_node_create() -> RenderNode;
    fn render_node_set_name(node: *mut RenderNode, name: *const u8, len: u32);
    fn render_node_add_input(node: *mut RenderNode, input_id: u32) -> bool;
    fn render_node_add_output(node: *mut RenderNode, output_id: u32) -> bool;
    fn render_node_set_enabled(node: *mut RenderNode, enabled: bool);
    fn render_node_is_enabled(node: *const RenderNode) -> bool;
    fn render_graph_create() -> RenderGraph;
    fn render_graph_add_node(graph: *mut RenderGraph, node: RenderNode) -> bool;
    fn render_graph_get_node(graph: *mut RenderGraph, index: u32) -> Option<*mut RenderNode>;
    fn render_graph_find_node(graph: *mut RenderGraph, name: *const u8, len: u32) -> Option<*mut RenderNode>;
    fn render_graph_remove_node(graph: *mut RenderGraph, index: u32) -> bool;
    fn render_graph_clear(graph: *mut RenderGraph);
    fn render_graph_update_execution_order(graph: *mut RenderGraph);
    fn render_graph_execute(graph: *mut RenderGraph, context: *mut std::ffi::c_void);
    fn render_graph_get_node_count(graph: *const RenderGraph) -> u32;
    fn render_graph_is_dirty(graph: *const RenderGraph) -> bool;
    fn render_graph_mark_dirty(graph: *mut RenderGraph);
});

impl RenderGraph {
    pub fn new() -> Self {
        render_graph_create()
    }

    pub fn add_node(&mut self, node: RenderNode) -> bool {
        render_graph_add_node(self, node)
    }

    pub fn node_count(&self) -> u32 {
        render_graph_get_node_count(self)
    }
}

impl Default for RenderGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Material
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: [f32; 3],
    pub padding: f32,
    pub textures: [Option<*mut std::ffi::c_void>; 4],
    pub texture_count: u32,
    pub has_base_color_texture: bool,
    pub has_normal_texture: bool,
    pub has_metallic_roughness_texture: bool,
    pub has_emissive_texture: bool,
}

include_zig!("src/zig/material.zig", {
    fn material_create() -> Material;
    fn material_from_color(r: f32, g: f32, b: f32, a: f32) -> Material;
    fn material_from_rgb(r: f32, g: f32, b: f32) -> Material;
    fn material_metallic(r: f32, g: f32, b: f32, metallic: f32, roughness: f32) -> Material;
    fn material_set_base_color(mat: *mut Material, r: f32, g: f32, b: f32, a: f32);
    fn material_set_metallic(mat: *mut Material, metallic: f32);
    fn material_set_roughness(mat: *mut Material, roughness: f32);
    fn material_set_emissive(mat: *mut Material, r: f32, g: f32, b: f32);
    fn material_set_texture(mat: *mut Material, slot: u32, texture: Option<*mut std::ffi::c_void>);
    fn material_get_texture(mat: *const Material, slot: u32) -> Option<*mut std::ffi::c_void>;
    fn material_has_texture(mat: *const Material, slot: u32) -> bool;
    fn material_clear_texture(mat: *mut Material, slot: u32);
    fn material_clear_all_textures(mat: *mut Material);
    fn material_get_metallic(mat: *const Material) -> f32;
    fn material_get_roughness(mat: *const Material) -> f32;
    fn material_get_texture_count(mat: *const Material) -> u32;
    fn material_has_any_texture(mat: *const Material) -> bool;
    fn material_copy(dest: *mut Material, src: *const Material);
    fn material_equals(a: *const Material, b: *const Material) -> bool;
});

impl Material {
    pub fn new() -> Self {
        material_create()
    }

    pub fn from_color(r: f32, g: f32, b: f32, a: f32) -> Self {
        material_from_color(r, g, b, a)
    }

    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        material_from_rgb(r, g, b)
    }

    pub fn set_base_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        material_set_base_color(self, r, g, b, a);
    }

    pub fn set_metallic(&mut self, metallic: f32) {
        material_set_metallic(self, metallic);
    }

    pub fn set_roughness(&mut self, roughness: f32) {
        material_set_roughness(self, roughness);
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Shader
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ShaderModule {
    pub handle: Option<*mut std::ffi::c_void>,
    pub entry_point: [u8; 64],
    pub entry_point_len: u32,
    pub stage: u32,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ShaderSource {
    pub source: [u8; 4096],
    pub source_len: u32,
    pub entry_point: [u8; 64],
    pub entry_point_len: u32,
    pub stage: u32,
}

include_zig!("src/zig/shader.zig", {
    fn shader_module_create() -> ShaderModule;
    fn shader_source_create() -> ShaderSource;
    fn shader_source_set_source(desc: *mut ShaderSource, source: *const u8, len: u32) -> bool;
    fn shader_source_set_entry_point(desc: *mut ShaderSource, entry: *const u8, len: u32);
    fn shader_source_set_stage(desc: *mut ShaderSource, stage: u32);
    fn shader_module_set_handle(module: *mut ShaderModule, handle: Option<*mut std::ffi::c_void>);
    fn shader_module_set_entry_point(module: *mut ShaderModule, entry: *const u8, len: u32);
    fn shader_module_set_stage(module: *mut ShaderModule, stage: u32);
    fn shader_module_is_valid(module: *const ShaderModule) -> bool;
    fn shader_module_get_stage(module: *const ShaderModule) -> u32;
    fn shader_module_destroy(module: *mut ShaderModule);
    fn shader_module_create_vertex_wgsl(entry: *const u8, entry_len: u32) -> ShaderModule;
    fn shader_module_create_fragment_wgsl(entry: *const u8, entry_len: u32) -> ShaderModule;
    fn shader_module_create_compute_wgsl(entry: *const u8, entry_len: u32) -> ShaderModule;
    fn shader_module_copy(dest: *mut ShaderModule, src: *const ShaderModule);
});

impl ShaderModule {
    pub fn new() -> Self {
        shader_module_create()
    }

    pub fn is_valid(&self) -> bool {
        shader_module_is_valid(self)
    }
}

impl Default for ShaderModule {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Texture
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextureDescriptor {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: u32,
    pub format: u32,
    pub usage: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Texture {
    pub handle: Option<*mut std::ffi::c_void>,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub format: u32,
    pub mip_levels: u32,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextureView {
    pub handle: Option<*mut std::ffi::c_void>,
    pub texture: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SamplerDescriptor {
    pub address_mode_u: u32,
    pub address_mode_v: u32,
    pub address_mode_w: u32,
    pub mag_filter: u32,
    pub min_filter: u32,
    pub mipmap_filter: u32,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare_function: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Sampler {
    pub handle: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
}

include_zig!("src/zig/texture.zig", {
    fn texture_descriptor_create() -> TextureDescriptor;
    fn texture_descriptor_2d(width: u32, height: u32, format: u32) -> TextureDescriptor;
    fn texture_descriptor_render_target(width: u32, height: u32, format: u32) -> TextureDescriptor;
    fn texture_descriptor_depth(width: u32, height: u32) -> TextureDescriptor;
    fn texture_create() -> Texture;
    fn texture_set_handle(texture: *mut Texture, handle: Option<*mut std::ffi::c_void>);
    fn texture_set_dimensions(texture: *mut Texture, width: u32, height: u32, depth: u32);
    fn texture_set_format(texture: *mut Texture, format: u32);
    fn texture_set_mip_levels(texture: *mut Texture, mip_levels: u32);
    fn texture_is_valid(texture: *const Texture) -> bool;
    fn texture_get_width(texture: *const Texture) -> u32;
    fn texture_get_height(texture: *const Texture) -> u32;
    fn texture_get_format(texture: *const Texture) -> u32;
    fn texture_destroy(texture: *mut Texture);
    fn texture_view_create() -> TextureView;
    fn texture_view_set_handle(view: *mut TextureView, handle: Option<*mut std::ffi::c_void>, texture: Option<*mut std::ffi::c_void>);
    fn texture_view_is_valid(view: *const TextureView) -> bool;
    fn texture_view_destroy(view: *mut TextureView);
    fn sampler_descriptor_create() -> SamplerDescriptor;
    fn sampler_descriptor_nearest() -> SamplerDescriptor;
    fn sampler_descriptor_repeat() -> SamplerDescriptor;
    fn sampler_create() -> Sampler;
    fn sampler_set_handle(sampler: *mut Sampler, handle: Option<*mut std::ffi::c_void>);
    fn sampler_is_valid(sampler: *const Sampler) -> bool;
    fn sampler_destroy(sampler: *mut Sampler);
});

impl Texture {
    pub fn new() -> Self {
        texture_create()
    }

    pub fn is_valid(&self) -> bool {
        texture_is_valid(self)
    }

    pub fn width(&self) -> u32 {
        texture_get_width(self)
    }

    pub fn height(&self) -> u32 {
        texture_get_height(self)
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Render Pass
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ColorAttachment {
    pub view: Option<*mut std::ffi::c_void>,
    pub resolve_target: Option<*mut std::ffi::c_void>,
    pub clear_color: [f32; 4],
    pub load_op: u32,
    pub store_op: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DepthStencilAttachment {
    pub view: Option<*mut std::ffi::c_void>,
    pub depth_clear_value: f32,
    pub depth_load_op: u32,
    pub depth_store_op: u32,
    pub stencil_clear_value: u32,
    pub stencil_load_op: u32,
    pub stencil_store_op: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderPassDescriptor {
    pub color_attachments: [ColorAttachment; 4],
    pub color_attachment_count: u32,
    pub depth_stencil_attachment: DepthStencilAttachment,
    pub has_depth_stencil: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderPass {
    pub encoder: Option<*mut std::ffi::c_void>,
    pub pass_encoder: Option<*mut std::ffi::c_void>,
    pub is_active: bool,
}

include_zig!("src/zig/render_pass.zig", {
    fn render_pass_color_attachment_create() -> ColorAttachment;
    fn render_pass_color_attachment_clear(r: f32, g: f32, b: f32, a: f32) -> ColorAttachment;
    fn render_pass_color_attachment_load() -> ColorAttachment;
    fn render_pass_color_attachment_set_view(attachment: *mut ColorAttachment, view: Option<*mut std::ffi::c_void>);
    fn render_pass_color_attachment_set_clear_color(attachment: *mut ColorAttachment, r: f32, g: f32, b: f32, a: f32);
    fn render_pass_depth_attachment_create() -> DepthStencilAttachment;
    fn render_pass_depth_attachment_set_view(attachment: *mut DepthStencilAttachment, view: Option<*mut std::ffi::c_void>);
    fn render_pass_depth_attachment_set_clear_value(attachment: *mut DepthStencilAttachment, value: f32);
    fn render_pass_descriptor_create() -> RenderPassDescriptor;
    fn render_pass_descriptor_add_color_attachment(desc: *mut RenderPassDescriptor, attachment: ColorAttachment) -> bool;
    fn render_pass_descriptor_set_depth_stencil(desc: *mut RenderPassDescriptor, attachment: DepthStencilAttachment);
    fn render_pass_descriptor_get_color_attachment(desc: *mut RenderPassDescriptor, index: u32) -> Option<*mut ColorAttachment>;
    fn render_pass_create() -> RenderPass;
    fn render_pass_set_encoder(pass: *mut RenderPass, encoder: Option<*mut std::ffi::c_void>, pass_encoder: Option<*mut std::ffi::c_void>);
    fn render_pass_is_active(pass: *const RenderPass) -> bool;
    fn render_pass_end(pass: *mut RenderPass);
    fn render_pass_draw(pass: *mut RenderPass, vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32);
    fn render_pass_draw_indexed(pass: *mut RenderPass, index_count: u32, instance_count: u32, first_index: u32, base_vertex: i32, first_instance: u32);
});

impl RenderPass {
    pub fn new() -> Self {
        render_pass_create()
    }

    pub fn is_active(&self) -> bool {
        render_pass_is_active(self)
    }

    pub fn end(&mut self) {
        render_pass_end(self);
    }
}

impl Default for RenderPass {
    fn default() -> Self {
        Self::new()
    }
}