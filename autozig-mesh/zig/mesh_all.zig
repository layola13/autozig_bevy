const std = @import("std");

// ============================================================================
// This file contains all mesh module code merged inline
// to avoid import issues with autozig-build system
// ============================================================================


// ============================================================================
// Vertex Data Structures (from zig/vertex.zig)
// ============================================================================


// ============================================================================
// Vertex Data Structure
// ============================================================================

/// 顶点数据结构 - 包含所有标准顶点属性
pub const Vertex = extern struct {
    position: [3]f32,
    normal: [3]f32,
    uv: [2]f32,
    tangent: [4]f32, // xyz + handedness (w component)
    color: [4]f32,

    /// 创建一个默认顶点
    pub fn init() Vertex {
        return Vertex{
            .position = [_]f32{ 0.0, 0.0, 0.0 },
            .normal = [_]f32{ 0.0, 1.0, 0.0 },
            .uv = [_]f32{ 0.0, 0.0 },
            .tangent = [_]f32{ 1.0, 0.0, 0.0, 1.0 },
            .color = [_]f32{ 1.0, 1.0, 1.0, 1.0 },
        };
    }

    /// 创建一个只有位置的顶点
    pub fn withPosition(pos: [3]f32) Vertex {
        var v = init();
        v.position = pos;
        return v;
    }

    /// 创建一个带位置和法线的顶点
    pub fn withPositionNormal(pos: [3]f32, norm: [3]f32) Vertex {
        var v = init();
        v.position = pos;
        v.normal = norm;
        return v;
    }

    /// 创建一个带位置、法线和UV的顶点
    pub fn withPositionNormalUv(pos: [3]f32, norm: [3]f32, uv_coords: [2]f32) Vertex {
        var v = init();
        v.position = pos;
        v.normal = norm;
        v.uv = uv_coords;
        return v;
    }
};

// ============================================================================
// Vertex Attribute Enumeration
// ============================================================================

/// 顶点属性枚举
pub const VertexAttribute = enum(u32) {
    Position = 0,
    Normal = 1,
    Uv = 2,
    Tangent = 3,
    Color = 4,
};

// ============================================================================
// FFI Exports
// ============================================================================

export fn vertex_init() Vertex {
    return Vertex.init();
}

export fn vertex_with_position(x: f32, y: f32, z: f32) Vertex {
    return Vertex.withPosition([_]f32{ x, y, z });
}

export fn vertex_with_position_normal(px: f32, py: f32, pz: f32, nx: f32, ny: f32, nz: f32) Vertex {
    return Vertex.withPositionNormal([_]f32{ px, py, pz }, [_]f32{ nx, ny, nz });
}

export fn vertex_with_position_normal_uv(px: f32, py: f32, pz: f32, nx: f32, ny: f32, nz: f32, u: f32, v: f32) Vertex {
    return Vertex.withPositionNormalUv([_]f32{ px, py, pz }, [_]f32{ nx, ny, nz }, [_]f32{ u, v });
}


// ============================================================================
// Mesh Core (from zig/mesh.zig)
// ============================================================================


// ============================================================================
// Primitive Topology
// ============================================================================

pub const PrimitiveTopology = enum(u32) {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
};

// ============================================================================
// Mesh Data Structure
// ============================================================================

