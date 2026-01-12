//! SSR - Screen Space Reflections (屏幕空间反射)

const std = @import("std");

pub const ScreenSpaceReflectionsSettings = extern struct {
    thickness: f32,
    linear_steps: u32,
    linear_march_exponent: f32,
    bisection_steps: u32,
    use_secant: bool,
    _padding: [3]u8,
};

export fn ssr_settings_init() ScreenSpaceReflectionsSettings {
    return ScreenSpaceReflectionsSettings{
        .thickness = 0.1,
        .linear_steps = 16,
        .linear_march_exponent = 1.0,
        .bisection_steps = 4,
        .use_secant = false,
        ._padding = [_]u8{0} ** 3,
    };
}

export fn ssr_settings_set_steps(settings: *ScreenSpaceReflectionsSettings, linear: u32, bisection: u32) void {
    settings.linear_steps = linear;
    settings.bisection_steps = bisection;
}

export fn ssr_settings_set_thickness(settings: *ScreenSpaceReflectionsSettings, thickness: f32) void {
    settings.thickness = thickness;
}
