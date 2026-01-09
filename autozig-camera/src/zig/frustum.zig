//! Frustum culling for efficient rendering
//! Implements AABB and sphere frustum tests

const std = @import("std");
const math = std.math;

/// 平面表示: ax + by + cz + d = 0
/// [a, b, c] 是归一化法向量
/// d 是距离原点的距离
pub const Plane = extern struct {
    normal: [3]f32,
    distance: f32,
};

/// 视锥体由6个平面组成：左、右、下、上、近、远
pub const Frustum = extern struct {
    planes: [6]Plane,
};

/// 从view-projection矩阵提取视锥体平面
/// view_proj_matrix: Combined view * projection matrix (column-major)
/// out: Output frustum with 6 planes
export fn frustum_from_matrix(view_proj_matrix: [*]const f32, out: *Frustum) void {
    const m = view_proj_matrix;

    // 提取6个平面（Gribb-Hartmann方法）
    // 左平面: m03 + m00, m13 + m10, m23 + m20, m33 + m30
    out.planes[0].normal[0] = m[3] + m[0];
    out.planes[0].normal[1] = m[7] + m[4];
    out.planes[0].normal[2] = m[11] + m[8];
    out.planes[0].distance = m[15] + m[12];

    // 右平面: m03 - m00, m13 - m10, m23 - m20, m33 - m30
    out.planes[1].normal[0] = m[3] - m[0];
    out.planes[1].normal[1] = m[7] - m[4];
    out.planes[1].normal[2] = m[11] - m[8];
    out.planes[1].distance = m[15] - m[12];

    // 下平面: m03 + m01, m13 + m11, m23 + m21, m33 + m31
    out.planes[2].normal[0] = m[3] + m[1];
    out.planes[2].normal[1] = m[7] + m[5];
    out.planes[2].normal[2] = m[11] + m[9];
    out.planes[2].distance = m[15] + m[13];

    // 上平面: m03 - m01, m13 - m11, m23 - m21, m33 - m31
    out.planes[3].normal[0] = m[3] - m[1];
    out.planes[3].normal[1] = m[7] - m[5];
    out.planes[3].normal[2] = m[11] - m[9];
    out.planes[3].distance = m[15] - m[13];

    // 近平面: m03 + m02, m13 + m12, m23 + m22, m33 + m32
    out.planes[4].normal[0] = m[3] + m[2];
    out.planes[4].normal[1] = m[7] + m[6];
    out.planes[4].normal[2] = m[11] + m[10];
    out.planes[4].distance = m[15] + m[14];

    // 远平面: m03 - m02, m13 - m12, m23 - m22, m33 - m32
    out.planes[5].normal[0] = m[3] - m[2];
    out.planes[5].normal[1] = m[7] - m[6];
    out.planes[5].normal[2] = m[11] - m[10];
    out.planes[5].distance = m[15] - m[14];

    // 归一化所有平面
    var i: usize = 0;
    while (i < 6) : (i += 1) {
        const nx = out.planes[i].normal[0];
        const ny = out.planes[i].normal[1];
        const nz = out.planes[i].normal[2];
        const len = @sqrt(nx * nx + ny * ny + nz * nz);

        if (len > 0.0001) {
            const inv_len = 1.0 / len;
            out.planes[i].normal[0] *= inv_len;
            out.planes[i].normal[1] *= inv_len;
            out.planes[i].normal[2] *= inv_len;
            out.planes[i].distance *= inv_len;
        }
    }
}

/// 测试点是否在视锥体内
/// frustum: View frustum
/// point: Point to test [x, y, z]
/// returns: true if point is inside frustum
export fn frustum_test_point(frustum: *const Frustum, point: [*]const f32) bool {
    var i: usize = 0;
    while (i < 6) : (i += 1) {
        const plane = &frustum.planes[i];
        const distance = plane.normal[0] * point[0] +
            plane.normal[1] * point[1] +
            plane.normal[2] * point[2] +
            plane.distance;

        // 如果点在平面背面（负距离），则在视锥体外
        if (distance < 0.0) {
            return false;
        }
    }
    return true;
}