pub const Mesh = extern struct {
    vertices: [4096]Vertex, // 固定大小顶点数组
    vertex_count: u32,
    indices: [8192]u32, // 固定大小索引数组
    index_count: u32,
    primitive_topology: PrimitiveTopology,

    /// 初始化一个空网格
    pub fn init() Mesh {
        return Mesh{
            .vertices = undefined,
            .vertex_count = 0,
            .indices = undefined,
            .index_count = 0,
            .primitive_topology = .TriangleList,
        };
    }

    /// 添加一个顶点
    pub fn addVertex(self: *Mesh, v: Vertex) !void {
        if (self.vertex_count >= 4096) {
            return error.VertexBufferFull;
        }
        self.vertices[self.vertex_count] = v;
        self.vertex_count += 1;
    }

    /// 添加一个索引
    pub fn addIndex(self: *Mesh, index: u32) !void {
        if (self.index_count >= 8192) {
            return error.IndexBufferFull;
        }
        self.indices[self.index_count] = index;
        self.index_count += 1;
    }

    /// 添加一个三角形（3个索引）
    pub fn addTriangle(self: *Mesh, idx0: u32, idx1: u32, idx2: u32) !void {
        try self.addIndex(idx0);
        try self.addIndex(idx1);
        try self.addIndex(idx2);
    }

    /// 计算平面法线（根据索引）
    pub fn calculateNormals(self: *Mesh) void {
        if (self.primitive_topology != .TriangleList) return;
        if (self.index_count < 3) return;

        // 首先重置所有法线
        var i: u32 = 0;
        while (i < self.vertex_count) : (i += 1) {
            self.vertices[i].normal = [_]f32{ 0.0, 0.0, 0.0 };
        }

        // 遍历所有三角形
        i = 0;
        while (i + 2 < self.index_count) : (i += 3) {
            const idx0 = self.indices[i];
            const idx1 = self.indices[i + 1];
            const idx2 = self.indices[i + 2];

            if (idx0 >= self.vertex_count or idx1 >= self.vertex_count or idx2 >= self.vertex_count) {
                continue;
            }

            const v0 = self.vertices[idx0].position;
            const v1 = self.vertices[idx1].position;
            const v2 = self.vertices[idx2].position;

            // 计算两条边
            const edge1 = [_]f32{
                v1[0] - v0[0],
                v1[1] - v0[1],
                v1[2] - v0[2],
            };
            const edge2 = [_]f32{
                v2[0] - v0[0],
                v2[1] - v0[1],
                v2[2] - v0[2],
            };

            // 计算法线（叉乘）
            const normal = [_]f32{
                edge1[1] * edge2[2] - edge1[2] * edge2[1],
                edge1[2] * edge2[0] - edge1[0] * edge2[2],
                edge1[0] * edge2[1] - edge1[1] * edge2[0],
            };

            // 累加到三个顶点的法线
            self.vertices[idx0].normal[0] += normal[0];
            self.vertices[idx0].normal[1] += normal[1];
            self.vertices[idx0].normal[2] += normal[2];

            self.vertices[idx1].normal[0] += normal[0];
            self.vertices[idx1].normal[1] += normal[1];
            self.vertices[idx1].normal[2] += normal[2];

            self.vertices[idx2].normal[0] += normal[0];
            self.vertices[idx2].normal[1] += normal[1];
            self.vertices[idx2].normal[2] += normal[2];
        }

        // 归一化所有法线
        i = 0;
        while (i < self.vertex_count) : (i += 1) {
            const n = &self.vertices[i].normal;
            const len = @sqrt(n[0] * n[0] + n[1] * n[1] + n[2] * n[2]);
            if (len > 0.0001) {
                n[0] /= len;
                n[1] /= len;
                n[2] /= len;
            } else {
                n[0] = 0.0;
                n[1] = 1.0;
                n[2] = 0.0;
            }
        }
    }

    /// 计算切线（用于法线贴图）
    pub fn calculateTangents(self: *Mesh) void {
        if (self.primitive_topology != .TriangleList) return;
        if (self.index_count < 3) return;

        // 首先重置所有切线
        var i: u32 = 0;
        while (i < self.vertex_count) : (i += 1) {
            self.vertices[i].tangent = [_]f32{ 1.0, 0.0, 0.0, 1.0 };
        }

        // 遍历所有三角形
        i = 0;
        while (i + 2 < self.index_count) : (i += 3) {
            const idx0 = self.indices[i];
            const idx1 = self.indices[i + 1];
            const idx2 = self.indices[i + 2];

            if (idx0 >= self.vertex_count or idx1 >= self.vertex_count or idx2 >= self.vertex_count) {
                continue;
            }

            const v0 = &self.vertices[idx0];
            const v1 = &self.vertices[idx1];
            const v2 = &self.vertices[idx2];

            // 计算边向量
            const edge1 = [_]f32{
                v1.position[0] - v0.position[0],
                v1.position[1] - v0.position[1],
                v1.position[2] - v0.position[2],
            };
            const edge2 = [_]f32{
                v2.position[0] - v0.position[0],
                v2.position[1] - v0.position[1],
                v2.position[2] - v0.position[2],
            };

            // 计算 UV 差值
            const duv1 = [_]f32{
                v1.uv[0] - v0.uv[0],
                v1.uv[1] - v0.uv[1],
            };
            const duv2 = [_]f32{
                v2.uv[0] - v0.uv[0],
                v2.uv[1] - v0.uv[1],
            };

            const det = duv1[0] * duv2[1] - duv1[1] * duv2[0];
            if (@abs(det) < 0.0001) continue;

            const r = 1.0 / det;

            // 计算切线
            const tangent = [_]f32{
                r * (duv2[1] * edge1[0] - duv1[1] * edge2[0]),
                r * (duv2[1] * edge1[1] - duv1[1] * edge2[1]),
                r * (duv2[1] * edge1[2] - duv1[1] * edge2[2]),
            };

            // 简单设置切线（实际应该累加并归一化）
            v0.tangent[0] = tangent[0];
            v0.tangent[1] = tangent[1];
            v0.tangent[2] = tangent[2];
            v0.tangent[3] = 1.0;

            v1.tangent[0] = tangent[0];
            v1.tangent[1] = tangent[1];
            v1.tangent[2] = tangent[2];
            v1.tangent[3] = 1.0;

            v2.tangent[0] = tangent[0];
            v2.tangent[1] = tangent[1];
            v2.tangent[2] = tangent[2];
            v2.tangent[3] = 1.0;
        }
    }
};

