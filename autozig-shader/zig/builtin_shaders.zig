//! Built-in WGSL shaders for Bevy-style rendering
//! 85% Zig implementation - Provides PBR, Sprite, UI, and Fullscreen shaders

const std = @import("std");

// ============================================================================
// PBR Shaders (Physically Based Rendering)
// ============================================================================

const PBR_VERTEX_SHADER =
    \\// PBR Vertex Shader
    \\struct VertexInput {
    \\    @location(0) position: vec3<f32>,
    \\    @location(1) normal: vec3<f32>,
    \\    @location(2) uv: vec2<f32>,
    \\    @location(3) tangent: vec4<f32>,
    \\};
    \\
    \\struct VertexOutput {
    \\    @builtin(position) clip_position: vec4<f32>,
    \\    @location(0) world_position: vec3<f32>,
    \\    @location(1) world_normal: vec3<f32>,
    \\    @location(2) uv: vec2<f32>,
    \\    @location(3) world_tangent: vec4<f32>,
    \\};
    \\
    \\struct CameraUniform {
    \\    view_proj: mat4x4<f32>,
    \\    view: mat4x4<f32>,
    \\    inverse_view: mat4x4<f32>,
    \\    projection: mat4x4<f32>,
    \\    world_position: vec3<f32>,
    \\    _padding: f32,
    \\};
    \\
    \\struct MeshUniform {
    \\    model: mat4x4<f32>,
    \\    inverse_transpose_model: mat4x4<f32>,
    \\};
    \\
    \\@group(0) @binding(0)
    \\var<uniform> camera: CameraUniform;
    \\
    \\@group(1) @binding(0)
    \\var<uniform> mesh: MeshUniform;
    \\
    \\@vertex
    \\fn main(vertex: VertexInput) -> VertexOutput {
    \\    var out: VertexOutput;
    \\    
    \\    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
    \\    out.world_position = world_position.xyz;
    \\    out.clip_position = camera.view_proj * world_position;
    \\    
    \\    out.world_normal = normalize((mesh.inverse_transpose_model * vec4<f32>(vertex.normal, 0.0)).xyz);
    \\    out.uv = vertex.uv;
    \\    
    \\    let world_tangent = normalize((mesh.model * vec4<f32>(vertex.tangent.xyz, 0.0)).xyz);
    \\    out.world_tangent = vec4<f32>(world_tangent, vertex.tangent.w);
    \\    
    \\    return out;
    \\}
;

