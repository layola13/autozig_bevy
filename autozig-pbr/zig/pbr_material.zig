//! PBR 材质管理模块
//!
//! 负责材质数据的创建、存储和管理

const std = @import("std");

/// PBR 材质数据结构
pub const Material = extern struct {
    /// 基础颜色 RGBA
    base_color: [4]f32,
    /// 金属度 (0.0 = 电介质, 1.0 = 金属)
    metallic: f32,
    /// 粗糙度 (0.0 = 光滑, 1.0 = 粗糙)
    roughness: f32,
    /// 自发光颜色 RGB
    emissive: [3]f32,
    /// 对齐填充
    _padding1: f32,

    /// 纹理指针（不透明句柄）
    base_color_texture: ?*anyopaque,
    normal_texture: ?*anyopaque,
    metallic_roughness_texture: ?*anyopaque,
    emissive_texture: ?*anyopaque,

    /// 纹理标志位
    has_base_color_texture: bool,
    has_normal_texture: bool,
    has_metallic_roughness_texture: bool,
    has_emissive_texture: bool,

    /// 对齐填充
    _padding2: [4]u8,
};

/// 材质句柄（索引）
pub const MaterialHandle = u32;

/// 材质池（静态分配，避免动态内存）
const MAX_MATERIALS = 1024;
var material_pool: [MAX_MATERIALS]Material = undefined;
var material_free_list: [MAX_MATERIALS]bool = [_]bool{true} ** MAX_MATERIALS;
var next_material_id: u32 = 0;

/// 创建新材质
pub fn create() MaterialHandle {
    // 查找空闲槽位
    var i: u32 = 0;
    while (i < MAX_MATERIALS) : (i += 1) {
        const idx = (next_material_id + i) % MAX_MATERIALS;
        if (material_free_list[idx]) {
            material_free_list[idx] = false;
            next_material_id = (idx + 1) % MAX_MATERIALS;

            // 初始化默认材质
            material_pool[idx] = Material{
                .base_color = [_]f32{ 1.0, 1.0, 1.0, 1.0 },
                .metallic = 0.0,
                .roughness = 0.5,
                .emissive = [_]f32{ 0.0, 0.0, 0.0 },
                ._padding1 = 0.0,
                .base_color_texture = null,
                .normal_texture = null,
                .metallic_roughness_texture = null,
                .emissive_texture = null,
                .has_base_color_texture = false,
                .has_normal_texture = false,
                .has_metallic_roughness_texture = false,
                .has_emissive_texture = false,
                ._padding2 = [_]u8{0} ** 4,
            };

            return idx;
        }
    }

    // 池已满，返回无效句柄
    @panic("Material pool exhausted");
}

/// 销毁材质
pub fn destroy(handle: MaterialHandle) void {
    if (handle >= MAX_MATERIALS) return;
    material_free_list[handle] = true;
}

/// 获取材质指针
pub fn get(handle: MaterialHandle) ?*Material {
    if (handle >= MAX_MATERIALS) return null;
    if (material_free_list[handle]) return null;
    return &material_pool[handle];
}

/// 获取只读材质指针
pub fn getConst(handle: MaterialHandle) ?*const Material {
    if (handle >= MAX_MATERIALS) return null;
    if (material_free_list[handle]) return null;
    return &material_pool[handle];
}

/// 设置基础颜色
pub fn setBaseColor(handle: MaterialHandle, r: f32, g: f32, b: f32, a: f32) void {
    if (get(handle)) |mat| {
        mat.base_color = [_]f32{ r, g, b, a };
    }
}

/// 设置金属度
pub fn setMetallic(handle: MaterialHandle, metallic: f32) void {
    if (get(handle)) |mat| {
        mat.metallic = std.math.clamp(metallic, 0.0, 1.0);
    }
}

/// 设置粗糙度
pub fn setRoughness(handle: MaterialHandle, roughness: f32) void {
    if (get(handle)) |mat| {
        mat.roughness = std.math.clamp(roughness, 0.0, 1.0);
    }
}

/// 设置自发光颜色
pub fn setEmissive(handle: MaterialHandle, r: f32, g: f32, b: f32) void {
    if (get(handle)) |mat| {
        mat.emissive = [_]f32{ r, g, b };
    }
}

/// 设置基础颜色纹理
pub fn setBaseColorTexture(handle: MaterialHandle, texture: ?*anyopaque) void {
    if (get(handle)) |mat| {
        mat.base_color_texture = texture;
        mat.has_base_color_texture = texture != null;
    }
}

/// 设置法线纹理
pub fn setNormalTexture(handle: MaterialHandle, texture: ?*anyopaque) void {
    if (get(handle)) |mat| {
        mat.normal_texture = texture;
        mat.has_normal_texture = texture != null;
    }
}