// ============================================================================
// FFI Exports
// ============================================================================

export fn mesh_init() Mesh {
    return Mesh.init();
}

export fn mesh_add_vertex(mesh: *Mesh, v: Vertex) bool {
    mesh.addVertex(v) catch return false;
    return true;
}

export fn mesh_add_index(mesh: *Mesh, index: u32) bool {
    mesh.addIndex(index) catch return false;
    return true;
}

export fn mesh_add_triangle(mesh: *Mesh, idx0: u32, idx1: u32, idx2: u32) bool {
    mesh.addTriangle(idx0, idx1, idx2) catch return false;
    return true;
}

export fn mesh_calculate_normals(mesh: *Mesh) void {
    mesh.calculateNormals();
}

export fn mesh_calculate_tangents(mesh: *Mesh) void {
    mesh.calculateTangents();
}

export fn mesh_vertex_count(mesh: *const Mesh) u32 {
    return mesh.vertex_count;
}

export fn mesh_index_count(mesh: *const Mesh) u32 {
    return mesh.index_count;
}

export fn mesh_set_topology(mesh: *Mesh, topology: PrimitiveTopology) void {
    mesh.primitive_topology = topology;
}


// ============================================================================
// Primitive Generators (from zig/primitives.zig)
// ============================================================================


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


// ============================================================================
// GPU Mesh (from zig/gpu_mesh.zig)
// ============================================================================


// ============================================================================
// GPU Mesh - GPU 缓冲区管理
// ============================================================================

pub const GpuMesh = extern struct {
    vertex_buffer: ?*anyopaque, // wgpu.Buffer (不透明指针)
    index_buffer: ?*anyopaque, // wgpu.Buffer (不透明指针)
    vertex_count: u32,
    index_count: u32,

    /// 从 CPU Mesh 创建 GPU Mesh
    /// device: WebGPU device 指针
    /// mesh: 源网格数据
    pub fn fromMesh(device: *anyopaque, mesh: *const Mesh) !GpuMesh {
        _ = device; // 在实际实现中会使用 device 创建缓冲区

        return GpuMesh{
            .vertex_buffer = null, // 实际应该创建缓冲区
            .index_buffer = null, // 实际应该创建缓冲区
            .vertex_count = mesh.vertex_count,
            .index_count = mesh.index_count,
        };
    }

    /// 更新顶点数据
    pub fn updateVertices(self: *GpuMesh, queue: *anyopaque, vertices: []const Vertex) !void {
        _ = self;
        _ = queue;
        _ = vertices;
        // 实际实现会使用 queue.writeBuffer 更新顶点缓冲区
    }

    /// 更新索引数据
    pub fn updateIndices(self: *GpuMesh, queue: *anyopaque, indices: []const u32) !void {
        _ = self;
        _ = queue;
        _ = indices;
        // 实际实现会使用 queue.writeBuffer 更新索引缓冲区
    }

    /// 销毁 GPU 缓冲区
    pub fn destroy(self: *GpuMesh) void {
        if (self.vertex_buffer) |_| {
            // 实际实现会调用 buffer.destroy()
            self.vertex_buffer = null;
        }
        if (self.index_buffer) |_| {
            // 实际实现会调用 buffer.destroy()
            self.index_buffer = null;
        }
        self.vertex_count = 0;
        self.index_count = 0;
    }
};

