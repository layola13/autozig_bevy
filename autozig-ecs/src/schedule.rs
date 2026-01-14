//! Schedule - System scheduling and execution

use autozig_macro::{include_zig, Resource};
use std::marker::PhantomData;
use crate::world::World;
use crate::system::System;
use crate::into_system::{BoxedSystem, RawClosure, SystemTrampolineFn};
use crate::system_set::{SystemSet, SystemSetConfigs, IntoSystemSetConfigs};

include_zig!("src/zig/system.zig", {
    fn schedule_create() -> *mut u8;
    fn schedule_add_system(
        schedule: *mut u8, 
        name: *const u8, 
        name_len: usize, 
        data: *mut std::ffi::c_void,
        vtable: *mut std::ffi::c_void,
        trampoline: SystemTrampolineFn,
        access: u8
    ) -> bool;
    fn schedule_run(schedule: *mut u8, world: *mut u8);
    fn schedule_system_count(schedule: *const u8) -> usize;
    fn schedule_add_dependency(
        schedule: *mut u8,
        from: *const u8,
        from_len: usize,
        to: *const u8,
        to_len: usize
    ) -> bool;
    fn schedule_build(schedule: *mut u8) -> bool;
});

unsafe extern "C" fn run_system_trampoline(closure_ptr: *mut std::ffi::c_void, world_ptr: *mut std::ffi::c_void) {
    let closure = closure_ptr as *mut RawClosure;
    let ptr: *mut dyn System<In=(), Out=()> = std::mem::transmute(((*closure).data, (*closure).vtable));
    let world = &mut *(world_ptr as *mut World);
    (*ptr).run((), world);
}

/// A collection of systems that run in a specific order
#[repr(C)]
pub struct Schedule {
    inner: *mut u8,
    label: Box<dyn ScheduleLabel>,
}

// SAFETY: Schedule inner pointer is only accessed during system execution
unsafe impl Send for Schedule {}
unsafe impl Sync for Schedule {}

impl Schedule {
    pub fn new(label: impl ScheduleLabel) -> Self {
        Self {
            inner: schedule_create(),
            label: Box::new(label),
        }
    }
    
    pub fn add_systems<M>(&mut self, systems: impl IntoSystemConfigs<M>) -> &mut Self {
        let configs = systems.into_configs();
        let mut prev_name: Option<String> = None;

        for config in configs.configs {
            let mut system = config.system;

            if !config.conditions.is_empty() {
                use crate::condition::ConditionalSystem;
                use crate::system::BoxedSystem;
                let meta = system.meta().clone();
                let conditional = ConditionalSystem::new(system, config.conditions);
                system = BoxedSystem::from_inner(Box::new(conditional), meta);
            }

            let name = system.name().to_string();
            let (data, vtable) = system.into_raw_parts();
            
            // Register system node
            schedule_add_system(
                self.inner,
                name.as_ptr(),
                name.len(),
                data as *mut std::ffi::c_void,
                vtable as *mut std::ffi::c_void,
                run_system_trampoline,
                0 // TODO: Access flags
            );
            
            // Create backbone: start.name -> name -> end.name
            let start_node = format!("start.{}", name);
            let end_node = format!("end.{}", name);
            
            self.add_dependency(&start_node, &name);
            self.add_dependency(&name, &end_node);

            // Process sets (Containment)
            for set in config.in_sets {
                let set_start = format!("start.{}", set);
                let set_end = format!("end.{}", set);
                // set_start -> sys_start
                self.add_dependency(&set_start, &start_node);
                // sys_end -> set_end
                self.add_dependency(&end_node, &set_end);
            }

            // Process before (this runs before target)
            // end.this -> start.target
            for target in config.before {
                let target_start = format!("start.{}", target);
                self.add_dependency(&end_node, &target_start);
            }
            
            // Process after (this runs after target)
            // end.target -> start.this
            for target in config.after {
                let target_end = format!("end.{}", target);
                self.add_dependency(&target_end, &start_node);
            }

            // Handle chaining
            if configs.chained {
                if let Some(ref prev) = prev_name {
                    let prev_end = format!("end.{}", prev);
                    self.add_dependency(&prev_end, &start_node);
                }
                prev_name = Some(name);
            }
        }
        self
    }
    
    pub fn add_dependency(&mut self, from: &str, to: &str) {
        schedule_add_dependency(
            self.inner,
            from.as_ptr(),
            from.len(),
            to.as_ptr(),
            to.len()
        );
    }
    
