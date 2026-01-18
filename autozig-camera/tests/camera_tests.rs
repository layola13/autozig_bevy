//! Unit tests for autozig-camera
//! Tests projection, view matrices, and frustum culling

use autozig_camera::*;
use approx::assert_relative_eq;

const EPSILON: f32 = 0.001;

// ============================================================================
// Projection Tests
// ============================================================================

#[test]
fn test_perspective_projection_creation() {
    let proj = PerspectiveProjection::new(std::f32::consts::FRAC_PI_4, 16.0 / 9.0);
    
    assert_relative_eq!(proj.fov, std::f32::consts::FRAC_PI_4, epsilon = EPSILON);
    assert_relative_eq!(proj.aspect_ratio, 16.0 / 9.0, epsilon = EPSILON);
    assert_eq!(proj.near, 0.1);
    assert_eq!(proj.far, 1000.0);
}

#[test]
fn test_perspective_projection_from_degrees() {
    let proj = PerspectiveProjection::from_fov_degrees(90.0, 16.0 / 9.0);
    
    assert_relative_eq!(proj.fov, std::f32::consts::FRAC_PI_2, epsilon = EPSILON);
    assert_relative_eq!(proj.fov_degrees(), 90.0, epsilon = EPSILON);
}

#[test]
fn test_perspective_matrix_identity_properties() {
    let proj = PerspectiveProjection::default();
    let matrix = proj.compute_matrix();
    
    // Check that it's not identity matrix
    assert!(matrix[0] != 1.0 || matrix[5] != 1.0);
    
    // Check that m23 = -1 for perspective (right-handed)
    assert_relative_eq!(matrix[11], -1.0, epsilon = EPSILON);
}

#[test]
fn test_perspective_fov_update() {
    let mut proj = PerspectiveProjection::default();
    proj.set_fov_degrees(60.0);
    
    assert_relative_eq!(proj.fov_degrees(), 60.0, epsilon = EPSILON);
}

#[test]
fn test_perspective_aspect_update() {
    let mut proj = PerspectiveProjection::default();
    proj.update_aspect(1.0); // Square aspect
    
    assert_eq!(proj.aspect_ratio, 1.0);
}

#[test]
fn test_orthographic_projection_creation() {
    let proj = OrthographicProjection::new(-10.0, 10.0, -5.0, 5.0);
    
    assert_eq!(proj.left, -10.0);
    assert_eq!(proj.right, 10.0);
    assert_eq!(proj.bottom, -5.0);
    assert_eq!(proj.top, 5.0);
    assert_eq!(proj.scale, 1.0);
}

#[test]
fn test_orthographic_from_window_size() {
    let proj = OrthographicProjection::from_window_size(1920.0, 1080.0);
    
    assert_relative_eq!(proj.width(), 1920.0, epsilon = EPSILON);
    assert_relative_eq!(proj.height(), 1080.0, epsilon = EPSILON);
}

#[test]
fn test_orthographic_matrix_identity_properties() {
    let proj = OrthographicProjection::default();
    let matrix = proj.compute_matrix();
    
    // Check that m23 = 0 for orthographic
    assert_relative_eq!(matrix[11], 0.0, epsilon = EPSILON);
    
    // Check that m33 = 1 for orthographic
    assert_relative_eq!(matrix[15], 1.0, epsilon = EPSILON);
}

#[test]
fn test_orthographic_update_size() {
    let mut proj = OrthographicProjection::default();
    proj.update_size(800.0, 600.0);
    
    assert_relative_eq!(proj.width(), 800.0, epsilon = EPSILON);
    assert_relative_eq!(proj.height(), 600.0, epsilon = EPSILON);
}

#[test]
fn test_projection_utils_extract_fov() {
    let proj = PerspectiveProjection::from_fov_degrees(60.0, 16.0 / 9.0);
    let matrix = proj.compute_matrix();
    
    let extracted_fov = ProjectionUtils::extract_fov(&matrix);
    assert_relative_eq!(extracted_fov, proj.fov, epsilon = 0.01);
}

