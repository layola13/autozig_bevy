//! PBR 纹理管理模块
//!
//! 负责纹理的绑定、采样和管理
//! 支持：基础颜色、法线贴图、金属度/粗糙度、自发光纹理

const std = @import("std");
const pbr_material = @import("pbr_material.zig");
const MaterialHandle = pbr_material.MaterialHandle;

// ============================================================================
// 纹理数据结构
// ============================================================================

/// 纹理格式
pub const TextureFormat = enum(u32) {
    RGBA8,
    RGB8,
    RG8,
    R8,
    RGBA16F,
    RGB16F,
};

/// 纹理描述符
pub const TextureDescriptor = extern struct {
    width: u32,
    height: u32,
    format: TextureFormat,
    data: [*]const u8,
    data_size: usize,
};

/// 纹理句柄（不透明指针）
pub const TextureHandle = ?*anyopaque;

// ============================================================================
// 纹理绑定函数
// ============================================================================

/// 设置基础颜色纹理
pub fn setBaseColorTexture(material: MaterialHandle, texture: [*]const u8) void {
    // 将纹理数据转换为不透明指针
    const tex_handle: ?*anyopaque = @ptrFromInt(@intFromPtr(texture));
    pbr_material.setBaseColorTexture(material, tex_handle);
}

/// 设置法线贴图
pub fn setNormalTexture(material: MaterialHandle, texture: [*]const u8) void {
    const tex_handle: ?*anyopaque = @ptrFromInt(@intFromPtr(texture));
    pbr_material.setNormalTexture(material, tex_handle);
}

/// 设置金属度/粗糙度纹理
pub fn setMetallicRoughnessTexture(material: MaterialHandle, texture: [*]const u8) void {
    const tex_handle: ?*anyopaque = @ptrFromInt(@intFromPtr(texture));
    pbr_material.setMetallicRoughnessTexture(material, tex_handle);
}

/// 设置自发光纹理
pub fn setEmissiveTexture(material: MaterialHandle, texture: [*]const u8) void {
    const tex_handle: ?*anyopaque = @ptrFromInt(@intFromPtr(texture));
    pbr_material.setEmissiveTexture(material, tex_handle);
}

// ============================================================================
// 纹理采样（简化版，实际应由 GPU 处理）
// ============================================================================

/// UV 坐标
pub const UV = struct {
    u: f32,
    v: f32,
};

/// 双线性插值采样 RGBA8 纹理
pub fn sampleRGBA8(texture: TextureHandle, uv: UV, width: u32, height: u32) [4]f32 {
    if (texture == null) return [4]f32{ 1.0, 1.0, 1.0, 1.0 };

    // 简化实现：返回默认值
    // 实际应实现双线性插值采样
    _ = uv;
    _ = width;
    _ = height;

    return [4]f32{ 1.0, 1.0, 1.0, 1.0 };
}

/// 采样法线贴图
pub fn sampleNormal(texture: TextureHandle, uv: UV, width: u32, height: u32) [3]f32 {
    if (texture == null) return [3]f32{ 0.0, 0.0, 1.0 }; // 默认法线向上

    const color = sampleRGBA8(texture, uv, width, height);

    // 法线贴图解码: [0,1] -> [-1,1]
    const nx = color[0] * 2.0 - 1.0;
    const ny = color[1] * 2.0 - 1.0;
    const nz = color[2] * 2.0 - 1.0;

    // 归一化
    const len = @sqrt(nx * nx + ny * ny + nz * nz);
    if (len < 0.0001) return [3]f32{ 0.0, 0.0, 1.0 };

    return [3]f32{ nx / len, ny / len, nz / len };
}

/// 采样金属度/粗糙度纹理
/// 通道布局: R = (unused), G = Roughness, B = Metallic
pub fn sampleMetallicRoughness(texture: TextureHandle, uv: UV, width: u32, height: u32) [2]f32 {
    if (texture == null) return [2]f32{ 0.0, 0.5 }; // 默认: 非金属, 中等粗糙度

    const color = sampleRGBA8(texture, uv, width, height);

    // glTF 2.0 标准: G = Roughness, B = Metallic
    const roughness = color[1];
    const metallic = color[2];

    return [2]f32{ metallic, roughness };
}

/// 采样自发光纹理
pub fn sampleEmissive(texture: TextureHandle, uv: UV, width: u32, height: u32) [3]f32 {
    if (texture == null) return [3]f32{ 0.0, 0.0, 0.0 }; // 默认: 无自发光

    const color = sampleRGBA8(texture, uv, width, height);
    return [3]f32{ color[0], color[1], color[2] };
}

// ============================================================================
// 法线贴图切线空间变换
// ============================================================================

/// TBN 矩阵（切线-副切线-法线）
pub const TBN = struct {
    tangent: [3]f32,
    bitangent: [3]f32,
    normal: [3]f32,
};

