//! AutoZig Light - Main entry point
//! Exports all lighting functionality for FFI

// Re-export all public modules
pub const point_light = @import("point_light.zig");
pub const directional_light = @import("directional_light.zig");
pub const spot_light = @import("spot_light.zig");
pub const ambient_light = @import("ambient_light.zig");
pub const lighting_utils = @import("lighting_utils.zig");
pub const shadow_map = @import("shadow_map.zig");
pub const light_scene = @import("light_scene.zig");
pub const gpu_light_data = @import("gpu_light_data.zig");

// Re-export main types
pub const PointLight = point_light.PointLight;
pub const DirectionalLight = directional_light.DirectionalLight;
pub const SpotLight = spot_light.SpotLight;
pub const AmbientLight = ambient_light.AmbientLight;
pub const ShadowMap = shadow_map.ShadowMap;
pub const CascadeShadowMap = shadow_map.CascadeShadowMap;
pub const LightScene = light_scene.LightScene;
pub const GpuPointLight = gpu_light_data.GpuPointLight;
pub const GpuDirectionalLight = gpu_light_data.GpuDirectionalLight;
pub const GpuSpotLight = gpu_light_data.GpuSpotLight;
pub const GpuLightBuffer = gpu_light_data.GpuLightBuffer;

// Constants
pub const MAX_POINT_LIGHTS = light_scene.MAX_POINT_LIGHTS;
pub const MAX_DIRECTIONAL_LIGHTS = light_scene.MAX_DIRECTIONAL_LIGHTS;
pub const MAX_SPOT_LIGHTS = light_scene.MAX_SPOT_LIGHTS;
