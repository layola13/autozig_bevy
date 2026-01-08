const std = @import("std");
const mesh_mod = @import("mesh.zig");
const vertex_mod = @import("vertex.zig");
const Mesh = mesh_mod.Mesh;
const Vertex = vertex_mod.Vertex;

// ============================================================================
// Mesh Primitives - 几何体生成器
// ============================================================================

pub const MeshPrimitives = struct {
    /// 创建立方体 (24 顶点, 36 索引)
    /// size: 立方体边长
    pub fn cube(size: f32) Mesh {
        var m = Mesh.init();
        const half = size * 0.5;

        // 前面 (+Z)
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, -half, half }, [_]f32{ 0.0, 0.0, 1.0 }, [_]f32{ 0.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, -half, half }, [_]f32{ 0.0, 0.0, 1.0 }, [_]f32{ 1.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, half, half }, [_]f32{ 0.0, 0.0, 1.0 }, [_]f32{ 1.0, 1.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, half, half }, [_]f32{ 0.0, 0.0, 1.0 }, [_]f32{ 0.0, 1.0 })) catch {};

        // 后面 (-Z)
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, -half, -half }, [_]f32{ 0.0, 0.0, -1.0 }, [_]f32{ 0.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, -half, -half }, [_]f32{ 0.0, 0.0, -1.0 }, [_]f32{ 1.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, half, -half }, [_]f32{ 0.0, 0.0, -1.0 }, [_]f32{ 1.0, 1.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, half, -half }, [_]f32{ 0.0, 0.0, -1.0 }, [_]f32{ 0.0, 1.0 })) catch {};

        // 右面 (+X)
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, -half, half }, [_]f32{ 1.0, 0.0, 0.0 }, [_]f32{ 0.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, -half, -half }, [_]f32{ 1.0, 0.0, 0.0 }, [_]f32{ 1.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, half, -half }, [_]f32{ 1.0, 0.0, 0.0 }, [_]f32{ 1.0, 1.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, half, half }, [_]f32{ 1.0, 0.0, 0.0 }, [_]f32{ 0.0, 1.0 })) catch {};

        // 左面 (-X)
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, -half, -half }, [_]f32{ -1.0, 0.0, 0.0 }, [_]f32{ 0.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, -half, half }, [_]f32{ -1.0, 0.0, 0.0 }, [_]f32{ 1.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, half, half }, [_]f32{ -1.0, 0.0, 0.0 }, [_]f32{ 1.0, 1.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, half, -half }, [_]f32{ -1.0, 0.0, 0.0 }, [_]f32{ 0.0, 1.0 })) catch {};

        // 上面 (+Y)
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, half, half }, [_]f32{ 0.0, 1.0, 0.0 }, [_]f32{ 0.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, half, half }, [_]f32{ 0.0, 1.0, 0.0 }, [_]f32{ 1.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, half, -half }, [_]f32{ 0.0, 1.0, 0.0 }, [_]f32{ 1.0, 1.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, half, -half }, [_]f32{ 0.0, 1.0, 0.0 }, [_]f32{ 0.0, 1.0 })) catch {};

        // 下面 (-Y)
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, -half, -half }, [_]f32{ 0.0, -1.0, 0.0 }, [_]f32{ 0.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, -half, -half }, [_]f32{ 0.0, -1.0, 0.0 }, [_]f32{ 1.0, 0.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ half, -half, half }, [_]f32{ 0.0, -1.0, 0.0 }, [_]f32{ 1.0, 1.0 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ -half, -half, half }, [_]f32{ 0.0, -1.0, 0.0 }, [_]f32{ 0.0, 1.0 })) catch {};

        // 索引 (6个面，每面2个三角形)
        const faces = [_][6]u32{
            [_]u32{ 0, 1, 2, 2, 3, 0 }, // 前
            [_]u32{ 4, 5, 6, 6, 7, 4 }, // 后
            [_]u32{ 8, 9, 10, 10, 11, 8 }, // 右
            [_]u32{ 12, 13, 14, 14, 15, 12 }, // 左
            [_]u32{ 16, 17, 18, 18, 19, 16 }, // 上
            [_]u32{ 20, 21, 22, 22, 23, 20 }, // 下
        };

        for (faces) |face| {
            for (face) |idx| {
                _ = m.addIndex(idx) catch {};
            }
        }

        return m;
    }

    /// 创建球体（UV球）
    /// radius: 半径
    /// segments: 经度分段数
    /// rings: 纬度分段数
    pub fn sphere(radius: f32, segments: u32, rings: u32) Mesh {
        var m = Mesh.init();
        const pi = std.math.pi;

        // 生成顶点
        var ring: u32 = 0;
        while (ring <= rings) : (ring += 1) {
            const phi = @as(f32, @floatFromInt(ring)) * pi / @as(f32, @floatFromInt(rings));
            const sin_phi = @sin(phi);
            const cos_phi = @cos(phi);

            var seg: u32 = 0;
            while (seg <= segments) : (seg += 1) {
                const theta = @as(f32, @floatFromInt(seg)) * 2.0 * pi / @as(f32, @floatFromInt(segments));
                const sin_theta = @sin(theta);
                const cos_theta = @cos(theta);

                const x = cos_theta * sin_phi;
                const y = cos_phi;
                const z = sin_theta * sin_phi;

                const pos = [_]f32{ x * radius, y * radius, z * radius };
                const normal = [_]f32{ x, y, z };
                const uv = [_]f32{
                    @as(f32, @floatFromInt(seg)) / @as(f32, @floatFromInt(segments)),
                    @as(f32, @floatFromInt(ring)) / @as(f32, @floatFromInt(rings)),
                };

                _ = m.addVertex(Vertex.withPositionNormalUv(pos, normal, uv)) catch {};
            }
        }

        // 生成索引
        ring = 0;
        while (ring < rings) : (ring += 1) {
            var seg: u32 = 0;
            while (seg < segments) : (seg += 1) {
                const current = ring * (segments + 1) + seg;
                const next = current + segments + 1;

                _ = m.addTriangle(current, next, current + 1) catch {};
                _ = m.addTriangle(next, next + 1, current + 1) catch {};
            }
        }

        return m;
    }

    /// 创建平面 (XZ平面，Y轴朝上)
    /// width: 宽度 (X方向)
    /// height: 高度 (Z方向)
    /// subdivisions_x: X方向细分数
    /// subdivisions_z: Z方向细分数
    pub fn plane(width: f32, height: f32, subdivisions_x: u32, subdivisions_z: u32) Mesh {
        var m = Mesh.init();
        const half_width = width * 0.5;
        const half_height = height * 0.5;

        // 生成顶点
        var z: u32 = 0;
        while (z <= subdivisions_z) : (z += 1) {
            var x: u32 = 0;
            while (x <= subdivisions_x) : (x += 1) {
                const fx = @as(f32, @floatFromInt(x)) / @as(f32, @floatFromInt(subdivisions_x));
                const fz = @as(f32, @floatFromInt(z)) / @as(f32, @floatFromInt(subdivisions_z));

                const px = fx * width - half_width;
                const pz = fz * height - half_height;

                const pos = [_]f32{ px, 0.0, pz };
                const normal = [_]f32{ 0.0, 1.0, 0.0 };
                const uv = [_]f32{ fx, fz };

                _ = m.addVertex(Vertex.withPositionNormalUv(pos, normal, uv)) catch {};
            }
        }

        // 生成索引
        z = 0;
        while (z < subdivisions_z) : (z += 1) {
            var x: u32 = 0;
            while (x < subdivisions_x) : (x += 1) {
                const idx0 = z * (subdivisions_x + 1) + x;
                const idx1 = idx0 + 1;
                const idx2 = idx0 + subdivisions_x + 1;
                const idx3 = idx2 + 1;

                _ = m.addTriangle(idx0, idx2, idx1) catch {};
                _ = m.addTriangle(idx1, idx2, idx3) catch {};
            }
        }

        return m;
    }

    /// 创建圆柱体
    /// radius: 半径
    /// height: 高度
    /// segments: 圆周分段数
    pub fn cylinder(radius: f32, height: f32, segments: u32) Mesh {
        var m = Mesh.init();
        const half_height = height * 0.5;
        const pi = std.math.pi;

        // 侧面顶点
        var seg: u32 = 0;
        while (seg <= segments) : (seg += 1) {
            const theta = @as(f32, @floatFromInt(seg)) * 2.0 * pi / @as(f32, @floatFromInt(segments));
            const cos_theta = @cos(theta);
            const sin_theta = @sin(theta);

            const x = cos_theta * radius;
            const z = sin_theta * radius;
            const normal = [_]f32{ cos_theta, 0.0, sin_theta };
            const u = @as(f32, @floatFromInt(seg)) / @as(f32, @floatFromInt(segments));

            // 底部顶点
            _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ x, -half_height, z }, normal, [_]f32{ u, 0.0 })) catch {};
            // 顶部顶点
            _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ x, half_height, z }, normal, [_]f32{ u, 1.0 })) catch {};
        }

        // 侧面索引
        seg = 0;
        while (seg < segments) : (seg += 1) {
            const current = seg * 2;
            const next = current + 2;

            _ = m.addTriangle(current, current + 1, next) catch {};
            _ = m.addTriangle(next, current + 1, next + 1) catch {};
        }

        // 顶部和底部中心点索引
        const top_center_idx = m.vertex_count;
        const bottom_center_idx = m.vertex_count + 1;

        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ 0.0, half_height, 0.0 }, [_]f32{ 0.0, 1.0, 0.0 }, [_]f32{ 0.5, 0.5 })) catch {};
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ 0.0, -half_height, 0.0 }, [_]f32{ 0.0, -1.0, 0.0 }, [_]f32{ 0.5, 0.5 })) catch {};

        // 顶部和底部盖子
        seg = 0;
        while (seg < segments) : (seg += 1) {
            const current = seg * 2;
            const next = (seg + 1) * 2;

            // 顶部盖子
            _ = m.addTriangle(top_center_idx, next + 1, current + 1) catch {};
            // 底部盖子
            _ = m.addTriangle(bottom_center_idx, current, next) catch {};
        }

        return m;
    }

    /// 创建圆锥体
    /// radius: 底面半径
    /// height: 高度
    /// segments: 圆周分段数
    pub fn cone(radius: f32, height: f32, segments: u32) Mesh {
        var m = Mesh.init();
        const pi = std.math.pi;

        // 顶点
        const apex_idx = m.vertex_count;
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ 0.0, height, 0.0 }, [_]f32{ 0.0, 1.0, 0.0 }, [_]f32{ 0.5, 1.0 })) catch {};

        // 底面圆周顶点
        var seg: u32 = 0;
        while (seg <= segments) : (seg += 1) {
            const theta = @as(f32, @floatFromInt(seg)) * 2.0 * pi / @as(f32, @floatFromInt(segments));
            const x = @cos(theta) * radius;
            const z = @sin(theta) * radius;

            const u = @as(f32, @floatFromInt(seg)) / @as(f32, @floatFromInt(segments));
            _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ x, 0.0, z }, [_]f32{ x / radius, 0.5, z / radius }, [_]f32{ u, 0.0 })) catch {};
        }

        // 侧面三角形
        seg = 0;
        while (seg < segments) : (seg += 1) {
            const current = apex_idx + 1 + seg;
            const next = current + 1;
            _ = m.addTriangle(apex_idx, current, next) catch {};
        }

        // 底面中心点和三角形
        const bottom_center_idx = m.vertex_count;
        _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ 0.0, 0.0, 0.0 }, [_]f32{ 0.0, -1.0, 0.0 }, [_]f32{ 0.5, 0.5 })) catch {};

        seg = 0;
        while (seg < segments) : (seg += 1) {
            const current = apex_idx + 1 + seg;
            const next = current + 1;
            _ = m.addTriangle(bottom_center_idx, next, current) catch {};
        }

        return m;
    }

    /// 创建环形体 (Torus)
    /// major_radius: 主半径（环的中心到管的中心）
    /// minor_radius: 次半径（管的半径）
    /// major_segments: 主方向分段数
    /// minor_segments: 次方向分段数
    pub fn torus(major_radius: f32, minor_radius: f32, major_segments: u32, minor_segments: u32) Mesh {
        var m = Mesh.init();
        const pi = std.math.pi;

        // 生成顶点
        var maj: u32 = 0;
        while (maj <= major_segments) : (maj += 1) {
            const theta = @as(f32, @floatFromInt(maj)) * 2.0 * pi / @as(f32, @floatFromInt(major_segments));
            const cos_theta = @cos(theta);
            const sin_theta = @sin(theta);

            var min: u32 = 0;
            while (min <= minor_segments) : (min += 1) {
                const phi = @as(f32, @floatFromInt(min)) * 2.0 * pi / @as(f32, @floatFromInt(minor_segments));
                const cos_phi = @cos(phi);
                const sin_phi = @sin(phi);

                const x = (major_radius + minor_radius * cos_phi) * cos_theta;
                const y = minor_radius * sin_phi;
                const z = (major_radius + minor_radius * cos_phi) * sin_theta;

                const nx = cos_phi * cos_theta;
                const ny = sin_phi;
                const nz = cos_phi * sin_theta;

                const u = @as(f32, @floatFromInt(maj)) / @as(f32, @floatFromInt(major_segments));
                const v = @as(f32, @floatFromInt(min)) / @as(f32, @floatFromInt(minor_segments));

                _ = m.addVertex(Vertex.withPositionNormalUv([_]f32{ x, y, z }, [_]f32{ nx, ny, nz }, [_]f32{ u, v })) catch {};
            }
        }

        // 生成索引
        maj = 0;
        while (maj < major_segments) : (maj += 1) {
            var min: u32 = 0;
            while (min < minor_segments) : (min += 1) {
                const current = maj * (minor_segments + 1) + min;
                const next = current + minor_segments + 1;

                _ = m.addTriangle(current, next, current + 1) catch {};
                _ = m.addTriangle(next, next + 1, current + 1) catch {};
            }
        }

        return m;
    }

    /// 创建胶囊体 (Capsule)
    /// radius: 半径
    /// height: 圆柱部分高度（不包括半球）
    /// rings: 半球的纬度分段数
    /// segments: 圆周分段数
    pub fn capsule(radius: f32, height: f32, rings: u32, segments: u32) Mesh {
        var m = Mesh.init();
        const half_height = height * 0.5;
        const pi = std.math.pi;

        // 上半球
        var ring: u32 = 0;
        while (ring <= rings) : (ring += 1) {
            const phi = @as(f32, @floatFromInt(ring)) * (pi * 0.5) / @as(f32, @floatFromInt(rings));
            const sin_phi = @sin(phi);
            const cos_phi = @cos(phi);

            var seg: u32 = 0;
            while (seg <= segments) : (seg += 1) {
                const theta = @as(f32, @floatFromInt(seg)) * 2.0 * pi / @as(f32, @floatFromInt(segments));
                const sin_theta = @sin(theta);
                const cos_theta = @cos(theta);

                const x = cos_theta * sin_phi;
                const y = cos_phi;
                const z = sin_theta * sin_phi;

                const pos = [_]f32{ x * radius, y * radius + half_height, z * radius };
                const normal = [_]f32{ x, y, z };
                const uv = [_]f32{
                    @as(f32, @floatFromInt(seg)) / @as(f32, @floatFromInt(segments)),
                    @as(f32, @floatFromInt(ring)) / @as(f32, @floatFromInt(rings)) * 0.5,
                };

                _ = m.addVertex(Vertex.withPositionNormalUv(pos, normal, uv)) catch {};
            }
        }

        // 下半球
        ring = 0;
        while (ring <= rings) : (ring += 1) {
            const phi = pi * 0.5 + @as(f32, @floatFromInt(ring)) * (pi * 0.5) / @as(f32, @floatFromInt(rings));
            const sin_phi = @sin(phi);
            const cos_phi = @cos(phi);

            var seg: u32 = 0;
            while (seg <= segments) : (seg += 1) {
                const theta = @as(f32, @floatFromInt(seg)) * 2.0 * pi / @as(f32, @floatFromInt(segments));
                const sin_theta = @sin(theta);
                const cos_theta = @cos(theta);

                const x = cos_theta * sin_phi;
                const y = cos_phi;
                const z = sin_theta * sin_phi;

                const pos = [_]f32{ x * radius, y * radius - half_height, z * radius };
                const normal = [_]f32{ x, y, z };
                const uv = [_]f32{
                    @as(f32, @floatFromInt(seg)) / @as(f32, @floatFromInt(segments)),
                    0.5 + @as(f32, @floatFromInt(ring)) / @as(f32, @floatFromInt(rings)) * 0.5,
                };

                _ = m.addVertex(Vertex.withPositionNormalUv(pos, normal, uv)) catch {};
            }
        }

        // 生成上半球索引
        ring = 0;
        while (ring < rings) : (ring += 1) {
            var seg: u32 = 0;
            while (seg < segments) : (seg += 1) {
                const current = ring * (segments + 1) + seg;
                const next = current + segments + 1;

                _ = m.addTriangle(current, next, current + 1) catch {};
                _ = m.addTriangle(next, next + 1, current + 1) catch {};
            }
        }

        // 生成下半球索引
        const lower_offset = (rings + 1) * (segments + 1);
        ring = 0;
        while (ring < rings) : (ring += 1) {
            var seg: u32 = 0;
            while (seg < segments) : (seg += 1) {
                const current = lower_offset + ring * (segments + 1) + seg;
                const next = current + segments + 1;

                _ = m.addTriangle(current, next, current + 1) catch {};
                _ = m.addTriangle(next, next + 1, current + 1) catch {};
            }
        }

        return m;
    }
};

// ============================================================================
// FFI Exports
// ============================================================================

export fn primitives_cube(size: f32) Mesh {
    return MeshPrimitives.cube(size);
}

export fn primitives_sphere(radius: f32, segments: u32, rings: u32) Mesh {
    return MeshPrimitives.sphere(radius, segments, rings);
}

export fn primitives_plane(width: f32, height: f32, subdivisions_x: u32, subdivisions_z: u32) Mesh {
    return MeshPrimitives.plane(width, height, subdivisions_x, subdivisions_z);
}

export fn primitives_cylinder(radius: f32, height: f32, segments: u32) Mesh {
    return MeshPrimitives.cylinder(radius, height, segments);
}

export fn primitives_cone(radius: f32, height: f32, segments: u32) Mesh {
    return MeshPrimitives.cone(radius, height, segments);
}

export fn primitives_torus(major_radius: f32, minor_radius: f32, major_segments: u32, minor_segments: u32) Mesh {
    return MeshPrimitives.torus(major_radius, minor_radius, major_segments, minor_segments);
}

export fn primitives_capsule(radius: f32, height: f32, rings: u32, segments: u32) Mesh {
    return MeshPrimitives.capsule(radius, height, rings, segments);
}
