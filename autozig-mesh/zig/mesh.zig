const std = @import("std");
const vertex = @import("vertex.zig");
const Vertex = vertex.Vertex;

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