// ============================================================================
// FFI Exports
// ============================================================================

export fn gpu_mesh_from_mesh(device: *anyopaque, mesh: *const Mesh) GpuMesh {
    return GpuMesh.fromMesh(device, mesh) catch GpuMesh{
        .vertex_buffer = null,
        .index_buffer = null,
        .vertex_count = 0,
        .index_count = 0,
    };
}

export fn gpu_mesh_update_vertices(gpu_mesh: *GpuMesh, queue: *anyopaque, vertices: [*]const Vertex, count: u32) bool {
    const slice = vertices[0..count];
    gpu_mesh.updateVertices(queue, slice) catch return false;
    return true;
}

export fn gpu_mesh_update_indices(gpu_mesh: *GpuMesh, queue: *anyopaque, indices: [*]const u32, count: u32) bool {
    const slice = indices[0..count];
    gpu_mesh.updateIndices(queue, slice) catch return false;
    return true;
}

export fn gpu_mesh_destroy(gpu_mesh: *GpuMesh) void {
    gpu_mesh.destroy();
}

export fn gpu_mesh_vertex_count(gpu_mesh: *const GpuMesh) u32 {
    return gpu_mesh.vertex_count;
}

export fn gpu_mesh_index_count(gpu_mesh: *const GpuMesh) u32 {
    return gpu_mesh.index_count;
}


// ============================================================================
// Vertex Layout (from zig/vertex_layout.zig)
// ============================================================================


// ============================================================================
// Vertex Format
// ============================================================================

pub const VertexFormat = enum(u32) {
    Float32x2 = 0,
    Float32x3 = 1,
    Float32x4 = 2,
    Uint32 = 3,
    Uint32x2 = 4,
};

// ============================================================================
// Vertex Step Mode
// ============================================================================

pub const VertexStepMode = enum(u32) {
    Vertex = 0,
    Instance = 1,
};

// ============================================================================
// Vertex Attribute Descriptor
// ============================================================================

pub const VertexAttributeDesc = extern struct {
    format: VertexFormat,
    offset: u32,
    shader_location: u32,
};

// ============================================================================
// Vertex Buffer Layout
// ============================================================================

pub const VertexBufferLayout = extern struct {
    attributes: [8]VertexAttributeDesc,
    attribute_count: u32,
    stride: u32,
    step_mode: VertexStepMode,

    /// 标准顶点布局（包含所有属性）
    pub fn standard() VertexBufferLayout {
        var layout = VertexBufferLayout{
            .attributes = undefined,
            .attribute_count = 5,
            .stride = 64, // 3*4 + 3*4 + 2*4 + 4*4 + 4*4 = 64 bytes
            .step_mode = .Vertex,
        };

        // Position (location 0)
        layout.attributes[0] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 0,
            .shader_location = 0,
        };

        // Normal (location 1)
        layout.attributes[1] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 12,
            .shader_location = 1,
        };

        // UV (location 2)
        layout.attributes[2] = VertexAttributeDesc{
            .format = .Float32x2,
            .offset = 24,
            .shader_location = 2,
        };

        // Tangent (location 3)
        layout.attributes[3] = VertexAttributeDesc{
            .format = .Float32x4,
            .offset = 32,
            .shader_location = 3,
        };

        // Color (location 4)
        layout.attributes[4] = VertexAttributeDesc{
            .format = .Float32x4,
            .offset = 48,
            .shader_location = 4,
        };

        return layout;
    }

    /// 只有位置的顶点布局
    pub fn positionOnly() VertexBufferLayout {
        var layout = VertexBufferLayout{
            .attributes = undefined,
            .attribute_count = 1,
            .stride = 12, // 3 * 4 = 12 bytes
            .step_mode = .Vertex,
        };

        layout.attributes[0] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 0,
            .shader_location = 0,
        };

        return layout;
    }

    /// 位置和法线的顶点布局
    pub fn positionNormal() VertexBufferLayout {
        var layout = VertexBufferLayout{
            .attributes = undefined,
            .attribute_count = 2,
            .stride = 24, // (3 + 3) * 4 = 24 bytes
            .step_mode = .Vertex,
        };

        layout.attributes[0] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 0,
            .shader_location = 0,
        };

        layout.attributes[1] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 12,
            .shader_location = 1,
        };

        return layout;
    }

    /// 位置和UV的顶点布局
    pub fn positionUv() VertexBufferLayout {
        var layout = VertexBufferLayout{
            .attributes = undefined,
            .attribute_count = 2,
            .stride = 20, // (3 + 2) * 4 = 20 bytes
            .step_mode = .Vertex,
        };

        layout.attributes[0] = VertexAttributeDesc{
            .format = .Float32x3,
            .offset = 0,
            .shader_location = 0,
        };

        layout.attributes[1] = VertexAttributeDesc{
            .format = .Float32x2,
            .offset = 12,
            .shader_location = 2,
        };

        return layout;
    }
};

