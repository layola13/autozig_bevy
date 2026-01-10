//! Plugin system - Bevy-compatible plugin architecture

use autozig::include_zig;
use crate::into_system::{BoxedSystem, IntoSystem};
use crate::world::World;

#[repr(C)]
pub struct PluginManagerOpaque {
    _private: u8,
}

pub type PluginBuildFn = fn(*mut std::ffi::c_void);

include_zig!("src/zig/plugin.zig", {
    fn plugin_manager_create() -> *mut PluginManagerOpaque;
    fn plugin_manager_destroy(manager: *mut PluginManagerOpaque);
    fn plugin_manager_add(
        manager: *mut PluginManagerOpaque,
        name_ptr: *const u8,
        name_len: usize,
        build_fn: PluginBuildFn,
    ) -> bool;
    fn plugin_manager_run_all(manager: *mut PluginManagerOpaque, app_ptr: *mut std::ffi::c_void);
    fn plugin_manager_count(manager: *const PluginManagerOpaque) -> usize;
});

/// Bevy-compatible Plugin trait
pub trait Plugin: Send + Sync {
    fn build(&self, app: &mut App);
}

/// Application builder
pub struct App {
    plugin_manager: *mut PluginManagerOpaque,
    systems: Vec<BoxedSystem>,
}

impl App {
    pub fn new() -> Self {
        let plugin_manager = plugin_manager_create();
        Self { 
            plugin_manager,
            systems: Vec::new(),
        }
    }
    
    /// Add a single plugin
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.build(self);
        self
    }
    
    /// Add multiple plugins (Bevy-style)
    pub fn add_plugins<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        self.add_plugin(plugin)
    }
    
    /// Add closure-based systems (Bevy-compatible)
    pub fn add_systems<Params, F>(&mut self, system: F) -> &mut Self
    where
        F: IntoSystem<Params>,
    {
        let boxed = system.into_system();
        self.systems.push(boxed);
        self
    }
    
    /// Register a plugin function
    pub fn register_plugin_fn(&mut self, name: &str, func: PluginBuildFn) {
        plugin_manager_add(self.plugin_manager, name.as_ptr(), name.len(), func);
    }
    
    /// Run all registered plugins
    pub fn finish(&mut self) {
        let app_ptr = self as *mut Self as *mut std::ffi::c_void;
        plugin_manager_run_all(self.plugin_manager, app_ptr);
    }
    
    /// Run all closure systems (Bevy-style)
    pub fn run(&mut self) {
        // Create a temporary World for systems that need it
        let mut world = World::new();
        
        // Execute all registered systems
        for system in self.systems.iter_mut() {
            (system.closure)(&mut world);
        }
    }
    
    pub fn plugin_count(&self) -> usize {
        plugin_manager_count(self.plugin_manager)
    }
    
    /// Get the number of registered closure systems
    pub fn closure_system_count(&self) -> usize {
        self.systems.len()
    }
}

impl Drop for App {
    fn drop(&mut self) {
        plugin_manager_destroy(self.plugin_manager);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

// Example plugins

/// Core plugin providing basic ECS functionality
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // In a real implementation, this would register resources, systems, etc.
        fn core_plugin_build(_app_ptr: *mut std::ffi::c_void) {
            // Initialize core ECS systems
        }
        
        app.register_plugin_fn("CorePlugin", core_plugin_build);
    }
}

/// Time plugin providing time tracking
pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        fn time_plugin_build(_app_ptr: *mut std::ffi::c_void) {
            // Initialize time resource
        }
        
        app.register_plugin_fn("TimePlugin", time_plugin_build);
    }
}

/// Default plugins bundle (Bevy-style)
pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugin(CorePlugin)
           .add_plugin(TimePlugin);
    }
}
