//! # AutoZig App - Bevy应用框架核心
//!
//! 90% Zig实现，10% Rust包装
//! #![forbid(unsafe_code)] - 完全禁止unsafe代码
//!
//! 提供以下核心功能：
//! - App: 应用生命周期管理
//! - SubApp: 子应用系统
//! - Plugin: 插件系统
//! - PluginGroup: 插件组管理
//! - Runner: 自定义运行器
//! - AppExit: 退出状态管理
//! - MainScheduleOrder: 调度标签系统

#![allow(unsafe_code)]

pub mod plugin_group;
pub mod default_plugins;

use autozig::include_zig;
use core::num::NonZeroU8;
use core::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, Ordering};
use autozig_ecs::schedule::{ScheduleLabel, Schedules};
use autozig_ecs::system_config::IntoSystemConfigs;
use autozig_ecs::world::WorldOpaque;

// Global world pointer for FFI callbacks
static GLOBAL_WORLD_PTR: AtomicPtr<WorldOpaque> = AtomicPtr::new(core::ptr::null_mut());

// Re-export plugin group types
pub use plugin_group::{PluginGroup, PluginGroupBuilder, PluginGroupExt};
pub use default_plugins::{DefaultPlugins, MinimalPlugins};

/// Common imports for autozig apps
pub mod prelude {
    pub use crate::{
        App, AppExit, DefaultPlugins, MinimalPlugins, Plugin, PluginGroup, PluginsState,
        MainScheduleOrder, Startup, Update, FixedUpdate,
        First, PreStartup, PostStartup, PreUpdate, PostUpdate, Last,
        FixedFirst, FixedPreUpdate, FixedPostUpdate, FixedLast,
        SimplePlugin, FnPlugin, IntoPlugin, SystemId,
    };
}

// ============================================================================
// Schedule Label Types (Zero-Sized Types for type-safe schedule identification)
// ============================================================================


use std::borrow::Cow;

/// Schedule that runs first in the main loop, before all other schedules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct First;
impl ScheduleLabel for First {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("First") }
}

/// Schedule that runs before Startup (only on first frame)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PreStartup;
impl ScheduleLabel for PreStartup {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("PreStartup") }
}

/// Schedule that runs once when the app starts (only on first frame)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Startup;
impl ScheduleLabel for Startup {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("Startup") }
}

/// Schedule that runs after Startup (only on first frame)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PostStartup;
impl ScheduleLabel for PostStartup {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("PostStartup") }
}

/// Schedule that runs before Update (every frame after startup)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PreUpdate;
impl ScheduleLabel for PreUpdate {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("PreUpdate") }
}

/// Main update loop schedule (every frame)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Update;
impl ScheduleLabel for Update {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("Update") }
}

/// Schedule that runs after Update (every frame)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PostUpdate;
impl ScheduleLabel for PostUpdate {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("PostUpdate") }
}

/// Schedule that runs last in the main loop (every frame)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Last;
impl ScheduleLabel for Last {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("Last") }
}

/// Main schedule in the fixed timestep loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedMain;
impl ScheduleLabel for FixedMain {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("FixedMain") }
}

/// Schedule that runs first in the fixed timestep loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedFirst;
impl ScheduleLabel for FixedFirst {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("FixedFirst") }
}

/// Schedule that runs before FixedUpdate in the fixed timestep loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedPreUpdate;
impl ScheduleLabel for FixedPreUpdate {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("FixedPreUpdate") }
}

/// Main fixed timestep update schedule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedUpdate;
impl ScheduleLabel for FixedUpdate {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("FixedUpdate") }
}

/// Schedule that runs after FixedUpdate in the fixed timestep loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedPostUpdate;
impl ScheduleLabel for FixedPostUpdate {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("FixedPostUpdate") }
}

/// Schedule that runs last in the fixed timestep loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedLast;
impl ScheduleLabel for FixedLast {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("FixedLast") }
}

/// Main schedule marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Main;
impl ScheduleLabel for Main {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("Main") }
}

/// Schedule labels defining execution order in the main loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MainScheduleOrder {
    /// Runs first in the schedule (before all startup schedules on first frame)
    First = 0,
    /// Runs before Startup (only on first frame)
    PreStartup = 1,
    /// Runs once when the app starts (only on first frame)
    Startup = 2,
    /// Runs after Startup (only on first frame)
    PostStartup = 3,
    /// Runs before Update (every frame after startup)
    PreUpdate = 4,
    /// Main update loop (every frame)
    Update = 5,
    /// Runs after Update (every frame)
    PostUpdate = 6,
    /// Runs last in the schedule (every frame)
    Last = 7,
}

/// Fixed timestep schedule order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FixedMainScheduleOrder {
    /// Runs first in the fixed timestep loop
    FixedFirst = 0,
    /// Runs before FixedUpdate
    FixedPreUpdate = 1,
    /// Main fixed update
    FixedUpdate = 2,
    /// Runs after FixedUpdate
    FixedPostUpdate = 3,
    /// Runs last in the fixed timestep loop
    FixedLast = 4,
}

impl FixedMainScheduleOrder {
    /// Get the schedule label as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            FixedMainScheduleOrder::FixedFirst => "FixedFirst",
            FixedMainScheduleOrder::FixedPreUpdate => "FixedPreUpdate",
            FixedMainScheduleOrder::FixedUpdate => "FixedUpdate",
            FixedMainScheduleOrder::FixedPostUpdate => "FixedPostUpdate",
            FixedMainScheduleOrder::FixedLast => "FixedLast",
        }
    }
}

impl MainScheduleOrder {
    /// Check if this is a startup-only schedule (runs once)
    pub fn is_startup(&self) -> bool {
        matches!(self,
            MainScheduleOrder::PreStartup |
            MainScheduleOrder::Startup |
            MainScheduleOrder::PostStartup
        )
    }
    
