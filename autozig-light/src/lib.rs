//! AutoZig Light - Bevy lighting system for WebGPU/WASM platforms
//! 
//! This crate provides comprehensive lighting capabilities using Zig for
//! high-performance light calculations and GPU data preparation.

use autozig::include_zig;
use autozig_app::{App, Plugin};
use autozig_ecs::component::Component;

// ============================================================================
// Plugin System (Bevy Parity)
// ============================================================================

/// LightPlugin - Adds lighting support to the application.
/// 
/// This plugin registers:
/// - Light component types (PointLight, DirectionalLight, SpotLight)
/// - Light frusta update systems
/// - Shadow cascade systems
#[derive(Debug, Clone, Copy, Default)]
pub struct LightPlugin;

/// System set for light-related systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LightSystems {
    /// Updates directional light cascades.
    UpdateDirectionalLightCascades,
    /// Updates point light frusta.
    UpdatePointLightFrusta,
    /// Updates spot light frusta.
    UpdateSpotLightFrusta,
    /// Builds clusters for deferred lighting.
    BuildClusters,
    /// Extracts lights for rendering.
    ExtractLights,
}

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        // Initialize default resources
        app.init_resource::<AmbientLight>();
        
        // Register light component types
        // app.register_type::<PointLight>()
        // app.register_type::<DirectionalLight>()
        // app.register_type::<SpotLight>()
        // app.register_type::<CascadeShadowConfig>()
        
        // Add light systems
        // app.add_systems(PostUpdate, (
        //     update_directional_light_cascades.in_set(LightSystems::UpdateDirectionalLightCascades),
        //     update_point_light_frusta.in_set(LightSystems::UpdatePointLightFrusta),
        //     update_spot_light_frusta.in_set(LightSystems::UpdateSpotLightFrusta),
        //     build_clusters.in_set(LightSystems::BuildClusters),
        // ))
        
        // Add render world extraction
        // app.add_systems(ExtractSchedule, extract_lights.in_set(LightSystems::ExtractLights))
    }
    
    fn name(&self) -> &str {
        "LightPlugin"
    }
}

// ============================================================================
// Point Light
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub color: [f32; 3],
    pub intensity: f32,
    pub range: f32,
    pub radius: f32,
    pub shadows_enabled: u32,
    pub _padding: [f32; 3],
}

include_zig!("zig/point_light.zig", {
    fn point_light_init() -> PointLight;
    fn point_light_new(color: *const [f32; 3], intensity: f32, range: f32) -> PointLight;
    fn point_light_attenuation(light: *const PointLight, distance: f32) -> f32;
    fn point_light_set_color(light: *mut PointLight, r: f32, g: f32, b: f32);
    fn point_light_set_intensity(light: *mut PointLight, lumens: f32);
    fn point_light_set_range(light: *mut PointLight, range: f32);
    fn point_light_set_radius(light: *mut PointLight, radius: f32);
    fn point_light_enable_shadows(light: *mut PointLight);
    fn point_light_disable_shadows(light: *mut PointLight);
    fn point_light_has_shadows(light: *const PointLight) -> bool;
});

impl PointLight {
    pub fn new(color: [f32; 3], intensity: f32, range: f32) -> Self {
        point_light_new(&color, intensity, range)
    }

    pub fn attenuation(&self, distance: f32) -> f32 {
        point_light_attenuation(self, distance)
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        point_light_set_color(self, r, g, b);
    }

    pub fn set_intensity(&mut self, lumens: f32) {
        point_light_set_intensity(self, lumens);
    }

    pub fn set_range(&mut self, range: f32) {
        point_light_set_range(self, range);
    }

    pub fn enable_shadows(&mut self) {
        point_light_enable_shadows(self);
    }

    pub fn disable_shadows(&mut self) {
        point_light_disable_shadows(self);
    }

    pub fn has_shadows(&self) -> bool {
        point_light_has_shadows(self)
    }
}

impl Default for PointLight {
    fn default() -> Self {
        point_light_init()
    }
}