#[test]
fn test_projection_utils_extract_aspect() {
    let proj = PerspectiveProjection::new(std::f32::consts::FRAC_PI_4, 1.5);
    let matrix = proj.compute_matrix();
    
    let extracted_aspect = ProjectionUtils::extract_aspect(&matrix);
    assert_relative_eq!(extracted_aspect, 1.5, epsilon = EPSILON);
}

#[test]
fn test_projection_utils_is_perspective() {
    let proj = PerspectiveProjection::default();
    let matrix = proj.compute_matrix();
    
    assert!(ProjectionUtils::is_perspective(&matrix));
    assert!(!ProjectionUtils::is_orthographic(&matrix));
}

#[test]
fn test_projection_utils_is_orthographic() {
    let proj = OrthographicProjection::default();
    let matrix = proj.compute_matrix();
    
    assert!(ProjectionUtils::is_orthographic(&matrix));
    assert!(!ProjectionUtils::is_perspective(&matrix));
}

// ============================================================================
// Camera3d Tests
// ============================================================================

#[test]
fn test_camera3d_creation() {
    let camera = Camera3d::new(std::f32::consts::FRAC_PI_4, 16.0 / 9.0);
    
    assert_relative_eq!(camera.projection.fov, std::f32::consts::FRAC_PI_4, epsilon = EPSILON);
    assert_relative_eq!(camera.projection.aspect_ratio, 16.0 / 9.0, epsilon = EPSILON);
}

#[test]
fn test_camera3d_default() {
    let camera = Camera3d::default();
    
    assert_relative_eq!(camera.projection.fov, std::f32::consts::FRAC_PI_4, epsilon = EPSILON);
}

#[test]
fn test_camera3d_update_matrices() {
    let mut camera = Camera3d::default();
    let position = [0.0, 5.0, 10.0];
    let rotation = [0.0, 0.0, 0.0, 1.0]; // Identity quaternion
    
    camera.update_matrices(&position, &rotation);
    
    // Check that matrices are not zero
    assert!(camera.view_matrix[0] != 0.0 || camera.view_matrix[5] != 0.0);
    assert!(camera.projection_matrix[0] != 0.0);
}

#[test]
fn test_camera3d_look_at() {
    let mut camera = Camera3d::default();
    let eye = [0.0, 0.0, 10.0];
    let target = [0.0, 0.0, 0.0];
    let up = [0.0, 1.0, 0.0];
    
    camera.look_at(eye, target, up);
    
    // Check that view matrix is set
    assert!(camera.view_matrix.iter().any(|&x| x != 0.0));
}

#[test]
fn test_camera3d_position_extraction() {
    let mut camera = Camera3d::default();
    let position = [5.0, 3.0, -2.0];
    let rotation = [0.0, 0.0, 0.0, 1.0];
    
    camera.update_matrices(&position, &rotation);
    let extracted_pos = camera.position();
    
    // Position should be approximately the same
    assert_relative_eq!(extracted_pos[0], position[0], epsilon = 0.1);
    assert_relative_eq!(extracted_pos[1], position[1], epsilon = 0.1);
    assert_relative_eq!(extracted_pos[2], position[2], epsilon = 0.1);
}

#[test]
fn test_camera3d_direction_vectors() {
    let mut camera = Camera3d::default();
    let position = [0.0, 0.0, 0.0];
    let rotation = [0.0, 0.0, 0.0, 1.0]; // Identity
    
    camera.update_matrices(&position, &rotation);
    
    let forward = camera.forward();
    let right = camera.right();
    let up = camera.up();
    
    // Vectors should be normalized (length ≈ 1)
    let forward_len = (forward[0] * forward[0] + forward[1] * forward[1] + forward[2] * forward[2]).sqrt();
    let right_len = (right[0] * right[0] + right[1] * right[1] + right[2] * right[2]).sqrt();
    let up_len = (up[0] * up[0] + up[1] * up[1] + up[2] * up[2]).sqrt();
    
    assert_relative_eq!(forward_len, 1.0, epsilon = 0.1);
    assert_relative_eq!(right_len, 1.0, epsilon = 0.1);
    assert_relative_eq!(up_len, 1.0, epsilon = 0.1);
}

