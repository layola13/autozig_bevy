//! AutoZig Light - Bevy lighting system for WebGPU/WASM platforms
//! 
//! This crate provides comprehensive lighting capabilities using Zig for
//! high-performance light calculations and GPU data preparation.

use autozig::include_zig;

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