// ============================================================================
// FFI Exports
// ============================================================================

export fn vertex_layout_standard() VertexBufferLayout {
    return VertexBufferLayout.standard();
}

export fn vertex_layout_position_only() VertexBufferLayout {
    return VertexBufferLayout.positionOnly();
}

export fn vertex_layout_position_normal() VertexBufferLayout {
    return VertexBufferLayout.positionNormal();
}

export fn vertex_layout_position_uv() VertexBufferLayout {
    return VertexBufferLayout.positionUv();
}

export fn vertex_layout_stride(layout: *const VertexBufferLayout) u32 {
    return layout.stride;
}

export fn vertex_layout_attribute_count(layout: *const VertexBufferLayout) u32 {
    return layout.attribute_count;
}


// ============================================================================
// Mesh Utils (from zig/mesh_utils.zig)
// ============================================================================


// ============================================================================
// Bounding Box
// ============================================================================

pub const BoundingBox = extern struct {
    min: [3]f32,
    max: [3]f32,

    /// 获取包围盒中心点
    pub fn center(self: *const BoundingBox) [3]f32 {
        return [_]f32{
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        };
    }

    /// 获取包围盒尺寸
    pub fn size(self: *const BoundingBox) [3]f32 {
        return [_]f32{
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        };
    }
};
pub const Vec3 = extern struct {
    x: f32,
    y: f32,
    z: f32,
};

// ============================================================================
// Mesh Utilities
// ============================================================================