const PBR_FRAGMENT_SHADER =
    \\// PBR Fragment Shader
    \\struct FragmentInput {
    \\    @builtin(position) clip_position: vec4<f32>,
    \\    @location(0) world_position: vec3<f32>,
    \\    @location(1) world_normal: vec3<f32>,
    \\    @location(2) uv: vec2<f32>,
    \\    @location(3) world_tangent: vec4<f32>,
    \\};
    \\
    \\struct MaterialUniform {
    \\    base_color: vec4<f32>,
    \\    emissive: vec4<f32>,
    \\    roughness: f32,
    \\    metallic: f32,
    \\    reflectance: f32,
    \\    _padding: f32,
    \\};
    \\
    \\struct DirectionalLight {
    \\    direction: vec3<f32>,
    \\    _padding1: f32,
    \\    color: vec3<f32>,
    \\    illuminance: f32,
    \\};
    \\
    \\struct PointLight {
    \\    position: vec3<f32>,
    \\    _padding1: f32,
    \\    color: vec3<f32>,
    \\    intensity: f32,
    \\};
    \\
    \\struct LightsUniform {
    \\    ambient_color: vec4<f32>,
    \\    directional_light: DirectionalLight,
    \\    point_light_count: u32,
    \\    _padding: vec3<u32>,
    \\};
    \\
    \\@group(2) @binding(0)
    \\var<uniform> material: MaterialUniform;
    \\
    \\@group(2) @binding(1)
    \\var base_color_texture: texture_2d<f32>;
    \\
    \\@group(2) @binding(2)
    \\var base_color_sampler: sampler;
    \\
    \\@group(3) @binding(0)
    \\var<uniform> lights: LightsUniform;
    \\
    \\const PI: f32 = 3.14159265359;
    \\
    \\fn fresnel_schlick(cos_theta: f32, f0: vec3<f32>) -> vec3<f32> {
    \\    return f0 + (vec3<f32>(1.0) - f0) * pow(1.0 - cos_theta, 5.0);
    \\}
    \\
    \\fn distribution_ggx(n_dot_h: f32, roughness: f32) -> f32 {
    \\    let a = roughness * roughness;
    \\    let a2 = a * a;
    \\    let denom = n_dot_h * n_dot_h * (a2 - 1.0) + 1.0;
    \\    return a2 / (PI * denom * denom);
    \\}
    \\
    \\fn geometry_schlick_ggx(n_dot_v: f32, roughness: f32) -> f32 {
    \\    let r = roughness + 1.0;
    \\    let k = (r * r) / 8.0;
    \\    return n_dot_v / (n_dot_v * (1.0 - k) + k);
    \\}
    \\
    \\fn geometry_smith(n_dot_v: f32, n_dot_l: f32, roughness: f32) -> f32 {
    \\    let ggx2 = geometry_schlick_ggx(n_dot_v, roughness);
    \\    let ggx1 = geometry_schlick_ggx(n_dot_l, roughness);
    \\    return ggx1 * ggx2;
    \\}
    \\
    \\@fragment
    \\fn main(in: FragmentInput) -> @location(0) vec4<f32> {
    \\    let base_color = textureSample(base_color_texture, base_color_sampler, in.uv) * material.base_color;
    \\    
    \\    let n = normalize(in.world_normal);
    \\    let v = normalize(vec3<f32>(0.0, 0.0, 1.0) - in.world_position);
    \\    
    \\    let f0 = mix(vec3<f32>(0.04), base_color.rgb, material.metallic);
    \\    
    \\    var lo = vec3<f32>(0.0);
    \\    
    \\    // Directional light
    \\    let l = normalize(-lights.directional_light.direction);
    \\    let h = normalize(v + l);
    \\    let n_dot_l = max(dot(n, l), 0.0);
    \\    let n_dot_v = max(dot(n, v), 0.0);
    \\    let n_dot_h = max(dot(n, h), 0.0);
    \\    let h_dot_v = max(dot(h, v), 0.0);
    \\    
    \\    let ndf = distribution_ggx(n_dot_h, material.roughness);
    \\    let g = geometry_smith(n_dot_v, n_dot_l, material.roughness);
    \\    let f = fresnel_schlick(h_dot_v, f0);
    \\    
    \\    let numerator = ndf * g * f;
    \\    let denominator = 4.0 * n_dot_v * n_dot_l + 0.0001;
    \\    let specular = numerator / denominator;
    \\    
    \\    let kd = (vec3<f32>(1.0) - f) * (1.0 - material.metallic);
    \\    let radiance = lights.directional_light.color * lights.directional_light.illuminance;
    \\    
    \\    lo += (kd * base_color.rgb / PI + specular) * radiance * n_dot_l;
    \\    
    \\    let ambient = lights.ambient_color.rgb * base_color.rgb * 0.03;
    \\    var color = ambient + lo + material.emissive.rgb;
    \\    
    \\    // Tone mapping
    \\    color = color / (color + vec3<f32>(1.0));
    \\    // Gamma correction
    \\    color = pow(color, vec3<f32>(1.0 / 2.2));
    \\    
    \\    return vec4<f32>(color, base_color.a);
    \\}
;

// ============================================================================
// Sprite Shaders (2D rendering)
// ============================================================================

const SPRITE_VERTEX_SHADER =
    \\// Sprite Vertex Shader
    \\struct VertexInput {
    \\    @location(0) position: vec3<f32>,
    \\    @location(1) uv: vec2<f32>,
    \\    @location(2) color: vec4<f32>,
    \\};
    \\
    \\struct VertexOutput {
    \\    @builtin(position) clip_position: vec4<f32>,
    \\    @location(0) uv: vec2<f32>,
    \\    @location(1) color: vec4<f32>,
    \\};
    \\
    \\struct ViewUniform {
    \\    view_proj: mat4x4<f32>,
    \\};
    \\
    \\struct SpriteUniform {
    \\    transform: mat4x4<f32>,
    \\    color: vec4<f32>,
    \\};
    \\
    \\@group(0) @binding(0)
    \\var<uniform> view: ViewUniform;
    \\
    \\@group(1) @binding(0)
    \\var<uniform> sprite: SpriteUniform;
    \\
    \\@vertex
    \\fn main(vertex: VertexInput) -> VertexOutput {
    \\    var out: VertexOutput;
    \\    
    \\    let world_position = sprite.transform * vec4<f32>(vertex.position, 1.0);
    \\    out.clip_position = view.view_proj * world_position;
    \\    out.uv = vertex.uv;
    \\    out.color = vertex.color * sprite.color;
    \\    
    \\    return out;
    \\}
