//! Light Scene management for autozig-light
//! Manages collections of lights in a scene

const std = @import("std");
const point_light = @import("point_light.zig");
const directional_light = @import("directional_light.zig");
const spot_light = @import("spot_light.zig");
const ambient_light = @import("ambient_light.zig");

const PointLight = point_light.PointLight;
const DirectionalLight = directional_light.DirectionalLight;
const SpotLight = spot_light.SpotLight;
const AmbientLight = ambient_light.AmbientLight;

/// Maximum number of lights per type
pub const MAX_POINT_LIGHTS = 32;
pub const MAX_DIRECTIONAL_LIGHTS = 4;
pub const MAX_SPOT_LIGHTS = 16;

/// Light scene containing all lights
pub const LightScene = extern struct {
    ambient: AmbientLight,
    point_lights: [MAX_POINT_LIGHTS]PointLight,
    point_light_count: u32,
    directional_lights: [MAX_DIRECTIONAL_LIGHTS]DirectionalLight,
    directional_light_count: u32,
    spot_lights: [MAX_SPOT_LIGHTS]SpotLight,
    spot_light_count: u32,
    _padding: u32, // Padding for alignment

    /// Initialize an empty light scene
    pub fn init() LightScene {
        var scene = LightScene{
            .ambient = AmbientLight.init(),
            .point_lights = undefined,
            .point_light_count = 0,
            .directional_lights = undefined,
            .directional_light_count = 0,
            .spot_lights = undefined,
            .spot_light_count = 0,
            ._padding = 0,
        };

        // Initialize all lights to default values
        var i: usize = 0;
        while (i < MAX_POINT_LIGHTS) : (i += 1) {
            scene.point_lights[i] = PointLight.init();
        }
        i = 0;
        while (i < MAX_DIRECTIONAL_LIGHTS) : (i += 1) {
            scene.directional_lights[i] = DirectionalLight.init();
        }
        i = 0;
        while (i < MAX_SPOT_LIGHTS) : (i += 1) {
            scene.spot_lights[i] = SpotLight.init();
        }

        return scene;
    }

    /// Add a point light to the scene
    pub fn addPointLight(self: *LightScene, light: PointLight) !void {
        if (self.point_light_count >= MAX_POINT_LIGHTS) {
            return error.TooManyPointLights;
        }
        self.point_lights[self.point_light_count] = light;
        self.point_light_count += 1;
    }

    /// Add a directional light to the scene
    pub fn addDirectionalLight(self: *LightScene, light: DirectionalLight) !void {
        if (self.directional_light_count >= MAX_DIRECTIONAL_LIGHTS) {
            return error.TooManyDirectionalLights;
        }
        self.directional_lights[self.directional_light_count] = light;
        self.directional_light_count += 1;
    }

    /// Add a spot light to the scene
    pub fn addSpotLight(self: *LightScene, light: SpotLight) !void {
        if (self.spot_light_count >= MAX_SPOT_LIGHTS) {
            return error.TooManySpotLights;
        }
        self.spot_lights[self.spot_light_count] = light;
        self.spot_light_count += 1;
    }

    /// Clear all lights from the scene
    pub fn clearLights(self: *LightScene) void {
        self.point_light_count = 0;
        self.directional_light_count = 0;
        self.spot_light_count = 0;
    }

    /// Clear only point lights
    pub fn clearPointLights(self: *LightScene) void {
        self.point_light_count = 0;
    }

    /// Clear only directional lights
    pub fn clearDirectionalLights(self: *LightScene) void {
        self.directional_light_count = 0;
    }

    /// Clear only spot lights
    pub fn clearSpotLights(self: *LightScene) void {
        self.spot_light_count = 0;
    }

    /// Get a point light by index
    pub fn getPointLight(self: *LightScene, index: u32) ?*PointLight {
        if (index >= self.point_light_count) return null;
        return &self.point_lights[index];
    }

    /// Get a directional light by index
    pub fn getDirectionalLight(self: *LightScene, index: u32) ?*DirectionalLight {
        if (index >= self.directional_light_count) return null;
        return &self.directional_lights[index];
    }

    /// Get a spot light by index
    pub fn getSpotLight(self: *LightScene, index: u32) ?*SpotLight {
        if (index >= self.spot_light_count) return null;
        return &self.spot_lights[index];
    }

    /// Set ambient light
    pub fn setAmbientLight(self: *LightScene, light: AmbientLight) void {
        self.ambient = light;
    }

    /// Get total light count
    pub fn getTotalLightCount(self: *const LightScene) u32 {
        return self.point_light_count + self.directional_light_count + self.spot_light_count;
    }
};

// C FFI exports
export fn light_scene_init() LightScene {
    return LightScene.init();
}

export fn light_scene_add_point_light(scene: *LightScene, light: PointLight) bool {
    scene.addPointLight(light) catch return false;
    return true;
}

export fn light_scene_add_directional_light(scene: *LightScene, light: DirectionalLight) bool {
    scene.addDirectionalLight(light) catch return false;
    return true;
}

export fn light_scene_add_spot_light(scene: *LightScene, light: SpotLight) bool {
    scene.addSpotLight(light) catch return false;
    return true;
}

export fn light_scene_clear_lights(scene: *LightScene) void {
    scene.clearLights();
}

export fn light_scene_clear_point_lights(scene: *LightScene) void {
    scene.clearPointLights();
}

export fn light_scene_clear_directional_lights(scene: *LightScene) void {
    scene.clearDirectionalLights();
}

export fn light_scene_clear_spot_lights(scene: *LightScene) void {
    scene.clearSpotLights();
}

export fn light_scene_get_point_light(scene: *LightScene, index: u32) ?*PointLight {
    return scene.getPointLight(index);
}

export fn light_scene_get_directional_light(scene: *LightScene, index: u32) ?*DirectionalLight {
    return scene.getDirectionalLight(index);
}

export fn light_scene_get_spot_light(scene: *LightScene, index: u32) ?*SpotLight {
    return scene.getSpotLight(index);
}

export fn light_scene_set_ambient_light(scene: *LightScene, light: AmbientLight) void {
    scene.setAmbientLight(light);
}

export fn light_scene_get_total_light_count(scene: *const LightScene) u32 {
    return scene.getTotalLightCount();
}

export fn light_scene_get_point_light_count(scene: *const LightScene) u32 {
    return scene.point_light_count;
}

export fn light_scene_get_directional_light_count(scene: *const LightScene) u32 {
    return scene.directional_light_count;
}

export fn light_scene_get_spot_light_count(scene: *const LightScene) u32 {
    return scene.spot_light_count;
}