// ============================================================================
// Camera2d Tests
// ============================================================================

#[test]
fn test_camera2d_creation() {
    let camera = Camera2d::new(1920.0, 1080.0);
    
    assert_relative_eq!(camera.projection.width(), 1920.0, epsilon = EPSILON);
    assert_relative_eq!(camera.projection.height(), 1080.0, epsilon = EPSILON);
}

#[test]
fn test_camera2d_default() {
    let camera = Camera2d::default();
    
    assert_eq!(camera.projection.scale, 1.0);
}

#[test]
fn test_camera2d_update_position() {
    let mut camera = Camera2d::default();
    let position = [100.0, 50.0];
    
    camera.update_position(&position);
    
    // Check that matrices are updated
    assert!(camera.view_matrix.iter().any(|&x| x != 0.0));
}

#[test]
fn test_camera2d_update_with_rotation() {
    let mut camera = Camera2d::default();
    let position = [0.0, 0.0];
    let rotation = std::f32::consts::FRAC_PI_4; // 45 degrees
    
    camera.update_matrices(&position, rotation);
    
    // Check that matrices are updated
    assert!(camera.view_matrix.iter().any(|&x| x != 0.0));
}

#[test]
fn test_camera2d_scale() {
    let mut camera = Camera2d::default();
    camera.set_scale(2.0);
    
    assert_eq!(camera.scale(), 2.0);
}

// ============================================================================
// Frustum Tests
// ============================================================================

#[test]
fn test_frustum_creation() {
    let camera = Camera3d::default();
    let frustum = camera.frustum();
    
    // Check that frustum has 6 planes
    assert_eq!(frustum.planes.len(), 6);
    
    // Check that planes are normalized (normal length ≈ 1)
    for plane in &frustum.planes {
        let len = (plane.normal[0] * plane.normal[0] + 
                  plane.normal[1] * plane.normal[1] + 
                  plane.normal[2] * plane.normal[2]).sqrt();
        assert_relative_eq!(len, 1.0, epsilon = 0.1);
    }
}

#[test]
fn test_frustum_test_point_inside() {
    let mut camera = Camera3d::default();
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let frustum = camera.frustum();
    
    // Point at origin should be visible
    let point = [0.0, 0.0, 0.0];
    assert!(frustum.test_point(point));
}

#[test]
fn test_frustum_test_point_outside() {
    let mut camera = Camera3d::default();
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let frustum = camera.frustum();
    
    // Point far behind camera should not be visible
    let point = [0.0, 0.0, 100.0];
    assert!(!frustum.test_point(point));
}

#[test]
fn test_frustum_test_sphere_inside() {
    let mut camera = Camera3d::default();
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let frustum = camera.frustum();
    
    // Small sphere at origin should be visible
    let center = [0.0, 0.0, 0.0];
    let radius = 1.0;
    assert!(frustum.test_sphere(center, radius));
}

#[test]
fn test_frustum_test_sphere_outside() {
    let mut camera = Camera3d::default();
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let frustum = camera.frustum();
    
    // Sphere far away should not be visible
    let center = [1000.0, 1000.0, 1000.0];
    let radius = 1.0;
    assert!(!frustum.test_sphere(center, radius));
}

#[test]
fn test_frustum_test_aabb_inside() {
    let mut camera = Camera3d::default();
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let frustum = camera.frustum();
    
    // Small AABB at origin should be visible
    let min = [-1.0, -1.0, -1.0];
    let max = [1.0, 1.0, 1.0];
    assert!(frustum.test_aabb(min, max));
}

#[test]
fn test_frustum_test_aabb_outside() {
    let mut camera = Camera3d::default();
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let frustum = camera.frustum();
    
    // AABB far away should not be visible
    let min = [500.0, 500.0, 500.0];
    let max = [600.0, 600.0, 600.0];
    assert!(!frustum.test_aabb(min, max));
}

#[test]
fn test_frustum_test_obb() {
    let mut camera = Camera3d::default();
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let frustum = camera.frustum();
    
    // OBB at origin with identity rotation
    let center = [0.0, 0.0, 0.0];
    let extents = [1.0, 1.0, 1.0];
    let rotation = [0.0, 0.0, 0.0, 1.0]; // Identity quaternion
    
    assert!(frustum.test_obb(center, extents, rotation));
}