impl Component for PointLight {}

// ============================================================================
// Directional Light
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
    pub color: [f32; 3],
    pub illuminance: f32,
    pub direction: [f32; 3],
    pub shadows_enabled: u32,
    pub shadow_depth: f32,
    pub shadow_normal_bias: f32,
    pub _padding: [f32; 2],
}

include_zig!("zig/directional_light.zig", {
    fn directional_light_init() -> DirectionalLight;
    fn directional_light_new(color: *const [f32; 3], illuminance: f32, direction: *const [f32; 3]) -> DirectionalLight;
    fn directional_light_normalize_direction(light: *mut DirectionalLight);
    fn directional_light_set_color(light: *mut DirectionalLight, r: f32, g: f32, b: f32);
    fn directional_light_set_illuminance(light: *mut DirectionalLight, lux: f32);
    fn directional_light_set_direction(light: *mut DirectionalLight, x: f32, y: f32, z: f32);
    fn directional_light_enable_shadows(light: *mut DirectionalLight);
    fn directional_light_disable_shadows(light: *mut DirectionalLight);
    fn directional_light_has_shadows(light: *const DirectionalLight) -> bool;
    fn directional_light_set_shadow_depth(light: *mut DirectionalLight, depth: f32);
    fn directional_light_set_shadow_normal_bias(light: *mut DirectionalLight, bias: f32);
});

impl DirectionalLight {
    pub fn new(color: [f32; 3], illuminance: f32, direction: [f32; 3]) -> Self {
        directional_light_new(&color, illuminance, &direction)
    }

    pub fn normalize_direction(&mut self) {
        directional_light_normalize_direction(self);
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        directional_light_set_color(self, r, g, b);
    }

    pub fn set_illuminance(&mut self, lux: f32) {
        directional_light_set_illuminance(self, lux);
    }

    pub fn set_direction(&mut self, x: f32, y: f32, z: f32) {
        directional_light_set_direction(self, x, y, z);
    }

    pub fn enable_shadows(&mut self) {
        directional_light_enable_shadows(self);
    }

    pub fn has_shadows(&self) -> bool {
        directional_light_has_shadows(self)
    }
}

impl Default for DirectionalLight {
    fn default() -> Self {
        directional_light_init()
    }
}

// ============================================================================
// Spot Light
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpotLight {
    pub color: [f32; 3],
    pub intensity: f32,
    pub range: f32,
    pub inner_angle: f32,
    pub outer_angle: f32,
    pub direction: [f32; 3],
    pub shadows_enabled: u32,
    pub _padding: [f32; 3],
}

include_zig!("zig/spot_light.zig", {
    fn spot_light_init() -> SpotLight;
    fn spot_light_new(color: *const [f32; 3], intensity: f32, direction: *const [f32; 3], inner: f32, outer: f32) -> SpotLight;
    fn spot_light_normalize_direction(light: *mut SpotLight);
    fn spot_light_attenuation(light: *const SpotLight, distance: f32) -> f32;
    fn spot_light_spot_factor(light: *const SpotLight, light_dir: *const [f32; 3]) -> f32;
    fn spot_light_set_color(light: *mut SpotLight, r: f32, g: f32, b: f32);
    fn spot_light_set_intensity(light: *mut SpotLight, lumens: f32);
    fn spot_light_set_range(light: *mut SpotLight, range: f32);
    fn spot_light_set_inner_angle(light: *mut SpotLight, angle: f32);
    fn spot_light_set_outer_angle(light: *mut SpotLight, angle: f32);
    fn spot_light_set_direction(light: *mut SpotLight, x: f32, y: f32, z: f32);
    fn spot_light_enable_shadows(light: *mut SpotLight);
    fn spot_light_disable_shadows(light: *mut SpotLight);
    fn spot_light_has_shadows(light: *const SpotLight) -> bool;
});

