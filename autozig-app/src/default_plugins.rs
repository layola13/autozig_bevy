//! Default plugins that provide essential application functionality
//!
//! This module contains the DefaultPlugins group which includes all the core
//! plugins needed for a typical application.

#![forbid(unsafe_code)]

use crate::{Plugin, PluginGroup, PluginGroupBuilder, SimplePlugin, App, AppTypeRegistry};

/// Placeholder LogPlugin
#[derive(Default)]
pub struct LogPlugin {
    pub level: Option<&'static str>,
    pub filter: String,
}

impl Plugin for LogPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "LogPlugin" }
}





/// Core plugin that provides basic application infrastructure
#[derive(Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, _app: &mut App) {
        // Core functionality - placeholder for now
        // In a full implementation, this would set up:
        // - Task pools
        // - Main schedule
        // - Core resources
    }
    
    fn name(&self) -> &str {
        "CorePlugin"
    }
}

/// Task pool plugin for async task management
#[derive(Default)]
pub struct TaskPoolPlugin;

impl Plugin for TaskPoolPlugin {
    fn build(&self, _app: &mut App) {
        // Task pool setup - placeholder
    }
    
    fn name(&self) -> &str {
        "TaskPoolPlugin"
    }
}

/// Type registration plugin for reflection system
#[derive(Default)]
pub struct TypeRegistrationPlugin;

impl Plugin for TypeRegistrationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AppTypeRegistry::default());
    }
    
    fn name(&self) -> &str {
        "TypeRegistrationPlugin"
    }
}

/// Frame count plugin for tracking frame numbers
#[derive(Default)]
pub struct FrameCountPlugin;

impl Plugin for FrameCountPlugin {
    fn build(&self, _app: &mut App) {
        // Frame counting - placeholder
    }
    
    fn name(&self) -> &str {
        "FrameCountPlugin"
    }
}

/// Time plugin for time tracking and delta time
#[derive(Default)]
pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, _app: &mut App) {
        // Time management - placeholder
    }
    
    fn name(&self) -> &str {
        "TimePlugin"
    }
}

/// Transform and hierarchy plugin
#[derive(Default)]
pub struct TransformPlugin;

impl Plugin for TransformPlugin {
    fn build(&self, _app: &mut App) {
        // Transform hierarchy - placeholder
    }
    
    fn name(&self) -> &str {
        "TransformPlugin"
    }
}

/// Diagnostic plugin for performance monitoring
#[derive(Default)]
pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, _app: &mut App) {
        // Diagnostics - placeholder
    }
    
    fn name(&self) -> &str {
        "DiagnosticsPlugin"
    }
}

/// Input plugin for keyboard, mouse, and gamepad input
#[derive(Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, _app: &mut App) {
        // Input handling - placeholder
    }
    
    fn name(&self) -> &str {
        "InputPlugin"
    }
}

/// Placeholder RenderPlugin
#[derive(Default)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "RenderPlugin" }
}

/// Placeholder WinitPlugin
#[derive(Default)]
pub struct WinitPlugin;

impl Plugin for WinitPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "WinitPlugin" }
}

/// Window plugin for window management
#[derive(Default)]
pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, _app: &mut App) {
        // Window management - placeholder
    }
    
    fn name(&self) -> &str {
        "WindowPlugin"
    }
}

/// Asset plugin for asset loading and management
#[derive(Default)]
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, _app: &mut App) {
        // Asset management - placeholder
    }
    
    fn name(&self) -> &str {
        "AssetPlugin"
    }
}

/// Scene plugin for scene management
#[derive(Default)]
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, _app: &mut App) {
        // Scene management - placeholder
    }
    
    fn name(&self) -> &str {
        "ScenePlugin"
    }
}

/// Default plugins group containing all essential plugins
///
/// This provides a standard set of plugins that most applications need.
/// Individual plugins can be disabled or customized using the builder pattern:
///
/// # Examples
///
/// ```no_run
/// use autozig_app::{App, DefaultPlugins};
///
/// // Use all default plugins
/// App::new()
///     .add_plugin_group(DefaultPlugins)
///     .run();
/// ```
///
/// ```no_run
/// # use autozig_app::{App, DefaultPlugins, plugin_group::PluginGroupExt};
/// # use autozig_app::default_plugins::WindowPlugin;
/// // Disable a specific plugin
/// App::new()
///     .add_plugin_group(DefaultPlugins.build().disable::<WindowPlugin>())
///     .run();
/// ```
pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        
        // Add plugins in the correct dependency order
        // Core infrastructure first
        group = group.add(TaskPoolPlugin::default());
        group = group.add(TypeRegistrationPlugin::default());
        group = group.add(FrameCountPlugin::default());
        group = group.add(TimePlugin::default());
        
        // Transform and hierarchy
        group = group.add(TransformPlugin::default());
        
        // Diagnostics
        group = group.add(DiagnosticsPlugin::default());
        
        // I/O
        group = group.add(InputPlugin::default());
        group = group.add(WindowPlugin::default());
        group = group.add(WinitPlugin::default());
        group = group.add(RenderPlugin::default());
        group = group.add(LogPlugin::default());
        
        // Assets and scenes
        group = group.add(AssetPlugin::default());
        group = group.add(ScenePlugin::default());
        
        group
    }
    
    fn name() -> &'static str {
        "DefaultPlugins"
    }
}

impl Default for DefaultPlugins {
    fn default() -> Self {
        Self
    }
}

/// Minimal plugins group with only the bare essentials
///
/// This is useful for headless applications or when you want maximum control
/// over which plugins are included.
pub struct MinimalPlugins;

impl PluginGroup for MinimalPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin::default())
            .add(FrameCountPlugin::default())
            .add(TimePlugin::default())
    }
    
    fn name() -> &'static str {
        "MinimalPlugins"
    }
}

impl Default for MinimalPlugins {
    fn default() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin_group::PluginGroupExt;
    
    #[test]
    fn test_default_plugins_build() {
        let builder = DefaultPlugins.build();
        assert!(builder.len() > 0);
        assert!(builder.contains::<TaskPoolPlugin>());
        assert!(builder.contains::<TimePlugin>());
    }
    
    #[test]
    fn test_minimal_plugins_build() {
        let builder = MinimalPlugins.build();
        assert!(builder.len() > 0);
        assert!(builder.contains::<TaskPoolPlugin>());
        assert!(builder.contains::<TimePlugin>());
    }
    
    #[test]
    fn test_default_plugins_disable() {
        let builder = DefaultPlugins.build().disable::<WindowPlugin>();
        assert!(builder.contains::<WindowPlugin>());
        assert!(!builder.enabled::<WindowPlugin>());
    }
    
    #[test]
    fn test_default_plugins_ordering() {
        let builder = DefaultPlugins.build();
        // Just verify it builds without panicking
        assert!(builder.enabled_count() > 0);
    }
}