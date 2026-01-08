//! # AutoZig Mesh - WebGPU Mesh System
//!
//! 90% Zig实现，10% Rust FFI包装
//!
//! 提供以下核心功能：
//! - Vertex: 顶点数据结构
//! - Mesh: 网格数据管理（固定大小数组：4096顶点，8192索引）
//! - MeshPrimitives: 7种基本几何体生成（cube, sphere, plane, cylinder, cone, torus, capsule）
//! - GpuMesh: GPU缓冲区管理
//! - VertexLayout: 顶点布局描述
//! - MeshUtils: 网格工具函数（bounds, merge, transform, wireframe等）

use autozig::include_zig;

// ============================================================================
// Vertex Types
// ============================================================================

/// 顶点数据结构
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
    pub tangent: [f32; 4], // xyz + handedness
    pub color: [f32; 4],
}

/// 顶点属性枚举
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexAttribute {
    Position = 0,
    Normal = 1,
    Uv = 2,
    Tangent = 3,
    Color = 4,
}

// ============================================================================
// Mesh Types
// ============================================================================

/// 图元拓扑类型
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
}

/// 网格数据结构
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Mesh {
    pub vertices: [Vertex; 4096],
    pub vertex_count: u32,
    pub indices: [u32; 8192],
    pub index_count: u32,
    pub primitive_topology: PrimitiveTopology,
}

// ============================================================================
// GPU Mesh Types
// ============================================================================

/// GPU网格缓冲区
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GpuMesh {
    pub vertex_buffer: Option<*mut std::ffi::c_void>,
    pub index_buffer: Option<*mut std::ffi::c_void>,
    pub vertex_count: u32,
    pub index_count: u32,
}

// ============================================================================
// Vertex Layout Types
// ============================================================================

/// 顶点格式
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    Float32x2 = 0,
    Float32x3 = 1,
    Float32x4 = 2,
    Uint32 = 3,
    Uint32x2 = 4,
}

/// 顶点步进模式
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexStepMode {
    Vertex = 0,
    Instance = 1,
}

/// 顶点属性描述
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VertexAttributeDesc {
    pub format: VertexFormat,
    pub offset: u32,
    pub shader_location: u32,
}

/// 顶点缓冲区布局
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VertexBufferLayout {
    pub attributes: [VertexAttributeDesc; 8],
    pub attribute_count: u32,
    pub stride: u32,
    pub step_mode: VertexStepMode,
}

// ============================================================================
// Mesh Utils Types
// ============================================================================