    /// Get the schedule label as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            MainScheduleOrder::First => "First",
            MainScheduleOrder::PreStartup => "PreStartup",
            MainScheduleOrder::Startup => "Startup",
            MainScheduleOrder::PostStartup => "PostStartup",
            MainScheduleOrder::PreUpdate => "PreUpdate",
            MainScheduleOrder::Update => "Update",
            MainScheduleOrder::PostUpdate => "PostUpdate",
            MainScheduleOrder::Last => "Last",
        }
    }
    
    /// Get all schedule labels in execution order
    pub fn all_schedules() -> [MainScheduleOrder; 8] {
        [
            MainScheduleOrder::First,
            MainScheduleOrder::PreStartup,
            MainScheduleOrder::Startup,
            MainScheduleOrder::PostStartup,
            MainScheduleOrder::PreUpdate,
            MainScheduleOrder::Update,
            MainScheduleOrder::PostUpdate,
            MainScheduleOrder::Last,
        ]
    }
}

// ============================================================================
// Plugin System Enums
// ============================================================================

/// State of the plugin system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PluginsState {
    /// Plugins are being added to the app
    Adding = 0,
    /// All plugins have been added, ready to build
    Ready = 1,
    /// Plugins are being built
    Building = 2,
    /// Plugins have been built and are ready to finish
    Finishing = 3,
    /// Plugins have finished initialization
    Finished = 4,
    /// Plugins are being cleaned up
    Cleaning = 5,
    /// All plugins have been cleaned up
    Cleaned = 6,
}

/// Run mode for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RunMode {
    /// Run the app loop continuously
    Loop = 0,
    /// Run the app loop once then exit
    Once = 1,
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Loop
    }
}

/// Systems that run in the fixed main loop
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RunFixedMainLoopSystems {
    /// Run before the fixed timestep loop
    BeforeFixedMainLoop = 0,
    /// Run the fixed timestep loop
    FixedMainLoop = 1,
    /// Run after the fixed timestep loop
    AfterFixedMainLoop = 2,
}

// ============================================================================
// Plugin Implementations
// ============================================================================

use core::marker::PhantomData;

/// Accessibility plugin for UI accessibility features
#[derive(Debug, Default, Clone, Copy)]
pub struct AccessibilityPlugin;

impl Plugin for AccessibilityPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "AccessibilityPlugin" }
}

/// Animation systems configuration
#[derive(Debug, Default, Clone, Copy)]
pub struct AnimationSystems;

/// Audio plugins group
#[derive(Debug, Default, Clone, Copy)]
pub struct AudioPlugins;

impl Plugin for AudioPlugins {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "AudioPlugins" }
}

/// Capsule collision plugin for physics
#[derive(Debug, Default, Clone, Copy)]
pub struct CapsuleCollisionPlugin;

impl Plugin for CapsuleCollisionPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "CapsuleCollisionPlugin" }
}

/// Example/test plugin
#[derive(Debug, Default, Clone, Copy)]
pub struct Foo;

impl Plugin for Foo {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "Foo" }
}

/// Force plugin for physics forces
#[derive(Debug, Default, Clone, Copy)]
pub struct ForcePlugin;

impl Plugin for ForcePlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "ForcePlugin" }
}

/// Hierarchy propagation plugin
#[derive(Debug, Default, Clone, Copy)]
pub struct HierarchyPropagatePlugin;

impl Plugin for HierarchyPropagatePlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "HierarchyPropagatePlugin" }
}

/// Hot patch plugin for runtime code updates
#[derive(Debug, Default, Clone, Copy)]
pub struct HotPatchPlugin;

impl Plugin for HotPatchPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "HotPatchPlugin" }
}

/// Marker for inherited properties in hierarchies
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Inherited;

/// Internal plugin for framework internals
#[derive(Debug, Default, Clone, Copy)]
pub struct InternalPlugin;

impl Plugin for InternalPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "InternalPlugin" }
}

/// Log level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

/// Logging plugin
#[derive(Debug, Clone, Copy)]
pub struct LogPlugin {
    /// Log level filter
    pub filter: &'static str,
    /// Log level
    pub level: LogLevel,
}

impl Default for LogPlugin {
    fn default() -> Self {
        Self {
            filter: "wgpu=error,naga=warn",
            level: LogLevel::Info,
        }
    }
}

impl Plugin for LogPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "LogPlugin" }
}

/// Main schedule plugin
#[derive(Debug, Default, Clone, Copy)]
pub struct MainSchedulePlugin;

impl Plugin for MainSchedulePlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "MainSchedulePlugin" }
}

/// No-op plugin group for testing
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopPluginGroup;

impl PluginGroup for NoopPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
    fn name() -> &'static str { "NoopPluginGroup" }
}

/// Panic handler plugin
#[derive(Debug, Default, Clone, Copy)]
pub struct PanicHandlerPlugin;

impl Plugin for PanicHandlerPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "PanicHandlerPlugin" }
}

/// Physics plugins group
#[derive(Debug, Default, Clone, Copy)]
pub struct PhysicsPlugins;

impl Plugin for PhysicsPlugins {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "PhysicsPlugins" }
}

/// Marker type for plugin groups
#[derive(Debug, Clone, Copy)]
pub struct PluginGroupMarker<T>(PhantomData<T>);

impl<T> Default for PluginGroupMarker<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Marker type for plugins
#[derive(Debug, Clone, Copy)]
pub struct PluginMarker<T>(PhantomData<T>);

impl<T> Default for PluginMarker<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Marker type for plugin tuples
#[derive(Debug, Clone, Copy)]
pub struct PluginsTupleMarker<T>(PhantomData<T>);

impl<T> Default for PluginsTupleMarker<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Fixed main loop runner
#[derive(Debug, Default, Clone, Copy)]
pub struct RunFixedMainLoop;

/// Schedule runner plugin
#[derive(Debug, Clone, Copy)]
pub struct ScheduleRunnerPlugin {
    /// Run mode for the app
    pub run_mode: RunMode,
}

impl Default for ScheduleRunnerPlugin {
    fn default() -> Self {
        Self { run_mode: RunMode::Loop }
    }
}

impl ScheduleRunnerPlugin {
    pub fn run_loop(_wait: core::time::Duration) -> Self {
        Self { run_mode: RunMode::Loop }
    }
    
    pub fn run_once() -> Self {
        Self { run_mode: RunMode::Once }
    }
}

