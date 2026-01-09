//! View matrix calculations for camera transformations
//! Converts world-space coordinates to camera-space coordinates

const std = @import("std");
const math = std.math;

/// 从位置和旋转四元数构建视图矩阵 (右手坐标系)
/// position: Camera world position [x, y, z]
/// rotation: Camera rotation as quaternion [x, y, z, w]
/// out: Output 4x4 view matrix in column-major order
export fn view_matrix_from_transform(position: [*]const f32, rotation: [*]const f32, out: [*]f32) void {
    // 四元数转旋转矩阵
    const x = rotation[0];
    const y = rotation[1];
    const z = rotation[2];
    const w = rotation[3];

    const x2 = x + x;
    const y2 = y + y;
    const z2 = z + z;
    const xx = x * x2;
    const xy = x * y2;
    const xz = x * z2;
    const yy = y * y2;
    const yz = y * z2;
    const zz = z * z2;
    const wx = w * x2;
    const wy = w * y2;
    const wz = w * z2;

    // 构建旋转矩阵（列主序）
    const r00 = 1.0 - (yy + zz);
    const r01 = xy + wz;
    const r02 = xz - wy;

    const r10 = xy - wz;
    const r11 = 1.0 - (xx + zz);
    const r12 = yz + wx;

    const r20 = xz + wy;
    const r21 = yz - wx;
    const r22 = 1.0 - (xx + yy);

    // 视图矩阵 = 旋转矩阵的转置 * 平移矩阵的逆
    // 相当于先平移-position，再应用旋转的逆（转置）
    const px = position[0];
    const py = position[1];
    const pz = position[2];

    // 列主序存储
    out[0] = r00;
    out[1] = r10;
    out[2] = r20;
    out[3] = 0.0;

    out[4] = r01;
    out[5] = r11;
    out[6] = r21;
    out[7] = 0.0;

    out[8] = r02;
    out[9] = r12;
    out[10] = r22;
    out[11] = 0.0;

    // 平移部分 = -R^T * position
    out[12] = -(r00 * px + r01 * py + r02 * pz);
    out[13] = -(r10 * px + r11 * py + r12 * pz);
    out[14] = -(r20 * px + r21 * py + r22 * pz);
    out[15] = 1.0;
}

/// Look-at 视图矩阵构建 (右手坐标系)
/// eye: Camera position
/// target: Look-at target position
/// up: Up vector (usually [0, 1, 0])
/// out: Output 4x4 view matrix
export fn view_look_at_rh(eye: [*]const f32, target: [*]const f32, up: [*]const f32, out: [*]f32) void {
    // 计算相机坐标系的三个轴
    // Forward (Z轴) = normalize(eye - target)
    var fx = eye[0] - target[0];
    var fy = eye[1] - target[1];
    var fz = eye[2] - target[2];
    const f_len = @sqrt(fx * fx + fy * fy + fz * fz);
    if (f_len > 0.0001) {
        fx /= f_len;
        fy /= f_len;
        fz /= f_len;
    }

    // Right (X轴) = normalize(up × forward)
    var rx = up[1] * fz - up[2] * fy;
    var ry = up[2] * fx - up[0] * fz;
    var rz = up[0] * fy - up[1] * fx;
    const r_len = @sqrt(rx * rx + ry * ry + rz * rz);
    if (r_len > 0.0001) {
        rx /= r_len;
        ry /= r_len;
        rz /= r_len;
    }

    // Up (Y轴) = forward × right
    const ux = fy * rz - fz * ry;
    const uy = fz * rx - fx * rz;
    const uz = fx * ry - fy * rx;

    // 构建视图矩阵（列主序）
    out[0] = rx;
    out[1] = ux;
    out[2] = fx;
    out[3] = 0.0;

    out[4] = ry;
    out[5] = uy;
    out[6] = fy;
    out[7] = 0.0;

    out[8] = rz;
    out[9] = uz;
    out[10] = fz;
    out[11] = 0.0;

    out[12] = -(rx * eye[0] + ry * eye[1] + rz * eye[2]);
    out[13] = -(ux * eye[0] + uy * eye[1] + uz * eye[2]);
    out[14] = -(fx * eye[0] + fy * eye[1] + fz * eye[2]);
    out[15] = 1.0;
}

/// Look-at 视图矩阵构建 (左手坐标系)
export fn view_look_at_lh(eye: [*]const f32, target: [*]const f32, up: [*]const f32, out: [*]f32) void {
    // Forward (Z轴) = normalize(target - eye)
    var fx = target[0] - eye[0];
    var fy = target[1] - eye[1];
    var fz = target[2] - eye[2];
    const f_len = @sqrt(fx * fx + fy * fy + fz * fz);
    if (f_len > 0.0001) {
        fx /= f_len;
        fy /= f_len;
        fz /= f_len;
    }

    // Right (X轴) = normalize(up × forward)
    var rx = up[1] * fz - up[2] * fy;
    var ry = up[2] * fx - up[0] * fz;
    var rz = up[0] * fy - up[1] * fx;
    const r_len = @sqrt(rx * rx + ry * ry + rz * rz);
    if (r_len > 0.0001) {
        rx /= r_len;
        ry /= r_len;
        rz /= r_len;
    }

    // Up (Y轴) = forward × right
    const ux = fy * rz - fz * ry;
    const uy = fz * rx - fx * rz;
    const uz = fx * ry - fy * rx;

    out[0] = rx;
    out[1] = ux;
    out[2] = fx;
    out[3] = 0.0;

    out[4] = ry;
    out[5] = uy;
    out[6] = fy;
    out[7] = 0.0;

    out[8] = rz;
    out[9] = uz;
    out[10] = fz;
    out[11] = 0.0;

    out[12] = -(rx * eye[0] + ry * eye[1] + rz * eye[2]);
    out[13] = -(ux * eye[0] + uy * eye[1] + uz * eye[2]);
    out[14] = -(fx * eye[0] + fy * eye[1] + fz * eye[2]);
    out[15] = 1.0;
}

