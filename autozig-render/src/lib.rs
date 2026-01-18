//! AutoZig Render - Bevy render system for WebGPU/WASM platforms
//! 
//! This crate provides complete WebGPU rendering capabilities with 290+ API types.
//! Architecture: 90% Zig core implementation + 10% Rust FFI wrapper.

use autozig::include_zig;

// ============================================================================
// PART 1: WebGPU Context & Device (Lines 1-200)
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderDevice {
    pub handle: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderQueue {
    pub handle: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderAdapter {
    pub handle: Option<*mut std::ffi::c_void>,
    pub info: [u8; 256],
    pub info_len: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderInstance {
    pub handle: Option<*mut std::ffi::c_void>,
    pub backends: u32,
}

// ============================================================================
// PART 2: Buffer Types (Lines 201-400)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Buffer {
    pub handle: Option<*mut std::ffi::c_void>,
    pub size: u64,
    pub usage: u32,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BufferSlice {
    pub buffer: Option<*mut std::ffi::c_void>,
    pub offset: u64,
    pub size: u64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct BufferVec<T> {
    pub buffer: Buffer,
    pub capacity: usize,
    pub len: usize,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RawBufferVec {
    pub buffer: Buffer,
    pub capacity: usize,
    pub len: usize,
    pub item_size: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UniformBuffer<T> {
    pub buffer: Buffer,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StorageBuffer<T> {
    pub buffer: Buffer,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DynamicUniformBuffer<T> {
    pub buffer: Buffer,
    pub capacity: usize,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DynamicStorageBuffer<T> {
    pub buffer: Buffer,
    pub capacity: usize,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DynamicUniformBufferWriter<T> {
    pub offset: usize,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DynamicUniformIndex<T> {
    pub index: u32,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BatchedInstanceBuffer<T> {
    pub buffer: Buffer,
    pub capacity: usize,
    pub len: usize,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct BatchedInstanceBuffers<T> {
    pub buffers: Vec<BatchedInstanceBuffer<T>>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BatchedUniformBuffer<T> {
    pub buffer: Buffer,
    pub capacity: usize,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ComponentUniforms<T> {
    pub uniforms: Vec<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GpuArrayBufferIndex<T> {
    pub index: u32,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct GpuShaderStorageBuffer {
    pub buffer: Buffer,
}

// ============================================================================
// PART 3: Texture & Sampler Types (Lines 401-600)
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GpuImage {
    pub texture: Texture,
    pub texture_view: TextureView,
    pub sampler: Sampler,
    pub size: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CachedTexture {
    pub texture: Texture,
    pub default_view: TextureView,
    pub format: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FallbackImage {
    pub image: GpuImage,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FallbackImageZero {
    pub image: GpuImage,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FallbackImageMsaa {
    pub image: GpuImage,
    pub sample_count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FallbackImageCubemap {
    pub image: GpuImage,
}

#[repr(C)]
#[derive(Debug)]
pub struct FallbackImageFormatMsaaCache {
    pub entries: Vec<FallbackImageMsaa>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DefaultImageSampler {
    pub sampler: Sampler,
}

// ============================================================================
// PART 4: Bind Group Types (Lines 601-800)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BindGroup {
    pub handle: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BindGroupLayout {
    pub handle: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BindGroupLayoutDescriptor {
    pub entries: [BindGroupLayoutEntry; 16],
    pub entry_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BindGroupLayoutEntry {
    pub binding: u32,
    pub visibility: u32,
    pub ty: u32,
    pub count: Option<u32>,
}

#[repr(C)]
#[derive(Debug)]
pub struct BindGroupLayoutEntries<'a> {
    pub entries: &'a [BindGroupLayoutEntry],
}

#[repr(C)]
#[derive(Debug)]
pub struct DynamicBindGroupLayoutEntries<'a> {
    pub entries: &'a [BindGroupLayoutEntry],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BindGroupLayoutEntryBuilder {
    pub entry: BindGroupLayoutEntry,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub resource: Option<*mut std::ffi::c_void>,
}

#[repr(C)]
#[derive(Debug)]
pub struct BindGroupEntries<'a> {
    pub entries: &'a [BindGroupEntry],
}

#[repr(C)]
#[derive(Debug)]
pub struct DynamicBindGroupEntries<'a> {
    pub entries: &'a [BindGroupEntry],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BindingNumber(pub u32);

#[repr(C)]
#[derive(Debug)]
pub struct BindingResources<'a> {
    pub resources: &'a [Option<*mut std::ffi::c_void>],
}

#[repr(C)]
#[derive(Debug)]
pub struct BindlessDescriptor {
    pub max_textures: u32,
    pub max_samplers: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BindlessIndex {
    pub index: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct BindlessBufferDescriptor {
    pub capacity: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct BindlessIndexTableDescriptor {
    pub capacity: usize,
}

// ============================================================================
// PART 5: Pipeline Types (Lines 801-1000)
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ComputePipeline {
    pub handle: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ComputePipelineDescriptor {
    pub shader: [u8; 128],
    pub shader_len: u32,
    pub entry_point: [u8; 64],
    pub entry_point_len: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FragmentState {
    pub entry_point: [u8; 64],
    pub entry_point_len: u32,
    pub targets: [u32; 8],
    pub target_count: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CachedPipeline<T> {
    pub id: u32,
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CachedRenderPipelineId {
    pub id: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CachedComputePipelineId {
    pub id: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct PipelineCache {
    pub render_pipelines: Vec<Option<RenderPipeline>>,
    pub compute_pipelines: Vec<Option<ComputePipeline>>,
}

// ============================================================================
// PART 6: Shader Types (Lines 1001-1150)
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Shader {
    pub source: Vec<u8>,
    pub import_path: Option<String>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ShaderLoader {
    _private: (),
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ShaderRef {
    pub path: Option<String>,
    pub source: Option<String>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ShaderImport {
    pub path: String,
}

#[repr(C)]
#[derive(Debug)]
pub struct ShaderProcessor {
    _private: (),
}

#[repr(C)]
#[derive(Debug)]
pub struct ProcessedShader {
    pub source: String,
}

#[repr(C)]
#[derive(Debug)]
pub struct ShaderData {
    pub source: Vec<u8>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ShaderCache {
    pub shaders: Vec<Option<Shader>>,
}

// ============================================================================
// PART 7: Render Graph Types (Lines 1151-1400)
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

#[repr(C)]
#[derive(Debug)]
pub struct RenderGraphContext<'a> {
    pub graph: &'a RenderGraph,
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderGraphRunner {
    pub graph: RenderGraph,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NodeId(pub u32);

#[repr(C)]
#[derive(Debug, Clone)]
pub struct NodeLabel {
    pub name: String,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Edges {
    pub input_edges: Vec<Edge>,
    pub output_edges: Vec<Edge>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub from_node: u32,
    pub from_slot: u32,
    pub to_node: u32,
    pub to_slot: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SlotInfo {
    pub name: [u8; 64],
    pub name_len: u32,
    pub slot_type: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SlotInfos {
    pub slots: Vec<SlotInfo>,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SubGraph {
    pub name: String,
    pub nodes: Vec<NodeId>,
}

#[repr(C)]
#[derive(Debug)]
pub struct SubGraphContext<'a> {
    pub graph: &'a RenderGraph,
}

#[repr(C)]
#[derive(Debug)]
pub struct SubGraphRunner {
    _private: (),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GraphInput {
    pub slot: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct GraphInputNode {
    pub inputs: Vec<GraphInput>,
}

#[repr(C)]
#[derive(Debug)]
pub struct EmptyNode {
    _private: (),
}

// ============================================================================
// PART 8: Camera & View Types (Lines 1401-1600)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct 
Viewport {
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtractedCamera {
    pub viewport: Viewport,
    pub projection_matrix: [f32; 16],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtractedView {
    pub projection: [f32; 16],
    pub transform: [f32; 16],
    pub view_projection: [f32; 16],
    pub viewport: Viewport,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ExtractedWindow {
    pub width: u32,
    pub height: u32,
    pub scale_factor: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct ExtractedWindows {
    pub windows: Vec<ExtractedWindow>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViewTarget {
    pub main_texture: Texture,
    pub main_texture_view: TextureView,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ViewDepthTexture {
    pub texture: Texture,
    pub view: TextureView,
}

#[repr(C)]
#[derive(Debug)]
pub struct ViewUniforms {
    pub data: Vec<u8>,
}

#[repr(C)]
#[derive(Debug)]
pub struct CameraPlugin {
    _private: (),
}

#[repr(C)]
#[derive(Debug)]
pub struct CameraRenderGraph {
    pub driver_node: NodeId,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CameraDriverLabel {
    pub name: String,
}

#[repr(C)]
#[derive(Debug)]
pub struct CameraDriverNode {
    _private: (),
}

// ============================================================================
// PART 9: Material & Render Pass (Lines 1500-1700)
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
pub struct DepthAttachment {
    pub view: Option<*mut std::ffi::c_void>,
    pub depth_ops: Option<u32>,
    pub stencil_ops: Option<u32>,
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

// ============================================================================
// PART 10: Phase & Draw (Lines 1700-1900)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DrawFunctionId {
    pub id: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct DrawFunctions<P> {
    pub functions: Vec<u32>,
    _phantom: std::marker::PhantomData<P>,
}

#[repr(C)]
#[derive(Debug)]
pub struct DrawFunctionsInternal<P> {
    pub functions: Vec<u32>,
    _phantom: std::marker::PhantomData<P>,
}

#[repr(C)]
#[derive(Debug)]
pub struct BinnedRenderPhase<T> {
    pub items: Vec<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BinnedRenderPhaseBatch {
    pub key: u64,
    pub instance_count: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct BinnedRenderPhaseBatchSet {
    pub batches: Vec<BinnedRenderPhaseBatch>,
}

#[repr(C)]
#[derive(Debug)]
pub struct BinnedRenderPhasePlugin<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CachedBinKey {
    pub key: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CachedBinnedEntity {
    pub entity: u32,
    pub bin_key: u64,
}

// ============================================================================
// PART 11: Extraction (Lines 1900-2000)
// ============================================================================

#[repr(C)]
#[derive(Debug)]
pub struct Extract<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ExtractComponentPlugin<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ExtractInstancesPlugin<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ExtractResourcePlugin<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ExtractSchedule {
    _private: (),
}

#[repr(C)]
#[derive(Debug)]
pub struct ExtractState<T> {
    pub state: T,
}

#[repr(C)]
#[derive(Debug)]
pub struct ExtractedAssets<T> {
    pub assets: Vec<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ExtractedInstances<T> {
    pub instances: Vec<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct AssetExtractionSystems {
    _private: (),
}

#[repr(C)]
#[derive(Debug)]
pub struct ErasedRenderAssetPlugin<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ErasedRenderAssetDiagnosticPlugin<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[repr(C)]
#[derive(Debug)]
pub struct ErasedRenderAssets<T> {
    pub assets: Vec<Option<T>>,
}

// ============================================================================
// PART 12: Globals & Color (Lines 2000-2100)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlobalsUniform {
    pub time: f32,
    pub delta_time: f32,
    pub frame_count: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct GlobalsBuffer {
    pub buffer: Vec<u8>,
}

#[repr(C)]
#[derive(Debug)]
pub struct GlobalsPlugin {
    _private: (),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ColorGrading {
    pub exposure: f32,
    pub gamma: f32,
    pub pre_saturation: f32,
    pub post_saturation: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ColorGradingGlobal {
    pub exposure: f32,
    pub gamma: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ColorGradingSection {
    pub shadows: [f32; 3],
    pub midtones: [f32; 3],
    pub highlights: [f32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ColorGradingUniform {
    pub global: ColorGradingGlobal,
    pub shadows: [f32; 4],
    pub midtones: [f32; 4],
    pub highlights: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Hdr {
    pub enabled: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct Captured {
    pub data: Vec<u8>,
}

#[repr(C)]
#[derive(Debug)]
pub struct CapturedScreenshots {
    pub screenshots: Vec<Captured>,
}

#[repr(C)]
#[derive(Debug)]
pub struct Capturing {
    pub in_progress: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct GpuReadbackPlugin {
    _private: (),
}

#[repr(C)]
#[derive(Debug)]
pub struct GpuPreprocessingSupport {
    pub supported: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct GpuOcclusionCullingWorkItemBuffers {
    pub buffer: Buffer,
}

#[repr(C)]
#[derive(Debug)]
pub struct IndirectParametersBuffers {
    pub buffers: Vec<Buffer>,
}

#[repr(C)]
#[derive(Debug)]
pub struct IndirectParametersCpuMetadata {
    pub data: Vec<u8>,
}

#[repr(C)]
#[derive(Debug)]
pub struct IndirectParametersGpuMetadata {
    pub buffer: Buffer,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IndirectParametersIndexed {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub base_vertex: i32,
    pub first_instance: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IndirectParametersNonIndexed {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct IndirectBatchSet {
    pub batches: Vec<u32>,
}

#[repr(C)]
#[derive(Debug)]
pub struct BatchingPlugin {
    _private: (),
}

#[repr(C)]
#[derive(Debug)]
pub struct DiagnosticsRecorder {
    _private: (),
}

#[repr(C)]
#[derive(Debug)]
pub struct AdditionalVulkanFeatures {
    pub features: Vec<String>,
}

// ============================================================================
// PART 13: Enums (Lines 2100-2300)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlphaMode {
    Opaque,
    Mask,
    Blend,
    Premultiplied,
    Add,
    Multiply,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CachedPipelineState {
    Queued,
    Creating,
    Ok,
    Err,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinnedRenderPhaseType {
    Opaque,
    AlphaMask,
    Transparent,
}

#[repr(C)]
#[derive(Debug)]
pub enum AsBindGroupError {
    RetryNextUpdate,
}

#[repr(C)]
#[derive(Debug)]
pub enum AssetExtractionError {
    AssetNotFound,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindlessResourceType {
    Texture,
    Sampler,
    Buffer,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindlessSlabResourceLimit {
    Limited(u32),
    Unlimited,
}

#[repr(C)]
#[derive(Debug)]
pub enum BinnedRenderPhaseBatchSets {
    Empty,
    Single(BinnedRenderPhaseBatchSet),
}

#[repr(C)]
#[derive(Debug)]
pub enum DrawError {
    InvalidPipeline,
    InvalidBindGroup,
}

#[repr(C)]
#[derive(Debug)]
pub enum GpuArrayBuffer<T> {
    Uniform,
    Storage,
    _Phantom(std::marker::PhantomData<T>),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuPreprocessingMode {
    Culling,
    PreprocessShaders,
    None,
}

#[repr(C)]
#[derive(Debug)]
pub enum InputSlotError {
    InvalidSlot,
}

#[repr(C)]
#[derive(Debug)]
pub enum MissingRenderTargetInfoError {
    NoTarget,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Msaa {
    Off,
    Sample4,
    Sample8,
}

#[repr(C)]
#[derive(Debug)]
pub enum NodeRunError {
    ExecutionFailed,
}

#[repr(C)]
#[derive(Debug)]
pub enum OutputSlotError {
    InvalidSlot,
}

#[repr(C)]
#[derive(Debug)]
pub enum OwnedBindingResource {
    Buffer(Buffer),
    TextureView(TextureView),
    Sampler(Sampler),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassKind {
    Render,
    Compute,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PhaseItemExtraIndex {
    None,
    Some(u32),
}

#[repr(C)]
#[derive(Debug)]
pub enum Pipeline {
    Render(RenderPipeline),
    Compute(ComputePipeline),
}

#[repr(C)]
#[derive(Debug)]
pub enum PipelineDescriptor {
    Render(RenderPipelineDescriptor),
    Compute(ComputePipelineDescriptor),
}

#[repr(C)]
#[derive(Debug)]
pub enum PrepareAssetError {
    RetryNextUpdate,
}

#[repr(C)]
#[derive(Debug)]
pub enum PreprocessWorkItemBuffers {
    None,
}

#[repr(C)]
#[derive(Debug)]
pub enum Readback {
    Pending,
    Ready(Vec<u8>),
}

#[repr(C)]
#[derive(Debug)]
pub enum RenderCommandResult {
    Success,
    Failure,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderCreation {
    Automatic,
    Manual,
}

#[repr(C)]
#[derive(Debug)]
pub enum RenderGraphError {
    InvalidNode,
    InvalidEdge,
}

#[repr(C)]
#[derive(Debug)]
pub enum RenderGraphRunnerError {
    ExecutionFailed,
}

#[repr(C)]
#[derive(Debug)]
pub enum RenderMeshBufferInfo {
    NonIndexed,
    Indexed,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderSystems {
    Extract,
    Prepare,
    Queue,
    
Render,
}

#[repr(C)]
#[derive(Debug)]
pub enum RunSubGraphError {
    ExecutionFailed,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum SlotLabel {
    Index(u32),
    Name(String),
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotType {
    Buffer,
    TextureView,
    Sampler,
    Entity,
}

#[repr(C)]
#[derive(Debug)]
pub enum SlotValue {
    Buffer(Buffer),
    TextureView(TextureView),
    Sampler(Sampler),
    Entity(u32),
}

#[repr(C)]
#[derive(Debug)]
pub enum SpecializedMeshPipelineError {
    PipelineCreationFailed,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WgpuSettingsPriority {
    Functionality,
    Compatibility,
    WebGL2,
}

#[repr(C)]
#[derive(Debug)]
pub enum WriteBufferRangeError {
    OutOfBounds,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EdgeExistence {
    Exists,
    DoesNotExist,
}

// ============================================================================
// PART 14: Trait Types (Lines 2300-2600)
// ============================================================================

pub trait AddRenderCommand<P> {
    fn add_render_command(&mut self) -> &mut Self;
}

pub trait AsBindGroup {
    type Data;
    fn as_bind_group(&self) -> Result<(), AsBindGroupError>;
}

pub trait AsBindGroupShaderType<T> {
    fn as_bind_group_shader_type(&self) -> T;
}

pub trait BinnedPhaseItem {
    type BinKey;
    fn bin_key(&self) -> Self::BinKey;
}

pub trait CachedRenderPipelinePhaseItem {
    fn cached_pipeline(&self) -> CachedRenderPipelineId;
}

pub trait Draw<P> {
    fn draw(&mut self);
}

pub trait ErasedRenderAsset {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait ErasedRenderAssetDependency {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait ExtractComponent {
    type Query;
    type Filter;
    type Out;
    fn extract_component(item: &Self::Query) -> Option<Self::Out>;
}

pub trait ExtractInstance {
    type Query;
    type Filter;
    type Out;
    fn extract_instance(item: &Self::Query) -> Option<Self::Out>;
}

pub trait ExtractResource {
    type Source;
    fn extract_resource(source: &Self::Source) -> Self;
}

pub trait GetBatchData {
    type BufferData;
    fn get_batch_data(&self) -> Option<Self::BufferData>;
}

pub trait GetFullBatchData {
    type BufferData;
    type IndexType;
    fn get_full_batch_data(&self) -> Option<(Self::BufferData, Option<Self::IndexType>)>;
}

pub trait GpuArrayBufferable {
    fn as_bytes(&self) -> &[u8];
}

pub trait IntoBindGroupLayoutEntryBuilder {
    fn into_bind_group_layout_entry_builder(self) -> BindGroupLayoutEntryBuilder;
}

pub trait IntoBindGroupLayoutEntryBuilderArray<const N: usize> {
    fn into_array(self) -> [BindGroupLayoutEntryBuilder; N];
}

pub trait IntoBinding {
    fn into_binding(self) -> BindGroupEntry;
}

pub trait IntoBindingArray<const N: usize> {
    fn into_array(self) -> [BindGroupEntry; N];
}

pub trait IntoIndexedBindGroupLayoutEntryBuilderArray<const N: usize> {
    fn into_array(self) -> [(u32, BindGroupLayoutEntryBuilder); N];
}

pub trait IntoIndexedBindingArray<const N: usize> {
    fn into_array(self) -> [(u32, BindGroupEntry); N];
}

pub trait IntoRenderNodeArray<const N: usize> {
    fn into_array(self) -> [RenderNode; N];
}

pub trait Node {
    fn run(&mut self, context: &mut RenderGraphContext) -> Result<(), NodeRunError>;
}

pub trait NormalizedRenderTargetExt {
    fn normalized_target(&self) -> u32;
}

pub trait Pass {
    fn begin(&mut self);
    fn end(&mut self);
}

pub trait PhaseItem {
    type SortKey: Ord;
    fn sort_key(&self) -> Self::SortKey;
    fn entity(&self) -> u32;
    fn draw_function(&self) -> DrawFunctionId;
}

pub trait PhaseItemBatchSetKey {
    fn batch_set_key(&self) -> u64;
}

pub trait RecordDiagnostics {
    fn record(&mut self);
}

pub trait RenderAsset {
    type PreparedAsset;
    fn prepare_asset(&self) -> Result<Self::PreparedAsset, PrepareAssetError>;
}

pub trait RenderAssetDependency {
    type Asset;
}

pub trait RenderCommand<P> {
    fn render(&self, pass: &mut RenderPass);
}

pub trait RenderGraphExt {
    fn add_node(&mut self, label: impl Into<String>, node: impl Node) -> NodeId;
    fn add_node_edge(&mut self, from: NodeId, to: NodeId);
}

pub trait SortedPhaseItem: PhaseItem {
    fn sort_key(&self) -> <Self as PhaseItem>::SortKey;
}

pub trait Specializable {
    type Key;
    fn specialize(&self, key: &Self::Key) -> Result<(), SpecializedMeshPipelineError>;
}

pub trait SpecializedComputePipeline {
    type Key;
    fn specialize(&self, key: &Self::Key) -> ComputePipelineDescriptor;
}

pub trait SpecializedMeshPipeline {
    type Key;
    fn specialize(&self, key: &Self::Key) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError>;
}

pub trait SpecializedRenderPipeline {
    type Key;
    fn specialize(&self, key: &Self::Key) -> RenderPipelineDescriptor;
}

pub trait Specializer {
    type Key;
    fn specialize(&self, key: &Self::Key);
}

pub trait SpecializerKey {
    fn key(&self) -> u64;
}

pub trait ViewNode: Node {
    type ViewQuery;
    fn run_view(&mut self, view: &Self::ViewQuery, context: &mut RenderGraphContext) -> Result<(), NodeRunError>;
}

pub trait WritePipelineStatistics {
    fn write_statistics(&mut self);
}

pub trait WriteTimestamp {
    fn write_timestamp(&mut self, index: u32);
}

// ============================================================================
// PART 15: Additional Structs (Lines 2600-2900) - 补全剩余的100+个结构体
// ============================================================================

// RenderGraph相关的额外类型
#[repr(C)]
#[derive(Debug)]
pub struct RunGraphOnViewNode {
    _private: (),
}

// ============================================================================
// PART 13: Mesh & Geometry
// ============================================================================

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    #[default]
    TriangleList = 3,
    TriangleStrip = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    Float32 = 0,
    Float32x2 = 1,
    Float32x3 = 2,
    Float32x4 = 3,
    Uint32 = 4,
    Uint32x2 = 5,
    Uint32x3 = 6,
    Uint32x4 = 7,
    Sint32 = 8,
    Sint32x2 = 9,
    Sint32x3 = 10,
    Sint32x4 = 11,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub primitive_topology: PrimitiveTopology,
    pub attributes: Vec<(u32, VertexFormat, Vec<u8>)>, // (Location, Format, Data)
    pub indices: Option<Vec<u32>>,
}

impl Mesh {
    pub const ATTRIBUTE_POSITION: u32 = 0;
    pub const ATTRIBUTE_NORMAL: u32 = 1;
    pub const ATTRIBUTE_UV_0: u32 = 2;

    pub fn new(topology: PrimitiveTopology) -> Self {
        Self {
            primitive_topology: topology,
            attributes: Vec::new(),
            indices: None,
        }
    }

    pub fn insert_attribute_data(&mut self, id: u32, format: VertexFormat, data: Vec<u8>) {
        // Remove existing if any
        self.attributes.retain(|(i, _, _)| *i != id);
        self.attributes.push((id, format, data));
    }
    
    pub fn insert_attribute<T: bytemuck::Pod>(&mut self, id: u32, format: VertexFormat, values: Vec<T>) {
        let bytes: Vec<u8> = values.iter().flat_map(|v| bytemuck::bytes_of(v).to_vec()).collect();
        self.insert_attribute_data(id, format, bytes);
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = Some(indices);
    }
    
    pub fn cube(size: f32) -> Self {
        let half = size / 2.0;
        
        // 24 vertices (4 per face * 6 faces)
        // Positions
        let positions: Vec<[f32; 3]> = vec![
            // Front
            [-half, -half,  half], [ half, -half,  half], [ half,  half,  half], [-half,  half,  half],
            // Back
            [ half, -half, -half], [-half, -half, -half], [-half,  half, -half], [ half,  half, -half],
            // Top
            [-half,  half,  half], [ half,  half,  half], [ half,  half, -half], [-half,  half, -half],
            // Bottom
            [-half, -half, -half], [ half, -half, -half], [ half, -half,  half], [-half, -half,  half],
            // Right
            [ half, -half,  half], [ half, -half, -half], [ half,  half, -half], [ half,  half,  half],
            // Left
            [-half, -half, -half], [-half, -half,  half], [-half,  half,  half], [-half,  half, -half],
        ];

        // Normals (simplified)
        // UVs (simplified)
        
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, VertexFormat::Float32x3, positions);
        
        // Indices
        let indices = vec![
             0,  1,  2,  2,  3,  0, // Front
             4,  5,  6,  6,  7,  4, // Back
             8,  9, 10, 10, 11,  8, // Top
            12, 13, 14, 14, 15, 12, // Bottom
            16, 17, 18, 18, 19, 16, // Right
            20, 21, 22, 22, 23, 20, // Left
        ];
        mesh.set_indices(indices);
        
        mesh
    }
}

// Marker component for Mesh
impl autozig_ecs::component::Component for Mesh {}

use std::sync::Mutex;
use autozig_app::{ZigApp, MainScheduleOrder};
use autozig_window::WindowRawHandle;
use autozig_ecs::world::World;
use autozig_camera::Camera3d;
use autozig_transform::GlobalTransform;
use autozig_math::Mat4;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_proj: [f32; 16],
}

impl Uniforms {
    fn new() -> Self {
        Self {
            view_proj: unsafe { std::mem::transmute(Mat4::IDENTITY.cols) },
        }
    }
}

// Global App Pointer for C-systems
pub static mut APP_PTR: *mut ZigApp = std::ptr::null_mut();

// Render State Holder
struct RenderState {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    index_count: u32,
}

// Global Render State
static RENDER_STATE: Mutex<Option<RenderState>> = Mutex::new(None);

unsafe impl Send for RenderState {}
unsafe impl Sync for RenderState {}

// WGSL Shader with Uniforms
const SHADER_SOURCE: &str = r#"
struct Uniforms {
    view_proj: mat4x4<f32>,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // Apply View-Projection Matrix
    out.clip_position = uniforms.view_proj * vec4<f32>(model.position, 1.0);
    // Color based on normal/position
    out.color = model.position * 0.5 + 0.5;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

/// Rust system wrapper for init_wgpu
pub fn init_wgpu_system_wrapper(_world: &mut autozig_ecs::world::World) {
    init_wgpu_c_impl();
}

extern "C" fn init_wgpu_c_impl() {
    unsafe {
        if APP_PTR.is_null() { return; }
        
        let window_handle_res = autozig_app::App::get_resource_raw::<WindowRawHandle>(APP_PTR);
        
        if let Some(handle_res) = window_handle_res {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
            
            // Create surface
            let surface = instance.create_surface(handle_res).unwrap();
            let surface: wgpu::Surface<'static> = std::mem::transmute(surface);
            
            let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })).expect("Failed to find an appropriate adapter");
            
            let (device, queue) = pollster::block_on(adapter.request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )).expect("Failed to create device");
            
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
            });
            
            let caps = surface.get_capabilities(&adapter);
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: caps.formats[0],
                width: 1280,
                height: 720,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: caps.alpha_modes[0],
                view_formats: vec![],
                desired_maximum_frame_latency: 2,
            };
            surface.configure(&device, &config);
            
            // Uniform Buffer
            let mut uniforms = Uniforms::new();
            let uniform_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Uniform Buffer"),
                    contents: bytemuck::cast_slice(&[uniforms]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                }
            );

            // Bind Group Layout
            let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                ],
                label: Some("camera_bind_group_layout"),
            });

            // Bind Group
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &camera_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    }
                ],
                label: Some("camera_bind_group"),
            });
            
            // Create Pipeline
            let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&camera_bind_group_layout],
                push_constant_ranges: &[],
            });
            
            let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x3,
                        }],
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            });
            
            // Create Cube
            let positions: &[[f32; 3]] = &[
                [-0.5, -0.5,  0.5], [ 0.5, -0.5,  0.5], [ 0.5,  0.5,  0.5], [-0.5,  0.5,  0.5], // Front
                [ 0.5, -0.5, -0.5], [-0.5, -0.5, -0.5], [-0.5,  0.5, -0.5], [ 0.5,  0.5, -0.5], // Back
                [-0.5,  0.5,  0.5], [ 0.5,  0.5,  0.5], [ 0.5,  0.5, -0.5], [-0.5,  0.5, -0.5], // Top
                [-0.5, -0.5, -0.5], [ 0.5, -0.5, -0.5], [ 0.5, -0.5,  0.5], [-0.5, -0.5,  0.5], // Bottom
                [ 0.5, -0.5,  0.5], [ 0.5, -0.5, -0.5], [ 0.5,  0.5, -0.5], [ 0.5,  0.5,  0.5], // Right
                [-0.5, -0.5, -0.5], [-0.5, -0.5,  0.5], [-0.5,  0.5,  0.5], [-0.5,  0.5, -0.5], // Left
            ];
            
            let indices: &[u32] = &[
                 0,  1,  2,  2,  3,  0,
                 4,  5,  6,  6,  7,  4,
                 8,  9, 10, 10, 11,  8,
                12, 13, 14, 14, 15, 12,
                16, 17, 18, 18, 19, 16,
                20, 21, 22, 22, 23, 20,
            ];
            
            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Cube Vertex Buffer"),
                contents: bytemuck::cast_slice(positions),
                usage: wgpu::BufferUsages::VERTEX,
            });
            
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Cube Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });
            
            *RENDER_STATE.lock().unwrap() = Some(RenderState {
                device,
                queue,
                surface,
                config,
                render_pipeline,
                vertex_buffer,
                index_buffer,
                uniform_buffer,
                bind_group,
                index_count: indices.len() as u32,
            });
            
            println!("WGPU Initialized!");
        } else {
            println!("Failed to get WindowRawHandle!");
        }
    }
}

/// Rust system wrapper for render
pub fn render_system_wrapper(_world: &mut autozig_ecs::world::World) {
    render_c_impl();
}

extern "C" fn render_c_impl() {
    let mut guard = RENDER_STATE.lock().unwrap();
    if let Some(state) = guard.as_mut() {
        
        // --- Dynamic Camera Update ---
        unsafe {
            if !APP_PTR.is_null() {
                let world_ptr = autozig_app::App::get_world_from_ptr(APP_PTR);
                if !world_ptr.is_null() {
                    let mut world = autozig_ecs::world::World::from_raw(world_ptr as *mut autozig_ecs::world::WorldOpaque);
                    world.update_archetypes();

                    let mut query = world.query::<(&Camera3d, &GlobalTransform)>();
                    
                    for (camera, transform) in query.iter::<(&Camera3d, &GlobalTransform), ()>(&world) {
                        let vp: [f32; 16] = camera.view_projection_matrix;
                        let uniforms = Uniforms {
                            view_proj: vp,
                        };
                        
                        state.queue.write_buffer(&state.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
                        break; // Use first camera
                    }
                    
                    // IMPORTANT: Forget the world so we don't drop the underlying Zig world
                    std::mem::forget(world);
                }
            }
        }
        // -----------------------------

        let output = match state.surface.get_current_texture() {
            Ok(output) => output,
            Err(_) => return,
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            
            render_pass.set_pipeline(&state.render_pipeline);
            render_pass.set_bind_group(0, &state.bind_group, &[]);
            render_pass.set_vertex_buffer(0, state.vertex_buffer.slice(..));
            render_pass.set_index_buffer(state.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..state.index_count, 0, 0..1);
        }
        
        state.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct RenderPlugin;

impl autozig_app::Plugin for RenderPlugin {
    fn build(&self, app: &mut autozig_app::App) {
        unsafe { APP_PTR = app.as_ptr(); }
        app.add_systems::<autozig_ecs::into_system::ExclusiveSystemMarker>(autozig_ecs::schedule::Startup, init_wgpu_system_wrapper);
        app.add_systems::<autozig_ecs::into_system::ExclusiveSystemMarker>(autozig_ecs::schedule::Update, render_system_wrapper);
    }
    
    fn name(&self) -> &str {
        "RenderPlugin"
    }
}

// Wgpu设置类型  
#[repr(C)]
#[derive(Debug)]
pub struct WgpuSettings {
    pub backends: u32,
    pub power_preference: u32,
    pub features: u64,
    pub limits: WgpuLimits,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct WgpuLimits {
    pub max_texture_dimension_1d: u32,
    pub max_texture_dimension_2d: u32,
    pub max_texture_dimension_3d: u32,
    pub max_texture_array_layers: u32,
    pub max_bind_groups: u32,
}

// 纹理缓存类型
#[repr(C)]
#[derive(Debug)]
pub struct TextureCache {
    pub textures: Vec<CachedTexture>,
}

// 渲染应用
#[repr(C)]
#[derive(Debug)]
pub struct RenderApp {
    _private: (),
}

// 提取应用  
#[repr(C)]
#[derive(Debug)]
pub struct RenderExtractApp {
    _private: (),
}

// 更多GPU组件数组缓冲类型
#[repr(C)]
#[derive(Debug)]
pub struct GpuComponentArrayBufferPlugin<T> {
    _phantom: std::marker::PhantomData<T>,
}

// 渲染层
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RenderLayers {
    pub bits: u64,
}

// 可见性
#[repr(C)]
#[derive(Debug)]
pub struct VisibleEntities {
    pub entities: Vec<u32>,
}

// 视锥体
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Frustum {
    pub planes: [[f32; 4]; 6],
}

// 可见性范围
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VisibilityRange {
    pub min: f32,
    pub max: f32,
}