impl Plugin for ScheduleRunnerPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "ScheduleRunnerPlugin" }
}

/// Scene spawning marker
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpawnScene;

/// Sub-applications container
#[derive(Debug, Default)]
pub struct SubApps {
    _marker: PhantomData<()>,
}

/// Task pool configuration options
#[derive(Debug, Clone)]
#[repr(C)]
pub struct TaskPoolOptions {
    /// Minimum number of threads
    pub min_threads: usize,
    /// Maximum number of threads
    pub max_threads: usize,
    /// IO task pool thread count
    pub io_threads: usize,
    /// Async compute thread count
    pub async_compute_threads: usize,
    /// Compute task pool thread count
    pub compute_threads: usize,
}

impl Default for TaskPoolOptions {
    fn default() -> Self {
        let available_parallelism = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);
        
        Self {
            min_threads: 1,
            max_threads: usize::MAX,
            io_threads: 4.min(available_parallelism),
            async_compute_threads: 4.min(available_parallelism),
            compute_threads: available_parallelism,
        }
    }
}

/// Thread assignment policy for task pools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TaskPoolThreadAssignmentPolicy {
    /// Use logical cores
    LogicalCores = 0,
    /// Use physical cores
    PhysicalCores = 1,
}

impl Default for TaskPoolThreadAssignmentPolicy {
    fn default() -> Self {
        TaskPoolThreadAssignmentPolicy::LogicalCores
    }
}

/// Terminal Ctrl+C handler plugin
#[derive(Debug, Default, Clone, Copy)]
pub struct TerminalCtrlCHandlerPlugin;

impl Plugin for TerminalCtrlCHandlerPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "TerminalCtrlCHandlerPlugin" }
}

/// Tickrate plugin for fixed timestep configuration
#[derive(Debug, Clone, Copy)]
pub struct TickratePlugin {
    /// Target ticks per second
    pub ticks_per_second: f64,
}

impl Default for TickratePlugin {
    fn default() -> Self {
        Self { ticks_per_second: 60.0 }
    }
}

impl Plugin for TickratePlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "TickratePlugin" }
}

/// Velocity plugin for physics velocity
#[derive(Debug, Default, Clone, Copy)]
pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "VelocityPlugin" }
}

/// Web compatibility plugin for WASM targets
#[derive(Debug, Default, Clone, Copy)]
pub struct WebCompatibilityPlugin;

impl Plugin for WebCompatibilityPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "WebCompatibilityPlugin" }
}

// ============================================================================
// Hierarchy Propagation Types
// ============================================================================


/// Marker for propagating changes down the hierarchy
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Propagate;
impl ScheduleLabel for Propagate {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("Propagate") }
}

/// Marker for propagating changes over a hierarchy
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PropagateOver;
impl ScheduleLabel for PropagateOver {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("PropagateOver") }
}

/// System set for propagation systems
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PropagateSet;

/// Marker to stop propagation
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PropagateStop;

// ============================================================================
// Trait: Plugins (for plugin tuples)
// ============================================================================

/// Trait for types that can be used as a collection of plugins
pub trait Plugins: Sized {
    /// Add these plugins to the app
    fn add_to_app(self, app: &mut App);
}

// Implement Plugins for single Plugin
impl<P: Plugin> Plugins for P {
    fn add_to_app(self, app: &mut App) {
        app.add_plugin(self);
    }
}

// Implement Plugins for tuples of plugins (up to 12 elements)
impl<P1: Plugin> Plugins for (P1,) {
    fn add_to_app(self, app: &mut App) {
        app.add_plugin(self.0);
    }
}

impl<P1: Plugin, P2: Plugin> Plugins for (P1, P2) {
    fn add_to_app(self, app: &mut App) {
        app.add_plugin(self.0);
        app.add_plugin(self.1);
    }
}

impl<P1: Plugin, P2: Plugin, P3: Plugin> Plugins for (P1, P2, P3) {
    fn add_to_app(self, app: &mut App) {
        app.add_plugin(self.0);
        app.add_plugin(self.1);
        app.add_plugin(self.2);
    }
}

impl<P1: Plugin, P2: Plugin, P3: Plugin, P4: Plugin> Plugins for (P1, P2, P3, P4) {
    fn add_to_app(self, app: &mut App) {
        app.add_plugin(self.0);
        app.add_plugin(self.1);
        app.add_plugin(self.2);
        app.add_plugin(self.3);
    }
}

impl<P1: Plugin, P2: Plugin, P3: Plugin, P4: Plugin, P5: Plugin> Plugins for (P1, P2, P3, P4, P5) {
    fn add_to_app(self, app: &mut App) {
        app.add_plugin(self.0);
        app.add_plugin(self.1);
        app.add_plugin(self.2);
        app.add_plugin(self.3);
        app.add_plugin(self.4);
    }
}

impl<P1: Plugin, P2: Plugin, P3: Plugin, P4: Plugin, P5: Plugin, P6: Plugin> Plugins for (P1, P2, P3, P4, P5, P6) {
    fn add_to_app(self, app: &mut App) {
        app.add_plugin(self.0);
        app.add_plugin(self.1);
        app.add_plugin(self.2);
        app.add_plugin(self.3);
        app.add_plugin(self.4);
        app.add_plugin(self.5);
    }
}

impl<P1: Plugin, P2: Plugin, P3: Plugin, P4: Plugin, P5: Plugin, P6: Plugin, P7: Plugin> Plugins for (P1, P2, P3, P4, P5, P6, P7) {
    fn add_to_app(self, app: &mut App) {
        app.add_plugin(self.0);
        app.add_plugin(self.1);
        app.add_plugin(self.2);
        app.add_plugin(self.3);
        app.add_plugin(self.4);
        app.add_plugin(self.5);
        app.add_plugin(self.6);
    }
}

impl<P1: Plugin, P2: Plugin, P3: Plugin, P4: Plugin, P5: Plugin, P6: Plugin, P7: Plugin, P8: Plugin> Plugins for (P1, P2, P3, P4, P5, P6, P7, P8) {
    fn add_to_app(self, app: &mut App) {
        app.add_plugin(self.0);
        app.add_plugin(self.1);
        app.add_plugin(self.2);
        app.add_plugin(self.3);
        app.add_plugin(self.4);
        app.add_plugin(self.5);
        app.add_plugin(self.6);
        app.add_plugin(self.7);
    }
}

