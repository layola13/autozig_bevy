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
#[derive(Debug, Clone, Copy)]
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


// ============================================================================
// Bevy Mesh API Compatibility Layer - 56 Missing Types
// ============================================================================

// ============================================================================
// Enumerations (15 types)
// ============================================================================

/// Capsule UV Profile
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CapsuleUvProfile {
    /// UV profile for uniform mapping
    Uniform = 0,
    /// UV profile for fixed mapping
    Fixed = 1,
}

/// Circular Mesh UV Mode
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CircularMeshUvMode {
    /// Map UVs to unit square
    Uniform = 0,
    /// Stretch UVs along the perimeter
    Stretched = 1,
}

/// Cone Anchor
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConeAnchor {
    /// Anchor at the tip
    Tip = 0,
    /// Anchor at the base
    Base = 1,
    /// Anchor at the center
    Center = 2,
}

/// Cylinder Anchor
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CylinderAnchor {
    /// Anchor at the top
    Top = 0,
    /// Anchor at the bottom
    Bottom = 1,
    /// Anchor at the center
    Center = 2,
}

/// Generate Tangents Error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenerateTangentsError {
    /// Missing positions
    MissingPositions = 0,
    /// Missing normals
    MissingNormals = 1,
    /// Missing UVs
    MissingUvs = 2,
    /// Invalid mesh topology
    InvalidTopology = 3,
}

/// Icosphere Error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IcosphereError {
    /// Subdivision level too high
    SubdivisionTooHigh = 0,
    /// Invalid radius
    InvalidRadius = 1,
}

/// Mesh indices
#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Indices {
    /// 16-bit indices
    U16(Vec<u16>),
    /// 32-bit indices
    U32(Vec<u32>),
}

/// Mesh Access Error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeshAccessError {
    /// Attribute not found
    AttributeNotFound = 0,
    /// Invalid attribute format
    InvalidFormat = 1,
}

/// Mesh Merge Error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeshMergeError {
    /// Incompatible vertex formats
    IncompatibleFormats = 0,
    /// Buffer overflow
    BufferOverflow = 1,
}

/// Mesh Triangles Error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeshTrianglesError {
    /// Missing indices
    MissingIndices = 0,
    /// Invalid topology
    InvalidTopology = 1,
}

/// Mesh Winding Invert Error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MeshWindingInvertError {
    /// Missing indices
    MissingIndices = 0,
    /// Invalid topology
    InvalidTopology = 1,
}

/// Morph Build Error
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MorphBuildError {
    /// Mismatched vertex count
    MismatchedVertexCount = 0,
    /// Too many morph targets
    TooManyTargets = 1,
}

/// Perimeter Segment
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PerimeterSegment {
    /// Straight segment
    Straight = 0,
    /// Arc segment
    Arc = 1,
}

/// Sphere Kind
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SphereKind {
    /// Icosphere (geodesic)
    Ico = 0,
    /// UV sphere (latitude-longitude)
    Uv = 1,
}

/// Vertex Attribute Values - all possible vertex attribute formats
#[derive(Debug, Clone, PartialEq)]
pub enum VertexAttributeValues {
    Float32(Vec<f32>),
    Float32x2(Vec<[f32; 2]>),
    Float32x3(Vec<[f32; 3]>),
    Float32x4(Vec<[f32; 4]>),
    Sint8x2(Vec<[i8; 2]>),
    Sint8x4(Vec<[i8; 4]>),
    Uint8x2(Vec<[u8; 2]>),
    Uint8x4(Vec<[u8; 4]>),
    Sint16x2(Vec<[i16; 2]>),
    Sint16x4(Vec<[i16; 4]>),
    Uint16x2(Vec<[u16; 2]>),
    Uint16x4(Vec<[u16; 4]>),
    Sint32(Vec<i32>),
    Sint32x2(Vec<[i32; 2]>),
    Sint32x3(Vec<[i32; 3]>),
    Sint32x4(Vec<[i32; 4]>),
    Uint32(Vec<u32>),
    Uint32x2(Vec<[u32; 2]>),
    Uint32x3(Vec<[u32; 3]>),
    Uint32x4(Vec<[u32; 4]>),
    Snorm8x2(Vec<[i8; 2]>),
    Snorm8x4(Vec<[i8; 4]>),
    Unorm8x2(Vec<[u8; 2]>),
    Unorm8x4(Vec<[u8; 4]>),
    Snorm16x2(Vec<[i16; 2]>),
    Snorm16x4(Vec<[i16; 4]>),
    Unorm16x2(Vec<[u16; 2]>),
    Unorm16x4(Vec<[u16; 4]>),
}