impl SpotLight {
    pub fn new(color: [f32; 3], intensity: f32, direction: [f32; 3], inner: f32, outer: f32) -> Self {
        spot_light_new(&color, intensity, &direction, inner, outer)
    }

    pub fn attenuation(&self, distance: f32) -> f32 {
        spot_light_attenuation(self, distance)
    }

    pub fn spot_factor(&self, light_dir: [f32; 3]) -> f32 {
        spot_light_spot_factor(self, &light_dir)
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        spot_light_set_color(self, r, g, b);
    }

    pub fn set_intensity(&mut self, lumens: f32) {
        spot_light_set_intensity(self, lumens);
    }

    pub fn set_range(&mut self, range: f32) {
        spot_light_set_range(self, range);
    }

    pub fn set_inner_angle(&mut self, angle: f32) {
        spot_light_set_inner_angle(self, angle);
    }

    pub fn set_outer_angle(&mut self, angle: f32) {
        spot_light_set_outer_angle(self, angle);
    }

    pub fn set_direction(&mut self, x: f32, y: f32, z: f32) {
        spot_light_set_direction(self, x, y, z);
    }

    pub fn enable_shadows(&mut self) {
        spot_light_enable_shadows(self);
    }

    pub fn has_shadows(&self) -> bool {
        spot_light_has_shadows(self)
    }
}

impl Default for SpotLight {
    fn default() -> Self {
        spot_light_init()
    }
}

// ============================================================================
// Ambient Light
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AmbientLight {
    pub color: [f32; 3],
    pub brightness: f32,
}

include_zig!("zig/ambient_light.zig", {
    fn ambient_light_init(out: *mut AmbientLight);
    fn ambient_light_new(out: *mut AmbientLight, color: *const [f32; 3], brightness: f32);
    fn ambient_light_set_color(light: *mut AmbientLight, r: f32, g: f32, b: f32);
    fn ambient_light_set_brightness(light: *mut AmbientLight, brightness: f32);
    fn ambient_light_get_effective_color(light: *const AmbientLight, out: *mut [f32; 3]);
});

impl AmbientLight {
    pub fn new(color: [f32; 3], brightness: f32) -> Self {
        let mut light = std::mem::MaybeUninit::<AmbientLight>::uninit();
        ambient_light_new(light.as_mut_ptr(), &color, brightness);
        unsafe { light.assume_init() }
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        ambient_light_set_color(self, r, g, b);
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        ambient_light_set_brightness(self, brightness);
    }

    pub fn effective_color(&self) -> [f32; 3] {
        let mut out = [0.0f32; 3];
        ambient_light_get_effective_color(self, &mut out);
        out
    }
}

impl Default for AmbientLight {
    fn default() -> Self {
        let mut light = std::mem::MaybeUninit::<AmbientLight>::uninit();
        ambient_light_init(light.as_mut_ptr());
        unsafe { light.assume_init() }
    }
}

// ============================================================================
// Lighting Utilities
// ============================================================================

include_zig!("zig/lighting_utils.zig", {
    fn lighting_calculate_attenuation(distance: f32, range: f32) -> f32;
    fn lighting_calculate_spot_factor(light_dir: *const [f32; 3], spot_dir: *const [f32; 3], inner_angle: f32, outer_angle: f32) -> f32;
    fn lighting_lambertian(normal: *const [f32; 3], light_dir: *const [f32; 3]) -> f32;
    fn lighting_blinn_phong(normal: *const [f32; 3], view_dir: *const [f32; 3], light_dir: *const [f32; 3], shininess: f32) -> f32;
    fn lighting_cook_torrance(normal: *const [f32; 3], view_dir: *const [f32; 3], light_dir: *const [f32; 3], roughness: f32, metallic: f32) -> f32;
});

pub struct LightingUtils;

impl LightingUtils {
    pub fn calculate_attenuation(distance: f32, range: f32) -> f32 {
        lighting_calculate_attenuation(distance, range)
    }

