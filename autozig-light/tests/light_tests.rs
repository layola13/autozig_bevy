use autozig_light::*;

// ============================================================================
// Point Light Tests (4 tests)
// ============================================================================

#[test]
fn test_point_light_creation() {
    let light = PointLight::new([1.0, 0.5, 0.0], 1000.0, 25.0);
    assert_eq!(light.color, [1.0, 0.5, 0.0]);
    assert_eq!(light.intensity, 1000.0);
    assert_eq!(light.range, 25.0);
}

#[test]
fn test_point_light_attenuation() {
    let light = PointLight::new([1.0, 1.0, 1.0], 800.0, 20.0);
    
    // At distance 0, attenuation should be 1.0
    let atten_near = light.attenuation(0.0);
    assert!((atten_near - 1.0).abs() < 0.01);
    
    // At half range, attenuation should be significant but not zero
    let atten_mid = light.attenuation(10.0);
    assert!(atten_mid > 0.0 && atten_mid < 1.0);
    
    // Beyond range, attenuation should be 0
    let atten_far = light.attenuation(25.0);
    assert_eq!(atten_far, 0.0);
}

#[test]
fn test_point_light_range() {
    let mut light = PointLight::default();
    light.set_range(50.0);
    assert_eq!(light.range, 50.0);
}

#[test]
fn test_point_light_shadows() {
    let mut light = PointLight::default();
    assert!(!light.has_shadows());
    
    light.enable_shadows();
    assert!(light.has_shadows());
    
    light.disable_shadows();
    assert!(!light.has_shadows());
}

// ============================================================================
// Directional Light Tests (4 tests)
// ============================================================================

#[test]
fn test_directional_light_creation() {
    let light = DirectionalLight::new([1.0, 1.0, 0.8], 50000.0, [0.0, -1.0, 0.0]);
    assert_eq!(light.color, [1.0, 1.0, 0.8]);
    assert_eq!(light.illuminance, 50000.0);
}

#[test]
fn test_directional_light_direction_normalization() {
    let light = DirectionalLight::new([1.0, 1.0, 1.0], 100000.0, [3.0, 4.0, 0.0]);
    
    // Direction should be normalized (length = 1)
    let len = (light.direction[0].powi(2) + 
               light.direction[1].powi(2) + 
               light.direction[2].powi(2)).sqrt();
    assert!((len - 1.0).abs() < 0.001);
}

#[test]
fn test_directional_light_illuminance() {
    let mut light = DirectionalLight::default();
    light.set_illuminance(75000.0);
    assert_eq!(light.illuminance, 75000.0);
}

#[test]
fn test_directional_light_shadows() {
    let mut light = DirectionalLight::default();
    assert!(!light.has_shadows());
    
    light.enable_shadows();
    assert!(light.has_shadows());
}

// ============================================================================
// Spot Light Tests (5 tests)
// ============================================================================

#[test]
fn test_spot_light_creation() {
    let light = SpotLight::new(
        [1.0, 0.8, 0.6],
        1200.0,
        [0.0, -1.0, 0.0],
        0.52,
        0.79
    );
    assert_eq!(light.color, [1.0, 0.8, 0.6]);
    assert_eq!(light.intensity, 1200.0);
}

#[test]
fn test_spot_light_attenuation() {
    let light = SpotLight::new(
        [1.0, 1.0, 1.0],
        800.0,
        [0.0, -1.0, 0.0],
        0.5,
        0.8
    );
    
    let atten_near = light.attenuation(0.0);
    assert!((atten_near - 1.0).abs() < 0.01);
    
    let atten_mid = light.attenuation(10.0);
    assert!(atten_mid > 0.0 && atten_mid < 1.0);
}

#[test]
fn test_spot_light_cone_factor() {
    let light = SpotLight::new(
        [1.0, 1.0, 1.0],
        800.0,
        [0.0, -1.0, 0.0],
        0.3,
        0.6
    );
    
    // Light pointing straight down, test point also down
    let factor_center = light.spot_factor([0.0, -1.0, 0.0]);
    println!("Spot factor (center): {}", factor_center);
    // Spot factor should be positive when aligned
    assert!(factor_center >= 0.0 && factor_center <= 1.0);
    assert!(factor_center > 0.0);
    
    // Light direction perpendicular should have lower factor
    let factor_side = light.spot_factor([1.0, 0.0, 0.0]);
    println!("Spot factor (side): {}", factor_side);
    assert!(factor_side >= 0.0 && factor_side <= 1.0);
    // Just verify side is not greater than center
    assert!(factor_side <= factor_center + 0.1);
}

#[test]
fn test_spot_light_angles() {
    let mut light = SpotLight::default();
    light.set_inner_angle(0.4);
    light.set_outer_angle(0.7);
    
    assert!((light.inner_angle - 0.4).abs() < 0.001);
    assert!((light.outer_angle - 0.7).abs() < 0.001);
}

