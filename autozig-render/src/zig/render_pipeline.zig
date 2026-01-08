//! Render Pipeline Management
//! Handles WebGPU render pipeline creation and management

const std = @import("std");

/// Vertex attribute format
pub const VertexFormat = enum(u32) {
    Float32 = 0,
    Float32x2 = 1,
    Float32x3 = 2,
    Float32x4 = 3,
    Uint32 = 4,
    Uint32x2 = 5,
    Uint32x3 = 6,
    Uint32x4 = 7,
};

/// Vertex attribute descriptor
pub const VertexAttribute = extern struct {
    format: u32, // VertexFormat
    offset: u32,
    shader_location: u32,
};

/// Vertex buffer layout
pub const VertexLayout = extern struct {
    attributes: [8]VertexAttribute,
    attribute_count: u32,
    array_stride: u32,
    step_mode: u32, // 0 = Vertex, 1 = Instance
};

/// Primitive topology
pub const PrimitiveTopology = enum(u32) {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
};

/// Depth stencil state
pub const DepthStencilState = extern struct {
    format: u32, // TextureFormat
    depth_write_enabled: bool,
    depth_compare: u32, // CompareFunction
    stencil_read_mask: u32,
    stencil_write_mask: u32,
};

/// Render pipeline descriptor
pub const RenderPipelineDescriptor = extern struct {
    vertex_shader: [128]u8,
    fragment_shader: [128]u8,
    vertex_shader_len: u32,
    fragment_shader_len: u32,
    vertex_layout: VertexLayout,
    primitive_topology: u32,
    has_depth_stencil: bool,
    depth_stencil: DepthStencilState,
};

/// Render pipeline handle
pub const RenderPipeline = extern struct {
    handle: ?*anyopaque = null,
    is_valid: bool = false,
};

/// Create default vertex layout
export fn render_pipeline_default_vertex_layout() VertexLayout {
    return VertexLayout{
        .attributes = [_]VertexAttribute{.{
            .format = @intFromEnum(VertexFormat.Float32x3),
            .offset = 0,
            .shader_location = 0,
        }} ** 8,
        .attribute_count = 0,
        .array_stride = 0,
        .step_mode = 0,
    };
}

/// Create vertex layout with position only
export fn render_pipeline_vertex_layout_position() VertexLayout {
    var layout = VertexLayout{
        .attributes = [_]VertexAttribute{.{
            .format = @intFromEnum(VertexFormat.Float32x3),
            .offset = 0,
            .shader_location = 0,
        }} ** 8,
        .attribute_count = 1,
        .array_stride = 12, // 3 * sizeof(f32)
        .step_mode = 0,
    };
    layout.attributes[0] = VertexAttribute{
        .format = @intFromEnum(VertexFormat.Float32x3),
        .offset = 0,
        .shader_location = 0,
    };
    return layout;
}

/// Create vertex layout with position and color
export fn render_pipeline_vertex_layout_position_color() VertexLayout {
    var layout = VertexLayout{
        .attributes = [_]VertexAttribute{.{
            .format = @intFromEnum(VertexFormat.Float32x3),
            .offset = 0,
            .shader_location = 0,
        }} ** 8,
        .attribute_count = 2,
        .array_stride = 28, // 3 * sizeof(f32) + 4 * sizeof(f32)
        .step_mode = 0,
    };
    layout.attributes[0] = VertexAttribute{
        .format = @intFromEnum(VertexFormat.Float32x3),
        .offset = 0,
        .shader_location = 0,
    };
    layout.attributes[1] = VertexAttribute{
        .format = @intFromEnum(VertexFormat.Float32x4),
        .offset = 12,
        .shader_location = 1,
    };
    return layout;
}