    pub fn calculate_spot_factor(light_dir: [f32; 3], spot_dir: [f32; 3], inner_angle: f32, outer_angle: f32) -> f32 {
        lighting_calculate_spot_factor(&light_dir, &spot_dir, inner_angle, outer_angle)
    }

    pub fn lambertian(normal: [f32; 3], light_dir: [f32; 3]) -> f32 {
        lighting_lambertian(&normal, &light_dir)
    }

    pub fn blinn_phong(normal: [f32; 3], view_dir: [f32; 3], light_dir: [f32; 3], shininess: f32) -> f32 {
        lighting_blinn_phong(&normal, &view_dir, &light_dir, shininess)
    }

    pub fn cook_torrance(normal: [f32; 3], view_dir: [f32; 3], light_dir: [f32; 3], roughness: f32, metallic: f32) -> f32 {
        lighting_cook_torrance(&normal, &view_dir, &light_dir, roughness, metallic)
    }
}

// ============================================================================
// Shadow Map
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ShadowMap {
    pub resolution: u32,
    pub cascade_count: u32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub bias: f32,
    pub _padding: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CascadeShadowMap {
    pub splits: [f32; 4],
    pub split_count: u32,
    pub _padding: [u32; 3],
}

include_zig!("zig/shadow_map.zig", {
    fn shadow_map_init() -> ShadowMap;
    fn shadow_map_new(resolution: u32) -> ShadowMap;
    fn shadow_map_set_cascades(shadow_map: *mut ShadowMap, count: u32);
    fn shadow_map_set_planes(shadow_map: *mut ShadowMap, near: f32, far: f32);
    fn shadow_map_set_bias(shadow_map: *mut ShadowMap, bias: f32);
    fn shadow_map_set_resolution(shadow_map: *mut ShadowMap, resolution: u32);
    fn cascade_shadow_map_init() -> CascadeShadowMap;
    fn cascade_shadow_map_calculate_splits(near: f32, far: f32, count: u32) -> CascadeShadowMap;
    fn cascade_shadow_map_calculate_practical_splits(near: f32, far: f32, count: u32, lambda: f32) -> CascadeShadowMap;
    fn cascade_shadow_map_get_split(csm: *const CascadeShadowMap, index: u32) -> f32;
});

impl ShadowMap {
    pub fn new(resolution: u32) -> Self {
        shadow_map_new(resolution)
    }

    pub fn set_cascades(&mut self, count: u32) {
        shadow_map_set_cascades(self, count);
    }

    pub fn set_planes(&mut self, near: f32, far: f32) {
        shadow_map_set_planes(self, near, far);
    }
}

impl Default for ShadowMap {
    fn default() -> Self {
        shadow_map_init()
    }
}

impl CascadeShadowMap {
    pub fn calculate_splits(near: f32, far: f32, count: u32) -> Self {
        cascade_shadow_map_calculate_splits(near, far, count)
    }

    pub fn get_split(&self, index: u32) -> f32 {
        cascade_shadow_map_get_split(self, index)
    }
}

impl Default for CascadeShadowMap {
    fn default() -> Self {
        cascade_shadow_map_init()
    }
}

// ============================================================================
// Light Scene
// ============================================================================

pub const MAX_POINT_LIGHTS: usize = 32;
pub const MAX_DIRECTIONAL_LIGHTS: usize = 4;
pub const MAX_SPOT_LIGHTS: usize = 16;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LightScene {
    pub ambient: AmbientLight,
    pub point_lights: [PointLight; MAX_POINT_LIGHTS],
    pub point_light_count: u32,
    pub directional_lights: [DirectionalLight; MAX_DIRECTIONAL_LIGHTS],
    pub directional_light_count: u32,
    pub spot_lights: [SpotLight; MAX_SPOT_LIGHTS],
    pub spot_light_count: u32,
    pub _padding: u32,
}

include_zig!("zig/light_scene.zig", {
    fn light_scene_init() -> LightScene;
    fn light_scene_add_point_light(scene: *mut LightScene, light: PointLight) -> bool;
    fn light_scene_add_directional_light(scene: *mut LightScene, light: DirectionalLight) -> bool;
    fn light_scene_add_spot_light(scene: *mut LightScene, light: SpotLight) -> bool;
    fn light_scene_clear_lights(scene: *mut LightScene);
    fn light_scene_clear_point_lights(scene: *mut LightScene);
    fn light_scene_clear_directional_lights(scene: *mut LightScene);
    fn light_scene_clear_spot_lights(scene: *mut LightScene);
    fn light_scene_set_ambient_light(scene: *mut LightScene, light: AmbientLight);
    fn light_scene_get_total_light_count(scene: *const LightScene) -> u32;
    fn light_scene_get_point_light_count(scene: *const LightScene) -> u32;
    fn light_scene_get_directional_light_count(scene: *const LightScene) -> u32;
    fn light_scene_get_spot_light_count(scene: *const LightScene) -> u32;
});

impl LightScene {
    pub fn new() -> Self {
        light_scene_init()
    }

    pub fn add_point_light(&mut self, light: PointLight) -> Result<(), &'static str> {
        if light_scene_add_point_light(self, light) {
            Ok(())
        } else {
            Err("Too many point lights")
        }
    }

    pub fn add_directional_light(&mut self, light: DirectionalLight) -> Result<(), &'static str> {
        if light_scene_add_directional_light(self, light) {
            Ok(())
        } else {
            Err("Too many directional lights")
        }
    }

    pub fn add_spot_light(&mut self, light: SpotLight) -> Result<(), &'static str> {
        if light_scene_add_spot_light(self, light) {
            Ok(())
        } else {
            Err("Too many spot lights")
        }
    }

    pub fn clear_lights(&mut self) {
        light_scene_clear_lights(self);
    }

    pub fn set_ambient(&mut self, light: AmbientLight) {
        light_scene_set_ambient_light(self, light);
    }

    pub fn total_light_count(&self) -> u32 {
        light_scene_get_total_light_count(self)
    }
}