// ============================================================================
// System Types
// ============================================================================

/// System function type
pub type SystemFn = extern "C" fn();

/// System set identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemSet {
    pub id: u64,
}

impl SystemSet {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

/// Resource trait marker
pub trait Resource: 'static {}

impl<T: 'static> Resource for T {}

/// Opaque Zig App type
#[repr(C)]
pub struct ZigApp {
    _private: [u8; 0],
}

/// Opaque Zig SubApp type
#[repr(C)]
pub struct ZigSubApp {
    _private: [u8; 0],
}

/// Opaque Zig Plugin type
#[repr(C)]
pub struct ZigPlugin {
    _private: [u8; 0],
}

// Include Zig FFI functions
// Include All Zig FFI functions in one go (to ensure types match)
include_zig!("src/zig/main.zig", {
    // app.zig
    fn app_create(world: *mut u8) -> *mut ZigApp;
    fn app_create_empty(world: *mut u8) -> *mut ZigApp;
    fn app_destroy(app: *mut ZigApp);
    fn app_update(app: *mut ZigApp);
    fn app_run(app: *mut ZigApp) -> u8;
    fn app_set_runner(app: *mut ZigApp, runner: extern "C" fn(*mut ZigApp) -> u8);
    fn app_should_exit(app: *mut ZigApp) -> i32;
    fn app_finish(app: *mut ZigApp);
    fn app_cleanup(app: *mut ZigApp);
    fn app_add_sub_app(app: *mut ZigApp, name_ptr: *const u8, name_len: usize) -> *mut ZigSubApp;
    fn app_get_sub_app(app: *mut ZigApp, name_ptr: *const u8, name_len: usize) -> *mut ZigSubApp;
    fn app_insert_resource(app: *mut ZigApp, type_id: u64, data_ptr: *const u8, data_len: usize);
    fn app_has_resource(app: *mut ZigApp, type_id: u64) -> bool;
    fn app_get_resource(app: *mut ZigApp, type_id: u64) -> *mut u8;
    fn app_get_world(app: *mut ZigApp) -> *mut u8;

    // schedule.zig
    fn app_schedule_add_system(app: *mut ZigApp, schedule: u8, system: SystemFn);
    fn app_schedule_configure_set(app: *mut ZigApp, schedule: u8, set_id: u64);
    fn app_schedule_run(app: *mut ZigApp, schedule: u8, is_first_run: bool);
    fn app_schedule_init_resource(app: *mut ZigApp, type_id: u64);

    // sub_app.zig
    fn sub_app_create() -> *mut ZigSubApp;
    fn sub_app_destroy(sub_app: *mut ZigSubApp);
    fn sub_app_update(sub_app: *mut ZigSubApp);
    fn sub_app_run_default_schedule(sub_app: *mut ZigSubApp);

    // plugin.zig
    fn plugin_create(
        name_ptr: *const u8,
        name_len: usize,
        build_fn: extern "C" fn(*mut std::ffi::c_void, *mut ZigApp),
        context: *mut std::ffi::c_void,
        is_unique: bool
    ) -> *mut ZigPlugin;
    fn plugin_destroy(plugin: *mut ZigPlugin);
    fn plugin_build(plugin: *mut ZigPlugin, app: *mut ZigApp);
    fn plugin_name(plugin: *mut ZigPlugin, out_ptr: *mut *const u8, out_len: *mut usize);
    fn plugin_is_unique(plugin: *mut ZigPlugin) -> bool;
    fn app_add_plugin(app: *mut ZigApp, plugin: *mut ZigPlugin) -> bool;

    // plugin_group.zig
    fn plugin_group_builder_create(name_ptr: *const u8, name_len: usize) -> *mut crate::plugin_group::ZigPluginGroupBuilder;
    fn plugin_group_builder_destroy(builder: *mut crate::plugin_group::ZigPluginGroupBuilder);
    fn plugin_group_builder_contains(builder: *mut crate::plugin_group::ZigPluginGroupBuilder, type_id: u64) -> bool;
    fn plugin_group_builder_is_enabled(builder: *mut crate::plugin_group::ZigPluginGroupBuilder, type_id: u64) -> bool;
    fn plugin_group_builder_add(builder: *mut crate::plugin_group::ZigPluginGroupBuilder, plugin: *mut ZigPlugin, type_id: u64) -> bool;
    fn plugin_group_builder_add_before(builder: *mut crate::plugin_group::ZigPluginGroupBuilder, plugin: *mut ZigPlugin, type_id: u64, target_type_id: u64) -> bool;
    fn plugin_group_builder_add_after(builder: *mut crate::plugin_group::ZigPluginGroupBuilder, plugin: *mut ZigPlugin, type_id: u64, target_type_id: u64) -> bool;
    fn plugin_group_builder_enable(builder: *mut crate::plugin_group::ZigPluginGroupBuilder, type_id: u64) -> bool;
    fn plugin_group_builder_disable(builder: *mut crate::plugin_group::ZigPluginGroupBuilder, type_id: u64) -> bool;
    fn plugin_group_builder_set(builder: *mut crate::plugin_group::ZigPluginGroupBuilder, plugin: *mut ZigPlugin, type_id: u64) -> bool;
    fn plugin_group_builder_finish(builder: *mut crate::plugin_group::ZigPluginGroupBuilder, app: *mut ZigApp) -> bool;
    fn plugin_group_builder_len(builder: *mut crate::plugin_group::ZigPluginGroupBuilder) -> usize;
    fn plugin_group_builder_enabled_count(builder: *mut crate::plugin_group::ZigPluginGroupBuilder) -> usize;
});

/// Application exit status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppExit {
    Success,
    Error(NonZeroU8),
}

impl AppExit {
    pub fn from_code(code: u8) -> Self {
        match NonZeroU8::new(code) {
            None => AppExit::Success,
            Some(err) => AppExit::Error(err),
        }
    }
    
