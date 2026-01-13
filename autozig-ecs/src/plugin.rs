//! Plugin system - Bevy-compatible plugin architecture

use autozig_macro::include_zig;
use crate::into_system::{IntoSystem};
use crate::system::{BoxedSystem, System};
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
    /// Build the plugin - add systems, resources, etc.
    fn build(&self, app: &mut App);
    
    /// Get the name of this plugin
    ///
    /// Defaults to the type name
    fn name(&self) -> &str {
        core::any::type_name::<Self>()
    }
    
    /// Check if the plugin is ready to be built
    ///
    /// This is called before `build`, and can be used to ensure dependencies are met
    fn ready(&self, _app: &App) -> bool {
        true
    }
    
    /// Called after all plugins have been built
    ///
    /// This is useful for initialization that depends on other plugins
    fn finish(&self, _app: &mut App) {}
    
    /// Called when the app is shutting down
    ///
    /// Use this for cleanup operations
    fn cleanup(&self, _app: &mut App) {}
    
    /// Check if this plugin is unique
    ///
    /// If true, the plugin can only be added once. Defaults to true.
    fn is_unique(&self) -> bool {
        true
    }
}

/// Event sent when the application should exit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppExit {
    #[default]
    Success,
    Error(u8),
}

/// Application builder
pub struct App {
    plugin_manager: *mut PluginManagerOpaque,
    pub(crate) world: World,
    pub(crate) startup_schedule: Vec<BoxedSystem>,
    pub(crate) update_schedule: Vec<BoxedSystem>,
    pub(crate) last_schedule: Vec<BoxedSystem>,
    runner: Option<Box<dyn FnOnce(App)>>,
}

use crate::system_config::{IntoSystemConfigs, SystemConfigs};
use crate::system_set::IntoSystemSetConfigs;

use crate::event::Events;

impl App {
    pub fn new() -> Self {
        let plugin_manager = plugin_manager_create();
        let mut world = World::new();
        // Initialize AppExit events
        world.insert_resource(Events::<AppExit>::default());
        
        Self { 
            plugin_manager,
            world,
            startup_schedule: Vec::new(),
            update_schedule: Vec::new(),
            last_schedule: Vec::new(),
            runner: None,
        }
    }
    
    pub fn run(mut self) {
        // Run startup systems once
        for system in self.startup_schedule.iter_mut() {
            system.run(&mut self.world);
        }

        // If a runner is set, delegate to it
        if let Some(runner) = self.runner.take() {
            runner(self);
        } else {
             // Default: Single run
             for system in self.update_schedule.iter_mut() {
                system.run(&mut self.world);
            }
            for system in self.last_schedule.iter_mut() {
                system.run(&mut self.world);
            }
        }
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.build(self);
        self
    }
    
    pub fn add_plugins<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        self.add_plugin(plugin)
    }
    
    /// Add systems to a specific schedule
    pub fn add_systems<M>(&mut self, schedule: impl crate::schedule::ScheduleLabel, systems: impl IntoSystemConfigs<M>) -> &mut Self {
        let configs = systems.into_configs();
        // Simple mapping based on type name or "debug" implementation of label
        // This is a hack for verification since we don't have a full Schedule map
        let label_str = schedule.as_str();
        
        let target_schedule = if label_str.contains("Startup") {
            &mut self.startup_schedule
        } else if label_str.contains("Last") {
            &mut self.last_schedule
        } else {
            &mut self.update_schedule // Default to Update
        };

        for config in configs.configs {
            let mut system = config.system;
            system.initialize(&mut self.world);
            target_schedule.push(system);
        }
        self
    }
    
    pub fn configure_sets(&mut self, _schedule: impl crate::schedule::ScheduleLabel, _sets: impl IntoSystemSetConfigs) -> &mut Self {
        // Placeholder: we implicitly respect sets by order in this simple implementation
        self
    }
    
    pub fn init_resource<R: crate::resource::Resource + Default>(&mut self) -> &mut Self {
        self.world.insert_resource(R::default());
        self
    }

    pub fn insert_resource<R: crate::resource::Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
        self
    }

    pub fn set_runner(&mut self, runner: impl FnOnce(App) + 'static) -> &mut Self {
        self.runner = Some(Box::new(runner));
        self
    }
    
    pub fn register_plugin_fn(&mut self, name: &str, func: PluginBuildFn) {
        plugin_manager_add(self.plugin_manager, name.as_ptr(), name.len(), func);
    }
    
    pub fn finish(&mut self) {
        let app_ptr = self as *mut Self as *mut std::ffi::c_void;
        plugin_manager_run_all(self.plugin_manager, app_ptr);
    }
    

    
    pub fn plugin_count(&self) -> usize {
        plugin_manager_count(self.plugin_manager)
    }
    
    pub fn closure_system_count(&self) -> usize {
        self.startup_schedule.len() + self.update_schedule.len() + self.last_schedule.len()
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