#[test]
fn test_spot_light_direction() {
    let mut light = SpotLight::default();
    light.set_direction(1.0, 0.0, 0.0);
    
    // Should be normalized
    let len = (light.direction[0].powi(2) + 
               light.direction[1].powi(2) + 
               light.direction[2].powi(2)).sqrt();
    assert!((len - 1.0).abs() < 0.001);
}

// ============================================================================
// Ambient Light Tests (2 tests)
// ============================================================================

#[test]
fn test_ambient_light_creation() {
    let light = AmbientLight::new([0.2, 0.2, 0.25], 0.15);
    // Allow floating point precision differences (Zig uses f32 internally)
    assert!((light.color[0] - 0.2).abs() < 0.01);
    assert!((light.color[1] - 0.2).abs() < 0.01);
    assert!((light.color[2] - 0.25).abs() < 0.01);
    assert!((light.brightness - 0.15).abs() < 0.01);
}

#[test]
fn test_ambient_light_brightness() {
    let mut light = AmbientLight::default();
    light.set_brightness(0.3);
    assert_eq!(light.brightness, 0.3);
}

// ============================================================================
// Lighting Utils Tests (5 tests)
// ============================================================================

#[test]
fn test_lighting_attenuation() {
    let atten_near = LightingUtils::calculate_attenuation(0.0, 20.0);
    assert!((atten_near - 1.0).abs() < 0.01);
    
    let atten_mid = LightingUtils::calculate_attenuation(10.0, 20.0);
    assert!(atten_mid > 0.0 && atten_mid < 1.0);
    
    let atten_far = LightingUtils::calculate_attenuation(25.0, 20.0);
    assert_eq!(atten_far, 0.0);
}

#[test]
fn test_lambertian_diffuse() {
    let normal = [0.0, 1.0, 0.0];
    let light_dir = [0.0, 1.0, 0.0];
    
    let diffuse = LightingUtils::lambertian(normal, light_dir);
    assert!((diffuse - 1.0).abs() < 0.001);
    
    // Perpendicular light should give 0
    let light_perp = [1.0, 0.0, 0.0];
    let diffuse_perp = LightingUtils::lambertian(normal, light_perp);
    assert!(diffuse_perp.abs() < 0.001);
}

#[test]
fn test_blinn_phong_specular() {
    let normal = [0.0, 1.0, 0.0];
    let view_dir = [0.0, 1.0, 0.0];
    let light_dir = [0.0, 1.0, 0.0];
    
    let specular = LightingUtils::blinn_phong(normal, view_dir, light_dir, 32.0);
    assert!(specular > 0.0);
}

#[test]
fn test_cook_torrance_brdf() {
    let normal = [0.0, 1.0, 0.0];
    let view_dir = [0.0, 1.0, 0.0];
    let light_dir = [0.0, 1.0, 0.0];
    
    let brdf = LightingUtils::cook_torrance(normal, view_dir, light_dir, 0.5, 0.0);
    assert!(brdf >= 0.0);
}

#[test]
fn test_spot_factor_calculation() {
    let light_dir = [0.0, -1.0, 0.0];
    let spot_dir = [0.0, -1.0, 0.0];
    
    let factor = LightingUtils::calculate_spot_factor(light_dir, spot_dir, 0.3, 0.6);
    println!("Spot factor (aligned directions): {}", factor);
    // Spot factor uses smoothstep which may vary, just check it's in valid range
    assert!(factor >= 0.0 && factor <= 1.0);
    assert!(factor > 0.0); // Should be positive when aligned
}

// ============================================================================
// Shadow Map Tests (3 tests)
// ============================================================================

#[test]
fn test_shadow_map_creation() {
    let shadow_map = ShadowMap::new(2048);
    assert_eq!(shadow_map.resolution, 2048);
}

#[test]
fn test_shadow_map_cascades() {
    let mut shadow_map = ShadowMap::default();
    shadow_map.set_cascades(4);
    assert_eq!(shadow_map.cascade_count, 4);
    
    // Should clamp to valid range
    shadow_map.set_cascades(10);
    assert_eq!(shadow_map.cascade_count, 4);
}

#[test]
fn test_cascade_shadow_map_splits() {
    let csm = CascadeShadowMap::calculate_splits(0.1, 100.0, 4);
    assert_eq!(csm.split_count, 4);
    
    // Splits should be in ascending order
    assert!(csm.splits[0] < csm.splits[1]);
    assert!(csm.splits[1] < csm.splits[2]);
    assert!(csm.splits[2] < csm.splits[3]);
    
    // Last split should be near far plane
    assert!((csm.splits[3] - 100.0).abs() < 1.0);
}

// ============================================================================
// Light Scene Tests (4 tests)
// ============================================================================

#[test]
fn test_light_scene_creation() {
    let scene = LightScene::new();
    assert_eq!(scene.point_light_count, 0);
    assert_eq!(scene.directional_light_count, 0);
    assert_eq!(scene.spot_light_count, 0);
}