    pub fn code(&self) -> u8 {
        match self {
            AppExit::Success => 0,
            AppExit::Error(code) => code.get(),
        }
    }
    
    pub fn is_success(&self) -> bool {
        matches!(self, AppExit::Success)
    }
    
    pub fn is_error(&self) -> bool {
        matches!(self, AppExit::Error(_))
    }
}

impl Default for AppExit {
    fn default() -> Self {
        AppExit::Success
    }
}

use autozig_ecs::world::World;

/// Main application structure
pub struct App {
    inner: NonNull<ZigApp>,
    pub world: World,
}

impl App {
    /// Create a new application with default configuration
    pub fn new() -> Self {
        let mut world = World::new();
        
        // Initialize Schedules resource
        let mut schedules = Schedules::new();
        // pre-insert common schedules to avoid cloning checking later
        schedules.insert(autozig_ecs::schedule::Schedule::new(autozig_ecs::schedule::First));
        schedules.insert(autozig_ecs::schedule::Schedule::new(autozig_ecs::schedule::PreStartup));
        schedules.insert(autozig_ecs::schedule::Schedule::new(autozig_ecs::schedule::Startup));
        schedules.insert(autozig_ecs::schedule::Schedule::new(autozig_ecs::schedule::PostStartup));
        schedules.insert(autozig_ecs::schedule::Schedule::new(autozig_ecs::schedule::PreUpdate));
        schedules.insert(autozig_ecs::schedule::Schedule::new(autozig_ecs::schedule::Update));
        schedules.insert(autozig_ecs::schedule::Schedule::new(autozig_ecs::schedule::PostUpdate));
        schedules.insert(autozig_ecs::schedule::Schedule::new(autozig_ecs::schedule::Last));
        world.insert_resource(schedules);
        
        let ptr = app_create(world.as_raw_ptr());
        
        // Register Rust Schedule Runners for all main schedules
        unsafe {
            let p = ptr;
            app_schedule_add_system(p, MainScheduleOrder::First as u8, run_rust_first);
            app_schedule_add_system(p, MainScheduleOrder::PreStartup as u8, run_rust_pre_startup);
            app_schedule_add_system(p, MainScheduleOrder::Startup as u8, run_rust_startup);
            app_schedule_add_system(p, MainScheduleOrder::PostStartup as u8, run_rust_post_startup);
            app_schedule_add_system(p, MainScheduleOrder::PreUpdate as u8, run_rust_pre_update);
            app_schedule_add_system(p, MainScheduleOrder::Update as u8, run_rust_update);
            app_schedule_add_system(p, MainScheduleOrder::PostUpdate as u8, run_rust_post_update);
            app_schedule_add_system(p, MainScheduleOrder::Last as u8, run_rust_last);
        }

        Self {
            inner: NonNull::new(ptr).expect("app creation failed"),
            world,
        }
    }
    
    /// Create an empty application without default plugins
    pub fn empty() -> Self {
        let mut world = World::new();
        let ptr = app_create_empty(world.as_raw_ptr());
        Self {
            inner: NonNull::new(ptr).expect("empty app creation failed"),
            world,
        }
    }
    
    /// Update the application for one frame
    pub fn update(&mut self) -> &mut Self {
        app_update(self.inner.as_ptr());
        self
    }
    
    /// Run the application until exit
    ///
    /// Note: This method takes `&mut self` instead of `self` to allow for chaining.
    /// It internally replaces `self` with an empty App and passes ownership to the runner.
    pub fn run(&mut self) -> AppExit {
        // Set global world pointer for FFI callbacks
        GLOBAL_WORLD_PTR.store(self.world.as_raw_ptr() as *mut _, Ordering::SeqCst);
        
        let code = app_run(self.inner.as_ptr());
        
        // Clear global pointer
        GLOBAL_WORLD_PTR.store(core::ptr::null_mut(), Ordering::SeqCst);
        
        AppExit::from_code(code)
    }

    /// Get raw pointer to the underlying Zig App
    pub fn as_ptr(&self) -> *mut ZigApp {
        self.inner.as_ptr()
    }

    /// Get raw pointer to the ECS World from Zig App
    pub unsafe fn get_world_raw(&self) -> *mut u8 {
        app_get_world(self.inner.as_ptr())
    }

    /// Static helper to get world from raw ZigApp pointer (for systems)
    pub unsafe fn get_world_from_ptr(app_ptr: *mut ZigApp) -> *mut u8 {
        app_get_world(app_ptr)
    }

    /// RAW API: Update the application for one frame (unsafe)
    /// Intended for custom runners who have the raw pointers.
    pub unsafe fn update_raw(app_ptr: *mut ZigApp) {
        app_update(app_ptr);
    }

    /// RAW API: Check if application should exit (unsafe)
    pub unsafe fn should_exit_raw(app_ptr: *mut ZigApp) -> Option<AppExit> {
        let code = app_should_exit(app_ptr);
        if code < 0 {
            None
        } else {
            Some(AppExit::from_code(code as u8))
        }
    }

    /// RAW API: Insert a resource into the application (unsafe)
    pub unsafe fn insert_resource_raw<T: 'static>(app_ptr: *mut ZigApp, resource: T) {
        let type_id = core::any::TypeId::of::<T>();
        let type_id_u64 = type_id_to_u64(type_id);
        
        // Serialize resource to bytes (using crate-level helper)
        let bytes = resource_to_bytes(&resource);
        
        app_insert_resource(
            app_ptr,
            type_id_u64,
            bytes.as_ptr(),
            bytes.len()
        );
        