    pub fn configure_sets(&mut self, sets: impl IntoSystemSetConfigs) -> &mut Self {
        let configs = sets.into_configs();
        for config in configs.configs {
            let set_name = config.set.as_str().to_string();
            let start_node = format!("start.{}", set_name);
            let end_node = format!("end.{}", set_name);
            
            // Create backbone for set: start -> end
            self.add_dependency(&start_node, &end_node);
            
            // Process containment: MySet in ParentSet
            for parent in config.in_sets {
                let parent_start = format!("start.{}", parent);
                let parent_end = format!("end.{}", parent);
                // parent_start -> my_start
                self.add_dependency(&parent_start, &start_node);
                // my_end -> parent_end
                self.add_dependency(&end_node, &parent_end);
            }
            
            // Process before: MySet runs before Target
            for target in config.before {
                let target_start = format!("start.{}", target);
                // my_end -> target_start
                self.add_dependency(&end_node, &target_start);
            }
            
            // Process after: MySet runs after Target
            for target in config.after {
                let target_end = format!("end.{}", target);
                // target_end -> my_start
                self.add_dependency(&target_end, &start_node);
            }
        }
        self
    }
    
    pub fn build(&mut self) -> Result<(), ScheduleBuildError> {
        if schedule_build(self.inner) {
            Ok(())
        } else {
            // TODO: Extract cycle info from Zig
            Err(ScheduleBuildError::DependencyCycle(vec!["Unknown cycle".to_string()]))
        }
    }

    pub fn run(&mut self, world: &mut World) {
        if self.build().is_err() {
            // Panic or log depending on requirements, Bevy usually panics on cycle during build
            panic!("Failed to build schedule: dependency cycle detected");
        }
        schedule_run(self.inner, world as *mut World as *mut u8);
    }
    
    pub fn system_count(&self) -> usize {
         schedule_system_count(self.inner)
    }
}

use crate::resource::Resource;

/// Container for multiple schedules
#[derive(Default)]
pub struct Schedules {
    schedules: Vec<Schedule>,
}

impl Resource for Schedules {}

use std::borrow::Cow;

/// Trait for schedule labels
pub trait ScheduleLabel: Send + Sync + 'static {
    fn label(&self) -> Cow<'static, str>;
}

impl ScheduleLabel for &'static str {
    fn label(&self) -> Cow<'static, str> {
        Cow::Borrowed(self)
    }
}

// ... other structs ...

impl Schedules {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn insert(&mut self, schedule: Schedule) {
        self.schedules.push(schedule);
    }
    
    pub fn get(&self, label: impl ScheduleLabel) -> Option<&Schedule> {
        let label_cow = label.label();
        self.schedules.iter().find(|s| s.label.label() == label_cow)
    }
    
    pub fn get_mut(&mut self, label: impl ScheduleLabel) -> Option<&mut Schedule> {
        let label_cow = label.label();
        self.schedules.iter_mut().find(|s| s.label.label() == label_cow)
    }
}
//...
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Startup;
impl ScheduleLabel for Startup {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("Startup") }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Update;
impl ScheduleLabel for Update {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("Update") }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FixedUpdate;
impl ScheduleLabel for FixedUpdate {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("FixedUpdate") }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PreUpdate;
impl ScheduleLabel for PreUpdate {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("PreUpdate") }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PostUpdate;
impl ScheduleLabel for PostUpdate {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("PostUpdate") }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Last;
impl ScheduleLabel for Last {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("Last") }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct First;
impl ScheduleLabel for First {
    fn label(&self) -> Cow<'static, str> { Cow::Borrowed("First") }
}
#[derive(Clone, Debug)]
pub struct ScheduleBuildSettings {
    pub ambiguity_detection: LogLevel,
    pub hierarchy_detection: LogLevel,
    pub auto_insert_apply_deferred: bool,
}

impl Default for ScheduleBuildSettings {
    fn default() -> Self {
        Self {
            ambiguity_detection: LogLevel::Ignore,
            hierarchy_detection: LogLevel::Warn,
            auto_insert_apply_deferred: true,
        }
    }
}

/// Executor type for running systems
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutorKind {
    Simple,
    SingleThreaded,
    MultiThreaded,
}

/// Log level for diagnostics
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Ignore,
    Warn,
    Error,
}

/// Internal schedule graph representation
pub struct ScheduleGraph {
    nodes: Vec<NodeId>,
}

impl ScheduleGraph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
}

/// Node identifier in schedule graph
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

/// Configuration for multiple nodes
pub struct NodeConfigs<T> {
    configs: Vec<T>,
}

