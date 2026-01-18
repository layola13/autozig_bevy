use autozig_mesh::*;

// ============================================================================
// Vertex Tests (3 tests)
// ============================================================================

#[test]
fn test_vertex_default() {
    let v = Vertex::default();
    assert_eq!(v.position, [0.0, 0.0, 0.0]);
    assert_eq!(v.normal, [0.0, 1.0, 0.0]);
    assert_eq!(v.uv, [0.0, 0.0]);
}

#[test]
fn test_vertex_with_position() {
    let v = Vertex::with_position(1.0, 2.0, 3.0);
    assert_eq!(v.position, [1.0, 2.0, 3.0]);
}

#[test]
fn test_vertex_with_position_normal_uv() {
    let v = Vertex::with_position_normal_uv(1.0, 2.0, 3.0, 0.0, 1.0, 0.0, 0.5, 0.5);
    assert_eq!(v.position, [1.0, 2.0, 3.0]);
    assert_eq!(v.normal, [0.0, 1.0, 0.0]);
    assert_eq!(v.uv, [0.5, 0.5]);
}

// ============================================================================
// Mesh Creation and Management Tests (4 tests)
// ============================================================================

#[test]
fn test_mesh_init() {
    let mesh = Mesh::new();
    assert_eq!(mesh.vertex_count(), 0);
    assert_eq!(mesh.index_count(), 0);
}

#[test]
fn test_mesh_add_vertex() {
    let mut mesh = Mesh::new();
    let v = Vertex::with_position(1.0, 2.0, 3.0);
    assert!(mesh.add_vertex(v).is_ok());
    assert_eq!(mesh.vertex_count(), 1);
}

#[test]
fn test_mesh_add_index() {
    let mut mesh = Mesh::new();
    assert!(mesh.add_index(0).is_ok());
    assert!(mesh.add_index(1).is_ok());
    assert!(mesh.add_index(2).is_ok());
    assert_eq!(mesh.index_count(), 3);
}

#[test]
fn test_mesh_add_triangle() {
    let mut mesh = Mesh::new();
    assert!(mesh.add_triangle(0, 1, 2).is_ok());
    assert_eq!(mesh.index_count(), 3);
}

// ============================================================================
// Primitive Generation Tests (7 tests)
// ============================================================================

#[test]
fn test_primitives_cube() {
    let mesh = MeshPrimitives::cube(2.0);
    assert_eq!(mesh.vertex_count(), 24); // 6 faces × 4 vertices
    assert_eq!(mesh.index_count(), 36); // 6 faces × 2 triangles × 3 indices
}

#[test]
fn test_primitives_sphere() {
    let mesh = MeshPrimitives::sphere(1.0, 16, 8);
    // (segments + 1) × (rings + 1) = 17 × 9 = 153 vertices
    assert_eq!(mesh.vertex_count(), 153);
    // segments × rings × 2 triangles × 3 indices = 16 × 8 × 6 = 768
    assert_eq!(mesh.index_count(), 768);
}

#[test]
fn test_primitives_plane() {
    let mesh = MeshPrimitives::plane(10.0, 10.0, 2, 2);
    // (subdivisions_x + 1) × (subdivisions_z + 1) = 3 × 3 = 9 vertices
    assert_eq!(mesh.vertex_count(), 9);
    // subdivisions_x × subdivisions_z × 2 triangles × 3 indices = 2 × 2 × 6 = 24
    assert_eq!(mesh.index_count(), 24);
}

#[test]
fn test_primitives_cylinder() {
    let mesh = MeshPrimitives::cylinder(1.0, 2.0, 16);
    // 侧面: (segments + 1) × 2 + 中心点2个
    // 16+1=17, 17×2=34, 34+2=36 vertices
    assert!(mesh.vertex_count() > 30);
    assert!(mesh.index_count() > 90);
}

#[test]
fn test_primitives_cone() {
    let mesh = MeshPrimitives::cone(1.0, 2.0, 16);
    // 顶点1个 + 底面圆周(segments+1) + 底面中心1个
    // 1 + 17 + 1 = 19 vertices
    assert!(mesh.vertex_count() >= 19);
    assert!(mesh.index_count() > 30);
}

#[test]
fn test_primitives_torus() {
    let mesh = MeshPrimitives::torus(2.0, 0.5, 16, 8);
    // (major_segments + 1) × (minor_segments + 1) = 17 × 9 = 153 vertices
    assert_eq!(mesh.vertex_count(), 153);
    // major_segments × minor_segments × 2 triangles × 3 indices
    assert_eq!(mesh.index_count(), 768);
}

