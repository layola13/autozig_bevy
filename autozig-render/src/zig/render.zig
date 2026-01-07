const std = @import("std");
const builtin = @import("builtin");

// ============================================================================
// 核心类型定义 - 参考 bevy_render 架构
// ============================================================================

/// 渲染器句柄
pub const Renderer = opaque {};

/// 渲染上下文
pub const RenderContext = struct {
    device_ptr: ?*anyopaque,
    queue_ptr: ?*anyopaque,
    surface_ptr: ?*anyopaque,
    allocator: std.mem.Allocator,
};

/// 渲染资源类型
pub const ResourceType = enum(u32) {
    buffer,
    texture,
    sampler,
    bind_group,
    pipeline,
    shader,
};

/// 缓冲区使用标志
pub const BufferUsage = packed struct {
    map_read: bool = false,
    map_write: bool = false,
    copy_src: bool = false,
    copy_dst: bool = false,
    index: bool = false,
    vertex: bool = false,
    uniform: bool = false,
    storage: bool = false,
    indirect: bool = false,
    query_resolve: bool = false,
    _padding: u22 = 0,
};

/// 纹理格式
pub const TextureFormat = enum(u32) {
    rgba8_unorm,
    rgba8_unorm_srgb,
    bgra8_unorm,
    bgra8_unorm_srgb,
    depth24_plus,
    depth32_float,
    depth24_plus_stencil8,
};

/// 纹理维度
pub const TextureDimension = enum(u32) {
    d1,
    d2,
    d3,
};

/// 纹理使用标志
pub const TextureUsage = packed struct {
    copy_src: bool = false,
    copy_dst: bool = false,
    texture_binding: bool = false,
    storage_binding: bool = false,
    render_attachment: bool = false,
    _padding: u27 = 0,
};

/// 渲染管线描述符
pub const RenderPipelineDescriptor = struct {
    vertex_shader: []const u8,
    fragment_shader: []const u8,
    vertex_buffers: []const VertexBufferLayout,
    bind_groups: []const BindGroupLayout,
    targets: []const ColorTargetState,
    depth_stencil: ?DepthStencilState = null,
    primitive: PrimitiveState = .{},
    multisample: MultisampleState = .{},
};

/// 顶点缓冲区布局
pub const VertexBufferLayout = struct {
    stride: u64,
    step_mode: VertexStepMode = .vertex,
    attributes: []const VertexAttribute,
};

/// 顶点步进模式
pub const VertexStepMode = enum(u32) {
    vertex,
    instance,
};

/// 顶点属性
pub const VertexAttribute = struct {
    format: VertexFormat,
    offset: u64,
    shader_location: u32,
};

/// 顶点格式
pub const VertexFormat = enum(u32) {
    float32,
    float32x2,
    float32x3,
    float32x4,
    uint32,
    uint32x2,
    uint32x3,
    uint32x4,
    sint32,
    sint32x2,
    sint32x3,
    sint32x4,
};

/// 绑定组布局
pub const BindGroupLayout = struct {
    entries: []const BindGroupLayoutEntry,
};

/// 绑定组布局条目
pub const BindGroupLayoutEntry = struct {
    binding: u32,
    visibility: ShaderStage,
    ty: BindingType,
};

/// 着色器阶段
pub const ShaderStage = packed struct {
    vertex: bool = false,
    fragment: bool = false,
    compute: bool = false,
    _padding: u29 = 0,
};

/// 绑定类型
pub const BindingType = union(enum) {
    buffer: BufferBindingType,
    sampler: SamplerBindingType,
    texture: TextureBindingType,
    storage_texture: StorageTextureBindingType,
};

pub const BufferBindingType = struct {
    ty: BufferBindingKind = .uniform,
    has_dynamic_offset: bool = false,
    min_binding_size: u64 = 0,
};

pub const BufferBindingKind = enum {
    uniform,
    storage,
    read_only_storage,
};

pub const SamplerBindingType = enum {
    filtering,
    non_filtering,
    comparison,
};

pub const TextureBindingType = struct {
    sample_type: TextureSampleType = .float,
    view_dimension: TextureViewDimension = .d2,
    multisampled: bool = false,
};

pub const TextureSampleType = enum {
    float,
    unfilterable_float,
    depth,
    sint,
    uint,
};

pub const TextureViewDimension = enum {
    d1,
    d2,
    d2_array,
    cube,
    cube_array,
    d3,
};

pub const StorageTextureBindingType = struct {
    access: StorageTextureAccess,
    format: TextureFormat,
    view_dimension: TextureViewDimension = .d2,
};

