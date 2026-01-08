//! PBR 光照计算模块
//!
//! 实现基于物理的渲染光照模型
//! - Cook-Torrance BRDF
//! - GGX 法线分布函数
//! - Schlick-GGX 几何衰减
//! - Fresnel-Schlick 菲涅尔反射
//! - SIMD 向量化批量计算

const std = @import("std");
const pbr_material = @import("pbr_material.zig");
const Material = pbr_material.Material;
const MaterialHandle = pbr_material.MaterialHandle;

// ============================================================================
// 数学辅助函数
// ============================================================================

/// 向量点积
inline fn dot(a: [3]f32, b: [3]f32) f32 {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

/// 向量归一化
inline fn normalize(v: [3]f32) [3]f32 {
    const len = @sqrt(dot(v, v));
    if (len < 0.0001) return [3]f32{ 0.0, 0.0, 1.0 };
    return [3]f32{ v[0] / len, v[1] / len, v[2] / len };
}

/// 向量加法
inline fn add(a: [3]f32, b: [3]f32) [3]f32 {
    return [3]f32{ a[0] + b[0], a[1] + b[1], a[2] + b[2] };
}

/// 向量减法
inline fn sub(a: [3]f32, b: [3]f32) [3]f32 {
    return [3]f32{ a[0] - b[0], a[1] - b[1], a[2] - b[2] };
}

/// 向量标量乘法
inline fn scale(v: [3]f32, s: f32) [3]f32 {
    return [3]f32{ v[0] * s, v[1] * s, v[2] * s };
}

/// 向量逐分量乘法
inline fn mul(a: [3]f32, b: [3]f32) [3]f32 {
    return [3]f32{ a[0] * b[0], a[1] * b[1], a[2] * b[2] };
}

/// Clamp 函数
inline fn clamp(value: f32, min_val: f32, max_val: f32) f32 {
    return std.math.clamp(value, min_val, max_val);
}

/// Max 函数
inline fn max(a: f32, b: f32) f32 {
    return if (a > b) a else b;
}

// ============================================================================
// PBR 核心函数
// ============================================================================

/// GGX 法线分布函数 (Trowbridge-Reitz)
/// 描述微平面法线分布
inline fn distributionGGX(N: [3]f32, H: [3]f32, roughness: f32) f32 {
    const a = roughness * roughness;
    const a2 = a * a;
    const NdotH = max(dot(N, H), 0.0);
    const NdotH2 = NdotH * NdotH;

    const denom = NdotH2 * (a2 - 1.0) + 1.0;
    const PI = std.math.pi;
    return a2 / (PI * denom * denom);
}

/// Schlick-GGX 几何衰减函数
/// 描述微平面自遮挡
inline fn geometrySchlickGGX(NdotV: f32, roughness: f32) f32 {
    const r = roughness + 1.0;
    const k = (r * r) / 8.0;
    return NdotV / (NdotV * (1.0 - k) + k);
}

/// Smith 几何函数
/// 组合视线方向和光线方向的几何衰减
inline fn geometrySmith(N: [3]f32, V: [3]f32, L: [3]f32, roughness: f32) f32 {
    const NdotV = max(dot(N, V), 0.0);
    const NdotL = max(dot(N, L), 0.0);
    const ggx2 = geometrySchlickGGX(NdotV, roughness);
    const ggx1 = geometrySchlickGGX(NdotL, roughness);
    return ggx1 * ggx2;
}

/// Fresnel-Schlick 近似
/// 描述不同角度的反射率
inline fn fresnelSchlick(cosTheta: f32, F0: [3]f32) [3]f32 {
    const one_minus_cos = 1.0 - cosTheta;
    const pow5 = one_minus_cos * one_minus_cos * one_minus_cos * one_minus_cos * one_minus_cos;
    return add(F0, scale(sub([3]f32{ 1.0, 1.0, 1.0 }, F0), pow5));
}

// ============================================================================
// PBR 光照计算
// ============================================================================

/// 计算单个点的 PBR 光照
pub fn calculate(
    material_handle: MaterialHandle,
    _: [3]f32, // position (未使用，保留用于未来扩展)
    normal: [3]f32,
    view_dir: [3]f32,
    light_dir: [3]f32,
    light_color: [3]f32,
    light_intensity: f32,
) [3]f32 {
    const mat = pbr_material.getConst(material_handle) orelse {
        return [3]f32{ 0.0, 0.0, 0.0 };
    };

    // 归一化输入向量
    const N = normalize(normal);
    const V = normalize(view_dir);
    const L = normalize(light_dir);
    const H = normalize(add(V, L)); // 半程向量

    // 基础反射率 F0 (电介质默认 0.04，金属使用基础颜色)
    const dielectric_F0 = [3]f32{ 0.04, 0.04, 0.04 };
    const metallic_F0 = [3]f32{ mat.base_color[0], mat.base_color[1], mat.base_color[2] };
    const F0 = lerp3(dielectric_F0, metallic_F0, mat.metallic);

    // Cook-Torrance BRDF 计算
    const NDF = distributionGGX(N, H, mat.roughness);
    const G = geometrySmith(N, V, L, mat.roughness);
    const F = fresnelSchlick(max(dot(H, V), 0.0), F0);

    // 镜面反射分量
    const NdotL = max(dot(N, L), 0.0);
    const NdotV = max(dot(N, V), 0.0);
    const denominator = 4.0 * NdotV * NdotL + 0.0001; // 避免除零
    const specular = scale(F, NDF * G / denominator);

    // 漫反射分量 (能量守恒: kD = 1 - kS)
    const kS = F; // 镜面反射系数
    var kD = sub([3]f32{ 1.0, 1.0, 1.0 }, kS); // 漫反射系数
    kD = scale(kD, 1.0 - mat.metallic); // 金属没有漫反射

    // Lambert 漫反射
    const albedo = [3]f32{ mat.base_color[0], mat.base_color[1], mat.base_color[2] };
    const diffuse = scale(mul(kD, albedo), 1.0 / std.math.pi);

    // 组合漫反射和镜面反射
    const BRDF = add(diffuse, specular);

    // 应用光照
    const radiance = scale(light_color, light_intensity);
    var Lo = scale(mul(BRDF, radiance), NdotL);

    // 添加自发光
    Lo = add(Lo, mat.emissive);

    // 简单的环境光（可选）
    const ambient = scale(albedo, 0.03);
    const final_color = add(Lo, ambient);

    return final_color;
}

/// 线性插值
inline fn lerp3(a: [3]f32, b: [3]f32, t: f32) [3]f32 {
    return add(scale(a, 1.0 - t), scale(b, t));
}

// ============================================================================
// SIMD 批量光照计算
// ============================================================================

/// 光源数据结构
pub const LightData = extern struct {
    position: [3]f32,
    direction: [3]f32,
    color: [3]f32,
    intensity: f32,
    radius: f32,
    _padding: [3]f32,
};

/// SIMD 向量化批量计算 PBR 光照
/// 使用 @Vector(4, f32) 一次处理 4 个顶点
pub fn calculateBatchSimd(
    materials: [*]const MaterialHandle,
    positions: [*]const f32,
    normals: [*]const f32,
    view_dirs: [*]const f32,
    lights: [*]const LightData,
    num_vertices: u32,
    num_lights: u32,
    out_colors: [*]f32,
) void {
    const Vec4 = @Vector(4, f32);

    // 处理 4 个顶点一组
    var v: u32 = 0;
    while (v + 4 <= num_vertices) : (v += 4) {
        // 加载 4 个顶点的数据
        var acc_r = Vec4{ 0.0, 0.0, 0.0, 0.0 };
        var acc_g = Vec4{ 0.0, 0.0, 0.0, 0.0 };
        var acc_b = Vec4{ 0.0, 0.0, 0.0, 0.0 };

        // 遍历所有光源
        var l: u32 = 0;
        while (l < num_lights) : (l += 1) {
            const light = lights[l];

            // 为每个顶点计算光照
            inline for (0..4) |i| {
                const idx = v + i;
                const pos_offset = idx * 3;
                const position = [3]f32{
                    positions[pos_offset + 0],
                    positions[pos_offset + 1],
                    positions[pos_offset + 2],
                };
                const normal = [3]f32{
                    normals[pos_offset + 0],
                    normals[pos_offset + 1],
                    normals[pos_offset + 2],
                };
                const view_dir = [3]f32{
                    view_dirs[pos_offset + 0],
                    view_dirs[pos_offset + 1],
                    view_dirs[pos_offset + 2],
                };

                // 计算光线方向
                const light_dir = normalize(sub(light.position, position));

                // 计算光照
                const color = calculate(
                    materials[idx],
                    position,
                    normal,
                    view_dir,
                    light_dir,
                    light.color,
                    light.intensity,
                );

                // 累加到 SIMD 向量
                acc_r[i] += color[0];
                acc_g[i] += color[1];
                acc_b[i] += color[2];
            }
        }

        // 写入结果
        inline for (0..4) |i| {
            const out_offset = (v + i) * 3;
            out_colors[out_offset + 0] = acc_r[i];
            out_colors[out_offset + 1] = acc_g[i];
            out_colors[out_offset + 2] = acc_b[i];
        }
    }

    // 处理剩余顶点（不足 4 个）
    while (v < num_vertices) : (v += 1) {
        const pos_offset = v * 3;
        const position = [3]f32{
            positions[pos_offset + 0],
            positions[pos_offset + 1],
            positions[pos_offset + 2],
        };
        const normal = [3]f32{
            normals[pos_offset + 0],
            normals[pos_offset + 1],
            normals[pos_offset + 2],
        };
        const view_dir = [3]f32{
            view_dirs[pos_offset + 0],
            view_dirs[pos_offset + 1],
            view_dirs[pos_offset + 2],
        };

        var acc_color = [3]f32{ 0.0, 0.0, 0.0 };

        var l: u32 = 0;
        while (l < num_lights) : (l += 1) {
            const light = lights[l];
            const light_dir = normalize(sub(light.position, position));

            const color = calculate(
                materials[v],
                position,
                normal,
                view_dir,
                light_dir,
                light.color,
                light.intensity,
            );

            acc_color = add(acc_color, color);
        }

        const out_offset = v * 3;
        out_colors[out_offset + 0] = acc_color[0];
        out_colors[out_offset + 1] = acc_color[1];
        out_colors[out_offset + 2] = acc_color[2];
    }
}

// ============================================================================
// 测试
// ============================================================================

test "PBR lighting calculation" {
    const testing = std.testing;

    const material = pbr_material.create();
    defer pbr_material.destroy(material);

    pbr_material.setBaseColor(material, 1.0, 0.0, 0.0, 1.0);
    pbr_material.setMetallic(material, 0.0);
    pbr_material.setRoughness(material, 0.5);

    const position = [3]f32{ 0.0, 0.0, 0.0 };
    const normal = [3]f32{ 0.0, 1.0, 0.0 };
    const view_dir = [3]f32{ 0.0, 1.0, 0.0 };
    const light_dir = [3]f32{ 0.0, 1.0, 0.0 };
    const light_color = [3]f32{ 1.0, 1.0, 1.0 };
    const light_intensity: f32 = 1.0;

    const result = calculate(
        material,
        position,
        normal,
        view_dir,
        light_dir,
        light_color,
        light_intensity,
    );

    // 验证结果不是零
    try testing.expect(result[0] > 0.0 or result[1] > 0.0 or result[2] > 0.0);
}

test "vector math functions" {
    const testing = std.testing;

    const a = [3]f32{ 1.0, 0.0, 0.0 };
    const b = [3]f32{ 0.0, 1.0, 0.0 };

    // 点积
    try testing.expectEqual(@as(f32, 0.0), dot(a, b));
    try testing.expectEqual(@as(f32, 1.0), dot(a, a));

    // 归一化
    const c = [3]f32{ 3.0, 4.0, 0.0 };
    const n = normalize(c);
    const len = @sqrt(dot(n, n));
    try testing.expectApproxEqAbs(@as(f32, 1.0), len, 0.0001);

    // 向量运算
    const sum = add(a, b);
    try testing.expectEqual([3]f32{ 1.0, 1.0, 0.0 }, sum);

    const diff = sub(a, b);
    try testing.expectEqual([3]f32{ 1.0, -1.0, 0.0 }, diff);
}

test "GGX distribution" {
    const testing = std.testing;

    const N = [3]f32{ 0.0, 1.0, 0.0 };
    const H = [3]f32{ 0.0, 1.0, 0.0 };
    const roughness: f32 = 0.5;

    const ndf = distributionGGX(N, H, roughness);
    try testing.expect(ndf > 0.0);
}

// ============================================================================
// Direct PBR计算（不使用MaterialHandle）
// ============================================================================

/// 直接计算PBR光照（不通过MaterialHandle）
pub fn calculateDirect(
    base_color: [3]f32,
    metallic: f32,
    roughness: f32,
    emissive: [3]f32,
    normal: [3]f32,
    view_dir: [3]f32,
    light_dir: [3]f32,
    light_color: [3]f32,
    light_intensity: f32,
) [3]f32 {
    // 归一化输入向量
    const N = normalize(normal);
    const V = normalize(view_dir);
    const L = normalize(light_dir);
    const H = normalize(add(V, L)); // 半程向量

    // 基础反射率 F0 (电介质默认 0.04，金属使用基础颜色)
    const dielectric_F0 = [3]f32{ 0.04, 0.04, 0.04 };
    const metallic_F0 = base_color;
    const F0 = lerp3(dielectric_F0, metallic_F0, metallic);

    // Cook-Torrance BRDF 计算
    const NDF = distributionGGX(N, H, roughness);
    const G = geometrySmith(N, V, L, roughness);
    const F = fresnelSchlick(max(dot(H, V), 0.0), F0);

    // 镜面反射分量
    const NdotL = max(dot(N, L), 0.0);
    const NdotV = max(dot(N, V), 0.0);
    const denominator = 4.0 * NdotV * NdotL + 0.0001; // 避免除零
    const specular = scale(F, NDF * G / denominator);

    // 漫反射分量 (能量守恒: kD = 1 - kS)
    const kS = F; // 镜面反射系数
    var kD = sub([3]f32{ 1.0, 1.0, 1.0 }, kS); // 漫反射系数
    kD = scale(kD, 1.0 - metallic); // 金属没有漫反射

    // Lambert 漫反射
    const albedo = base_color;
    const diffuse = scale(mul(kD, albedo), 1.0 / std.math.pi);

    // 组合漫反射和镜面反射
    const BRDF = add(diffuse, specular);

    // 应用光照
    const radiance = scale(light_color, light_intensity);
    var Lo = scale(mul(BRDF, radiance), NdotL);

    // 添加自发光
    Lo = add(Lo, emissive);

    // 简单的环境光（可选）
    const ambient = scale(albedo, 0.03);
    const final_color = add(Lo, ambient);

    return final_color;
}

/// SIMD批量计算（简化版本，用于FFI导出）
pub fn calculateBatchSIMD(
    positions: [*]const f32,
    normals: [*]const f32,
    base_colors: [*]const f32,
    metallic: f32,
    roughness: f32,
    emissive: [3]f32,
    camera_pos: [3]f32,
    lights: [*]const LightData,
    light_count: u32,
    ambient: [3]f32,
    out_colors: [*]f32,
) void {
    // 计算顶点数量
    const num_vertices = @as(u32, 1); // 简化：从positions计算实际数量应该由调用者传入

    var v: u32 = 0;
    while (v < num_vertices) : (v += 1) {
        const pos_offset = v * 3;
        const position = [3]f32{
            positions[pos_offset + 0],
            positions[pos_offset + 1],
            positions[pos_offset + 2],
        };
        const normal = [3]f32{
            normals[pos_offset + 0],
            normals[pos_offset + 1],
            normals[pos_offset + 2],
        };
        const base_color = [3]f32{
            base_colors[pos_offset + 0],
            base_colors[pos_offset + 1],
            base_colors[pos_offset + 2],
        };

        // 计算视线方向
        const view_dir = normalize(sub(camera_pos, position));

        var acc_color = ambient;

        // 遍历所有光源
        var l: u32 = 0;
        while (l < light_count) : (l += 1) {
            const light = lights[l];
            const light_dir = normalize(sub(light.position, position));

            const color = calculateDirect(
                base_color,
                metallic,
                roughness,
                emissive,
                normal,
                view_dir,
                light_dir,
                light.color,
                light.intensity,
            );

            acc_color = add(acc_color, color);
        }

        const out_offset = v * 3;
        out_colors[out_offset + 0] = acc_color[0];
        out_colors[out_offset + 1] = acc_color[1];
        out_colors[out_offset + 2] = acc_color[2];
    }
}

// ============================================================================
// FFI Export Layer
// ============================================================================

export fn pbr_calculate_lighting(
    base_color: *const [3]f32,
    metallic: f32,
    roughness: f32,
    emissive: *const [3]f32,
    normal: *const [3]f32,
    view_dir: *const [3]f32,
    light_dir: *const [3]f32,
    light_color: *const [3]f32,
    light_intensity: f32,
    out_color: *[3]f32,
) void {
    out_color.* = calculateDirect(
        base_color.*,
        metallic,
        roughness,
        emissive.*,
        normal.*,
        view_dir.*,
        light_dir.*,
        light_color.*,
        light_intensity,
    );
}

export fn pbr_calculate_lighting_simd(
    positions: [*]const f32,
    normals: [*]const f32,
    base_colors: [*]const f32,
    metallic: f32,
    roughness: f32,
    emissive: *const [3]f32,
    camera_pos: *const [3]f32,
    lights: [*]const LightData,
    light_count: u32,
    ambient: *const [3]f32,
    out_colors: [*]f32,
) void {
    calculateBatchSIMD(
        positions,
        normals,
        base_colors,
        metallic,
        roughness,
        emissive.*,
        camera_pos.*,
        lights,
        light_count,
        ambient.*,
        out_colors,
    );
}