/// 从切线空间变换法线到世界空间
pub fn transformNormal(tangent_normal: [3]f32, tbn: TBN) [3]f32 {
    const x = tangent_normal[0] * tbn.tangent[0] +
        tangent_normal[1] * tbn.bitangent[0] +
        tangent_normal[2] * tbn.normal[0];

    const y = tangent_normal[0] * tbn.tangent[1] +
        tangent_normal[1] * tbn.bitangent[1] +
        tangent_normal[2] * tbn.normal[1];

    const z = tangent_normal[0] * tbn.tangent[2] +
        tangent_normal[1] * tbn.bitangent[2] +
        tangent_normal[2] * tbn.normal[2];

    // 归一化
    const len = @sqrt(x * x + y * y + z * z);
    if (len < 0.0001) return tbn.normal;

    return [3]f32{ x / len, y / len, z / len };
}

/// 计算切线和副切线（Gram-Schmidt 正交化）
pub fn calculateTBN(normal: [3]f32, tangent: [3]f32) TBN {
    // 归一化法线
    const N = normalizeVec3(normal);

    // 归一化切线
    var T = normalizeVec3(tangent);

    // Gram-Schmidt 正交化: T = T - (T·N)N
    const dot_TN = dotVec3(T, N);
    T = normalizeVec3([3]f32{
        T[0] - dot_TN * N[0],
        T[1] - dot_TN * N[1],
        T[2] - dot_TN * N[2],
    });

    // 副切线 B = N × T
    const B = crossVec3(N, T);

    return TBN{
        .tangent = T,
        .bitangent = B,
        .normal = N,
    };
}

// ============================================================================
// 辅助函数
// ============================================================================

inline fn normalizeVec3(v: [3]f32) [3]f32 {
    const len = @sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2]);
    if (len < 0.0001) return [3]f32{ 0.0, 0.0, 1.0 };
    return [3]f32{ v[0] / len, v[1] / len, v[2] / len };
}

inline fn dotVec3(a: [3]f32, b: [3]f32) f32 {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

inline fn crossVec3(a: [3]f32, b: [3]f32) [3]f32 {
    return [3]f32{
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    };
}

// ============================================================================
// 测试
// ============================================================================

test "texture format" {
    const testing = std.testing;

    const fmt = TextureFormat.RGBA8;
    try testing.expectEqual(TextureFormat.RGBA8, fmt);
}

test "UV wrapping" {
    const testing = std.testing;

    const uv = UV{ .u = 1.5, .v = -0.5 };
    const wrapped_u = uv.u - @floor(uv.u);
    const wrapped_v = uv.v - @floor(uv.v);

    try testing.expect(wrapped_u >= 0.0 and wrapped_u <= 1.0);
    try testing.expect(wrapped_v >= 0.0 and wrapped_v <= 1.0);
}

test "TBN calculation" {
    const testing = std.testing;

    const normal = [3]f32{ 0.0, 1.0, 0.0 };
    const tangent = [3]f32{ 1.0, 0.0, 0.0 };

    const tbn = calculateTBN(normal, tangent);

    // 验证 TBN 是正交的
    const dot_TN = dotVec3(tbn.tangent, tbn.normal);
    const dot_BN = dotVec3(tbn.bitangent, tbn.normal);
    const dot_TB = dotVec3(tbn.tangent, tbn.bitangent);

    try testing.expectApproxEqAbs(@as(f32, 0.0), dot_TN, 0.0001);
    try testing.expectApproxEqAbs(@as(f32, 0.0), dot_BN, 0.0001);
    try testing.expectApproxEqAbs(@as(f32, 0.0), dot_TB, 0.0001);
}

test "normal transformation" {
    const testing = std.testing;

    const tangent_normal = [3]f32{ 0.0, 0.0, 1.0 };
    const tbn = TBN{
        .tangent = [3]f32{ 1.0, 0.0, 0.0 },
        .bitangent = [3]f32{ 0.0, 1.0, 0.0 },
        .normal = [3]f32{ 0.0, 0.0, 1.0 },
    };

    const world_normal = transformNormal(tangent_normal, tbn);

    // 应该等于 TBN 的法线
    try testing.expectApproxEqAbs(@as(f32, 0.0), world_normal[0], 0.0001);
    try testing.expectApproxEqAbs(@as(f32, 0.0), world_normal[1], 0.0001);
    try testing.expectApproxEqAbs(@as(f32, 1.0), world_normal[2], 0.0001);
}

// ============================================================================
// FFI Export Layer
// ============================================================================

export fn pbr_material_bind_base_color_texture(handle: MaterialHandle, data: *const u8, width: u32, height: u32) bool {
    _ = width;
    _ = height;
    setBaseColorTexture(handle, @ptrFromInt(@intFromPtr(data)));
    return true;
}

export fn pbr_material_bind_normal_texture(handle: MaterialHandle, data: *const u8, width: u32, height: u32) bool {
    _ = width;
    _ = height;
    setNormalTexture(handle, @ptrFromInt(@intFromPtr(data)));
    return true;
}

export fn pbr_material_bind_metallic_roughness_texture(handle: MaterialHandle, data: *const u8, width: u32, height: u32) bool {
    _ = width;
    _ = height;
    setMetallicRoughnessTexture(handle, @ptrFromInt(@intFromPtr(data)));
    return true;
}

export fn pbr_material_bind_emissive_texture(handle: MaterialHandle, data: *const u8, width: u32, height: u32) bool {
    _ = width;
    _ = height;
    setEmissiveTexture(handle, @ptrFromInt(@intFromPtr(data)));
    return true;
}