pub use crate::system_config::{SystemConfigs, IntoSystemConfigs};


/// Stepping controller for debugging
pub struct Stepping {
    enabled: bool,
    state: SteppingState,
}

impl Stepping {
    pub fn new() -> Self {
        Self {
            enabled: false,
            state: SteppingState::default(),
        }
    }
    
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// State for stepping through systems
#[derive(Default)]
pub struct SteppingState {
    current_system: usize,
    paused: bool,
}

// ============================================================================
// Schedule Advanced Types - Schedule高级类型
// ============================================================================

/// ScheduleBuildError - Schedule构建错误
#[derive(Debug, Clone)]
pub enum ScheduleBuildError {
    /// 依赖循环
    DependencyCycle(Vec<String>),
    /// 冲突的系统
    ConflictingSystems(String, String),
    /// 未找到系统集
    SetNotFound(String),
}

impl std::fmt::Display for ScheduleBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DependencyCycle(cycle) => write!(f, "Dependency cycle: {:?}", cycle),
            Self::ConflictingSystems(a, b) => write!(f, "Conflicting systems: {} and {}", a, b),
            Self::SetNotFound(name) => write!(f, "System set not found: {}", name),
        }
    }
}

impl std::error::Error for ScheduleBuildError {}

/// ScheduleBuildWarning - Schedule构建警告
#[derive(Debug, Clone)]
pub enum ScheduleBuildWarning {
    /// 系统歧义
    Ambiguity(String, String),
    /// 层次结构问题
    HierarchyIssue(String),
}

/// ScheduleError - Schedule运行时错误
#[derive(Debug, Clone)]
pub enum ScheduleError {
    /// Schedule未找到
    NotFound(String),
    /// Schedule正在运行
    AlreadyRunning(String),
    /// 执行失败
    ExecutionFailed(String),
}

impl std::fmt::Display for ScheduleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(label) => write!(f, "Schedule not found: {}", label),
            Self::AlreadyRunning(label) => write!(f, "Schedule already running: {}", label),
            Self::ExecutionFailed(msg) => write!(f, "Schedule execution failed: {}", msg),
        }
    }
}

impl std::error::Error for ScheduleError {}

/// ScheduleCleanupPolicy - Schedule清理策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleCleanupPolicy {
    /// 从不清理
    Never,
    /// 每帧清理
    EveryFrame,
    /// 自定义间隔
    Interval(u32),
}

impl Default for ScheduleCleanupPolicy {
    fn default() -> Self {
        Self::EveryFrame
    }
}

/// ScheduleConfigs - Schedule配置枚举
pub enum ScheduleConfigs {
    Single(Schedule),
    Multiple(Vec<Schedule>),
}

/// ShouldUpdateMessages - 是否应该更新消息
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShouldUpdateMessages {
    Yes,
    No,
}

// ============================================================================
// World Advanced Types - World高级类型 (Moved to world module)
// ============================================================================

/// WorldChildBuilder - World子实体构建器
pub struct WorldChildBuilder<'w> {
    world: &'w mut World,
    parent: crate::entity::Entity,
}

impl<'w> WorldChildBuilder<'w> {
    pub fn new(world: &'w mut World, parent: crate::entity::Entity) -> Self {
        Self { world, parent }
    }
    
    pub fn spawn(&mut self) -> crate::entity::Entity {
        let entity = self.world.spawn_empty();
        // 实际应该设置parent-child关系
        entity.id()
    }
    
    pub fn spawn_bundle<B: crate::bundle::Bundle>(&mut self, bundle: B) -> crate::entity::Entity {
        let entity = self.world.spawn_empty();
        // TODO: 实际应该插入bundle到entity
        entity.id()
    }
    
    pub fn parent(&self) -> crate::entity::Entity {
        self.parent
    }
}

/// AutoInsertApplyDeferredPass - 自动插入apply_deferred pass
pub struct AutoInsertApplyDeferredPass {
    enabled: bool,
}

impl AutoInsertApplyDeferredPass {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
    
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for AutoInsertApplyDeferredPass {
    fn default() -> Self {
        Self::new(true)
    }
}

/// ConflictingSystems - 冲突的系统信息
pub struct ConflictingSystems {
    pub system_a: String,
    pub system_b: String,
    pub conflicts: Vec<String>,
}

impl ConflictingSystems {
    pub fn new(system_a: String, system_b: String) -> Self {
        Self {
            system_a,
            system_b,
            conflicts: Vec::new(),
        }
    }
    