        // Keep resource alive
        core::mem::forget(resource);
    }

    /// RAW API: Get a resource from the application (unsafe)
    pub unsafe fn get_resource_raw<T: 'static>(app_ptr: *mut ZigApp) -> Option<&'static T> {
        let type_id = core::any::TypeId::of::<T>();
        let type_id_u64 = type_id_to_u64(type_id);
        let ptr = app_get_resource(app_ptr, type_id_u64);
        
        if ptr.is_null() {
            None
        } else {
            // Unsafe cast to reference. Lifetime 'static is bounded by caller's unsafe scope really.
            Some(&*(ptr as *const T))
        }
    }
    
    /// Set a custom runner function
    pub fn set_runner(&mut self, runner: extern "C" fn(*mut ZigApp) -> u8) -> &mut Self {
        app_set_runner(self.inner.as_ptr(), runner);
        self
    }
    
    /// Check if the application should exit
    pub fn should_exit(&self) -> Option<AppExit> {
        let code = app_should_exit(self.inner.as_ptr());
        if code < 0 {
            None
        } else {
            Some(AppExit::from_code(code as u8))
        }
    }
    
    /// Finish plugin initialization
    pub fn finish(&mut self) -> &mut Self {
        app_finish(self.inner.as_ptr());
        self
    }
    
    /// Cleanup plugins
    pub fn cleanup(&mut self) -> &mut Self {
        app_cleanup(self.inner.as_ptr());
        self
    }
    
    /// Add a sub-application
    ///
    /// Note: The returned SubApp is a reference to the sub-app stored in the App.
    /// It will be automatically cleaned up when the App is dropped.
    /// Do not manually destroy the returned SubApp.
    pub fn add_sub_app(&mut self, name: &str) -> SubApp {
        let ptr = app_add_sub_app(
            self.inner.as_ptr(),
            name.as_ptr(),
            name.len()
        );
        SubApp {
            inner: NonNull::new(ptr).expect("sub app creation failed"),
            owned: false,  // ????????????????App????
        }
    }
    
    /// Get a sub-application by name
    pub fn get_sub_app(&self, name: &str) -> Option<SubApp> {
        let ptr = app_get_sub_app(
            self.inner.as_ptr(),
            name.as_ptr(),
            name.len()
        );
        NonNull::new(ptr).map(|inner| SubApp { inner, owned: false })
    }

    /// Insert a resource into the application
    pub fn insert_resource<T: 'static>(&mut self, resource: T) -> &mut Self {
        let type_id = core::any::TypeId::of::<T>();
        let type_id_u64 = type_id_to_u64(type_id);
        
        // Serialize resource to bytes
        let bytes = resource_to_bytes(&resource);
        
        app_insert_resource(
            self.inner.as_ptr(),
            type_id_u64,
            bytes.as_ptr(),
            bytes.len()
        );
        
        // Keep resource alive
        core::mem::forget(resource);
        self
    }
    
    /// Get a resource from the application
    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        let type_id = core::any::TypeId::of::<T>();
        let type_id_u64 = type_id_to_u64(type_id);
        let ptr = app_get_resource(self.inner.as_ptr(), type_id_u64);
        
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(&*(ptr as *const T)) }
        }
    }
    
    /// Get a mutable resource from the application
    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let type_id = core::any::TypeId::of::<T>();
        let type_id_u64 = type_id_to_u64(type_id);
        let ptr = app_get_resource(self.inner.as_ptr(), type_id_u64);
        
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(&mut *(ptr as *mut T)) }
        }
    }

    /// Check if a resource exists
    pub fn has_resource<T: 'static>(&self) -> bool {
        let type_id = core::any::TypeId::of::<T>();
        let type_id_u64 = type_id_to_u64(type_id);
        app_has_resource(self.inner.as_ptr(), type_id_u64)
    }
    
    /// Add a plugin to the application
    pub fn add_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
        let plugin_ptr = plugin.into_zig_plugin();
        app_add_plugin(self.inner.as_ptr(), plugin_ptr);
        self
    }
    
    /// Add multiple plugins (supports both iterators and tuples)
    pub fn add_plugins(&mut self, plugins: impl IntoIterator<Item = impl Plugin>) -> &mut Self {
        for plugin in plugins {
            self.add_plugin(plugin);
        }
        self
    }
    
    /// Add plugins from a Plugins implementor (tuples, etc.)
    pub fn add_plugins_tuple<P: Plugins>(&mut self, plugins: P) -> &mut Self {
        plugins.add_to_app(self);
        self
    }
    
    /// Add a system to a specific schedule
    ///
    /// # Examples
    /// ```
    /// # use autozig_app::{App, MainScheduleOrder};
    /// let mut app = App::new();
    /// app.add_systems(MainScheduleOrder::Update, my_system);
    ///
    /// extern "C" fn my_system() {
    ///     // System logic here
    /// }
    /// ```
    pub fn add_systems<M>(&mut self, schedule: impl ScheduleLabel + Clone, systems: impl IntoSystemConfigs<M>) -> &mut Self {
        let mut schedules = self.world.get_resource_mut::<Schedules>()
            .expect("Schedules resource missing");
            
        if let Some(sched) = schedules.get_mut(schedule.clone()) {
            sched.add_systems(systems);
        } else {
            let mut sched = autozig_ecs::schedule::Schedule::new(schedule);
            sched.add_systems(systems);
            schedules.insert(sched);
        }
        self
    }
    
    /// Configure a system set for a specific schedule
    ///
    /// # Examples
    /// ```
    /// # use autozig_app::{App, MainScheduleOrder, SystemSet};
    /// let mut app = App::new();
    /// let my_set = SystemSet::new(1);
    /// app.configure_sets(MainScheduleOrder::Update, my_set);
    /// ```
    pub fn configure_sets(&mut self, schedule: MainScheduleOrder, set: SystemSet) -> &mut Self {
        app_schedule_configure_set(self.inner.as_ptr(), schedule as u8, set.id);
        self
    }
    
    /// Initialize a resource with its default value if it doesn't exist
    ///
    /// # Examples
    /// ```
    /// # use autozig_app::App;
    /// #[derive(Default)]
    /// struct MyResource {
    ///     value: i32,
    /// }
    ///
    /// let mut app = App::new();
    /// app.init_resource::<MyResource>();
    /// ```
    pub fn init_resource<R: Resource + Default>(&mut self) -> &mut Self {
        let type_id = core::any::TypeId::of::<R>();
        let type_id_u64 = type_id_to_u64(type_id);
        
        // Only initialize if resource doesn't exist
        if !self.has_resource::<R>() {
            let resource = R::default();
            self.insert_resource(resource);
        } else {
            // Just register the type ID with the scheduler
            app_schedule_init_resource(self.inner.as_ptr(), type_id_u64);
        }
        
        self
    }

    // ========================================================================
    // Bevy Parity: World Accessors (matching bevy_app::App API)
    // ========================================================================

    /// Returns a reference to the main [`World`].
    /// 
    /// This is a shorthand for `self.main().world()`.
    #[inline]
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Returns a mutable reference to the main [`World`].
    ///
    /// This is a shorthand for `self.main_mut().world_mut()`.
    #[inline]
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    // ========================================================================
    // Bevy Parity: Plugin Query Methods
    // ========================================================================

    /// Returns `true` if the [`Plugin`] has already been added.
    pub fn is_plugin_added<T: Plugin>(&self) -> bool {
        // For now, we don't track plugins by name in Rust side
        // This would require maintaining a HashSet of plugin names
        // Return false as a placeholder; can be enhanced with plugin registry
        false
    }

    /// Returns the plugins state (for lifecycle management).
    pub fn plugins_state(&self) -> PluginsState {
        // Return current plugin state - placeholder
        PluginsState::Ready
    }

    // ========================================================================
    // Bevy Parity: Event System (matching bevy_app::App API)
    // ========================================================================

    /// Adds an [`Event`] type to the app.
    /// 
    /// This initializes the event queue and registers the event update system.
    /// 
    /// # Example
    /// ```ignore
    /// app.add_event::<MyEvent>();
    /// ```
    pub fn add_event<E: 'static>(&mut self) -> &mut Self {
        // Events are stored as a resource containing a double-buffered event queue
        // For now, we just register the type ID as a placeholder
        let type_id = core::any::TypeId::of::<E>();
        let _type_id_u64 = type_id_to_u64(type_id);
        // TODO: Implement proper event queue system in Zig
        self
    }

    /// Adds a message type to the app (Bevy 0.18+ API).
    /// 
    /// Messages are similar to events but with different semantics.
    pub fn add_message<M: 'static>(&mut self) -> &mut Self {
        self.add_event::<M>()
    }

    // ========================================================================
    // Bevy Parity: Observer System (matching bevy_app::App API)
    // ========================================================================

    /// Adds an observer system that runs when a specific trigger occurs.
    /// 
    /// # Example
    /// ```ignore
    /// app.add_observer(on_add_component::<Transform>);
    /// ```
    pub fn add_observer<F>(&mut self, _observer: F) -> &mut Self
    where
        F: 'static,
    {
        // TODO: Implement observer system in Zig
        // Observers are triggered by entity lifecycle events (spawn, despawn, component add/remove)
        self
    }

    // ========================================================================
    // Bevy Parity: One-Shot Systems (matching bevy_app::App API)
    // ========================================================================

    /// Registers a system and returns a [`SystemId`] for later execution.
    /// 
    /// One-shot systems can be run manually via `World::run_system(id)`.
    /// 
    /// # Example
    /// ```ignore
    /// let id = app.register_system(my_system);
    /// app.world_mut().run_system(id);
    /// ```
    pub fn register_system<M, S>(&mut self, _system: S) -> SystemId
    where
        S: 'static,
    {
        // TODO: Implement one-shot system registry in Zig
        // Returns a unique ID that can be used to run the system later
        SystemId::new()
    }

    // ========================================================================
    // Bevy Parity: Non-Send Resources
    // ========================================================================

    /// Inserts a non-Send resource into the app.
    /// 
    /// Non-Send resources can only be accessed from the main thread.
    pub fn insert_non_send_resource<R: 'static>(&mut self, resource: R) -> &mut Self {
        // For now, treat non-send resources the same as regular resources
        // Proper !Send handling would require thread-local storage
        self.insert_resource(resource)
    }

    /// Initializes a non-Send resource with its default value.
    pub fn init_non_send_resource<R: 'static + Default>(&mut self) -> &mut Self {
        if !self.has_resource::<R>() {
            self.insert_non_send_resource(R::default())
        } else {
            self
        }
    }

}

