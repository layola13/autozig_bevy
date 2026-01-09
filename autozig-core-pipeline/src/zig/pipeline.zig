//! Core Pipeline
//! Integrates render graph, pass scheduler, and resource management

const std = @import("std");

/// Pipeline configuration
pub const PipelineConfig = extern struct {
    max_passes: u32,
    max_resources: u32,
    enable_validation: bool,
    enable_debug_markers: bool,
};

/// Pipeline statistics
pub const PipelineStats = extern struct {
    frame_count: u64,
    pass_count: u32,
    draw_calls: u32,
    triangles: u32,
    resources_created: u32,
    resources_destroyed: u32,
    barriers_executed: u32,
};

/// Core rendering pipeline
pub const Pipeline = extern struct {
    config: PipelineConfig,
    stats: PipelineStats,
    is_initialized: bool,
    is_recording: bool,
};

/// Create default pipeline config
export fn pipeline_config_default() PipelineConfig {
    return PipelineConfig{
        .max_passes = 64,
        .max_resources = 256,
        .enable_validation = true,
        .enable_debug_markers = false,
    };
}

/// Create pipeline config
export fn pipeline_config_create(
    max_passes: u32,
    max_resources: u32,
    enable_validation: bool,
    enable_debug_markers: bool,
) PipelineConfig {
    return PipelineConfig{
        .max_passes = max_passes,
        .max_resources = max_resources,
        .enable_validation = enable_validation,
        .enable_debug_markers = enable_debug_markers,
    };
}

/// Create empty pipeline stats
export fn pipeline_stats_create() PipelineStats {
    return PipelineStats{
        .frame_count = 0,
        .pass_count = 0,
        .draw_calls = 0,
        .triangles = 0,
        .resources_created = 0,
        .resources_destroyed = 0,
        .barriers_executed = 0,
    };
}

/// Reset pipeline stats
export fn pipeline_stats_reset(stats: *PipelineStats) void {
    stats.pass_count = 0;
    stats.draw_calls = 0;
    stats.triangles = 0;
}

/// Increment frame count
export fn pipeline_stats_increment_frame(stats: *PipelineStats) void {
    stats.frame_count += 1;
}

/// Create pipeline
export fn pipeline_create() Pipeline {
    return Pipeline{
        .config = pipeline_config_default(),
        .stats = pipeline_stats_create(),
        .is_initialized = false,
        .is_recording = false,
    };
}

/// Create pipeline with config
export fn pipeline_create_with_config(config: PipelineConfig) Pipeline {
    return Pipeline{
        .config = config,
        .stats = pipeline_stats_create(),
        .is_initialized = false,
        .is_recording = false,
    };
}

/// Initialize pipeline
export fn pipeline_init(pipeline: *Pipeline) void {
    pipeline.is_initialized = true;
    pipeline.is_recording = false;
    pipeline_stats_reset(&pipeline.stats);
}

/// Shutdown pipeline
export fn pipeline_shutdown(pipeline: *Pipeline) void {
    pipeline.is_initialized = false;
    pipeline.is_recording = false;
}

/// Begin frame
export fn pipeline_begin_frame(pipeline: *Pipeline) bool {
    if (!pipeline.is_initialized) return false;
    if (pipeline.is_recording) return false;

    pipeline.is_recording = true;
    pipeline_stats_reset(&pipeline.stats);
    return true;
}

/// End frame
export fn pipeline_end_frame(pipeline: *Pipeline) bool {
    if (!pipeline.is_recording) return false;

    pipeline.is_recording = false;
    pipeline_stats_increment_frame(&pipeline.stats);
    return true;
}

/// Check if pipeline is initialized
export fn pipeline_is_initialized(pipeline: *const Pipeline) bool {
    return pipeline.is_initialized;
}

/// Check if pipeline is recording
export fn pipeline_is_recording(pipeline: *const Pipeline) bool {
    return pipeline.is_recording;
}

/// Get pipeline stats
export fn pipeline_get_stats(pipeline: *const Pipeline, out_stats: *PipelineStats) void {
    out_stats.* = pipeline.stats;
}

/// Get frame count
export fn pipeline_get_frame_count(pipeline: *const Pipeline) u64 {
    return pipeline.stats.frame_count;
}

/// Get pass count
export fn pipeline_get_pass_count(pipeline: *const Pipeline) u32 {
    return pipeline.stats.pass_count;
}

/// Increment pass count
export fn pipeline_increment_pass_count(pipeline: *Pipeline) void {
    pipeline.stats.pass_count += 1;
}

/// Increment draw call count
export fn pipeline_increment_draw_calls(pipeline: *Pipeline, count: u32) void {
    pipeline.stats.draw_calls += count;
}

/// Increment triangle count
export fn pipeline_increment_triangles(pipeline: *Pipeline, count: u32) void {
    pipeline.stats.triangles += count;
}

/// Increment resources created
export fn pipeline_increment_resources_created(pipeline: *Pipeline) void {
    pipeline.stats.resources_created += 1;
}

/// Increment resources destroyed
export fn pipeline_increment_resources_destroyed(pipeline: *Pipeline) void {
    pipeline.stats.resources_destroyed += 1;
}

/// Increment barriers executed
export fn pipeline_increment_barriers_executed(pipeline: *Pipeline, count: u32) void {
    pipeline.stats.barriers_executed += count;
}

/// Get config
export fn pipeline_get_config(pipeline: *const Pipeline, out_config: *PipelineConfig) void {
    out_config.* = pipeline.config;
}

/// Check if validation is enabled
export fn pipeline_is_validation_enabled(pipeline: *const Pipeline) bool {
    return pipeline.config.enable_validation;
}

/// Check if debug markers are enabled
export fn pipeline_is_debug_markers_enabled(pipeline: *const Pipeline) bool {
    return pipeline.config.enable_debug_markers;
}
