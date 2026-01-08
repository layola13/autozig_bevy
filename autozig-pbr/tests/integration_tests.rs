//! AutoZig-PBR 集成测试
//! 
//! 全面测试 PBR 材质系统的所有功能

use autozig_pbr::prelude::*;

// ============================================================================
// 材质创建和属性测试 (Tests 1-10)
// ============================================================================

#[test]
fn test_01_material_default_creation() {
    let material = StandardMaterial::new();
    assert_eq!(material.base_color(), [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(material.metallic(), 0.0);
    assert_eq!(material.roughness(), 0.5);
    assert_eq!(material.emissive(), [0.0, 0.0, 0.0]);
}

#[test]
fn test_02_material_base_color() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 0.0, 0.0, 1.0);
    assert_eq!(material.base_color(), [1.0, 0.0, 0.0, 1.0]);
}

#[test]
fn test_03_material_metallic() {
    let material = StandardMaterial::new()
        .with_metallic(0.8);
    assert_eq!(material.metallic(), 0.8);
}

#[test]
fn test_04_material_roughness() {
    let material = StandardMaterial::new()
        .with_roughness(0.2);
    assert_eq!(material.roughness(), 0.2);
}

#[test]
fn test_05_material_emissive() {
    let material = StandardMaterial::new()
        .with_emissive(0.5, 0.5, 0.5);
    assert_eq!(material.emissive(), [0.5, 0.5, 0.5]);
}

#[test]
fn test_06_material_builder_pattern() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 0.0, 0.0, 1.0)
        .with_metallic(0.8)
        .with_roughness(0.2)
        .with_emissive(0.1, 0.1, 0.1);
    
    assert_eq!(material.base_color(), [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(material.metallic(), 0.8);
    assert_eq!(material.roughness(), 0.2);
    assert_eq!(material.emissive(), [0.1, 0.1, 0.1]);
}

#[test]
fn test_07_metallic_clamping_upper() {
    let material = StandardMaterial::new().with_metallic(1.5);
    assert_eq!(material.metallic(), 1.0);
}

#[test]
fn test_08_metallic_clamping_lower() {
    let material = StandardMaterial::new().with_metallic(-0.5);
    assert_eq!(material.metallic(), 0.0);
}

#[test]
fn test_09_roughness_clamping_upper() {
    let material = StandardMaterial::new().with_roughness(1.5);
    assert_eq!(material.roughness(), 1.0);
}

#[test]
fn test_10_roughness_clamping_lower() {
    let material = StandardMaterial::new().with_roughness(-0.5);
    assert_eq!(material.roughness(), 0.0);
}

// ============================================================================
// 纹理绑定测试 (Tests 11-15)
// ============================================================================

#[test]
fn test_11_base_color_texture() {
    let texture_data = vec![255u8, 0, 0, 255]; // 红色 1x1 纹理
    let material = StandardMaterial::new()
        .with_base_color_texture(texture_data);
    
    // 验证材质创建成功
    assert_eq!(material.base_color(), [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn test_12_normal_texture() {
    let texture_data = vec![128u8, 128, 255, 255]; // 法线贴图 1x1
    let material = StandardMaterial::new()
        .with_normal_texture(texture_data);
    
    assert_eq!(material.metallic(), 0.0);
}

#[test]
fn test_13_metallic_roughness_texture() {
    let texture_data = vec![0u8, 128, 255, 255]; // 金属度/粗糙度 1x1
    let material = StandardMaterial::new()
        .with_metallic_roughness_texture(texture_data);
    
    assert_eq!(material.roughness(), 0.5);
}

#[test]
fn test_14_emissive_texture() {
    let texture_data = vec![255u8, 255, 255, 255]; // 自发光 1x1
    let material = StandardMaterial::new()
        .with_emissive_texture(texture_data);
    
    assert_eq!(material.emissive(), [0.0, 0.0, 0.0]);
}

#[test]
fn test_15_multiple_textures() {
    let texture1 = vec![255u8, 0, 0, 255];
    let texture2 = vec![128u8, 128, 255, 255];
    
    let material = StandardMaterial::new()
        .with_base_color_texture(texture1)
        .with_normal_texture(texture2);
    
    assert_eq!(material.base_color(), [1.0, 1.0, 1.0, 1.0]);
}

// ============================================================================
// PBR 光照计算测试 (Tests 16-25)
// ============================================================================

#[test]
fn test_16_lighting_calculation_basic() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 0.0, 0.0, 1.0);
    
    let result = material.calculate_lighting(
        [0.0, 0.0, 0.0],        // position
        [0.0, 1.0, 0.0],        // normal
        [0.0, 1.0, 0.0],        // view_dir
        [0.0, 1.0, 0.0],        // light_dir
        [1.0, 1.0, 1.0],        // light_color
        1.0,                     // light_intensity
    );
    
    // 验证结果不是零
    assert!(result[0] > 0.0 || result[1] > 0.0 || result[2] > 0.0);
}

#[test]
fn test_17_lighting_with_metallic() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 1.0, 1.0, 1.0)
        .with_metallic(1.0)
        .with_roughness(0.0);
    
    let result = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        1.0,
    );
    
    assert!(result[0] > 0.0 && result[1] > 0.0 && result[2] > 0.0);
}