pub const StorageTextureAccess = enum {
    write_only,
    read_only,
    read_write,
};

/// 颜色目标状态
pub const ColorTargetState = struct {
    format: TextureFormat,
    blend: ?BlendState = null,
    write_mask: ColorWrite = .all,
};

/// 混合状态
pub const BlendState = struct {
    color: BlendComponent,
    alpha: BlendComponent,
};

pub const BlendComponent = struct {
    src_factor: BlendFactor = .one,
    dst_factor: BlendFactor = .zero,
    operation: BlendOperation = .add,
};

pub const BlendFactor = enum {
    zero,
    one,
    src,
    one_minus_src,
    src_alpha,
    one_minus_src_alpha,
    dst,
    one_minus_dst,
    dst_alpha,
    one_minus_dst_alpha,
    src_alpha_saturated,
    constant,
    one_minus_constant,
};

pub const BlendOperation = enum {
    add,
    subtract,
    reverse_subtract,
    min,
    max,
};

pub const ColorWrite = enum(u32) {
    red = 0x1,
    green = 0x2,
    blue = 0x4,
    alpha = 0x8,
    all = 0xF,
};

/// 深度模板状态
pub const DepthStencilState = struct {
    format: TextureFormat,
    depth_write_enabled: bool = true,
    depth_compare: CompareFunction = .less,
    stencil: StencilState = .{},
    depth_bias: i32 = 0,
    depth_bias_slope_scale: f32 = 0.0,
    depth_bias_clamp: f32 = 0.0,
};

pub const CompareFunction = enum {
    never,
    less,
    equal,
    less_equal,
    greater,
    not_equal,
    greater_equal,
    always,
};

pub const StencilState = struct {
    front: StencilFaceState = .{},
    back: StencilFaceState = .{},
    read_mask: u32 = 0xFFFFFFFF,
    write_mask: u32 = 0xFFFFFFFF,
};

pub const StencilFaceState = struct {
    compare: CompareFunction = .always,
    fail_op: StencilOperation = .keep,
    depth_fail_op: StencilOperation = .keep,
    pass_op: StencilOperation = .keep,
};

pub const StencilOperation = enum {
    keep,
    zero,
    replace,
    invert,
    increment_clamp,
    decrement_clamp,
    increment_wrap,
    decrement_wrap,
};

/// 图元状态
pub const PrimitiveState = struct {
    topology: PrimitiveTopology = .triangle_list,
    strip_index_format: ?IndexFormat = null,
    front_face: FrontFace = .ccw,
    cull_mode: ?CullMode = null,
    unclipped_depth: bool = false,
    polygon_mode: PolygonMode = .fill,
};

pub const PrimitiveTopology = enum {
    point_list,
    line_list,
    line_strip,
    triangle_list,
    triangle_strip,
};

pub const IndexFormat = enum {
    uint16,
    uint32,
};

pub const FrontFace = enum {
    ccw,
    cw,
};

pub const CullMode = enum {
    none,
    front,
    back,
};

pub const PolygonMode = enum {
    fill,
    line,
    point,
};

/// 多重采样状态
pub const MultisampleState = struct {
    count: u32 = 1,
    mask: u64 = 0xFFFFFFFF,
    alpha_to_coverage_enabled: bool = false,
};

// ============================================================================
// 渲染器 API - 参考 bevy_render 接口
// ============================================================================

/// 创建渲染器
pub export fn renderer_create(allocator: *std.mem.Allocator) ?*Renderer {
    _ = allocator;
    // 实现将在后续添加
    return null;
}

/// 销毁渲染器
pub export fn renderer_destroy(renderer: *Renderer) void {
    _ = renderer;
    // 实现将在后续添加
}

/// 创建缓冲区
pub export fn create_buffer(
    renderer: *Renderer,
    size: u64,
    usage: BufferUsage,
    mapped_at_creation: bool,
) ?*anyopaque {
    _ = renderer;
    _ = size;
    _ = usage;
    _ = mapped_at_creation;
    return null;
}

/// 销毁缓冲区
pub export fn destroy_buffer(renderer: *Renderer, buffer: *anyopaque) void {
    _ = renderer;
    _ = buffer;
}

/// 写入缓冲区数据
pub export fn write_buffer(
    renderer: *Renderer,
    buffer: *anyopaque,
    offset: u64,
    data: [*]const u8,
    size: u64,
) void {
    _ = renderer;
    _ = buffer;
    _ = offset;
    _ = data;
    _ = size;
}

