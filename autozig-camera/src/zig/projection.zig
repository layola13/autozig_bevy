//! Projection matrix calculations for Camera2d and Camera3d
//! Implements perspective and orthographic projections for WebGPU

const std = @import("std");
const math = std.math;

/// Perspective projection matrix (右手坐标系, NDC: Z [0, 1] for WebGPU)
/// fov_y: Field of view in radians (vertical)
/// aspect: Aspect ratio (width / height)
/// z_near: Near clipping plane
/// z_far: Far clipping plane
/// out: Output 4x4 matrix in column-major order
export fn projection_perspective_rh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32, out: [*]f32) void {
    const tan_half_fov = @tan(fov_y * 0.5);
    const f = 1.0 / tan_half_fov;

    // Initialize to zero
    var i: usize = 0;
    while (i < 16) : (i += 1) {
        out[i] = 0.0;
    }

    // WebGPU uses column-major order and Z [0, 1] NDC
    out[0] = f / aspect; // m00
    out[5] = f; // m11
    out[10] = z_far / (z_near - z_far); // m22
    out[11] = -1.0; // m23
    out[14] = -(z_far * z_near) / (z_far - z_near); // m32
}

/// Perspective projection matrix (左手坐标系, NDC: Z [0, 1] for WebGPU)
export fn projection_perspective_lh(fov_y: f32, aspect: f32, z_near: f32, z_far: f32, out: [*]f32) void {
    const tan_half_fov = @tan(fov_y * 0.5);
    const f = 1.0 / tan_half_fov;

    var i: usize = 0;
    while (i < 16) : (i += 1) {
        out[i] = 0.0;
    }

    out[0] = f / aspect;
    out[5] = f;
    out[10] = z_far / (z_far - z_near);
    out[11] = 1.0;
    out[14] = -(z_far * z_near) / (z_far - z_near);
}

/// Infinite perspective projection with reverse-Z (最佳深度精度)
/// 适用于WebGPU的reverse-Z深度缓冲优化
export fn projection_perspective_infinite_reverse_z(fov_y: f32, aspect: f32, z_near: f32, out: [*]f32) void {
    const tan_half_fov = @tan(fov_y * 0.5);
    const f = 1.0 / tan_half_fov;

    var i: usize = 0;
    while (i < 16) : (i += 1) {
        out[i] = 0.0;
    }

    out[0] = f / aspect;
    out[5] = f;
    out[10] = 0.0; // Reverse-Z: far plane at 0
    out[11] = -1.0;
    out[14] = z_near; // Near plane mapping
}

/// Orthographic projection matrix (右手坐标系, for Camera2d)
/// left, right, bottom, top: View frustum bounds
/// z_near, z_far: Depth range
/// out: Output 4x4 matrix in column-major order
export fn projection_orthographic_rh(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32, out: [*]f32) void {
    const width = right - left;
    const height = top - bottom;
    const depth = z_far - z_near;

    // Initialize to identity
    var i: usize = 0;
    while (i < 16) : (i += 1) {
        out[i] = if (i % 5 == 0) 1.0 else 0.0;
    }

    // Column-major order, WebGPU NDC Z [0, 1]
    out[0] = 2.0 / width;
    out[5] = 2.0 / height;
    out[10] = -1.0 / depth;
    out[12] = -(right + left) / width;
    out[13] = -(top + bottom) / height;
    out[14] = -z_near / depth;
}

/// Orthographic projection matrix (左手坐标系)
export fn projection_orthographic_lh(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32, out: [*]f32) void {
    const width = right - left;
    const height = top - bottom;
    const depth = z_far - z_near;

    var i: usize = 0;
    while (i < 16) : (i += 1) {
        out[i] = if (i % 5 == 0) 1.0 else 0.0;
    }

    out[0] = 2.0 / width;
    out[5] = 2.0 / height;
    out[10] = 1.0 / depth;
    out[12] = -(right + left) / width;
    out[13] = -(top + bottom) / height;
    out[14] = -z_near / depth;
}

/// 从窗口尺寸创建正交投影（2D相机常用）
/// window_width, window_height: 窗口像素尺寸
/// out: Output 4x4 matrix
export fn projection_orthographic_2d(window_width: f32, window_height: f32, out: [*]f32) void {
    const half_width = window_width * 0.5;
    const half_height = window_height * 0.5;
    projection_orthographic_rh(-half_width, half_width, -half_height, half_height, -1.0, 1.0, out);
}

/// 缩放正交投影（支持Camera2d的scale参数）
export fn projection_orthographic_scaled(left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32, scale: f32, out: [*]f32) void {
    const scaled_left = left * scale;
    const scaled_right = right * scale;
    const scaled_bottom = bottom * scale;
    const scaled_top = top * scale;
    projection_orthographic_rh(scaled_left, scaled_right, scaled_bottom, scaled_top, z_near, z_far, out);
}

/// 从FOV计算投影矩阵（支持动态FOV调整）
export fn projection_from_fov(fov_degrees: f32, aspect: f32, z_near: f32, z_far: f32, out: [*]f32) void {
    const fov_radians = fov_degrees * (math.pi / 180.0);
    projection_perspective_rh(fov_radians, aspect, z_near, z_far, out);
}

/// 提取投影矩阵的FOV（用于调试和序列化）
export fn projection_extract_fov(matrix: [*]const f32) f32 {
    // 从投影矩阵提取FOV
    // m11 = 1 / tan(fov_y / 2)
    const m11 = matrix[5];
    if (@abs(m11) < 0.0001) return 0.0;

    const tan_half_fov = 1.0 / m11;
    const half_fov = math.atan(tan_half_fov);
    return half_fov * 2.0;
}

/// 提取投影矩阵的aspect ratio
export fn projection_extract_aspect(matrix: [*]const f32) f32 {
    // m00 = f / aspect, m11 = f
    // aspect = m11 / m00
    const m00 = matrix[0];
    const m11 = matrix[5];
    if (@abs(m00) < 0.0001) return 1.0;

    return m11 / m00;
}

/// 提取投影矩阵的near plane
export fn projection_extract_near(matrix: [*]const f32) f32 {
    // For perspective: z_near = -m32 / m22
    const m22 = matrix[10];
    const m32 = matrix[14];
    if (@abs(m22) < 0.0001) return 0.1;

    return -m32 / m22;
}

/// 提取投影矩阵的far plane
export fn projection_extract_far(matrix: [*]const f32) f32 {
    // For perspective: z_far = m32 / (m22 + 1)
    const m22 = matrix[10];
    const m32 = matrix[14];
    const denominator = m22 + 1.0;
    if (@abs(denominator) < 0.0001) return 1000.0;

    return m32 / denominator;
}

/// 检查矩阵是否为透视投影
export fn projection_is_perspective(matrix: [*]const f32) bool {
    // 透视投影特征: m23 = -1 或 1, m33 = 0
    const m23 = matrix[11];
    const m33 = matrix[15];
    return @abs(m33) < 0.0001 and @abs(@abs(m23) - 1.0) < 0.0001;
}

/// 检查矩阵是否为正交投影
export fn projection_is_orthographic(matrix: [*]const f32) bool {
    // 正交投影特征: m23 = 0, m33 = 1
    const m23 = matrix[11];
    const m33 = matrix[15];
    return @abs(m23) < 0.0001 and @abs(m33 - 1.0) < 0.0001;
}