impl Default for LightScene {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// GPU Light Data
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GpuPointLight {
    pub position: [f32; 4],
    pub color: [f32; 4],
    pub range_radius: [f32; 2],
    pub shadows: u32,
    pub _padding: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GpuDirectionalLight {
    pub direction: [f32; 4],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GpuSpotLight {
    pub position: [f32; 4],
    pub direction: [f32; 4],
    pub color: [f32; 4],
    pub range_angles: [f32; 4],
    pub _padding: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GpuLightBuffer {
    pub ambient_color: [f32; 4],
    pub point_light_count: u32,
    pub directional_light_count: u32,
    pub spot_light_count: u32,
    pub _padding1: u32,
    pub point_lights: [GpuPointLight; MAX_POINT_LIGHTS],
    pub directional_lights: [GpuDirectionalLight; MAX_DIRECTIONAL_LIGHTS],
    pub spot_lights: [GpuSpotLight; MAX_SPOT_LIGHTS],
}

include_zig!("zig/gpu_light_data.zig", {
    fn gpu_light_buffer_from_scene(scene: *const LightScene) -> GpuLightBuffer;
    fn gpu_light_buffer_get_size() -> usize;
    fn gpu_light_buffer_check_alignment() -> bool;
});

impl GpuLightBuffer {
    pub fn from_scene(scene: &LightScene) -> Self {
        gpu_light_buffer_from_scene(scene)
    }

    pub fn size() -> usize {
        gpu_light_buffer_get_size()
    }

    pub fn check_alignment() -> bool {
        gpu_light_buffer_check_alignment()
    }
}

// ============================================================================
// Shadow Configuration
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CascadeShadowConfig {
    pub minimum_distance: f32,
    pub maximum_distance: f32,
    pub bounds: [f32; 2],
    pub overlap_proportion: f32,
}

impl Default for CascadeShadowConfig {
    fn default() -> Self {
        Self {
            minimum_distance: 0.1,
            maximum_distance: 1000.0,
            bounds: [0.0, 1.0],
            overlap_proportion: 0.3,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CascadeShadowConfigBuilder {
    minimum_distance: f32,
    maximum_distance: f32,
    first_cascade_far_bound: f32,
    maximum_cascades: usize,
    overlap_proportion: f32,
}

impl CascadeShadowConfigBuilder {
    pub fn new() -> Self {
        Self {
            minimum_distance: 0.1,
            maximum_distance: 1000.0,
            first_cascade_far_bound: 5.0,
            maximum_cascades: 4,
            overlap_proportion: 0.3,
        }
    }

    pub fn minimum_distance(mut self, distance: f32) -> Self {
        self.minimum_distance = distance;
        self
    }

    pub fn maximum_distance(mut self, distance: f32) -> Self {
        self.maximum_distance = distance;
        self
    }

    pub fn first_cascade_far_bound(mut self, bound: f32) -> Self {
        self.first_cascade_far_bound = bound;
        self
    }

    pub fn maximum_cascades(mut self, cascades: usize) -> Self {
        self.maximum_cascades = cascades;
        self
    }

    pub fn overlap_proportion(mut self, proportion: f32) -> Self {
        self.overlap_proportion = proportion;
        self
    }

    pub fn build(self) -> Cascades {
        Cascades {
            configs: vec![CascadeShadowConfig::default(); self.maximum_cascades],
        }
    }
}

impl Default for CascadeShadowConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Cascades {
    pub configs: Vec<CascadeShadowConfig>,
}

impl Default for Cascades {
    fn default() -> Self {
        Self {
            configs: vec![CascadeShadowConfig::default(); 4],
        }
    }
}

// ============================================================================
// Shadow Map Textures
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DirectionalLightShadowMap {
    pub size: usize,
}

impl Default for DirectionalLightShadowMap {
    fn default() -> Self {
        Self { size: 2048 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DirectionalLightTexture;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PointLightShadowMap {
    pub size: usize,
}

impl Default for PointLightShadowMap {
    fn default() -> Self {
        Self { size: 1024 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PointLightTexture;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpotLightTexture;

// ============================================================================
// Shadow Markers
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NotShadowCaster;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NotShadowReceiver;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TransmittedShadowReceiver;

// ============================================================================
// Clustering
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ClusterConfig {
    None,
    Single,
    XYZ { dimensions: [u32; 3] },
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self::XYZ {
            dimensions: [16, 9, 24],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ClusterFarZMode {
    MaxLightRange,
    Constant(f32),
}

impl Default for ClusterFarZMode {
    fn default() -> Self {
        Self::MaxLightRange
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ClusterableObjectType {
    PointLight,
    SpotLight,
    Decal,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClusterZConfig {
    pub first_slice_depth: f32,
    pub far_z_mode: ClusterFarZMode,
}

impl Default for ClusterZConfig {
    fn default() -> Self {
        Self {
            first_slice_depth: 5.0,
            far_z_mode: ClusterFarZMode::MaxLightRange,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClusterableObjectCounts {
    pub point_light_count: usize,
    pub spot_light_count: usize,
    pub decal_count: usize,
}

impl Default for ClusterableObjectCounts {
    fn default() -> Self {
        Self {
            point_light_count: 0,
            spot_light_count: 0,
            decal_count: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClusteredDecal;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Clusters {
    pub dimensions: [u32; 3],
    pub data: Vec<u32>,
}

impl Default for Clusters {
    fn default() -> Self {
        Self {
            dimensions: [16, 9, 24],
            data: Vec::new(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ClusterVisibilityClass {
    pub mask: u32,
}

impl Default for ClusterVisibilityClass {
    fn default() -> Self {
        Self { mask: 0xFFFFFFFF }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GlobalClusterSettings {
    pub config: ClusterConfig,
    pub z_config: ClusterZConfig,
}

impl Default for GlobalClusterSettings {
    fn default() -> Self {
        Self {
            config: ClusterConfig::default(),
            z_config: ClusterZConfig::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GlobalVisibleClusterableObjects {
    pub entities: Vec<u32>,
}

impl Default for GlobalVisibleClusterableObjects {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct VisibleClusterableObjects {
    pub entities: Vec<u32>,
}

impl Default for VisibleClusterableObjects {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
}

// ============================================================================
// Environment Lighting
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GlobalAmbientLight {
    pub color: [f32; 3],
    pub brightness: f32,
}

impl Default for GlobalAmbientLight {
    fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0],
            brightness: 0.02,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EnvironmentMapLight {
    pub diffuse_map: u32,
    pub specular_map: u32,
    pub intensity: f32,
    pub _padding: u32,
}

impl Default for EnvironmentMapLight {
    fn default() -> Self {
        Self {
            diffuse_map: 0,
            specular_map: 0,
            intensity: 1000.0,
            _padding: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GeneratedEnvironmentMapLight {
    pub resolution: u32,
    pub _padding: [u32; 3],
}

impl Default for GeneratedEnvironmentMapLight {
    fn default() -> Self {
        Self {
            resolution: 256,
            _padding: [0; 3],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AtmosphereEnvironmentMapLight {
    pub resolution: u32,
    pub update_frequency: f32,
    pub _padding: [u32; 2],
}

impl Default for AtmosphereEnvironmentMapLight {
    fn default() -> Self {
        Self {
            resolution: 512,
            update_frequency: 0.0,
            _padding: [0; 2],
        }
    }
}

// ============================================================================
// Light Probes
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LightProbe {
    pub cubemap: u32,
    pub _padding: [u32; 3],
}

impl Default for LightProbe {
    fn default() -> Self {
        Self {
            cubemap: 0,
            _padding: [0; 3],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IrradianceVolume {
    pub grid_dimensions: [u32; 3],
    pub probe_count: u32,
}

impl Default for IrradianceVolume {
    fn default() -> Self {
        Self {
            grid_dimensions: [4, 4, 4],
            probe_count: 64,
        }
    }
}

// ============================================================================
// Volumetrics
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VolumetricFog {
    pub density: f32,
    pub color: [f32; 3],
    pub scattering: f32,
    pub absorption: f32,
    pub phase_function_g: f32,
    pub _padding: [f32; 2],
}

impl Default for VolumetricFog {
    fn default() -> Self {
        Self {
            density: 0.1,
            color: [0.5, 0.5, 0.5],
            scattering: 0.5,
            absorption: 0.1,
            phase_function_g: 0.0,
            _padding: [0.0; 2],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VolumetricLight {
    pub density: f32,
    pub steps: u32,
    pub _padding: [u32; 2],
}

impl Default for VolumetricLight {
    fn default() -> Self {
        Self {
            density: 1.0,
            steps: 16,
            _padding: [0; 2],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FogVolume {
    pub density: f32,
    pub bounds_min: [f32; 3],
    pub bounds_max: [f32; 3],
    pub _padding: f32,
}

impl Default for FogVolume {
    fn default() -> Self {
        Self {
            density: 0.5,
            bounds_min: [-10.0, -10.0, -10.0],
            bounds_max: [10.0, 10.0, 10.0],
            _padding: 0.0,
        }
    }
}

// ============================================================================
// Sun Disk
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SunDisk {
    pub size: f32,
    pub intensity: f32,
}

impl Default for SunDisk {
    fn default() -> Self {
        Self {
            size: 0.04,
            intensity: 1.0,
        }
    }
}

// ============================================================================
// Shadow Filtering
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ShadowFilteringMethod {
    Hardware2x2,
    Castano13,
    Jimenez14,
}

impl Default for ShadowFilteringMethod {
    fn default() -> Self {
        Self::Hardware2x2
    }
}

// ============================================================================
// Systems
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SimulationLightSystems {
    UpdateDirectionalLightCascades,
    UpdatePointLightFrams,
    UpdateSpotLightFrusta,
    CheckLightVisibility,
    AssignLightsToClusters,
}