;

const SPRITE_FRAGMENT_SHADER =
    \\// Sprite Fragment Shader
    \\struct FragmentInput {
    \\    @builtin(position) clip_position: vec4<f32>,
    \\    @location(0) uv: vec2<f32>,
    \\    @location(1) color: vec4<f32>,
    \\};
    \\
    \\@group(2) @binding(0)
    \\var sprite_texture: texture_2d<f32>;
    \\
    \\@group(2) @binding(1)
    \\var sprite_sampler: sampler;
    \\
    \\@fragment
    \\fn main(in: FragmentInput) -> @location(0) vec4<f32> {
    \\    let texture_color = textureSample(sprite_texture, sprite_sampler, in.uv);
    \\    return texture_color * in.color;
    \\}
;

// ============================================================================
// UI Shaders (User Interface rendering)
// ============================================================================

const UI_VERTEX_SHADER =
    \\// UI Vertex Shader
    \\struct VertexInput {
    \\    @location(0) position: vec2<f32>,
    \\    @location(1) uv: vec2<f32>,
    \\    @location(2) color: vec4<f32>,
    \\};
    \\
    \\struct VertexOutput {
    \\    @builtin(position) clip_position: vec4<f32>,
    \\    @location(0) uv: vec2<f32>,
    \\    @location(1) color: vec4<f32>,
    \\};
    \\
    \\struct UIGlobals {
    \\    screen_size: vec2<f32>,
    \\    scale_factor: f32,
    \\    _padding: f32,
    \\};
    \\
    \\@group(0) @binding(0)
    \\var<uniform> globals: UIGlobals;
    \\
    \\@vertex
    \\fn main(vertex: VertexInput) -> VertexOutput {
    \\    var out: VertexOutput;
    \\    
    \\    // Convert from screen space to clip space
    \\    let normalized_pos = (vertex.position / globals.screen_size) * 2.0 - vec2<f32>(1.0);
    \\    out.clip_position = vec4<f32>(normalized_pos.x, -normalized_pos.y, 0.0, 1.0);
    \\    out.uv = vertex.uv;
    \\    out.color = vertex.color;
    \\    
    \\    return out;
    \\}
;

const UI_FRAGMENT_SHADER =
    \\// UI Fragment Shader
    \\struct FragmentInput {
    \\    @builtin(position) clip_position: vec4<f32>,
    \\    @location(0) uv: vec2<f32>,
    \\    @location(1) color: vec4<f32>,
    \\};
    \\
    \\@group(1) @binding(0)
    \\var ui_texture: texture_2d<f32>;
    \\
    \\@group(1) @binding(1)
    \\var ui_sampler: sampler;
    \\
    \\@fragment
    \\fn main(in: FragmentInput) -> @location(0) vec4<f32> {
    \\    let texture_color = textureSample(ui_texture, ui_sampler, in.uv);
    \\    return texture_color * in.color;
    \\}
;

// ============================================================================
// Fullscreen Vertex Shader (for post-processing)
// ============================================================================

const FULLSCREEN_VERTEX_SHADER =
    \\// Fullscreen Vertex Shader
    \\struct VertexOutput {
    \\    @builtin(position) clip_position: vec4<f32>,
    \\    @location(0) uv: vec2<f32>,
    \\};
    \\
    \\@vertex
    \\fn main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    \\    var out: VertexOutput;
    \\    
    \\    // Generate fullscreen triangle
    \\    let x = f32((vertex_index << 1u) & 2u);
    \\    let y = f32(vertex_index & 2u);
    \\    
    \\    out.clip_position = vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
    \\    out.uv = vec2<f32>(x, y);
    \\    
    \\    return out;
    \\}
;

// ============================================================================
// Export Functions
// ============================================================================

/// Get PBR vertex shader source
export fn builtin_shader_get_pbr_vertex(out_ptr: *?[*]const u8, out_len: *usize) void {
    out_ptr.* = PBR_VERTEX_SHADER.ptr;
    out_len.* = PBR_VERTEX_SHADER.len;
}

