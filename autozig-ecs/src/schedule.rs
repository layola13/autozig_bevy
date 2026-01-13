//! Schedule - System scheduling and execution

use autozig_macro::include_zig;
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
});

unsafe extern "C" fn run_system_trampoline(closure_ptr: *mut std::ffi::c_void, world_ptr: *mut std::ffi::c_void) {
    let closure = closure_ptr as *mut RawClosure;
    let ptr: *mut dyn System = std::mem::transmute(((*closure).data, (*closure).vtable));
    let world = &mut *(world_ptr as *mut World);
    (*ptr).run(world);
}

/// A collection of systems that run in a specific order
#[repr(C)]
pub struct Schedule {
    inner: *mut u8,
    label: Box<dyn ScheduleLabel>,
}

impl Schedule {
    pub fn new(label: impl ScheduleLabel) -> Self {
        Self {
            inner: schedule_create(),
            label: Box::new(label),
        }
    }
    
    pub fn add_systems<M>(&mut self, _systems: impl IntoSystemConfigs<M>) -> &mut Self {
        // Stub: Actual scheduling handled by App struct in plugin.rs for now
        self
    }
    
    pub fn configure_sets(&mut self, sets: impl IntoSystemSetConfigs) -> &mut Self {
        self
    }
    
    pub fn set_build_settings(&mut self, settings: ScheduleBuildSettings) -> &mut Self {
        self
    }
    
    pub fn run(&mut self, world: &mut World) {
        schedule_run(self.inner, world as *mut World as *mut u8);
    }
}

/// Container for multiple schedules
#[derive(Default)]
pub struct Schedules {
    schedules: Vec<Schedule>,
}

impl Schedules {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn insert(&mut self, schedule: Schedule) {
        self.schedules.push(schedule);
    }
    
    pub fn get(&self, label: impl ScheduleLabel) -> Option<&Schedule> {
        self.schedules.first()
    }
    
    pub fn get_mut(&mut self, label: impl ScheduleLabel) -> Option<&mut Schedule> {
        self.schedules.first_mut()
    }
}

/// Trait for schedule labels
pub trait ScheduleLabel: Send + Sync + 'static {
    fn as_str(&self) -> &str;
}

impl ScheduleLabel for &'static str {
    fn as_str(&self) -> &str {
        self
    }
}

/// Settings for building a schedule
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



// Common schedule labels
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Startup;
impl ScheduleLabel for Startup {
    fn as_str(&self) -> &str { "Startup" }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Update;
impl ScheduleLabel for Update {
    fn as_str(&self) -> &str { "Update" }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FixedUpdate;
impl ScheduleLabel for FixedUpdate {
    fn as_str(&self) -> &str { "FixedUpdate" }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PreUpdate;
impl ScheduleLabel for PreUpdate {
    fn as_str(&self) -> &str { "PreUpdate" }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PostUpdate;
impl ScheduleLabel for PostUpdate {
    fn as_str(&self) -> &str { "PostUpdate" }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Last;
impl ScheduleLabel for Last {
    fn as_str(&self) -> &str { "Last" }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct First;
impl ScheduleLabel for First {
    fn as_str(&self) -> &str { "First" }
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
             match mode {
                RunMode::Once => {
                    for system in app.update_schedule.iter_mut() {
                        system.run(&mut app.world);
                    }
                    for system in app.last_schedule.iter_mut() {
                        system.run(&mut app.world);
                    }
                }
                RunMode::Loop => {
                    let mut ticks = 0;
                     loop {
                        for system in app.update_schedule.iter_mut() {
                            system.run(&mut app.world);
                        }
                        for system in app.last_schedule.iter_mut() {
                            system.run(&mut app.world);
                        }
                        
                        // Check for AppExit
                        if let Some(events) = app.world.get_resource::<Events<AppExit>>() {
                            // Note: Reader requires mutable access to events usually? 
                            // Bevy's Events<T> get_reader is &self -> Reader.
                            // Reader interacts with queue.
                            // My implementation: get_reader(&self).
                            let reader = events.get_reader();
                            if reader.iter().count() > 0 {
                                break; 
                            }
                        }

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