/// 创建纹理
pub export fn create_texture(
    renderer: *Renderer,
    width: u32,
    height: u32,
    depth: u32,
    format: TextureFormat,
    dimension: TextureDimension,
    usage: TextureUsage,
) ?*anyopaque {
    _ = renderer;
    _ = width;
    _ = height;
    _ = depth;
    _ = format;
    _ = dimension;
    _ = usage;
    return null;
}

/// 销毁纹理
pub export fn destroy_texture(renderer: *Renderer, texture: *anyopaque) void {
    _ = renderer;
    _ = texture;
}

/// 创建渲染管线
pub export fn create_render_pipeline(
    renderer: *Renderer,
    desc: *const RenderPipelineDescriptor,
) ?*anyopaque {
    _ = renderer;
    _ = desc;
    return null;
}

/// 销毁渲染管线
pub export fn destroy_render_pipeline(renderer: *Renderer, pipeline: *anyopaque) void {
    _ = renderer;
    _ = pipeline;
}

/// 开始渲染通道
pub export fn begin_render_pass(
    renderer: *Renderer,
    color_attachments: [*]const ColorAttachment,
    color_attachment_count: u32,
    depth_stencil_attachment: ?*const DepthStencilAttachment,
) ?*anyopaque {
    _ = renderer;
    _ = color_attachments;
    _ = color_attachment_count;
    _ = depth_stencil_attachment;
    return null;
}

pub const ColorAttachment = struct {
    view: *anyopaque,
    resolve_target: ?*anyopaque = null,
    load_op: LoadOp,
    store_op: StoreOp,
    clear_color: [4]f32,
};

pub const DepthStencilAttachment = struct {
    view: *anyopaque,
    depth_load_op: LoadOp,
    depth_store_op: StoreOp,
    depth_clear_value: f32,
    stencil_load_op: LoadOp,
    stencil_store_op: StoreOp,
    stencil_clear_value: u32,
};

pub const LoadOp = enum {
    clear,
    load,
};

pub const StoreOp = enum {
    store,
    discard,
};

/// 结束渲染通道
pub export fn end_render_pass(renderer: *Renderer, pass: *anyopaque) void {
    _ = renderer;
    _ = pass;
}

/// 设置渲染管线
pub export fn set_pipeline(pass: *anyopaque, pipeline: *anyopaque) void {
    _ = pass;
    _ = pipeline;
}

/// 设置顶点缓冲区
pub export fn set_vertex_buffer(
    pass: *anyopaque,
    slot: u32,
    buffer: *anyopaque,
    offset: u64,
    size: u64,
) void {
    _ = pass;
    _ = slot;
    _ = buffer;
    _ = offset;
    _ = size;
}

/// 设置索引缓冲区
pub export fn set_index_buffer(
    pass: *anyopaque,
    buffer: *anyopaque,
    format: IndexFormat,
    offset: u64,
    size: u64,
) void {
    _ = pass;
    _ = buffer;
    _ = format;
    _ = offset;
    _ = size;
}

/// 设置绑定组
pub export fn set_bind_group(
    pass: *anyopaque,
    index: u32,
    bind_group: *anyopaque,
    dynamic_offsets: [*]const u32,
    dynamic_offset_count: u32,
) void {
    _ = pass;
    _ = index;
    _ = bind_group;
    _ = dynamic_offsets;
    _ = dynamic_offset_count;
}

/// 绘制
pub export fn draw(
    pass: *anyopaque,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
) void {
    _ = pass;
    _ = vertex_count;
    _ = instance_count;
    _ = first_vertex;
    _ = first_instance;
}

/// 绘制索引
pub export fn draw_indexed(
    pass: *anyopaque,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
) void {
    _ = pass;
    _ = index_count;
    _ = instance_count;
    _ = first_index;
    _ = base_vertex;
    _ = first_instance;
}

/// 提交命令
pub export fn submit_commands(renderer: *Renderer) void {
    _ = renderer;
}

/// 呈现帧
pub export fn present_frame(renderer: *Renderer) void {
    _ = renderer;
}

// ============================================================================
// 测试
// ============================================================================

test "basic types" {
    const usage = BufferUsage{
        .vertex = true,
        .uniform = true,
    };
    try std.testing.expect(usage.vertex);
    try std.testing.expect(usage.uniform);
}

test "render pipeline descriptor" {
    const desc = RenderPipelineDescriptor{
        .vertex_shader = "vertex.wgsl",
        .fragment_shader = "fragment.wgsl",
        .vertex_buffers = &[_]VertexBufferLayout{},
        .bind_groups = &[_]BindGroupLayout{},
        .targets = &[_]ColorTargetState{},
    };
    try std.testing.expect(desc.vertex_shader.len > 0);
}