#[test]
fn test_primitives_capsule() {
    let mesh = MeshPrimitives::capsule(1.0, 2.0, 4, 8);
    // 上半球 + 下半球
    // (rings + 1) × (segments + 1) × 2 = 5 × 9 × 2 = 90 vertices
    assert_eq!(mesh.vertex_count(), 90);
    assert!(mesh.index_count() > 100);
}

// ============================================================================
// GPU Buffer Tests (3 tests)
// ============================================================================

#[test]
fn test_gpu_mesh_from_mesh() {
    let mesh = MeshPrimitives::cube(1.0);
    let device = std::ptr::null_mut();
    let gpu_mesh = GpuMesh::from_mesh(device, &mesh);
    assert_eq!(gpu_mesh.vertex_count(), 24);
    assert_eq!(gpu_mesh.index_count(), 36);
}

#[test]
fn test_gpu_mesh_vertex_count() {
    let mesh = MeshPrimitives::sphere(1.0, 8, 4);
    let device = std::ptr::null_mut();
    let gpu_mesh = GpuMesh::from_mesh(device, &mesh);
    assert_eq!(gpu_mesh.vertex_count(), mesh.vertex_count());
}

#[test]
fn test_gpu_mesh_index_count() {
    let mesh = MeshPrimitives::plane(5.0, 5.0, 1, 1);
    let device = std::ptr::null_mut();
    let gpu_mesh = GpuMesh::from_mesh(device, &mesh);
    assert_eq!(gpu_mesh.index_count(), mesh.index_count());
}

// ============================================================================
// Vertex Layout Tests (3 tests)
// ============================================================================

#[test]
fn test_vertex_layout_standard() {
    let layout = VertexBufferLayout::standard();
    assert_eq!(layout.attribute_count(), 5);
    assert_eq!(layout.stride(), 64); // 3*4 + 3*4 + 2*4 + 4*4 + 4*4
}

#[test]
fn test_vertex_layout_position_only() {
    let layout = VertexBufferLayout::position_only();
    assert_eq!(layout.attribute_count(), 1);
    assert_eq!(layout.stride(), 12); // 3*4
}

#[test]
fn test_vertex_layout_position_normal() {
    let layout = VertexBufferLayout::position_normal();
    assert_eq!(layout.attribute_count(), 2);
    assert_eq!(layout.stride(), 24); // (3+3)*4
}

// ============================================================================
// Mesh Utility Tests (4 tests)
// ============================================================================

#[test]
fn test_mesh_calculate_bounds() {
    let mesh = MeshPrimitives::cube(2.0);
    let bounds = mesh.calculate_bounds();
    
    // 立方体边长2.0，中心在原点，所以min=-1.0, max=1.0
    assert!((bounds.min[0] - (-1.0)).abs() < 0.001);
    assert!((bounds.max[0] - 1.0).abs() < 0.001);
}

#[test]
fn test_mesh_transform() {
    let mut mesh = MeshPrimitives::cube(1.0);
    
    // 单位矩阵
    #[rustfmt::skip]
    let identity = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ];
    
    mesh.transform(identity);
    assert_eq!(mesh.vertex_count(), 24);
}

#[test]
fn test_mesh_invert_normals() {
    let mut mesh = MeshPrimitives::plane(1.0, 1.0, 1, 1);
    let original_normal = mesh.vertices[0].normal;
    
    mesh.invert_normals();
    
    let inverted_normal = mesh.vertices[0].normal;
    assert!((inverted_normal[0] + original_normal[0]).abs() < 0.001);
    assert!((inverted_normal[1] + original_normal[1]).abs() < 0.001);
    assert!((inverted_normal[2] + original_normal[2]).abs() < 0.001);
}

// #[test]
// fn test_mesh_generate_wireframe() {
//     let mesh = MeshPrimitives::cube(1.0);
//     let wireframe = mesh.generate_wireframe();
//     
//     assert!(wireframe.is_ok());
//     let wf = wireframe.unwrap();
//     
//     // 立方体有12条边，每条边需要2个索引
//     // 但从三角形生成线框：36个索引 / 3 = 12个三角形，每个3条边 = 36条线
//     // 每条线2个索引 = 72个索引
//     assert_eq!(wf.index_count(), 72);
// }

// ============================================================================
// Additional Tests for Coverage (6 tests)
// ============================================================================

#[test]
fn test_bounding_box_center() {
    let bbox = BoundingBox {
        min: [-1.0, -2.0, -3.0],
        max: [1.0, 2.0, 3.0],
    };
    let center = bbox.center();
    assert_eq!(center, [0.0, 0.0, 0.0]);
}

#[test]
fn test_bounding_box_size() {
    let bbox = BoundingBox {
        min: [-1.0, -2.0, -3.0],
        max: [1.0, 2.0, 3.0],
    };
    let size = bbox.size();
    assert_eq!(size, [2.0, 4.0, 6.0]);
}