#[test]
fn test_18_lighting_with_roughness() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 1.0, 1.0, 1.0)
        .with_metallic(0.0)
        .with_roughness(1.0);
    
    let result = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        1.0,
    );
    
    assert!(result[0] > 0.0 || result[1] > 0.0 || result[2] > 0.0);
}

#[test]
fn test_19_lighting_with_emissive() {
    let material = StandardMaterial::new()
        .with_base_color(0.0, 0.0, 0.0, 1.0)
        .with_emissive(1.0, 1.0, 1.0);
    
    let result = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0],  // 无光照
        0.0,
    );
    
    // 自发光应该使结果非零
    assert!(result[0] > 0.0 && result[1] > 0.0 && result[2] > 0.0);
}

#[test]
fn test_20_lighting_different_angles() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 1.0, 1.0, 1.0);
    
    // 45度角光照
    let result1 = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.707, 0.707, 0.0],  // 45度
        [1.0, 1.0, 1.0],
        1.0,
    );
    
    // 90度角光照
    let result2 = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],  // 90度
        [1.0, 1.0, 1.0],
        1.0,
    );
    
    // 45度应该比90度更亮
    let brightness1 = result1[0] + result1[1] + result1[2];
    let brightness2 = result2[0] + result2[1] + result2[2];
    assert!(brightness1 > brightness2);
}

#[test]
fn test_21_lighting_colored_light() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 1.0, 1.0, 1.0);
    
    let result = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],  // 红光
        1.0,
    );
    
    // 红光应该使红色分量最高
    assert!(result[0] > result[1] && result[0] > result[2]);
}

#[test]
fn test_22_lighting_intensity_scaling() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 1.0, 1.0, 1.0);
    
    let result1 = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        0.5,  // 50% 强度
    );
    
    let result2 = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        1.0,  // 100% 强度
    );
    
    // 100%强度应该更亮
    let brightness1 = result1[0] + result1[1] + result1[2];
    let brightness2 = result2[0] + result2[1] + result2[2];
    assert!(brightness2 > brightness1);
}

#[test]
fn test_23_batch_lighting_single_vertex() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 0.0, 0.0, 1.0);
    
    let materials = vec![material.handle()];
    let positions = vec![0.0f32, 0.0, 0.0];
    let normals = vec![0.0f32, 1.0, 0.0];
    let view_dirs = vec![0.0f32, 1.0, 0.0];
    let lights = vec![LightData {
        position: [0.0, 10.0, 0.0],
        direction: [0.0, -1.0, 0.0],
        color: [1.0, 1.0, 1.0],
        intensity: 1.0,
        radius: 100.0,
        _padding: [0.0; 3],
    }];
    
    let result = PbrLightingCalculator::calculate_batch_simd(
        &materials,
        &positions,
        &normals,
        &view_dirs,
        &lights,
    );
    
    assert_eq!(result.len(), 3);
    assert!(result[0] > 0.0 || result[1] > 0.0 || result[2] > 0.0);
}

#[test]
fn test_24_batch_lighting_multiple_vertices() {
    let material1 = StandardMaterial::new().with_base_color(1.0, 0.0, 0.0, 1.0);
    let material2 = StandardMaterial::new().with_base_color(0.0, 1.0, 0.0, 1.0);
    
    let materials = vec![material1.handle(), material2.handle()];
    let positions = vec![
        0.0f32, 0.0, 0.0,
        1.0, 0.0, 0.0,
    ];
    let normals = vec![
        0.0f32, 1.0, 0.0,
        0.0, 1.0, 0.0,
    ];
    let view_dirs = vec![
        0.0f32, 1.0, 0.0,
        0.0, 1.0, 0.0,
    ];
    let lights = vec![LightData {
        position: [0.5, 10.0, 0.0],
        direction: [0.0, -1.0, 0.0],
        color: [1.0, 1.0, 1.0],
        intensity: 1.0,
        radius: 100.0,
        _padding: [0.0; 3],
    }];
    
    let result = PbrLightingCalculator::calculate_batch_simd(
        &materials,
        &positions,
        &normals,
        &view_dirs,
        &lights,
    );
    
    assert_eq!(result.len(), 6);
}