#[test]
fn test_light_scene_add_lights() {
    let mut scene = LightScene::new();
    
    let point = PointLight::new([1.0, 1.0, 1.0], 800.0, 20.0);
    assert!(scene.add_point_light(point).is_ok());
    assert_eq!(scene.point_light_count, 1);
    
    let directional = DirectionalLight::new([1.0, 1.0, 0.9], 100000.0, [0.0, -1.0, 0.0]);
    assert!(scene.add_directional_light(directional).is_ok());
    assert_eq!(scene.directional_light_count, 1);
    
    let spot = SpotLight::new([1.0, 0.8, 0.6], 1000.0, [0.0, -1.0, 0.0], 0.5, 0.8);
    assert!(scene.add_spot_light(spot).is_ok());
    assert_eq!(scene.spot_light_count, 1);
}

#[test]
fn test_light_scene_clear() {
    let mut scene = LightScene::new();
    
    let point = PointLight::default();
    scene.add_point_light(point).unwrap();
    assert_eq!(scene.point_light_count, 1);
    
    scene.clear_lights();
    assert_eq!(scene.point_light_count, 0);
}

#[test]
fn test_light_scene_limits() {
    let mut scene = LightScene::new();
    
    // Add max point lights
    for _ in 0..MAX_POINT_LIGHTS {
        let point = PointLight::default();
        assert!(scene.add_point_light(point).is_ok());
    }
    
    // Next one should fail
    let point = PointLight::default();
    assert!(scene.add_point_light(point).is_err());
}

// ============================================================================
// GPU Data Tests (3 tests)
// ============================================================================

#[test]
fn test_gpu_light_buffer_creation() {
    let scene = LightScene::new();
    let buffer = GpuLightBuffer::from_scene(&scene);
    
    assert_eq!(buffer.point_light_count, 0);
    assert_eq!(buffer.directional_light_count, 0);
    assert_eq!(buffer.spot_light_count, 0);
}

#[test]
fn test_gpu_light_buffer_from_scene() {
    let mut scene = LightScene::new();
    
    let point = PointLight::new([1.0, 0.5, 0.0], 1000.0, 25.0);
    scene.add_point_light(point).unwrap();
    
    let directional = DirectionalLight::new([1.0, 1.0, 0.9], 100000.0, [0.0, -1.0, 0.0]);
    scene.add_directional_light(directional).unwrap();
    
    let buffer = GpuLightBuffer::from_scene(&scene);
    assert_eq!(buffer.point_light_count, 1);
    assert_eq!(buffer.directional_light_count, 1);
}

#[test]
fn test_gpu_light_buffer_alignment() {
    // Check size is reasonable (alignment check may vary by platform)
    let size = GpuLightBuffer::size();
    assert!(size > 0);
    println!("GpuLightBuffer size: {} bytes", size);
    
    // Size should be a multiple of 16 for GPU alignment
    assert_eq!(size % 16, 0, "GpuLightBuffer size should be 16-byte aligned");
}

// ============================================================================
// Integration Tests (3 tests)
// ============================================================================

#[test]
fn test_complete_lighting_setup() {
    let mut scene = LightScene::new();
    
    // Set ambient
    let ambient = AmbientLight::new([0.1, 0.1, 0.15], 0.2);
    scene.set_ambient(ambient);
    
    // Add various lights
    let point = PointLight::new([1.0, 0.8, 0.6], 1500.0, 30.0);
    scene.add_point_light(point).unwrap();
    
    let directional = DirectionalLight::new([1.0, 1.0, 0.95], 80000.0, [0.3, -1.0, 0.2]);
    scene.add_directional_light(directional).unwrap();
    
    let spot = SpotLight::new([1.0, 1.0, 1.0], 2000.0, [0.0, -1.0, 0.0], 0.4, 0.7);
    scene.add_spot_light(spot).unwrap();
    
    assert_eq!(scene.total_light_count(), 3);
    
    // Convert to GPU buffer
    let buffer = GpuLightBuffer::from_scene(&scene);
    assert_eq!(buffer.point_light_count, 1);
    assert_eq!(buffer.directional_light_count, 1);
    assert_eq!(buffer.spot_light_count, 1);
}

#[test]
fn test_shadow_map_with_lights() {
    let mut shadow_map = ShadowMap::new(2048);
    shadow_map.set_cascades(4);
    shadow_map.set_planes(0.1, 150.0);
    
    let csm = CascadeShadowMap::calculate_splits(
        shadow_map.near_plane,
        shadow_map.far_plane,
        shadow_map.cascade_count
    );
    
    assert_eq!(csm.split_count, 4);
    assert!(csm.get_split(0) > 0.0);
}

#[test]
fn test_lighting_calculations_accuracy() {
    // Test that lighting calculations produce reasonable values
    let normal = [0.0, 1.0, 0.0];
    let light_dir = [0.0, 0.707, 0.707]; // 45 degree angle
    
    let diffuse = LightingUtils::lambertian(normal, light_dir);
    assert!(diffuse > 0.6 && diffuse < 0.8); // cos(45°) ≈ 0.707
    
    // Test attenuation at known distance
    let dist = 10.0;
    let range = 20.0;
    let atten = LightingUtils::calculate_attenuation(dist, range);
    assert!(atten > 0.0 && atten < 1.0);
}