// ============================================================================
// Struct Types (42 types)
// ============================================================================

/// Annulus Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnnulusMeshBuilder {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub segments: u32,
}

/// Base Mesh Pipeline Key
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BaseMeshPipelineKey {
    pub bits: u32,
}

/// Capsule 2D Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Capsule2dMeshBuilder {
    pub radius: f32,
    pub half_length: f32,
    pub segments: u32,
}

/// Capsule 3D Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Capsule3dMeshBuilder {
    pub radius: f32,
    pub half_length: f32,
    pub rings: u32,
    pub segments: u32,
    pub uv_profile: CapsuleUvProfile,
}

/// Circle Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleMeshBuilder {
    pub radius: f32,
    pub segments: u32,
}

/// Circular Sector Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircularSectorMeshBuilder {
    pub radius: f32,
    pub angle: f32,
    pub segments: u32,
}

/// Circular Segment Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircularSegmentMeshBuilder {
    pub radius: f32,
    pub angle: f32,
    pub segments: u32,
}

/// Cone Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConeMeshBuilder {
    pub radius: f32,
    pub height: f32,
    pub segments: u32,
    pub anchor: ConeAnchor,
}

/// Conical Frustum Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConicalFrustumMeshBuilder {
    pub radius_top: f32,
    pub radius_bottom: f32,
    pub height: f32,
    pub segments: u32,
}

/// Convex Polygon Mesh Builder
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ConvexPolygonMeshBuilder {
    // Using a pointer to Vec for FFI compatibility
    points_ptr: *const [f32; 2],
    points_len: usize,
}

/// Cuboid Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CuboidMeshBuilder {
    pub half_size: [f32; 3],
}

/// Cylinder Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CylinderMeshBuilder {
    pub radius: f32,
    pub height: f32,
    pub segments: u32,
    pub anchor: CylinderAnchor,
}

/// Ellipse Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EllipseMeshBuilder {
    pub half_size: [f32; 2],
    pub segments: u32,
}

/// Extrusion Builder
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ExtrusionBuilder {
    // Using pointers for FFI compatibility
    base_shape_ptr: *const u8,
    base_shape_len: usize,
    pub depth: f32,
}

/// From Vertex Attribute Error
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FromVertexAttributeError {
    pub error_code: u32,
}

/// Inherit Weight Systems
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InheritWeightSystems {
    pub bits: u32,
}

/// Mesh 2D marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mesh2d;

/// Mesh 3D marker
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mesh3d;

/// Mesh Deserializer
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshDeserializer {
    // Opaque pointer for deserializer state
    state: *mut std::ffi::c_void,
}

/// Mesh Morph Weights
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshMorphWeights {
    // Pointer to weights array
    weights_ptr: *const f32,
    weights_len: usize,
}

/// Mesh Plugin
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshPlugin;

/// Mesh Tag
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshTag {
    pub id: u64,
}

/// Mesh Vertex Attribute
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeshVertexAttribute {
    pub id: MeshVertexAttributeId,
    pub descriptor: VertexAttributeDescriptor,
}

/// Mesh Vertex Attribute ID
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MeshVertexAttributeId {
    pub id: u64,
}

/// Mesh Vertex Buffer Layout (alias for VertexBufferLayout)
pub type MeshVertexBufferLayout = VertexBufferLayout;

