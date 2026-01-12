//! SSAO - Screen Space Ambient Occlusion (屏幕空间环境光遮蔽)

const std = @import("std");

pub const ScreenSpaceAmbientOcclusionQualityLevel = enum(c_int) {
    Low = 0,
    Medium = 1,
    High = 2,
    Ultra = 3,
    Custom = 4,
};

pub const ScreenSpaceAmbientOcclusionSettings = extern struct {
    radius: f32,
    bias: f32,
    intensity: f32,
    quality_level: ScreenSpaceAmbientOcclusionQualityLevel,
};

export fn ssao_settings_init() ScreenSpaceAmbientOcclusionSettings {
    return ScreenSpaceAmbientOcclusionSettings{
        .radius = 0.5,
        .bias = 0.025,
        .intensity = 1.0,
        .quality_level = .Medium,
    };
}

export fn ssao_settings_set_quality(settings: *ScreenSpaceAmbientOcclusionSettings, quality: ScreenSpaceAmbientOcclusionQualityLevel) void {
    settings.quality_level = quality;
}

export fn ssao_settings_set_radius(settings: *ScreenSpaceAmbientOcclusionSettings, radius: f32) void {
    settings.radius = radius;
}

export fn ssao_settings_set_intensity(settings: *ScreenSpaceAmbientOcclusionSettings, intensity: f32) void {
    settings.intensity = intensity;
}