    pub fn add_conflict(&mut self, conflict: String) {
        self.conflicts.push(conflict);
    }
}

/// ConditionWithAccess - 带访问信息的条件
pub struct ConditionWithAccess<C> {
    condition: C,
    access: crate::query::access::Access,
}

impl<C> ConditionWithAccess<C> {
    pub fn new(condition: C) -> Self {
        Self {
            condition,
            access: crate::query::access::Access::new(),
        }
    }
    
    pub fn condition(&self) -> &C {
        &self.condition
    }
    
    pub fn access(&self) -> &crate::query::access::Access {
        &self.access
    }
}

/// AnonymousSet - 匿名系统集
pub struct AnonymousSet {
    id: usize,
}

impl AnonymousSet {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
    
    pub fn id(&self) -> usize {
        self.id
    }
}

/// IntoSystemSetConfig - 转换为系统集配置trait
pub trait IntoSystemSetConfig {
    fn into_config(self) -> SystemSetConfigs;
}

/// Schedulable - 可调度trait
pub trait Schedulable {
    fn schedule_label(&self) -> &dyn ScheduleLabel;
}

/// ScheduleBuildPass - Schedule构建pass trait
pub trait ScheduleBuildPass: Send + Sync + 'static {
    fn run(&mut self, schedule: &mut Schedule);
}

// ============================================================================
// Schedule Configuration Helpers - Schedule配置辅助类型
// ============================================================================

/// IntoScheduleConfigs - 转换为Schedule配置trait
pub trait IntoScheduleConfigs {
    fn into_schedule_configs(self) -> ScheduleConfigs;
}

impl IntoScheduleConfigs for Schedule {
    fn into_schedule_configs(self) -> ScheduleConfigs {
        ScheduleConfigs::Single(self)
    }
}

impl IntoScheduleConfigs for Vec<Schedule> {
    fn into_schedule_configs(self) -> ScheduleConfigs {
        ScheduleConfigs::Multiple(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunMode {
    Once,
    Loop,
}

pub struct ScheduleRunnerPlugin {
    run_mode: RunMode,
    wait: Option<std::time::Duration>,
}

impl Default for ScheduleRunnerPlugin {
    fn default() -> Self {
        Self { run_mode: RunMode::Loop, wait: None }
    }
}

impl ScheduleRunnerPlugin {
    pub fn run_loop(wait: std::time::Duration) -> Self {
        Self { run_mode: RunMode::Loop, wait: Some(wait) }
    }
    
    pub fn run_once() -> Self {
        Self { run_mode: RunMode::Once, wait: None }
    }
}

use crate::plugin::{App, Plugin};
use crate::event::{Events, AppExit};


impl Plugin for ScheduleRunnerPlugin {
    fn build(&self, app: &mut App) {
        let mode = self.run_mode;
        let wait = self.wait;
        app.set_runner(move |mut app| {
             // Helper to run a schedule by removing it temporarily
             // Helper to run a schedule by removing it temporarily
             let run_schedule = |world: &mut crate::world::World, label: &dyn crate::schedule::ScheduleLabel| {
                 use crate::schedule::Schedules;
                 if let Some(mut schedules) = world.remove_resource::<Schedules>() {
                     // Manual lookup to avoid trait object issues with generic get_mut
                     let label_cow = label.label();
                     let schedule_opt = schedules.schedules.iter_mut()
                        .find(|s| s.label.label() == label_cow);
                        
                     if let Some(schedule) = schedule_opt {
                         schedule.run(world);
                     }
                     world.insert_resource(schedules);
                 }
             };

             match mode {
                RunMode::Once => {
                    run_schedule(&mut app.world, &Update);
                    run_schedule(&mut app.world, &Last);
                }
                RunMode::Loop => {
                    let mut ticks = 0;
                     loop {
                        run_schedule(&mut app.world, &Update);
                        
                        // Update event queues (especially AppExit)
                        if let Some(mut events) = app.world.get_resource_mut::<Events<AppExit>>() {
                            events.update();
                        }

                        run_schedule(&mut app.world, &Last);
                        
                        // Check for AppExit
                        let should_exit = if let Some(events) = app.world.get_resource::<Events<AppExit>>() {
                            let reader = events.get_reader();
                            reader.iter().count() > 0
                        } else {
                            false
                        };
                        
                        if should_exit { break; }

                        ticks += 1;
                        if ticks > 100 { break; } // Safety brake for CI/Test
                        
                        if let Some(w) = wait {
                            std::thread::sleep(w);
                        }
                    }
                }
             }
        });
    }
}