/// Vec3 辅助类型
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn to_array(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

/// 包围盒
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

// ============================================================================
// Zig FFI Imports
// ============================================================================

include_zig!("zig/mesh_all.zig", {
    // Vertex functions
    fn vertex_init() -> Vertex;
    fn vertex_with_position(x: f32, y: f32, z: f32) -> Vertex;
    fn vertex_with_position_normal(px: f32, py: f32, pz: f32, nx: f32, ny: f32, nz: f32) -> Vertex;
    fn vertex_with_position_normal_uv(px: f32, py: f32, pz: f32, nx: f32, ny: f32, nz: f32, u: f32, v: f32) -> Vertex;
    
    // Mesh functions
    fn mesh_init() -> Mesh;
    fn mesh_add_vertex(mesh: *mut Mesh, vertex: Vertex) -> bool;
    fn mesh_add_index(mesh: *mut Mesh, index: u32) -> bool;
    fn mesh_add_triangle(mesh: *mut Mesh, idx0: u32, idx1: u32, idx2: u32) -> bool;
    fn mesh_calculate_normals(mesh: *mut Mesh);
    fn mesh_calculate_tangents(mesh: *mut Mesh);
    fn mesh_vertex_count(mesh: *const Mesh) -> u32;
    fn mesh_index_count(mesh: *const Mesh) -> u32;
    fn mesh_set_topology(mesh: *mut Mesh, topology: PrimitiveTopology);
    
    // Primitives functions
    fn primitives_cube(size: f32) -> Mesh;
    fn primitives_sphere(radius: f32, segments: u32, rings: u32) -> Mesh;
    fn primitives_plane(width: f32, height: f32, subdivisions_x: u32, subdivisions_z: u32) -> Mesh;
    fn primitives_cylinder(radius: f32, height: f32, segments: u32) -> Mesh;
    fn primitives_cone(radius: f32, height: f32, segments: u32) -> Mesh;
    fn primitives_torus(major_radius: f32, minor_radius: f32, major_segments: u32, minor_segments: u32) -> Mesh;
    fn primitives_capsule(radius: f32, height: f32, rings: u32, segments: u32) -> Mesh;
    
    // GPU Mesh functions
    fn gpu_mesh_from_mesh(device: *mut std::ffi::c_void, mesh: *const Mesh) -> GpuMesh;
    fn gpu_mesh_update_vertices(gpu_mesh: *mut GpuMesh, queue: *mut std::ffi::c_void, vertices: *const Vertex, count: u32) -> bool;
    fn gpu_mesh_update_indices(gpu_mesh: *mut GpuMesh, queue: *mut std::ffi::c_void, indices: *const u32, count: u32) -> bool;
    fn gpu_mesh_destroy(gpu_mesh: *mut GpuMesh);
    fn gpu_mesh_vertex_count(gpu_mesh: *const GpuMesh) -> u32;
    fn gpu_mesh_index_count(gpu_mesh: *const GpuMesh) -> u32;
    
    // Vertex Layout functions
    fn vertex_layout_standard() -> VertexBufferLayout;
    fn vertex_layout_position_only() -> VertexBufferLayout;
    fn vertex_layout_position_normal() -> VertexBufferLayout;
    fn vertex_layout_position_uv() -> VertexBufferLayout;
    fn vertex_layout_stride(layout: *const VertexBufferLayout) -> u32;
    fn vertex_layout_attribute_count(layout: *const VertexBufferLayout) -> u32;
    
    // Mesh Utils functions
    fn mesh_utils_calculate_bounds(mesh: *const Mesh) -> BoundingBox;
    fn mesh_utils_merge_meshes(meshes: *const Mesh, count: u32, output: *mut Mesh) -> bool;
    fn mesh_utils_transform_mesh(mesh: *mut Mesh, matrix: [f32; 16]);
    fn mesh_utils_invert_normals(mesh: *mut Mesh);
    fn mesh_utils_generate_wireframe(mesh: *const Mesh, output: *mut Mesh) -> bool;
    fn bounding_box_center(bbox: *const BoundingBox) -> Vec3;
    fn bounding_box_size(bbox: *const BoundingBox) -> Vec3;
});

// ============================================================================
// Rust API Implementations
// ============================================================================

impl Vertex {
    pub fn new() -> Self {
        vertex_init()
    }

    pub fn with_position(x: f32, y: f32, z: f32) -> Self {
        vertex_with_position(x, y, z)
    }

    pub fn with_position_normal(px: f32, py: f32, pz: f32, nx: f32, ny: f32, nz: f32) -> Self {
        vertex_with_position_normal(px, py, pz, nx, ny, nz)
    }

    pub fn with_position_normal_uv(
        px: f32,
        py: f32,
        pz: f32,
        nx: f32,
        ny: f32,
        nz: f32,
        u: f32,
        v: f32,
    ) -> Self {
        vertex_with_position_normal_uv(px, py, pz, nx, ny, nz, u, v)
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self::new()
    }
}

impl Mesh {
    pub fn new() -> Self {
        mesh_init()
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> Result<(), &'static str> {
        if mesh_add_vertex(self, vertex) {
            Ok(())
        } else {
            Err("Vertex buffer full")
        }
    }

    pub fn add_index(&mut self, index: u32) -> Result<(), &'static str> {
        if mesh_add_index(self, index) {
            Ok(())
        } else {
            Err("Index buffer full")
        }
    }

    pub fn add_triangle(&mut self, idx0: u32, idx1: u32, idx2: u32) -> Result<(), &'static str> {
        if mesh_add_triangle(self, idx0, idx1, idx2) {
            Ok(())
        } else {
            Err("Index buffer full")
        }
    }

    pub fn calculate_normals(&mut self) {
        mesh_calculate_normals(self);
    }

    pub fn calculate_tangents(&mut self) {
        mesh_calculate_tangents(self);
    }

    pub fn vertex_count(&self) -> u32 {
        mesh_vertex_count(self)
    }

    pub fn index_count(&self) -> u32 {
        mesh_index_count(self)
    }

    pub fn set_topology(&mut self, topology: PrimitiveTopology) {
        mesh_set_topology(self, topology);
    }

    pub fn calculate_bounds(&self) -> BoundingBox {
        mesh_utils_calculate_bounds(self)
    }

    pub fn transform(&mut self, matrix: [f32; 16]) {
        mesh_utils_transform_mesh(self, matrix);
    }

    pub fn invert_normals(&mut self) {
        mesh_utils_invert_normals(self);
    }

    pub fn generate_wireframe(&self) -> Result<Mesh, &'static str> {
        let mut output = Mesh::new();
        if mesh_utils_generate_wireframe(self, &mut output) {
            Ok(output)
        } else {
            Err("Failed to generate wireframe")
        }
    }
}