pub const MeshUtils = struct {
    /// 计算网格的包围盒
    pub fn calculateBounds(mesh: *const Mesh) BoundingBox {
        if (mesh.vertex_count == 0) {
            return BoundingBox{
                .min = [_]f32{ 0.0, 0.0, 0.0 },
                .max = [_]f32{ 0.0, 0.0, 0.0 },
            };
        }

        var bounds = BoundingBox{
            .min = mesh.vertices[0].position,
            .max = mesh.vertices[0].position,
        };

        var i: u32 = 1;
        while (i < mesh.vertex_count) : (i += 1) {
            const pos = mesh.vertices[i].position;

            bounds.min[0] = @min(bounds.min[0], pos[0]);
            bounds.min[1] = @min(bounds.min[1], pos[1]);
            bounds.min[2] = @min(bounds.min[2], pos[2]);

            bounds.max[0] = @max(bounds.max[0], pos[0]);
            bounds.max[1] = @max(bounds.max[1], pos[1]);
            bounds.max[2] = @max(bounds.max[2], pos[2]);
        }

        return bounds;
    }

    /// 合并多个网格到一个输出网格
    pub fn mergeMeshes(meshes: []const Mesh, output: *Mesh) !void {
        output.vertex_count = 0;
        output.index_count = 0;

        for (meshes) |mesh| {
            const vertex_offset = output.vertex_count;

            // 复制顶点
            var i: u32 = 0;
            while (i < mesh.vertex_count) : (i += 1) {
                if (output.vertex_count >= 4096) {
                    return error.VertexBufferFull;
                }
                output.vertices[output.vertex_count] = mesh.vertices[i];
                output.vertex_count += 1;
            }

            // 复制索引（需要偏移）
            i = 0;
            while (i < mesh.index_count) : (i += 1) {
                if (output.index_count >= 8192) {
                    return error.IndexBufferFull;
                }
                output.indices[output.index_count] = mesh.indices[i] + vertex_offset;
                output.index_count += 1;
            }
        }
    }

    /// 变换网格（应用4x4变换矩阵）
    pub fn transformMesh(mesh: *Mesh, matrix: [16]f32) void {
        var i: u32 = 0;
        while (i < mesh.vertex_count) : (i += 1) {
            const v = &mesh.vertices[i];
            const pos = v.position;

            // 应用变换矩阵到位置
            v.position[0] = matrix[0] * pos[0] + matrix[4] * pos[1] + matrix[8] * pos[2] + matrix[12];
            v.position[1] = matrix[1] * pos[0] + matrix[5] * pos[1] + matrix[9] * pos[2] + matrix[13];
            v.position[2] = matrix[2] * pos[0] + matrix[6] * pos[1] + matrix[10] * pos[2] + matrix[14];

            // 应用变换矩阵到法线（只旋转部分，不平移）
            const norm = v.normal;
            v.normal[0] = matrix[0] * norm[0] + matrix[4] * norm[1] + matrix[8] * norm[2];
            v.normal[1] = matrix[1] * norm[0] + matrix[5] * norm[1] + matrix[9] * norm[2];
            v.normal[2] = matrix[2] * norm[0] + matrix[6] * norm[1] + matrix[10] * norm[2];

            // 归一化法线
            const len = @sqrt(v.normal[0] * v.normal[0] + v.normal[1] * v.normal[1] + v.normal[2] * v.normal[2]);
            if (len > 0.0001) {
                v.normal[0] /= len;
                v.normal[1] /= len;
                v.normal[2] /= len;
            }
        }
    }

    /// 反转所有法线
    pub fn invertNormals(mesh: *Mesh) void {
        var i: u32 = 0;
        while (i < mesh.vertex_count) : (i += 1) {
            mesh.vertices[i].normal[0] = -mesh.vertices[i].normal[0];
            mesh.vertices[i].normal[1] = -mesh.vertices[i].normal[1];
            mesh.vertices[i].normal[2] = -mesh.vertices[i].normal[2];
        }

        // 反转三角形绕序
        i = 0;
        while (i + 2 < mesh.index_count) : (i += 3) {
            const temp = mesh.indices[i];
            mesh.indices[i] = mesh.indices[i + 2];
            mesh.indices[i + 2] = temp;
        }
    }

    /// 生成线框网格
    pub fn generateWireframe(mesh: *const Mesh, output: *Mesh) !void {
        output.vertex_count = 0;
        output.index_count = 0;
        output.primitive_topology = .LineList;

        // 复制所有顶点
        var i: u32 = 0;
        while (i < mesh.vertex_count) : (i += 1) {
            if (output.vertex_count >= 4096) {
                return error.VertexBufferFull;
            }
            output.vertices[output.vertex_count] = mesh.vertices[i];
            output.vertex_count += 1;
        }

        // 为每个三角形创建3条线
        i = 0;
        while (i + 2 < mesh.index_count) : (i += 3) {
            const idx0 = mesh.indices[i];
            const idx1 = mesh.indices[i + 1];
            const idx2 = mesh.indices[i + 2];

            // 线 0-1
            try output.addIndex(idx0);
            try output.addIndex(idx1);

            // 线 1-2
            try output.addIndex(idx1);
            try output.addIndex(idx2);

            // 线 2-0
            try output.addIndex(idx2);
            try output.addIndex(idx0);
        }
    }
};

// ============================================================================
// FFI Exports
// ============================================================================

export fn mesh_utils_calculate_bounds(mesh: *const Mesh) BoundingBox {
    return MeshUtils.calculateBounds(mesh);
}

export fn mesh_utils_merge_meshes(meshes: [*]const Mesh, count: u32, output: *Mesh) bool {
    const slice = meshes[0..count];
    MeshUtils.mergeMeshes(slice, output) catch return false;
    return true;
}

export fn mesh_utils_transform_mesh(mesh: *Mesh, matrix: *const [16]f32) void {
    MeshUtils.transformMesh(mesh, matrix.*);
}

export fn mesh_utils_invert_normals(mesh: *Mesh) void {
    MeshUtils.invertNormals(mesh);
}

export fn mesh_utils_generate_wireframe(mesh: *const Mesh, output: *Mesh) bool {
    MeshUtils.generateWireframe(mesh, output) catch return false;
    return true;
}

export fn bounding_box_center(bbox: *const BoundingBox) Vec3 {
    const c = bbox.center();
    return Vec3{ .x = c[0], .y = c[1], .z = c[2] };
}

export fn bounding_box_size(bbox: *const BoundingBox) Vec3 {
    const s = bbox.size();
    return Vec3{ .x = s[0], .y = s[1], .z = s[2] };
}

