use autozig_transform::{Transform, Hierarchy, LocalToWorld, TransformSystem};

#[test]
fn test_transform_identity() {
    let transform = Transform::identity();
    assert_eq!(transform.translation, [0.0, 0.0, 0.0]);
    assert_eq!(transform.rotation, [0.0, 0.0, 0.0, 1.0]); // Identity quaternion
    assert_eq!(transform.scale, [1.0, 1.0, 1.0]);
}

#[test]
fn test_transform_translation() {
    let translation = [1.0, 2.0, 3.0];
    let transform = Transform::from_translation(translation);
    assert_eq!(transform.translation, translation);
    assert_eq!(transform.rotation, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(transform.scale, [1.0, 1.0, 1.0]);
}

#[test]
fn test_transform_rotation() {
    let rotation = [0.0, 0.707, 0.0, 0.707]; // 90 degrees around Y axis
    let transform = Transform::from_rotation(rotation);
    assert_eq!(transform.translation, [0.0, 0.0, 0.0]);
    assert_eq!(transform.rotation, rotation);
    assert_eq!(transform.scale, [1.0, 1.0, 1.0]);
}

#[test]
fn test_transform_scale() {
    let scale = [2.0, 3.0, 4.0];
    let transform = Transform::from_scale(scale);
    assert_eq!(transform.translation, [0.0, 0.0, 0.0]);
    assert_eq!(transform.rotation, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(transform.scale, scale);
}

#[test]
fn test_transform_composition() {
    let mut transform = Transform::identity();
    transform.translation = [1.0, 2.0, 3.0];
    transform.scale = [2.0, 2.0, 2.0];
    
    let matrix = transform.compute_matrix();
    
    // Check translation is in the last column (column-major)
    assert_eq!(matrix[12], 1.0);
    assert_eq!(matrix[13], 2.0);
    assert_eq!(matrix[14], 3.0);
    assert_eq!(matrix[15], 1.0);
    
    // Check scale is applied (diagonal elements)
    assert!((matrix[0] - 2.0).abs() < 0.001);
    assert!((matrix[5] - 2.0).abs() < 0.001);
    assert!((matrix[10] - 2.0).abs() < 0.001);
}

#[test]
fn test_hierarchy_create() {
    let hierarchy = Hierarchy::new();
    assert_eq!(hierarchy.parent, 0);
    assert_eq!(hierarchy.children_count, 0);
    assert!(!hierarchy.has_parent());
    assert!(!hierarchy.has_children());
}

#[test]
fn test_hierarchy_add_child() {
    let mut hierarchy = Hierarchy::new();
    
    assert!(hierarchy.add_child(1));
    assert_eq!(hierarchy.children_count(), 1);
    assert_eq!(hierarchy.get_child(0), 1);
    
    assert!(hierarchy.add_child(2));
    assert_eq!(hierarchy.children_count(), 2);
    assert_eq!(hierarchy.get_child(1), 2);
    
    assert!(hierarchy.has_children());
}

#[test]
fn test_hierarchy_remove_child() {
    let mut hierarchy = Hierarchy::new();
    
    hierarchy.add_child(1);
    hierarchy.add_child(2);
    hierarchy.add_child(3);
    
    assert_eq!(hierarchy.children_count(), 3);
    
    assert!(hierarchy.remove_child(2));
    assert_eq!(hierarchy.children_count(), 2);
    assert_eq!(hierarchy.get_child(0), 1);
    assert_eq!(hierarchy.get_child(1), 3);
    
    assert!(!hierarchy.remove_child(99)); // Non-existent child
}

#[test]
fn test_hierarchy_children() {
    let mut hierarchy = Hierarchy::new();
    
    hierarchy.add_child(10);
    hierarchy.add_child(20);
    hierarchy.add_child(30);
    
    let children = hierarchy.children();
    assert_eq!(children.len(), 3);
    assert_eq!(children[0], 10);
    assert_eq!(children[1], 20);
    assert_eq!(children[2], 30);
}

#[test]
fn test_local_to_world_identity() {
    let ltw = LocalToWorld::identity();
    assert!(ltw.is_identity());
    
    let matrix = ltw.matrix();
    assert_eq!(matrix[0], 1.0);
    assert_eq!(matrix[5], 1.0);
    assert_eq!(matrix[10], 1.0);
    assert_eq!(matrix[15], 1.0);
}

#[test]
fn test_local_to_world_parent() {
    let mut parent_ltw = LocalToWorld::identity();
    parent_ltw.set_matrix([
        2.0, 0.0, 0.0, 0.0,
        0.0, 2.0, 0.0, 0.0,
        0.0, 0.0, 2.0, 0.0,
        5.0, 6.0, 7.0, 1.0,
    ]);
    
    let translation = parent_ltw.translation();
    assert_eq!(translation[0], 5.0);
    assert_eq!(translation[1], 6.0);
    assert_eq!(translation[2], 7.0);
    
    let scale = parent_ltw.scale();
    assert!((scale[0] - 2.0).abs() < 0.001);
    assert!((scale[1] - 2.0).abs() < 0.001);
    assert!((scale[2] - 2.0).abs() < 0.001);
}

#[test]
fn test_local_to_world_composition() {
    let mut ltw1 = LocalToWorld::from_matrix([
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        1.0, 0.0, 0.0, 1.0,
    ]);
    
    let matrix2 = [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        2.0, 0.0, 0.0, 1.0,
    ];
    
    ltw1.multiply(matrix2);
    let result = ltw1.translation();
    assert!((result[0] - 3.0).abs() < 0.001);
}

#[test]
fn test_system_hierarchy_update() {
    let mut hierarchy = Hierarchy::new();
    let mut transform = Transform::identity();
    let mut local_to_world = LocalToWorld::identity();
    
    TransformSystem::update_hierarchy(&mut hierarchy, &mut transform, &mut local_to_world);
    
    // Should have updated local_to_world
    assert!(local_to_world.is_identity());
}

#[test]
fn test_system_local_to_world_update() {
    let mut transform = Transform::from_translation([1.0, 2.0, 3.0]);
    let mut parent_ltw = LocalToWorld::identity();
    let mut local_to_world = LocalToWorld::identity();
    
    TransformSystem::update_local_to_world(&mut transform, &mut parent_ltw, &mut local_to_world);
    
    let translation = local_to_world.translation();
    assert!((translation[0] - 1.0).abs() < 0.001);
    assert!((translation[1] - 2.0).abs() < 0.001);
    assert!((translation[2] - 3.0).abs() < 0.001);
}

#[test]
fn test_transform_2d() {
    // Test 2D transform (z = 0)
    let transform = Transform::from_translation([10.0, 20.0, 0.0]);
    let matrix = transform.compute_matrix();
    
    assert_eq!(matrix[12], 10.0);
    assert_eq!(matrix[13], 20.0);
    assert_eq!(matrix[14], 0.0);
    
    // Check it's still a valid transform matrix
    assert_eq!(matrix[15], 1.0);
}

#[test]
fn test_transform_3d() {
    // Test 3D transform
    let transform = Transform::from_translation([10.0, 20.0, 30.0]);
    let matrix = transform.compute_matrix();
    
    assert_eq!(matrix[12], 10.0);
    assert_eq!(matrix[13], 20.0);
    assert_eq!(matrix[14], 30.0);
    assert_eq!(matrix[15], 1.0);
    
    // Verify identity rotation and unit scale
    assert!((matrix[0] - 1.0).abs() < 0.001);
    assert!((matrix[5] - 1.0).abs() < 0.001);
    assert!((matrix[10] - 1.0).abs() < 0.001);
}

#[test]
fn test_hierarchy_parent_child_relationship() {
    let mut parent = Hierarchy::new();
    let mut child = Hierarchy::new();
    
    parent.add_child(100);
    child.set_parent(99);
    
    assert!(parent.has_children());
    assert_eq!(parent.children_count(), 1);
    
    assert!(child.has_parent());
    assert_eq!(child.parent(), 99);
}

#[test]
fn test_transform_default() {
    let transform = Transform::default();
    let identity = Transform::identity();
    
    assert_eq!(transform.translation, identity.translation);
    assert_eq!(transform.rotation, identity.rotation);
    assert_eq!(transform.scale, identity.scale);
}

#[test]
fn test_hierarchy_clear_operations() {
    let mut hierarchy = Hierarchy::new();
    
    hierarchy.add_child(1);
    hierarchy.add_child(2);
    hierarchy.set_parent(99);
    
    assert!(hierarchy.has_children());
    assert!(hierarchy.has_parent());
    
    hierarchy.clear_children();
    assert!(!hierarchy.has_children());
    assert_eq!(hierarchy.children_count(), 0);
    
    hierarchy.clear_parent();
    assert!(!hierarchy.has_parent());
    assert_eq!(hierarchy.parent(), 0);
}

#[test]
fn test_local_to_world_copy_operations() {
    let mut ltw1 = LocalToWorld::from_matrix([
        2.0, 0.0, 0.0, 0.0,
        0.0, 2.0, 0.0, 0.0,
        0.0, 0.0, 2.0, 0.0,
        1.0, 2.0, 3.0, 1.0,
    ]);
    
    let mut ltw2 = LocalToWorld::identity();
    ltw2.copy_from(&ltw1);
    
    assert_eq!(ltw1.matrix(), ltw2.matrix());
    
    let translation = ltw2.translation();
    assert_eq!(translation[0], 1.0);
    assert_eq!(translation[1], 2.0);
    assert_eq!(translation[2], 3.0);
}

#[test]
fn test_hierarchy_max_children() {
    let mut hierarchy = Hierarchy::new();
    
    // Add 32 children (max)
    for i in 1..=32 {
        assert!(hierarchy.add_child(i));
    }
    
    assert_eq!(hierarchy.children_count(), 32);
    
    // Try to add 33rd child (should fail)
    assert!(!hierarchy.add_child(33));
    assert_eq!(hierarchy.children_count(), 32);
}

#[test]
fn test_transform_matrix_column_major() {
    let transform = Transform::from_translation([5.0, 10.0, 15.0]);
    let matrix = transform.compute_matrix();
    
    // Column-major order: translation is in indices 12, 13, 14
    assert_eq!(matrix[12], 5.0);
    assert_eq!(matrix[13], 10.0);
    assert_eq!(matrix[14], 15.0);
    
    // Check identity rotation (diagonal)
    assert!((matrix[0] - 1.0).abs() < 0.001);
    assert!((matrix[5] - 1.0).abs() < 0.001);
    assert!((matrix[10] - 1.0).abs() < 0.001);
}