impl Default for Mesh {
    fn default() -> Self {
        Self::new()
    }
}

/// 几何体生成器
pub struct MeshPrimitives;

impl MeshPrimitives {
    pub fn cube(size: f32) -> Mesh {
        primitives_cube(size)
    }

    pub fn sphere(radius: f32, segments: u32, rings: u32) -> Mesh {
        primitives_sphere(radius, segments, rings)
    }

    pub fn plane(width: f32, height: f32, subdivisions_x: u32, subdivisions_z: u32) -> Mesh {
        primitives_plane(width, height, subdivisions_x, subdivisions_z)
    }

    pub fn cylinder(radius: f32, height: f32, segments: u32) -> Mesh {
        primitives_cylinder(radius, height, segments)
    }

    pub fn cone(radius: f32, height: f32, segments: u32) -> Mesh {
        primitives_cone(radius, height, segments)
    }

    pub fn torus(
        major_radius: f32,
        minor_radius: f32,
        major_segments: u32,
        minor_segments: u32,
    ) -> Mesh {
        primitives_torus(major_radius, minor_radius, major_segments, minor_segments)
    }

    pub fn capsule(radius: f32, height: f32, rings: u32, segments: u32) -> Mesh {
        primitives_capsule(radius, height, rings, segments)
    }
}

impl GpuMesh {
    pub fn from_mesh(device: *mut std::ffi::c_void, mesh: &Mesh) -> Self {
        gpu_mesh_from_mesh(device, mesh)
    }

    pub fn update_vertices(
        &mut self,
        queue: *mut std::ffi::c_void,
        vertices: &[Vertex],
    ) -> Result<(), &'static str> {
        if gpu_mesh_update_vertices(self, queue, vertices.as_ptr(), vertices.len() as u32) {
            Ok(())
        } else {
            Err("Failed to update vertices")
        }
    }

    pub fn update_indices(
        &mut self,
        queue: *mut std::ffi::c_void,
        indices: &[u32],
    ) -> Result<(), &'static str> {
        if gpu_mesh_update_indices(self, queue, indices.as_ptr(), indices.len() as u32) {
            Ok(())
        } else {
            Err("Failed to update indices")
        }
    }

    pub fn destroy(&mut self) {
        gpu_mesh_destroy(self);
    }

    pub fn vertex_count(&self) -> u32 {
        gpu_mesh_vertex_count(self)
    }

    pub fn index_count(&self) -> u32 {
        gpu_mesh_index_count(self)
    }
}

impl VertexBufferLayout {
    pub fn standard() -> Self {
        vertex_layout_standard()
    }

    pub fn position_only() -> Self {
        vertex_layout_position_only()
    }

    pub fn position_normal() -> Self {
        vertex_layout_position_normal()
    }

    pub fn position_uv() -> Self {
        vertex_layout_position_uv()
    }

    pub fn stride(&self) -> u32 {
        vertex_layout_stride(self)
    }

    pub fn attribute_count(&self) -> u32 {
        vertex_layout_attribute_count(self)
    }
}

impl BoundingBox {
    pub fn center(&self) -> [f32; 3] {
        bounding_box_center(self).to_array()
    }

    pub fn size(&self) -> [f32; 3] {
        bounding_box_size(self).to_array()
    }
}

/// 网格工具函数
pub struct MeshUtils;

impl MeshUtils {
    pub fn calculate_bounds(mesh: &Mesh) -> BoundingBox {
        mesh_utils_calculate_bounds(mesh)
    }

    pub fn merge_meshes(meshes: &[Mesh]) -> Result<Mesh, &'static str> {
        let mut output = Mesh::new();
        if mesh_utils_merge_meshes(meshes.as_ptr(), meshes.len() as u32, &mut output) {
            Ok(output)
        } else {
            Err("Failed to merge meshes")
        }
    }

    pub fn transform_mesh(mesh: &mut Mesh, matrix: [f32; 16]) {
        mesh_utils_transform_mesh(mesh, matrix);
    }

    pub fn invert_normals(mesh: &mut Mesh) {
        mesh_utils_invert_normals(mesh);
    }

    pub fn generate_wireframe(mesh: &Mesh) -> Result<Mesh, &'static str> {
        mesh.generate_wireframe()
    }
}