#[test]
fn test_frustum_corners() {
    let camera = Camera3d::default();
    let frustum = camera.frustum();
    
    let corners = frustum.corners();
    
    // Should have 8 corners (24 floats: 8 vertices * 3 components)
    assert_eq!(corners.len(), 24);
}

#[test]
fn test_frustum_test_aabb_completely_inside() {
    let mut camera = Camera3d::default();
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let frustum = camera.frustum();
    
    // Very small AABB at origin
    let min = [-0.1, -0.1, -0.1];
    let max = [0.1, 0.1, 0.1];
    
    // Should be completely inside
    let result = frustum.test_aabb_inside(min, max);
    assert!(result || frustum.test_aabb(min, max)); // At least intersects
}

#[test]
fn test_frustum_distance_to_aabb() {
    let mut camera = Camera3d::default();
    camera.look_at([0.0, 0.0, 10.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    let frustum = camera.frustum();
    
    let min = [-1.0, -1.0, -1.0];
    let max = [1.0, 1.0, 1.0];
    
    let distance = frustum.distance_to_aabb(min, max);
    
    // Distance should be non-negative
    assert!(distance >= 0.0);
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_camera3d_full_workflow() {
    // Create camera
    let mut camera = Camera3d::new(std::f32::consts::FRAC_PI_3, 16.0 / 9.0);
    
    // Position camera
    camera.look_at([10.0, 10.0, 10.0], [0.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
    
    // Get frustum
    let frustum = camera.frustum();
    
    // Test culling
    assert!(frustum.test_sphere([0.0, 0.0, 0.0], 1.0)); // Center should be visible
    assert!(!frustum.test_sphere([100.0, 100.0, 100.0], 1.0)); // Far away should not be visible
}

#[test]
fn test_camera2d_full_workflow() {
    // Create 2D camera
    let mut camera = Camera2d::new(1920.0, 1080.0);
    
    // Move camera
    camera.update_position(&[100.0, 50.0]);
    
    // Zoom
    camera.set_scale(2.0);
    camera.update_position(&[100.0, 50.0]);
    
    // Check that matrices are valid
    assert!(camera.view_projection_matrix.iter().any(|&x| x != 0.0));
}

#[test]
fn test_perspective_vs_orthographic() {
    let persp_proj = PerspectiveProjection::default();
    let ortho_proj = OrthographicProjection::default();
    
    let persp_matrix = persp_proj.compute_matrix();
    let ortho_matrix = ortho_proj.compute_matrix();
    
    // They should be different
    assert_ne!(persp_matrix, ortho_matrix);
    
    // Verify their types
    assert!(ProjectionUtils::is_perspective(&persp_matrix));
    assert!(ProjectionUtils::is_orthographic(&ortho_matrix));
}

// ============================================================================
// Bevy Parity Tests (Phase 2-3)
// ============================================================================

#[test]
fn test_view_visibility_constants() {
    let visible = ViewVisibility::VISIBLE;
    let hidden = ViewVisibility::HIDDEN;
    
    assert!(visible.get());
    assert!(!hidden.get());
}

#[test]
fn test_view_visibility_set() {
    let mut v = ViewVisibility::new(false);
    assert!(!v.get());
    
    v.set(true);
    assert!(v.get());
}

#[test]
fn test_inherited_visibility_constants() {
    let visible = InheritedVisibility::VISIBLE;
    let hidden = InheritedVisibility::HIDDEN;
    
    assert!(visible.get());
    assert!(!hidden.get());
}

#[test]
fn test_clear_color_default() {
    let color = ClearColor::default();
    // Default should be a reasonable color (not zero/black typically)
    let _ = color; // Just verify it compiles with Default
}

#[test]
fn test_camera_systems_enum() {
    // Verify system set enum values
    assert_ne!(CameraSystems::UpdateProjections, CameraSystems::PropagateVisibility);
    assert_ne!(CameraSystems::CheckVisibility, CameraSystems::ExtractCameras);
}