/// Mesh Vertex Buffer Layout Reference
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MeshVertexBufferLayoutRef<'a> {
    layout: &'a VertexBufferLayout,
}

/// Mesh Vertex Buffer Layouts
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MeshVertexBufferLayouts {
    layouts_ptr: *const VertexBufferLayout,
    layouts_len: usize,
}

/// Missing Vertex Attribute Error
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissingVertexAttributeError {
    pub attribute_id: MeshVertexAttributeId,
}

/// Morph Attributes
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MorphAttributes {
    // Pointer to attribute data
    data_ptr: *const u8,
    data_len: usize,
}

/// Morph Target Image
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MorphTargetImage {
    // Opaque pointer to image data
    image_ptr: *mut std::ffi::c_void,
}

/// Morph Weights
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MorphWeights {
    weights_ptr: *const f32,
    weights_len: usize,
}

/// Plane Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlaneMeshBuilder {
    pub half_size: [f32; 2],
    pub subdivisions: [u32; 2],
}

/// Polyline 2D Mesh Builder
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Polyline2dMeshBuilder {
    points_ptr: *const [f32; 2],
    points_len: usize,
}

/// Polyline 3D Mesh Builder
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Polyline3dMeshBuilder {
    points_ptr: *const [f32; 3],
    points_len: usize,
}

/// Rectangle Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectangleMeshBuilder {
    pub half_size: [f32; 2],
}

/// Regular Polygon Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RegularPolygonMeshBuilder {
    pub circumradius: f32,
    pub sides: u32,
}

/// Rhombus Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RhombusMeshBuilder {
    pub half_diagonals: [f32; 2],
}

/// Ring Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RingMeshBuilder {
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub segments: u32,
}

/// Segment 2D Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment2dMeshBuilder {
    pub start: [f32; 2],
    pub end: [f32; 2],
}

/// Segment 3D Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment3dMeshBuilder {
    pub start: [f32; 3],
    pub end: [f32; 3],
}

/// Serialized Mesh
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SerializedMesh {
    data_ptr: *const u8,
    data_len: usize,
}

/// Skinned Mesh
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SkinnedMesh {
    inverse_bindposes_ptr: *const [f32; 16],
    inverse_bindposes_len: usize,
    joints_ptr: *const u32,
    joints_len: usize,
}

/// Skinned Mesh Inverse Bindposes
#[repr(C)]
#[derive(Debug, Clone)]
pub struct SkinnedMeshInverseBindposes {
    matrices_ptr: *const [f32; 16],
    matrices_len: usize,
}

/// Sphere Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SphereMeshBuilder {
    pub radius: f32,
    pub sectors: u32,
    pub stacks: u32,
    pub kind: SphereKind,
}

/// Tetrahedron Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TetrahedronMeshBuilder {
    pub radius: f32,
}

/// Torus Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TorusMeshBuilder {
    pub major_radius: f32,
    pub minor_radius: f32,
    pub major_segments: u32,
    pub minor_segments: u32,
}

/// Triangle 2D Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle2dMeshBuilder {
    pub vertices: [[f32; 2]; 3],
}

/// Triangle 3D Mesh Builder
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle3dMeshBuilder {
    pub vertices: [[f32; 3]; 3],
}

/// Vertex Attribute Descriptor
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VertexAttributeDescriptor {
    pub format: VertexFormat,
    pub offset: u64,
    pub shader_location: u32,
}

// ============================================================================
// Trait Definitions (3 types)
// ============================================================================

/// Trait for types that can be extruded into 3D meshes
pub trait Extrudable {
    /// Extrude the shape along the Z axis
    fn extrude(&self, depth: f32) -> Mesh;
}

/// Trait for building meshes
pub trait MeshBuilder {
    /// Build the mesh
    fn build(&self) -> Mesh;
}

/// Trait for types that can be converted into meshes
pub trait Meshable {
    /// Convert to a mesh
    fn mesh(&self) -> Mesh;
}