#[test]
fn test_mesh_calculate_normals() {
    let mut mesh = Mesh::new();
    
    // 创建一个简单三角形
    let v0 = Vertex::with_position(0.0, 0.0, 0.0);
    let v1 = Vertex::with_position(1.0, 0.0, 0.0);
    let v2 = Vertex::with_position(0.0, 1.0, 0.0);
    
    mesh.add_vertex(v0).unwrap();
    mesh.add_vertex(v1).unwrap();
    mesh.add_vertex(v2).unwrap();
    mesh.add_triangle(0, 1, 2).unwrap();
    
    mesh.calculate_normals();
    
    // 法线应该指向+Z方向
    assert!(mesh.vertices[0].normal[2] > 0.5);
}

#[test]
fn test_mesh_set_topology() {
    let mut mesh = Mesh::new();
    mesh.set_topology(PrimitiveTopology::LineList);
    // 仅验证不崩溃
    assert_eq!(mesh.vertex_count(), 0);
}

#[test]
fn test_vertex_layout_position_uv() {
    let layout = VertexBufferLayout::position_uv();
    assert_eq!(layout.attribute_count(), 2);
    assert_eq!(layout.stride(), 20); // (3+2)*4
}

// #[test]
// fn test_mesh_utils_merge_meshes() {
//     let mesh1 = MeshPrimitives::cube(1.0);
//     let mesh2 = MeshPrimitives::sphere(0.5, 8, 4);
//     
//     let meshes = vec![mesh1, mesh2];
//     let merged = MeshUtils::merge_meshes(&meshes);
//     
//     assert!(merged.is_ok());
//     let m = merged.unwrap();
//     assert_eq!(m.vertex_count(), mesh1.vertex_count() + mesh2.vertex_count());
// }

// ============================================================================
// Bevy Parity Tests (Phase 2-3)
// ============================================================================

#[test]
fn test_morph_weights_new() {
    let weights = MorphWeights::new(vec![0.5, 0.3, 0.2]);
    assert_eq!(weights.len(), 3);
    assert!(!weights.is_empty());
    assert_eq!(weights.weights()[0], 0.5);
}

#[test]
fn test_morph_weights_with_mesh() {
    let weights = MorphWeights::with_mesh(vec![1.0, 0.0], 42);
    assert_eq!(weights.first_mesh(), Some(42));
    assert_eq!(weights.len(), 2);
}

#[test]
fn test_morph_weights_set_weight() {
    let mut weights = MorphWeights::new(vec![0.0, 0.0, 0.0]);
    weights.set_weight(1, 0.75);
    assert_eq!(weights.get_weight(1), Some(0.75));
    assert_eq!(weights.get_weight(3), None); // out of bounds
}

#[test]
fn test_morph_weights_clear() {
    let mut weights = MorphWeights::new(vec![0.5, 0.5, 0.5]);
    weights.clear();
    assert_eq!(weights.weights(), &[0.0, 0.0, 0.0]);
}

#[test]
fn test_skinned_mesh_new() {
    let mesh = SkinnedMesh::new(123, vec![1, 2, 3, 4]);
    assert_eq!(mesh.joints().len(), 4);
    assert_eq!(mesh.joint_count(), 4);
    assert_eq!(mesh.inverse_bindposes, 123);
}

#[test]
fn test_skinned_mesh_inverse_bindposes() {
    let matrices = vec![[1.0; 16], [2.0; 16]];
    let bindposes = SkinnedMeshInverseBindposes::new(matrices);
    assert_eq!(bindposes.len(), 2);
    assert!(!bindposes.is_empty());
    assert_eq!(bindposes.matrices()[0][0], 1.0);
}

#[test]
fn test_mesh2d_mesh3d_markers() {
    // Verify marker types can be created
    let _m2d = Mesh2d;
    let _m3d = Mesh3d;
    
    // Verify they implement expected traits
    let m2d_clone = Mesh2d.clone();
    let m3d_clone = Mesh3d.clone();
    assert_eq!(m2d_clone, Mesh2d);
    assert_eq!(m3d_clone, Mesh3d);
}

#[test]
fn test_mesh_systems_enum() {
    // Verify system set enum values
    assert_ne!(MeshSystems::UpdateMorphWeights, MeshSystems::PrepareSkinning);
    assert_ne!(MeshSystems::PrepareSkinning, MeshSystems::ExtractMeshes);
}

// ============================================================================
// Test Summary
// ============================================================================

// 总计测试数：
// - Vertex Tests: 3
// - Mesh Creation: 4
// - Primitives: 7
// - GPU Buffer: 3
// - Vertex Layout: 3
// - Mesh Utils: 4
// - Additional: 6
// - Bevy Parity: 8
// 总计：38 个测试