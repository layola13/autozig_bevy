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
    pub(crate) schedules: crate::schedule::Schedules,
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
        
        // Initialize standard schedules
        use crate::schedule::{Startup, Update, Last};
        let mut schedules = crate::schedule::Schedules::new();
        schedules.insert(crate::schedule::Schedule::new(Startup));
        schedules.insert(crate::schedule::Schedule::new(Update));
        schedules.insert(crate::schedule::Schedule::new(Last));
        
        Self { 
            plugin_manager,
            world,
            schedules,
            runner: None,
        }
    }
    
    pub fn run(mut self) {
        use crate::schedule::{Startup, Update, Last};
        
        // Run startup systems once
        if let Some(startup) = self.schedules.get_mut(Startup) {
            startup.run(&mut self.world);
        }

        // If a runner is set, delegate to it
        if let Some(runner) = self.runner.take() {
            runner(self);
        } else {
             // Default: Single run
             if let Some(update) = self.schedules.get_mut(Update) {
                update.run(&mut self.world);
             }
             if let Some(last) = self.schedules.get_mut(Last) {
                last.run(&mut self.world);
             }
        }
    }

    /// Run the Update schedule once without consuming App
    pub fn update(&mut self) {
        use crate::schedule::Update;
        if let Some(update) = self.schedules.get_mut(Update) {
            update.run(&mut self.world);
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
        let label_str = schedule.as_str().to_string();
        if let Some(sched) = self.schedules.get_mut(schedule) {
            sched.add_systems(systems);
        } else {
            // Panic or Create new schedule?
            // Bevy panics if schedule doesn't exist usually, or implicitly creates.
            // For now, let's panic to be explicit about supported schedules.
            panic!("Schedule not found: {}", label_str);
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
    
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn closure_system_count(&self) -> usize {
        // Iterate all schedules
        use crate::schedule::{Startup, Update, Last};
        let mut count = 0;
        if let Some(s) = self.schedules.get(Startup) { count += s.system_count(); }
        if let Some(s) = self.schedules.get(Update) { count += s.system_count(); }
        if let Some(s) = self.schedules.get(Last) { count += s.system_count(); }
        count
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