// ============================================================================
// Trait Implementations
// ============================================================================

impl Default for AnnulusMeshBuilder {
    fn default() -> Self {
        Self {
            inner_radius: 0.5,
            outer_radius: 1.0,
            segments: 32,
        }
    }
}

impl Default for Capsule2dMeshBuilder {
    fn default() -> Self {
        Self {
            radius: 0.5,
            half_length: 0.5,
            segments: 16,
        }
    }
}

impl Default for Capsule3dMeshBuilder {
    fn default() -> Self {
        Self {
            radius: 0.5,
            half_length: 0.5,
            rings: 8,
            segments: 16,
            uv_profile: CapsuleUvProfile::Uniform,
        }
    }
}

impl Default for CircleMeshBuilder {
    fn default() -> Self {
        Self {
            radius: 1.0,
            segments: 32,
        }
    }
}

impl Default for CircularSectorMeshBuilder {
    fn default() -> Self {
        Self {
            radius: 1.0,
            angle: std::f32::consts::PI / 2.0,
            segments: 16,
        }
    }
}

impl Default for CircularSegmentMeshBuilder {
    fn default() -> Self 
{
        Self {
            radius: 1.0,
            angle: std::f32::consts::PI / 4.0,
            segments: 16,
        }
    }
}

impl Default for ConeMeshBuilder {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 2.0,
            segments: 32,
            anchor: ConeAnchor::Base,
        }
    }
}

impl Default for ConicalFrustumMeshBuilder {
    fn default() -> Self {
        Self {
            radius_top: 0.5,
            radius_bottom: 1.0,
            height: 2.0,
            segments: 32,
        }
    }
}

impl Default for CuboidMeshBuilder {
    fn default() -> Self {
        Self {
            half_size: [0.5, 0.5, 0.5],
        }
    }
}

impl Default for CylinderMeshBuilder {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 2.0,
            segments: 32,
            anchor: CylinderAnchor::Center,
        }
    }
}

impl Default for EllipseMeshBuilder {
    fn default() -> Self {
        Self {
            half_size: [1.0, 0.5],
            segments: 32,
        }
    }
}

impl Default for PlaneMeshBuilder {
    fn default() -> Self {
        Self {
            half_size: [1.0, 1.0],
            subdivisions: [1, 1],
        }
    }
}

impl Default for RectangleMeshBuilder {
    fn default() -> Self {
        Self {
            half_size: [1.0, 1.0],
        }
    }
}

impl Default for RegularPolygonMeshBuilder {
    fn default() -> Self {
        Self {
            circumradius: 1.0,
            sides: 6,
        }
    }
}

impl Default for RhombusMeshBuilder {
    fn default() -> Self {
        Self {
            half_diagonals: [1.0, 1.0],
        }
    }
}

impl Default for RingMeshBuilder {
    fn default() -> Self {
        Self {
            inner_radius: 0.5,
            outer_radius: 1.0,
            segments: 32,
        }
    }
}

impl Default for SphereMeshBuilder {
    fn default() -> Self {
        Self {
            radius: 1.0,
            sectors: 36,
            stacks: 18,
            kind: SphereKind::Uv,
        }
    }
}

impl Default for TetrahedronMeshBuilder {
    fn default() -> Self {
        Self {
            radius: 1.0,
        }
    }
}

impl Default for TorusMeshBuilder {
    fn default() -> Self {
        Self {
            major_radius: 1.0,
            minor_radius: 0.25,
            major_segments: 32,
            minor_segments: 16,
        }
    }
}

impl Default for Triangle2dMeshBuilder {
    fn default() -> Self {
        Self {
            vertices: [[0.0, 1.0], [-1.0, -1.0], [1.0, -1.0]],
        }
    }
}

impl Default for Triangle3dMeshBuilder {
    fn default() -> Self {
        Self {
            vertices: [
                [0.0, 1.0, 0.0],
                [-1.0, -1.0, 0.0],
                [1.0, -1.0, 0.0],
            ],
        }
    }
}