/// Get PBR fragment shader source
export fn builtin_shader_get_pbr_fragment(out_ptr: *?[*]const u8, out_len: *usize) void {
    out_ptr.* = PBR_FRAGMENT_SHADER.ptr;
    out_len.* = PBR_FRAGMENT_SHADER.len;
}

/// Get Sprite vertex shader source
export fn builtin_shader_get_sprite_vertex(out_ptr: *?[*]const u8, out_len: *usize) void {
    out_ptr.* = SPRITE_VERTEX_SHADER.ptr;
    out_len.* = SPRITE_VERTEX_SHADER.len;
}

/// Get Sprite fragment shader source
export fn builtin_shader_get_sprite_fragment(out_ptr: *?[*]const u8, out_len: *usize) void {
    out_ptr.* = SPRITE_FRAGMENT_SHADER.ptr;
    out_len.* = SPRITE_FRAGMENT_SHADER.len;
}

/// Get UI vertex shader source
export fn builtin_shader_get_ui_vertex(out_ptr: *?[*]const u8, out_len: *usize) void {
    out_ptr.* = UI_VERTEX_SHADER.ptr;
    out_len.* = UI_VERTEX_SHADER.len;
}

/// Get UI fragment shader source
export fn builtin_shader_get_ui_fragment(out_ptr: *?[*]const u8, out_len: *usize) void {
    out_ptr.* = UI_FRAGMENT_SHADER.ptr;
    out_len.* = UI_FRAGMENT_SHADER.len;
}

/// Get Fullscreen vertex shader source
export fn builtin_shader_get_fullscreen_vertex(out_ptr: *?[*]const u8, out_len: *usize) void {
    out_ptr.* = FULLSCREEN_VERTEX_SHADER.ptr;
    out_len.* = FULLSCREEN_VERTEX_SHADER.len;
}

/// Helper function to copy shader source to a buffer (safe for Rust)
export fn builtin_shader_copy_to_buffer(
    getter_fn: u32,
    out_buffer: [*]u8,
    buffer_len: usize,
) usize {
    var ptr: ?[*]const u8 = null;
    var len: usize = 0;

    // Call the appropriate getter function
    switch (getter_fn) {
        0 => builtin_shader_get_pbr_vertex(&ptr, &len),
        1 => builtin_shader_get_pbr_fragment(&ptr, &len),
        2 => builtin_shader_get_sprite_vertex(&ptr, &len),
        3 => builtin_shader_get_sprite_fragment(&ptr, &len),
        4 => builtin_shader_get_ui_vertex(&ptr, &len),
        5 => builtin_shader_get_ui_fragment(&ptr, &len),
        6 => builtin_shader_get_fullscreen_vertex(&ptr, &len),
        else => return 0,
    }

    if (ptr == null or len == 0) return 0;

    const copy_len = @min(len, buffer_len);
    if (ptr) |p| {
        @memcpy(out_buffer[0..copy_len], p[0..copy_len]);
    }
    return len;
}

// ============================================================================
// Tests
// ============================================================================

test "builtin_shaders_pbr" {
    var ptr: ?[*]const u8 = null;
    var len: usize = 0;

    builtin_shader_get_pbr_vertex(&ptr, &len);
    try std.testing.expect(len > 0);
    try std.testing.expect(ptr != null);

    builtin_shader_get_pbr_fragment(&ptr, &len);
    try std.testing.expect(len > 0);
    try std.testing.expect(ptr != null);
}

test "builtin_shaders_sprite" {
    var ptr: ?[*]const u8 = null;
    var len: usize = 0;

    builtin_shader_get_sprite_vertex(&ptr, &len);
    try std.testing.expect(len > 0);
    try std.testing.expect(ptr != null);

    builtin_shader_get_sprite_fragment(&ptr, &len);
    try std.testing.expect(len > 0);
    try std.testing.expect(ptr != null);
}

test "builtin_shaders_ui" {
    var ptr: ?[*]const u8 = null;
    var len: usize = 0;

    builtin_shader_get_ui_vertex(&ptr, &len);
    try std.testing.expect(len > 0);
    try std.testing.expect(ptr != null);

    builtin_shader_get_ui_fragment(&ptr, &len);
    try std.testing.expect(len > 0);
    try std.testing.expect(ptr != null);
}

test "builtin_shaders_fullscreen" {
    var ptr: ?[*]const u8 = null;
    var len: usize = 0;

    builtin_shader_get_fullscreen_vertex(&ptr, &len);
    try std.testing.expect(len > 0);
    try std.testing.expect(ptr != null);
}