/// 设置金属度/粗糙度纹理
pub fn setMetallicRoughnessTexture(handle: MaterialHandle, texture: ?*anyopaque) void {
    if (get(handle)) |mat| {
        mat.metallic_roughness_texture = texture;
        mat.has_metallic_roughness_texture = texture != null;
    }
}

/// 设置自发光纹理
pub fn setEmissiveTexture(handle: MaterialHandle, texture: ?*anyopaque) void {
    if (get(handle)) |mat| {
        mat.emissive_texture = texture;
        mat.has_emissive_texture = texture != null;
    }
}

// ============================================================================
// 测试
// ============================================================================

test "material creation and destruction" {
    const testing = std.testing;

    const handle1 = create();
    try testing.expect(handle1 < MAX_MATERIALS);

    const handle2 = create();
    try testing.expect(handle2 < MAX_MATERIALS);
    try testing.expect(handle1 != handle2);

    destroy(handle1);
    destroy(handle2);
}

test "material default values" {
    const testing = std.testing;

    const handle = create();
    defer destroy(handle);

    const mat = get(handle).?;
    try testing.expectEqual([_]f32{ 1.0, 1.0, 1.0, 1.0 }, mat.base_color);
    try testing.expectEqual(@as(f32, 0.0), mat.metallic);
    try testing.expectEqual(@as(f32, 0.5), mat.roughness);
    try testing.expectEqual([_]f32{ 0.0, 0.0, 0.0 }, mat.emissive);
    try testing.expect(!mat.has_base_color_texture);
    try testing.expect(!mat.has_normal_texture);
}

test "material setters" {
    const testing = std.testing;

    const handle = create();
    defer destroy(handle);

    setBaseColor(handle, 1.0, 0.0, 0.0, 1.0);
    setMetallic(handle, 0.8);
    setRoughness(handle, 0.2);
    setEmissive(handle, 0.5, 0.5, 0.5);

    const mat = get(handle).?;
    try testing.expectEqual([_]f32{ 1.0, 0.0, 0.0, 1.0 }, mat.base_color);
    try testing.expectEqual(@as(f32, 0.8), mat.metallic);
    try testing.expectEqual(@as(f32, 0.2), mat.roughness);
    try testing.expectEqual([_]f32{ 0.5, 0.5, 0.5 }, mat.emissive);
}

test "metallic clamping" {
    const testing = std.testing;

    const handle = create();
    defer destroy(handle);

    setMetallic(handle, 1.5);
    try testing.expectEqual(@as(f32, 1.0), get(handle).?.metallic);

    setMetallic(handle, -0.5);
    try testing.expectEqual(@as(f32, 0.0), get(handle).?.metallic);
}

test "roughness clamping" {
    const testing = std.testing;

    const handle = create();
    defer destroy(handle);

    setRoughness(handle, 1.5);
    try testing.expectEqual(@as(f32, 1.0), get(handle).?.roughness);

    setRoughness(handle, -0.5);
    try testing.expectEqual(@as(f32, 0.0), get(handle).?.roughness);
}

// ============================================================================
// FFI Export Layer
// ============================================================================

export fn pbr_material_create() MaterialHandle {
    return create();
}

export fn pbr_material_destroy(handle: MaterialHandle) void {
    destroy(handle);
}

export fn pbr_material_set_base_color(handle: MaterialHandle, r: f32, g: f32, b: f32, a: f32) void {
    setBaseColor(handle, r, g, b, a);
}

export fn pbr_material_set_metallic(handle: MaterialHandle, metallic: f32) void {
    setMetallic(handle, metallic);
}

export fn pbr_material_set_roughness(handle: MaterialHandle, roughness: f32) void {
    setRoughness(handle, roughness);
}

export fn pbr_material_set_emissive(handle: MaterialHandle, r: f32, g: f32, b: f32) void {
    setEmissive(handle, r, g, b);
}

export fn pbr_material_get_base_color(handle: MaterialHandle, out: *[4]f32) void {
    const mat = getConst(handle) orelse return;
    out.* = mat.base_color;
}

export fn pbr_material_get_metallic(handle: MaterialHandle) f32 {
    const mat = getConst(handle) orelse return 0.0;
    return mat.metallic;
}

export fn pbr_material_get_roughness(handle: MaterialHandle) f32 {
    const mat = getConst(handle) orelse return 0.5;
    return mat.roughness;
}

export fn pbr_material_get_emissive(handle: MaterialHandle, out: *[3]f32) void {
    const mat = getConst(handle) orelse return;
    out.* = mat.emissive;
}