/// 测试AABB（轴对齐包围盒）是否与视锥体相交
/// frustum: View frustum
/// min: AABB minimum corner [x, y, z]
/// max: AABB maximum corner [x, y, z]
/// returns: true if AABB intersects or is inside frustum
export fn frustum_test_aabb(frustum: *const Frustum, min: [*]const f32, max: [*]const f32) bool {
    var i: usize = 0;
    while (i < 6) : (i += 1) {
        const plane = &frustum.planes[i];

        // 计算AABB的positive vertex（沿法向量方向最远的顶点）
        var px = min[0];
        var py = min[1];
        var pz = min[2];

        if (plane.normal[0] >= 0.0) px = max[0];
        if (plane.normal[1] >= 0.0) py = max[1];
        if (plane.normal[2] >= 0.0) pz = max[2];

        // 如果positive vertex在平面背面，则整个AABB在视锥体外
        const distance = plane.normal[0] * px +
            plane.normal[1] * py +
            plane.normal[2] * pz +
            plane.distance;

        if (distance < 0.0) {
            return false;
        }
    }
    return true;
}

/// 测试球体是否与视锥体相交
/// frustum: View frustum
/// center: Sphere center [x, y, z]
/// radius: Sphere radius
/// returns: true if sphere intersects or is inside frustum
export fn frustum_test_sphere(frustum: *const Frustum, center: [*]const f32, radius: f32) bool {
    var i: usize = 0;
    while (i < 6) : (i += 1) {
        const plane = &frustum.planes[i];

        // 计算球心到平面的距离
        const distance = plane.normal[0] * center[0] +
            plane.normal[1] * center[1] +
            plane.normal[2] * center[2] +
            plane.distance;

        // 如果球心到平面的距离 < -radius，则球体完全在平面背面
        if (distance < -radius) {
            return false;
        }
    }
    return true;
}

/// 测试OBB（有向包围盒）是否与视锥体相交（更精确但更慢）
/// frustum: View frustum
/// center: OBB center [x, y, z]
/// extents: OBB half-extents [x, y, z]
/// rotation: OBB rotation quaternion [x, y, z, w]
/// returns: true if OBB intersects or is inside frustum
export fn frustum_test_obb(frustum: *const Frustum, center: [*]const f32, extents: [*]const f32, rotation: [*]const f32) bool {
    // 从四元数构建旋转矩阵的三个轴
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

    // OBB的三个轴
    const axis_x = [3]f32{ 1.0 - (yy + zz), xy + wz, xz - wy };
    const axis_y = [3]f32{ xy - wz, 1.0 - (xx + zz), yz + wx };
    const axis_z = [3]f32{ xz + wy, yz - wx, 1.0 - (xx + yy) };

    var i: usize = 0;
    while (i < 6) : (i += 1) {
        const plane = &frustum.planes[i];

        // 计算OBB在平面法向量方向的投影半径
        const r = @abs(extents[0] * (plane.normal[0] * axis_x[0] + plane.normal[1] * axis_x[1] + plane.normal[2] * axis_x[2])) +
            @abs(extents[1] * (plane.normal[0] * axis_y[0] + plane.normal[1] * axis_y[1] + plane.normal[2] * axis_y[2])) +
            @abs(extents[2] * (plane.normal[0] * axis_z[0] + plane.normal[1] * axis_z[1] + plane.normal[2] * axis_z[2]));

        // 计算中心到平面的距离
        const distance = plane.normal[0] * center[0] +
            plane.normal[1] * center[1] +
            plane.normal[2] * center[2] +
            plane.distance;

        // 如果距离 < -r，则OBB完全在平面背面
        if (distance < -r) {
            return false;
        }
    }
    return true;
}