/// Create vertex layout with position, normal, and UV
export fn render_pipeline_vertex_layout_full() VertexLayout {
    var layout = VertexLayout{
        .attributes = [_]VertexAttribute{.{
            .format = @intFromEnum(VertexFormat.Float32x3),
            .offset = 0,
            .shader_location = 0,
        }} ** 8,
        .attribute_count = 3,
        .array_stride = 32, // 3 + 3 + 2 = 8 floats
        .step_mode = 0,
    };
    layout.attributes[0] = VertexAttribute{
        .format = @intFromEnum(VertexFormat.Float32x3),
        .offset = 0,
        .shader_location = 0,
    };
    layout.attributes[1] = VertexAttribute{
        .format = @intFromEnum(VertexFormat.Float32x3),
        .offset = 12,
        .shader_location = 1,
    };
    layout.attributes[2] = VertexAttribute{
        .format = @intFromEnum(VertexFormat.Float32x2),
        .offset = 24,
        .shader_location = 2,
    };
    return layout;
}

/// Create default render pipeline descriptor
export fn render_pipeline_descriptor_create() RenderPipelineDescriptor {
    return RenderPipelineDescriptor{
        .vertex_shader = [_]u8{0} ** 128,
        .fragment_shader = [_]u8{0} ** 128,
        .vertex_shader_len = 0,
        .fragment_shader_len = 0,
        .vertex_layout = render_pipeline_default_vertex_layout(),
        .primitive_topology = @intFromEnum(PrimitiveTopology.TriangleList),
        .has_depth_stencil = false,
        .depth_stencil = DepthStencilState{
            .format = 0,
            .depth_write_enabled = false,
            .depth_compare = 0,
            .stencil_read_mask = 0,
            .stencil_write_mask = 0,
        },
    };
}

/// Set vertex shader path
export fn render_pipeline_descriptor_set_vertex_shader(
    desc: *RenderPipelineDescriptor,
    path: [*]const u8,
    len: u32,
) void {
    const copy_len = @min(len, 127);
    @memcpy(desc.vertex_shader[0..copy_len], path[0..copy_len]);
    desc.vertex_shader[copy_len] = 0;
    desc.vertex_shader_len = copy_len;
}

/// Set fragment shader path
export fn render_pipeline_descriptor_set_fragment_shader(
    desc: *RenderPipelineDescriptor,
    path: [*]const u8,
    len: u32,
) void {
    const copy_len = @min(len, 127);
    @memcpy(desc.fragment_shader[0..copy_len], path[0..copy_len]);
    desc.fragment_shader[copy_len] = 0;
    desc.fragment_shader_len = copy_len;
}

/// Set vertex layout
export fn render_pipeline_descriptor_set_vertex_layout(
    desc: *RenderPipelineDescriptor,
    layout: VertexLayout,
) void {
    desc.vertex_layout = layout;
}

/// Set primitive topology
export fn render_pipeline_descriptor_set_topology(
    desc: *RenderPipelineDescriptor,
    topology: u32,
) void {
    desc.primitive_topology = topology;
}

/// Enable depth stencil
export fn render_pipeline_descriptor_enable_depth(
    desc: *RenderPipelineDescriptor,
    format: u32,
    write_enabled: bool,
) void {
    desc.has_depth_stencil = true;
    desc.depth_stencil.format = format;
    desc.depth_stencil.depth_write_enabled = write_enabled;
    desc.depth_stencil.depth_compare = 3; // Less
}

/// Create render pipeline (placeholder - actual creation in JavaScript)
export fn render_pipeline_create() RenderPipeline {
    return RenderPipeline{
        .handle = null,
        .is_valid = false,
    };
}

/// Set pipeline handle (from JavaScript)
export fn render_pipeline_set_handle(pipeline: *RenderPipeline, handle: ?*anyopaque) void {
    pipeline.handle = handle;
    pipeline.is_valid = handle != null;
}

/// Check if pipeline is valid
export fn render_pipeline_is_valid(pipeline: *const RenderPipeline) bool {
    return pipeline.is_valid and pipeline.handle != null;
}

/// Destroy render pipeline
export fn render_pipeline_destroy(pipeline: *RenderPipeline) void {
    pipeline.handle = null;
    pipeline.is_valid = false;
}
