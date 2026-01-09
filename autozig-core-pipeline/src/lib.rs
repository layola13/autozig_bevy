//! AutoZig Core Pipeline - Rendering pipeline orchestration for WebGPU/WASM
//!
//! This crate provides a comprehensive rendering pipeline system inspired by
//! bevy_core_pipeline, implemented using Zig for high-performance graphics operations.
//!
//! # Architecture
//!
//! - **Pass Scheduler**: Manages render pass execution order and priorities
//! - **Resource Tracker**: Tracks resource states and manages GPU barriers
//! - **Command Encoder**: WebGPU command encoding and submission
//! - **Pipeline**: Core pipeline orchestration and statistics

#![forbid(unsafe_code)]

use autozig::include_zig;

// Re-export from autozig-render for convenience
pub use autozig_render::{
    Camera, ColorAttachment, DepthStencilAttachment, Material, RenderGraph, RenderNode,
    RenderPass, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModule,
    Texture, TextureDescriptor, Viewport, WgpuContext,
};

// ============================================================================
// Pass Scheduler
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassType {
    ClearPass = 0,
    MainOpaquePass = 1,
    MainTransparentPass = 2,
    PostProcessPass = 3,
    TonemappingPass = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassPriority {
    Early = 0,
    Normal = 100,
    Late = 200,
    PostProcess = 300,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pass {
    pub name: [u8; 64],
    pub name_len: u32,
    pub pass_type: u32,
    pub priority: u32,
    pub inputs: [u32; 8],
    pub input_count: u32,
    pub outputs: [u32; 8],
    pub output_count: u32,
    pub execute_fn: Option<extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>,
    pub user_data: Option<*mut std::ffi::c_void>,
    pub is_enabled: bool,
    pub is_dirty: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PassScheduler {
    pub passes: [Pass; 64],
    pub pass_count: u32,
    pub execution_order: [u32; 64],
    pub is_dirty: bool,
}

include_zig!("src/zig/pass_scheduler.zig", {
    fn pass_create() -> Pass;
    fn pass_set_name(pass: *mut Pass, name: *const u8, len: u32);
    fn pass_set_type(pass: *mut Pass, pass_type: u32);
    fn pass_set_priority(pass: *mut Pass, priority: u32);
    fn pass_add_input(pass: *mut Pass, resource_id: u32) -> bool;
    fn pass_add_output(pass: *mut Pass, resource_id: u32) -> bool;
    fn pass_set_execute_fn(pass: *mut Pass, execute_fn: Option<extern "C" fn(*mut std::ffi::c_void, *mut std::ffi::c_void)>);
    fn pass_set_user_data(pass: *mut Pass, user_data: Option<*mut std::ffi::c_void>);
    fn pass_set_enabled(pass: *mut Pass, enabled: bool);
    fn pass_is_enabled(pass: *const Pass) -> bool;
    fn pass_scheduler_create() -> PassScheduler;
    fn pass_scheduler_add_pass(scheduler: *mut PassScheduler, pass: Pass) -> bool;
    fn pass_scheduler_get_pass(scheduler: *mut PassScheduler, index: u32) -> Option<*mut Pass>;
    fn pass_scheduler_find_pass(scheduler: *mut PassScheduler, name: *const u8, len: u32) -> Option<*mut Pass>;
    fn pass_scheduler_remove_pass(scheduler: *mut PassScheduler, index: u32) -> bool;
    fn pass_scheduler_clear(scheduler: *mut PassScheduler);
    fn pass_scheduler_update_execution_order(scheduler: *mut PassScheduler);
    fn pass_scheduler_execute(scheduler: *mut PassScheduler, context: *mut std::ffi::c_void);
    fn pass_scheduler_get_pass_count(scheduler: *const PassScheduler) -> u32;
    fn pass_scheduler_is_dirty(scheduler: *const PassScheduler) -> bool;
    fn pass_scheduler_mark_dirty(scheduler: *mut PassScheduler);
    fn pass_scheduler_get_execution_order(scheduler: *const PassScheduler, out_order: *mut u32, max_count: u32) -> u32;
});

impl Pass {
    pub fn new() -> Self {
        pass_create()
    }

    pub fn set_name(&mut self, name: &str) {
        pass_set_name(self, name.as_ptr(), name.len() as u32);
    }

    pub fn set_type(&mut self, pass_type: PassType) {
        pass_set_type(self, pass_type as u32);
    }

    pub fn set_priority(&mut self, priority: PassPriority) {
        pass_set_priority(self, priority as u32);
    }

    pub fn add_input(&mut self, resource_id: u32) -> bool {
        pass_add_input(self, resource_id)
    }

    pub fn add_output(&mut self, resource_id: u32) -> bool {
        pass_add_output(self, resource_id)
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        pass_set_enabled(self, enabled);
    }

    pub fn is_enabled(&self) -> bool {
        pass_is_enabled(self)
    }
}

impl Default for Pass {
    fn default() -> Self {
        Self::new()
    }
}

impl PassScheduler {
    pub fn new() -> Self {
        pass_scheduler_create()
    }

    pub fn add_pass(&mut self, pass: Pass) -> bool {
        pass_scheduler_add_pass(self, pass)
    }

    pub fn find_pass(&mut self, name: &str) -> Option<*mut Pass> {
        pass_scheduler_find_pass(self, name.as_ptr(), name.len() as u32)
    }

    pub fn clear(&mut self) {
        pass_scheduler_clear(self);
    }

    pub fn execute(&mut self, context: *mut std::ffi::c_void) {
        pass_scheduler_execute(self, context);
    }

    pub fn pass_count(&self) -> u32 {
        pass_scheduler_get_pass_count(self)
    }

    pub fn is_dirty(&self) -> bool {
        pass_scheduler_is_dirty(self)
    }
}

impl Default for PassScheduler {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Resource Barrier Management
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceState {
    Undefined = 0,
    RenderTarget = 1,
    DepthWrite = 2,
    DepthRead = 3,
    ShaderResource = 4,
    UnorderedAccess = 5,
    CopySource = 6,
    CopyDest = 7,
    Present = 8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    Buffer = 0,
    Texture = 1,
    TextureView = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Resource {
    pub id: u32,
    pub resource_type: u32,
    pub handle: Option<*mut std::ffi::c_void>,
    pub current_state: u32,
    pub name: [u8; 64],
    pub name_len: u32,
    pub is_valid: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResourceBarrier {
    pub resource_id: u32,
    pub state_before: u32,
    pub state_after: u32,
    pub is_executed: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct ResourceTracker {
    pub resources: [Resource; 256],
    pub resource_count: u32,
    pub barriers: [ResourceBarrier; 512],
    pub barrier_count: u32,
    pub next_resource_id: u32,
}

include_zig!("src/zig/resource_barrier.zig", {
    fn resource_create() -> Resource;
    fn resource_set_name(resource: *mut Resource, name: *const u8, len: u32);
    fn resource_set_type(resource: *mut Resource, resource_type: u32);
    fn resource_set_handle(resource: *mut Resource, handle: Option<*mut std::ffi::c_void>);
    fn resource_set_state(resource: *mut Resource, state: u32);
    fn resource_is_valid(resource: *const Resource) -> bool;
    fn resource_barrier_create() -> ResourceBarrier;
    fn resource_tracker_create() -> ResourceTracker;
    fn resource_tracker_register(tracker: *mut ResourceTracker, resource_type: u32, handle: Option<*mut std::ffi::c_void>, initial_state: u32) -> u32;
    fn resource_tracker_unregister(tracker: *mut ResourceTracker, resource_id: u32) -> bool;
    fn resource_tracker_find(tracker: *mut ResourceTracker, resource_id: u32) -> Option<*mut Resource>;
    fn resource_tracker_get(tracker: *mut ResourceTracker, index: u32) -> Option<*mut Resource>;
    fn resource_tracker_add_barrier(tracker: *mut ResourceTracker, resource_id: u32, state_after: u32) -> bool;
    fn resource_tracker_execute_barriers(tracker: *mut ResourceTracker);
    fn resource_tracker_clear_barriers(tracker: *mut ResourceTracker);
    fn resource_tracker_get_barrier_count(tracker: *const ResourceTracker) -> u32;
    fn resource_tracker_get_resource_count(tracker: *const ResourceTracker) -> u32;
    fn resource_tracker_get_state(tracker: *const ResourceTracker, resource_id: u32) -> u32;
    fn resource_tracker_needs_barrier(tracker: *const ResourceTracker, resource_id: u32, target_state: u32) -> bool;
    fn resource_tracker_clear(tracker: *mut ResourceTracker);
});

impl Resource {
    pub fn new() -> Self {
        resource_create()
    }

    pub fn set_name(&mut self, name: &str) {
        resource_set_name(self, name.as_ptr(), name.len() as u32);
    }

    pub fn is_valid(&self) -> bool {
        resource_is_valid(self)
    }
}

impl Default for Resource {
    fn default() -> Self {
        Self::new()
    }
}

impl ResourceTracker {
    pub fn new() -> Self {
        resource_tracker_create()
    }

    pub fn register(
        &mut self,
        resource_type: ResourceType,
        handle: Option<*mut std::ffi::c_void>,
        initial_state: ResourceState,
    ) -> u32 {
        resource_tracker_register(
            self,
            resource_type as u32,
            handle,
            initial_state as u32,
        )
    }

    pub fn unregister(&mut self, resource_id: u32) -> bool {
        resource_tracker_unregister(self, resource_id)
    }

    pub fn add_barrier(&mut self, resource_id: u32, state_after: ResourceState) -> bool {
        resource_tracker_add_barrier(self, resource_id, state_after as u32)
    }

    pub fn execute_barriers(&mut self) {
        resource_tracker_execute_barriers(self);
    }

    pub fn clear_barriers(&mut self) {
        resource_tracker_clear_barriers(self);
    }

    pub fn barrier_count(&self) -> u32 {
        resource_tracker_get_barrier_count(self)
    }

    pub fn resource_count(&self) -> u32 {
        resource_tracker_get_resource_count(self)
    }

    pub fn get_state(&self, resource_id: u32) -> ResourceState {
        let state = resource_tracker_get_state(self, resource_id);
        match state {
            0 => ResourceState::Undefined,
            1 => ResourceState::RenderTarget,
            2 => ResourceState::DepthWrite,
            3 => ResourceState::DepthRead,
            4 => ResourceState::ShaderResource,
            5 => ResourceState::UnorderedAccess,
            6 => ResourceState::CopySource,
            7 => ResourceState::CopyDest,
            8 => ResourceState::Present,
            _ => ResourceState::Undefined,
        }
    }

    pub fn needs_barrier(&self, resource_id: u32, target_state: ResourceState) -> bool {
        resource_tracker_needs_barrier(self, resource_id, target_state as u32)
    }

    pub fn clear(&mut self) {
        resource_tracker_clear(self);
    }
}

impl Default for ResourceTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Command Encoder
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CommandEncoder {
    pub handle: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
    pub command_count: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CommandBuffer {
    pub handle: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
    pub is_submitted: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CommandQueue {
    pub handle: Option<*mut std::ffi::c_void>,
    pub is_valid: bool,
}

include_zig!("src/zig/command_encoder.zig", {
    fn command_encoder_create() -> CommandEncoder;
    fn command_encoder_set_handle(encoder: *mut CommandEncoder, handle: Option<*mut std::ffi::c_void>);
    fn command_encoder_is_valid(encoder: *const CommandEncoder) -> bool;
    fn command_encoder_get_command_count(encoder: *const CommandEncoder) -> u32;
    fn command_encoder_reset(encoder: *mut CommandEncoder);
    fn command_encoder_finish(encoder: *mut CommandEncoder) -> CommandBuffer;
    fn command_buffer_create() -> CommandBuffer;
    fn command_buffer_set_handle(buffer: *mut CommandBuffer, handle: Option<*mut std::ffi::c_void>);
    fn command_buffer_is_valid(buffer: *const CommandBuffer) -> bool;
    fn command_buffer_mark_submitted(buffer: *mut CommandBuffer);
    fn command_queue_create() -> CommandQueue;
    fn command_queue_set_handle(queue: *mut CommandQueue, handle: Option<*mut std::ffi::c_void>);
    fn command_queue_is_valid(queue: *const CommandQueue) -> bool;
});

impl CommandEncoder {
    pub fn new() -> Self {
        command_encoder_create()
    }

    pub fn is_valid(&self) -> bool {
        command_encoder_is_valid(self)
    }

    pub fn command_count(&self) -> u32 {
        command_encoder_get_command_count(self)
    }

    pub fn reset(&mut self) {
        command_encoder_reset(self);
    }

    pub fn finish(&mut self) -> CommandBuffer {
        command_encoder_finish(self)
    }
}

impl Default for CommandEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandBuffer {
    pub fn new() -> Self {
        command_buffer_create()
    }

    pub fn is_valid(&self) -> bool {
        command_buffer_is_valid(self)
    }

    pub fn mark_submitted(&mut self) {
        command_buffer_mark_submitted(self);
    }
}

impl Default for CommandBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandQueue {
    pub fn new() -> Self {
        command_queue_create()
    }

    pub fn is_valid(&self) -> bool {
        command_queue_is_valid(self)
    }
}

impl Default for CommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Core Pipeline
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PipelineConfig {
    pub max_passes: u32,
    pub max_resources: u32,
    pub enable_validation: bool,
    pub enable_debug_markers: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PipelineStats {
    pub frame_count: u64,
    pub pass_count: u32,
    pub draw_calls: u32,
    pub triangles: u32,
    pub resources_created: u32,
    pub resources_destroyed: u32,
    pub barriers_executed: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pipeline {
    pub config: PipelineConfig,
    pub stats: PipelineStats,
    pub is_initialized: bool,
    pub is_recording: bool,
}

include_zig!("src/zig/pipeline.zig", {
    fn pipeline_config_default() -> PipelineConfig;
    fn pipeline_config_create(max_passes: u32, max_resources: u32, enable_validation: bool, enable_debug_markers: bool) -> PipelineConfig;
    fn pipeline_stats_create() -> PipelineStats;
    fn pipeline_stats_reset(stats: *mut PipelineStats);
    fn pipeline_create() -> Pipeline;
    fn pipeline_create_with_config(config: PipelineConfig) -> Pipeline;
    fn pipeline_init(pipeline: *mut Pipeline);
    fn pipeline_shutdown(pipeline: *mut Pipeline);
    fn pipeline_begin_frame(pipeline: *mut Pipeline) -> bool;
    fn pipeline_end_frame(pipeline: *mut Pipeline) -> bool;
    fn pipeline_is_initialized(pipeline: *const Pipeline) -> bool;
    fn pipeline_is_recording(pipeline: *const Pipeline) -> bool;
    fn pipeline_get_frame_count(pipeline: *const Pipeline) -> u64;
    fn pipeline_get_pass_count(pipeline: *const Pipeline) -> u32;
    fn pipeline_is_validation_enabled(pipeline: *const Pipeline) -> bool;
    fn pipeline_is_debug_markers_enabled(pipeline: *const Pipeline) -> bool;
});

impl PipelineConfig {
    pub fn default() -> Self {
        pipeline_config_default()
    }

    pub fn new(
        max_passes: u32,
        max_resources: u32,
        enable_validation: bool,
        enable_debug_markers: bool,
    ) -> Self {
        pipeline_config_create(max_passes, max_resources, enable_validation, enable_debug_markers)
    }
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self::default()
    }
}

impl Pipeline {
    pub fn new() -> Self {
        pipeline_create()
    }

    pub fn with_config(config: PipelineConfig) -> Self {
        pipeline_create_with_config(config)
    }

    pub fn init(&mut self) {
        pipeline_init(self);
    }

    pub fn shutdown(&mut self) {
        pipeline_shutdown(self);
    }

    pub fn begin_frame(&mut self) -> bool {
        pipeline_begin_frame(self)
    }

    pub fn end_frame(&mut self) -> bool {
        pipeline_end_frame(self)
    }

    pub fn is_initialized(&self) -> bool {
        pipeline_is_initialized(self)
    }

    pub fn is_recording(&self) -> bool {
        pipeline_is_recording(self)
    }

    pub fn frame_count(&self) -> u64 {
        pipeline_get_frame_count(self)
    }

    pub fn pass_count(&self) -> u32 {
        pipeline_get_pass_count(self)
    }

    pub fn is_validation_enabled(&self) -> bool {
        pipeline_is_validation_enabled(self)
    }

    pub fn is_debug_markers_enabled(&self) -> bool {
        pipeline_is_debug_markers_enabled(self)
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}