#[test]
fn test_25_batch_lighting_multiple_lights() {
    let material = StandardMaterial::new().with_base_color(1.0, 1.0, 1.0, 1.0);
    
    let materials = vec![material.handle()];
    let positions = vec![0.0f32, 0.0, 0.0];
    let normals = vec![0.0f32, 1.0, 0.0];
    let view_dirs = vec![0.0f32, 1.0, 0.0];
    
    let lights = vec![
        LightData {
            position: [10.0, 10.0, 0.0],
            direction: [-1.0, -1.0, 0.0],
            color: [1.0, 0.0, 0.0],  // 红光
            intensity: 1.0,
            radius: 100.0,
            _padding: [0.0; 3],
        },
        LightData {
            position: [-10.0, 10.0, 0.0],
            direction: [1.0, -1.0, 0.0],
            color: [0.0, 0.0, 1.0],  // 蓝光
            intensity: 1.0,
            radius: 100.0,
            _padding: [0.0; 3],
        },
    ];
    
    let result = PbrLightingCalculator::calculate_batch_simd(
        &materials,
        &positions,
        &normals,
        &view_dirs,
        &lights,
    );
    
    assert_eq!(result.len(), 3);
    // 红光+蓝光应该产生混合颜色
    assert!(result[0] > 0.0 && result[2] > 0.0);
}

// ============================================================================
// 材质预设测试 (Tests 26-30)
// ============================================================================

#[test]
fn test_26_metallic_material_preset() {
    let material = StandardMaterial::new()
        .with_base_color(0.8, 0.8, 0.8, 1.0)
        .with_metallic(1.0)
        .with_roughness(0.2);
    
    assert_eq!(material.metallic(), 1.0);
    assert_eq!(material.roughness(), 0.2);
}

#[test]
fn test_27_dielectric_material_preset() {
    let material = StandardMaterial::new()
        .with_base_color(0.5, 0.5, 0.5, 1.0)
        .with_metallic(0.0)
        .with_roughness(0.8);
    
    assert_eq!(material.metallic(), 0.0);
    assert_eq!(material.roughness(), 0.8);
}

#[test]
fn test_28_emissive_material_preset() {
    let material = StandardMaterial::new()
        .with_base_color(0.0, 0.0, 0.0, 1.0)
        .with_emissive(1.0, 1.0, 0.0);  // 黄色发光
    
    assert_eq!(material.emissive(), [1.0, 1.0, 0.0]);
}

#[test]
fn test_29_rough_plastic_preset() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 0.0, 0.0, 1.0)
        .with_metallic(0.0)
        .with_roughness(0.9);
    
    assert_eq!(material.metallic(), 0.0);
    assert_eq!(material.roughness(), 0.9);
}

#[test]
fn test_30_smooth_metal_preset() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 0.84, 0.0, 1.0)  // 金色
        .with_metallic(1.0)
        .with_roughness(0.1);
    
    assert_eq!(material.metallic(), 1.0);
    assert_eq!(material.roughness(), 0.1);
}

// ============================================================================
// 边界条件和错误处理测试 (Tests 31-35)
// ============================================================================

#[test]
fn test_31_zero_intensity_light() {
    let material = StandardMaterial::new();
    
    let result = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        0.0,  // 零强度
    );
    
    // 应该只有环境光
    assert!(result[0] >= 0.0 && result[1] >= 0.0 && result[2] >= 0.0);
}

#[test]
fn test_32_parallel_normal_and_light() {
    let material = StandardMaterial::new();
    
    // 法线和光线平行（最大光照）
    let result = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        1.0,
    );
    
    assert!(result[0] > 0.0 || result[1] > 0.0 || result[2] > 0.0);
}

#[test]
fn test_33_perpendicular_normal_and_light() {
    let material = StandardMaterial::new();
    
    // 法线和光线垂直（无光照，只有环境光）
    let result = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 0.0, 0.0],  // 垂直
        [1.0, 1.0, 1.0],
        1.0,
    );
    
    // 应该接近环境光强度
    assert!(result[0] >= 0.0 && result[1] >= 0.0 && result[2] >= 0.0);
}

#[test]
fn test_34_black_base_color() {
    let material = StandardMaterial::new()
        .with_base_color(0.0, 0.0, 0.0, 1.0);
    
    let result = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        1.0,
    );
    
    // 黑色基础颜色应该产生很暗的结果
    assert!(result[0] < 0.1 && result[1] < 0.1 && result[2] < 0.1);
}

#[test]
fn test_35_maximum_values() {
    let material = StandardMaterial::new()
        .with_base_color(1.0, 1.0, 1.0, 1.0)
        .with_metallic(1.0)
        .with_roughness(0.0)
        .with_emissive(1.0, 1.0, 1.0);
    
    let result = material.calculate_lighting(
        [0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 1.0],
        10.0,  // 高强度
    );
    
    // 应该非常亮
    assert!(result[0] > 0.5 && result[1] > 0.5 && result[2] > 0.5);
}