// ============================================================================
// MeshBuilder Trait Implementations for all builders
// ============================================================================

impl MeshBuilder for SphereMeshBuilder {
    fn build(&self) -> Mesh {
        primitives_sphere(self.radius, self.sectors, self.stacks)
    }
}

impl MeshBuilder for CuboidMeshBuilder {
    fn build(&self) -> Mesh {
        let size = self.half_size[0] * 2.0;
        primitives_cube(size)
    }
}

impl MeshBuilder for CylinderMeshBuilder {
    fn build(&self) -> Mesh {
        primitives_cylinder(self.radius, self.height, self.segments)
    }
}

impl MeshBuilder for ConeMeshBuilder {
    fn build(&self) -> Mesh {
        primitives_cone(self.radius, self.height, self.segments)
    }
}

impl MeshBuilder for TorusMeshBuilder {
    fn build(&self) -> Mesh {
        primitives_torus(
            self.major_radius,
            self.minor_radius,
            self.major_segments,
            self.minor_segments,
        )
    }
}

impl MeshBuilder for Capsule3dMeshBuilder {
    fn build(&self) -> Mesh {
        primitives_capsule(self.radius, self.half_length * 2.0, self.rings, self.segments)
    }
}

impl MeshBuilder for PlaneMeshBuilder {
    fn build(&self) -> Mesh {
        primitives_plane(
            self.half_size[0] * 2.0,
            self.half_size[1] * 2.0,
            self.subdivisions[0],
            self.subdivisions[1],
        )
    }
}

// Additional utility implementations

impl Indices {
    /// Get the number of indices
    pub fn len(&self) -> usize {
        match self {
            Indices::U16(v) => v.len(),
            Indices::U32(v) => v.len(),
        }
    }

    /// Check if indices are empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl VertexAttributeValues {
    /// Get the number of vertices
    pub fn len(&self) -> usize {
        match self {
            VertexAttributeValues::Float32(v) => v.len(),
            VertexAttributeValues::Float32x2(v) => v.len(),
            VertexAttributeValues::Float32x3(v) => v.len(),
            VertexAttributeValues::Float32x4(v) => v.len(),
            VertexAttributeValues::Sint8x2(v) => v.len(),
            VertexAttributeValues::Sint8x4(v) => v.len(),
            VertexAttributeValues::Uint8x2(v) => v.len(),
            VertexAttributeValues::Uint8x4(v) => v.len(),
            VertexAttributeValues::Sint16x2(v) => v.len(),
            VertexAttributeValues::Sint16x4(v) => v.len(),
            VertexAttributeValues::Uint16x2(v) => v.len(),
            VertexAttributeValues::Uint16x4(v) => v.len(),
            VertexAttributeValues::Sint32(v) => v.len(),
            VertexAttributeValues::Sint32x2(v) => v.len(),
            VertexAttributeValues::Sint32x3(v) => v.len(),
            VertexAttributeValues::Sint32x4(v) => v.len(),
            VertexAttributeValues::Uint32(v) => v.len(),
            VertexAttributeValues::Uint32x2(v) => v.len(),
            VertexAttributeValues::Uint32x3(v) => v.len(),
            VertexAttributeValues::Uint32x4(v) => v.len(),
            VertexAttributeValues::Snorm8x2(v) => v.len(),
            VertexAttributeValues::Snorm8x4(v) => v.len(),
            VertexAttributeValues::Unorm8x2(v) => v.len(),
            VertexAttributeValues::Unorm8x4(v) => v.len(),
            VertexAttributeValues::Snorm16x2(v) => v.len(),
            VertexAttributeValues::Snorm16x4(v) => v.len(),
            VertexAttributeValues::Unorm16x2(v) => v.len(),
            VertexAttributeValues::Unorm16x4(v) => v.len(),
        }
    }

    /// Check if values are empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}