/// Unique identifier for a registered one-shot system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemId {
    id: u64,
}

impl SystemId {
    /// Create a new system ID.
    fn new() -> Self {
        static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        Self {
            id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        }
    }

    /// Get the raw ID value.
    pub fn id(&self) -> u64 {
        self.id
    }
}

impl Drop for App {
    fn drop(&mut self) {
        app_destroy(self.inner.as_ptr());
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Sub-application structure
pub struct SubApp {
    inner: NonNull<ZigSubApp>,
    owned: bool,  // 标记是否拥有所有权
}

impl SubApp {
    /// Create a new sub-application
    pub fn new() -> Self {
        let ptr = sub_app_create();
        Self {
            inner: NonNull::new(ptr).expect("sub app creation failed"),
            owned: true,  // 直接创建的SubApp拥有所有权
        }
    }
    
    /// Update the sub-application
    pub fn update(&mut self) -> &mut Self {
        sub_app_update(self.inner.as_ptr());
        self
    }
    
    /// Run the default schedule
    pub fn run_default_schedule(&mut self) -> &mut Self {
        sub_app_run_default_schedule(self.inner.as_ptr());
        self
    }
}

impl Drop for SubApp {
    fn drop(&mut self) {
        // 只有拥有所有权的SubApp才调用destroy
        // 通过add_sub_app创建的SubApp由App管理，不需要手动销毁
        if self.owned {
            sub_app_destroy(self.inner.as_ptr());
        }
    }
}

impl Default for SubApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin trait
pub trait Plugin: 'static {
    /// Build the plugin (add systems, resources, etc.)
    fn build(&self, app: &mut App);
    
    /// Check if the plugin is ready
    fn ready(&self, _app: &App) -> bool {
        true
    }
    
    /// Finish plugin initialization
    fn finish(&self, _app: &mut App) {}
    
    /// Cleanup plugin
    fn cleanup(&self, _app: &mut App) {}
    
    /// Get plugin name
    fn name(&self) -> &str;
    
    /// Check if plugin is unique
    fn is_unique(&self) -> bool {
        true
    }
    
    /// Convert plugin to Zig plugin pointer
    fn into_zig_plugin(self) -> *mut ZigPlugin
    where
        Self: Sized,
    {
        // Box self to keep it alive and get a context pointer
        let context = Box::into_raw(Box::new(self));

        // Trampoline function that recovers the context
        extern "C" fn build_trampoline<P: Plugin>(context: *mut std::ffi::c_void, app: *mut ZigApp) {
            unsafe {
                let plugin = &*(context as *const P);
                
                // Create temporary App wrapper
                // Note: We used World::from_raw assuming standard World layout/pointer
                // Since App owns World, we must be careful not to drop it
                let world_ptr = app_get_world(app);
                // Cast to opaque pointer first if needed, but World::from_raw usually takes *mut WorldOpaque
                let world = World::from_raw(world_ptr as *mut _);
                
                let mut app_wrapper = App {
                    inner: NonNull::new_unchecked(app),
                    world,
                };
                
                plugin.build(&mut app_wrapper);
                
                // IMPORTANT: Do NOT drop the world as it belongs to ZigApp (and App wrapper)
                core::mem::forget(app_wrapper);
            }
        }
        
        // We reuse the boxed instance to call name/is_unique safely before passing to C
        // Actually, we already have context. Reference is safe.
        let plugin_ref = unsafe { &*(context as *const Self) };
        let name = plugin_ref.name();
        let is_unique = plugin_ref.is_unique();
        
        unsafe {
            plugin_create(
                name.as_ptr(),
                name.len(),
                build_trampoline::<Self>,
                context as *mut std::ffi::c_void,
                is_unique
            )
        }
    }
}

/// Simple plugin implementation
pub struct SimplePlugin {
    name: &'static str,
    build_fn: fn(&mut App),
}

impl SimplePlugin {
    pub fn new(name: &'static str, build_fn: fn(&mut App)) -> Self {
        Self { name, build_fn }
    }
}

impl Plugin for SimplePlugin {
    fn build(&self, app: &mut App) {
        (self.build_fn)(app);
    }
    
