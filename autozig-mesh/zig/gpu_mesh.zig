const std = @import("std");
const mesh_mod = @import("mesh.zig");
const vertex_mod = @import("vertex.zig");
const Mesh = mesh_mod.Mesh;
const Vertex = vertex_mod.Vertex;

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
