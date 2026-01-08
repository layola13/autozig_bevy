const std = @import("std");
const mesh_mod = @import("mesh.zig");
const vertex_mod = @import("vertex.zig");
const Mesh = mesh_mod.Mesh;
const Vertex = vertex_mod.Vertex;

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