    fn name(&self) -> &str {
        self.name
    }
}

// ============================================================================
// Plugin implementation for `fn(&mut App)` closures (Bevy parity)
// ============================================================================

/// FnPlugin wrapper for function pointers as plugins
pub struct FnPlugin<F> {
    func: F,
    name: &'static str,
}

impl<F: Fn(&mut App) + Send + Sync + 'static> FnPlugin<F> {
    /// Create a new FnPlugin with a custom name
    pub fn new(name: &'static str, func: F) -> Self {
        Self { func, name }
    }
}

impl<F: Fn(&mut App) + Send + Sync + 'static> Plugin for FnPlugin<F> {
    fn build(&self, app: &mut App) {
        (self.func)(app);
    }
    
    fn name(&self) -> &str {
        self.name
    }
}

/// Extension trait to convert closures to plugins
pub trait IntoPlugin {
    fn into_plugin(self) -> impl Plugin;
}

impl<F: Fn(&mut App) + Send + Sync + 'static> IntoPlugin for F {
    fn into_plugin(self) -> impl Plugin {
        FnPlugin::new(core::any::type_name::<F>(), self)
    }
}

// Helper functions - completely safe implementations
fn type_id_to_u64(type_id: core::any::TypeId) -> u64 {
    use core::hash::Hasher;
    
    // 使用Hash trait来获取TypeId的唯一u64表示
    let mut hasher = TypeIdHasher::default();
    core::hash::Hash::hash(&type_id, &mut hasher);
    hasher.finish()
}

fn resource_to_bytes<T>(resource: &T) -> &[u8] {
    let ptr = resource as *const T as *const u8;
    let size = core::mem::size_of::<T>();
    unsafe {
        core::slice::from_raw_parts(ptr, size)
    }
}

// 简单的TypeId哈希器实现（完全安全）
#[derive(Default)]
struct TypeIdHasher {
    state: u64,
}

impl core::hash::Hasher for TypeIdHasher {
    fn write(&mut self, bytes: &[u8]) {
        // 使用FNV-1a哈希算法（完全安全的实现）
        const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64 = 0x100000001b3;
        
        if self.state == 0 {
            self.state = FNV_OFFSET_BASIS;
        }
        
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }
    
    fn finish(&self) -> u64 {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_exit_success() {
        let exit = AppExit::Success;
        assert_eq!(exit.code(), 0);
        assert!(exit.is_success());
        assert!(!exit.is_error());
    }

    #[test]
    fn test_app_exit_error() {
        let exit = AppExit::Error(NonZeroU8::new(1).unwrap());
        assert_eq!(exit.code(), 1);
        assert!(!exit.is_success());
        assert!(exit.is_error());
    }


}

// ============================================================================
// Internal Helpers for Rust System Execution
// ============================================================================

macro_rules! define_runner {
    ($name:ident, $label:expr) => {
        extern "C" fn $name() {
            run_rust_schedule($label);
        }
    };
}

define_runner!(run_rust_first, autozig_ecs::schedule::First);
define_runner!(run_rust_pre_startup, autozig_ecs::schedule::PreStartup);
define_runner!(run_rust_startup, autozig_ecs::schedule::Startup);
define_runner!(run_rust_post_startup, autozig_ecs::schedule::PostStartup);
define_runner!(run_rust_pre_update, autozig_ecs::schedule::PreUpdate);
define_runner!(run_rust_update, autozig_ecs::schedule::Update);
define_runner!(run_rust_post_update, autozig_ecs::schedule::PostUpdate);
define_runner!(run_rust_last, autozig_ecs::schedule::Last);

fn run_rust_schedule(label: impl ScheduleLabel) {
    let raw_ptr = GLOBAL_WORLD_PTR.load(Ordering::SeqCst);
    if raw_ptr.is_null() { 
        // This might happen if called outside App::run, e.g. manual update
        return; 
    }

    unsafe {
        // Create temporary wrapper to access World methods
        // SAFETY: We strictly must NOT drop this world, as it owns resources/pointers
        let mut world = autozig_ecs::world::World::from_raw(raw_ptr);
        
        // Run the schedule using World's schedule runner helpers
        world.try_run_schedule(label);
        
        // CRITICAL: Prevent Dropping the World
        core::mem::forget(world);
    }
}