/// 获取视锥体的8个顶点（用于调试和可视化）
/// frustum: View frustum
/// out_vertices: Output array of 8 vertices [x, y, z] * 8
export fn frustum_get_corners(frustum: *const Frustum, out_vertices: [*]f32) void {
    // 计算三个平面的交点来获得顶点
    // 这是一个简化实现，实际可能需要更复杂的计算

    // 近平面的4个顶点
    _ = frustum;

    // 简化：设置一些默认值（实际应该从平面计算）
    const near_corners = [_][3]f32{
        .{ -1.0, -1.0, 0.0 },
        .{ 1.0, -1.0, 0.0 },
        .{ 1.0, 1.0, 0.0 },
        .{ -1.0, 1.0, 0.0 },
    };

    const far_corners = [_][3]f32{
        .{ -10.0, -10.0, 10.0 },
        .{ 10.0, -10.0, 10.0 },
        .{ 10.0, 10.0, 10.0 },
        .{ -10.0, 10.0, 10.0 },
    };

    var i: usize = 0;
    while (i < 4) : (i += 1) {
        out_vertices[i * 3 + 0] = near_corners[i][0];
        out_vertices[i * 3 + 1] = near_corners[i][1];
        out_vertices[i * 3 + 2] = near_corners[i][2];

        out_vertices[(i + 4) * 3 + 0] = far_corners[i][0];
        out_vertices[(i + 4) * 3 + 1] = far_corners[i][1];
        out_vertices[(i + 4) * 3 + 2] = far_corners[i][2];
    }
}

/// 测试两个球体是否可见（用于级联阴影等）
export fn frustum_test_sphere_conservative(frustum: *const Frustum, center: [*]const f32, radius: f32, margin: f32) bool {
    const expanded_radius = radius + margin;
    var i: usize = 0;
    while (i < 6) : (i += 1) {
        const plane = &frustum.planes[i];
        const distance = plane.normal[0] * center[0] +
            plane.normal[1] * center[1] +
            plane.normal[2] * center[2] +
            plane.distance;

        if (distance < -expanded_radius) {
            return false;
        }
    }
    return true;
}

/// 测试AABB是否完全在视锥体内（非相交）
export fn frustum_test_aabb_inside(frustum: *const Frustum, min: [*]const f32, max: [*]const f32) bool {
    var i: usize = 0;
    while (i < 6) : (i += 1) {
        const plane = &frustum.planes[i];

        // 计算negative vertex（沿法向量反方向最远的顶点）
        var nx = max[0];
        var ny = max[1];
        var nz = max[2];

        if (plane.normal[0] >= 0.0) nx = min[0];
        if (plane.normal[1] >= 0.0) ny = min[1];
        if (plane.normal[2] >= 0.0) nz = min[2];

        // 如果negative vertex在平面背面，则AABB不完全在内部
        const distance = plane.normal[0] * nx +
            plane.normal[1] * ny +
            plane.normal[2] * nz +
            plane.distance;

        if (distance < 0.0) {
            return false;
        }
    }
    return true;
}

/// 计算AABB到视锥体的最近距离（用于LOD等）
export fn frustum_distance_to_aabb(frustum: *const Frustum, min: [*]const f32, max: [*]const f32) f32 {
    var max_distance: f32 = -999999.0;

    var i: usize = 0;
    while (i < 6) : (i += 1) {
        const plane = &frustum.planes[i];

        var px = min[0];
        var py = min[1];
        var pz = min[2];

        if (plane.normal[0] >= 0.0) px = max[0];
        if (plane.normal[1] >= 0.0) py = max[1];
        if (plane.normal[2] >= 0.0) pz = max[2];

        const distance = plane.normal[0] * px +
            plane.normal[1] * py +
            plane.normal[2] * pz +
            plane.distance;

        if (distance > max_distance) {
            max_distance = distance;
        }
    }

    return if (max_distance < 0.0) -max_distance else 0.0;
}