/// 从视图矩阵提取相机位置
export fn view_extract_position(view_matrix: [*]const f32, out_position: [*]f32) void {
    // 视图矩阵的逆矩阵的平移部分就是相机位置
    // 对于正交矩阵（旋转），逆 = 转置
    // position = -R^T * translation_part

    const r00 = view_matrix[0];
    const r10 = view_matrix[1];
    const r20 = view_matrix[2];
    const r01 = view_matrix[4];
    const r11 = view_matrix[5];
    const r21 = view_matrix[6];
    const r02 = view_matrix[8];
    const r12 = view_matrix[9];
    const r22 = view_matrix[10];
    const tx = view_matrix[12];
    const ty = view_matrix[13];
    const tz = view_matrix[14];

    out_position[0] = -(r00 * tx + r01 * ty + r02 * tz);
    out_position[1] = -(r10 * tx + r11 * ty + r12 * tz);
    out_position[2] = -(r20 * tx + r21 * ty + r22 * tz);
}

/// 从视图矩阵提取相机方向向量（forward）
export fn view_extract_forward(view_matrix: [*]const f32, out_forward: [*]f32) void {
    // Forward = -Z轴（第三列的负向）
    out_forward[0] = -view_matrix[8];
    out_forward[1] = -view_matrix[9];
    out_forward[2] = -view_matrix[10];
}

/// 从视图矩阵提取相机右向量（right）
export fn view_extract_right(view_matrix: [*]const f32, out_right: [*]f32) void {
    // Right = X轴（第一列）
    out_right[0] = view_matrix[0];
    out_right[1] = view_matrix[1];
    out_right[2] = view_matrix[2];
}

/// 从视图矩阵提取相机上向量（up）
export fn view_extract_up(view_matrix: [*]const f32, out_up: [*]f32) void {
    // Up = Y轴（第二列）
    out_up[0] = view_matrix[4];
    out_up[1] = view_matrix[5];
    out_up[2] = view_matrix[6];
}

/// 2D相机视图矩阵（只有平移和缩放）
/// position: 2D position [x, y]
/// scale: Uniform scale factor
/// out: Output 4x4 view matrix
export fn view_matrix_2d(position: [*]const f32, scale: f32, out: [*]f32) void {
    // 初始化为单位矩阵
    var i: usize = 0;
    while (i < 16) : (i += 1) {
        out[i] = if (i % 5 == 0) 1.0 else 0.0;
    }

    // 应用缩放和平移
    const inv_scale = 1.0 / scale;
    out[0] = inv_scale;
    out[5] = inv_scale;
    out[12] = -position[0] * inv_scale;
    out[13] = -position[1] * inv_scale;
}

/// 2D相机视图矩阵（支持旋转）
/// position: 2D position [x, y]
/// rotation: Rotation angle in radians
/// scale: Uniform scale factor
/// out: Output 4x4 view matrix
export fn view_matrix_2d_rotated(position: [*]const f32, rotation: f32, scale: f32, out: [*]f32) void {
    const cos_r = @cos(rotation);
    const sin_r = @sin(rotation);
    const inv_scale = 1.0 / scale;

    // 旋转 + 缩放
    out[0] = cos_r * inv_scale;
    out[1] = sin_r * inv_scale;
    out[2] = 0.0;
    out[3] = 0.0;

    out[4] = -sin_r * inv_scale;
    out[5] = cos_r * inv_scale;
    out[6] = 0.0;
    out[7] = 0.0;

    out[8] = 0.0;
    out[9] = 0.0;
    out[10] = 1.0;
    out[11] = 0.0;

    // 平移（需要先旋转再平移）
    const px = position[0];
    const py = position[1];
    out[12] = -(cos_r * px - sin_r * py) * inv_scale;
    out[13] = -(sin_r * px + cos_r * py) * inv_scale;
    out[14] = 0.0;
    out[15] = 1.0;
}

/// 视图矩阵求逆（用于将相机空间坐标转回世界空间）
export fn view_matrix_inverse(view_matrix: [*]const f32, out: [*]f32) void {
    // 对于视图矩阵（正交旋转 + 平移），求逆比较简单
    // 旋转部分求逆 = 转置
    // 平移部分求逆需要特殊处理

    const r00 = view_matrix[0];
    const r10 = view_matrix[1];
    const r20 = view_matrix[2];
    const r01 = view_matrix[4];
    const r11 = view_matrix[5];
    const r21 = view_matrix[6];
    const r02 = view_matrix[8];
    const r12 = view_matrix[9];
    const r22 = view_matrix[10];
    const tx = view_matrix[12];
    const ty = view_matrix[13];
    const tz = view_matrix[14];

    // 旋转转置
    out[0] = r00;
    out[1] = r01;
    out[2] = r02;
    out[3] = 0.0;

    out[4] = r10;
    out[5] = r11;
    out[6] = r12;
    out[7] = 0.0;

    out[8] = r20;
    out[9] = r21;
    out[10] = r22;
    out[11] = 0.0;

    // 平移逆变换
    out[12] = -(r00 * tx + r01 * ty + r02 * tz);
    out[13] = -(r10 * tx + r11 * ty + r12 * tz);
    out[14] = -(r20 * tx + r21 * ty + r22 * tz);
    out[15] = 1.0;
}
