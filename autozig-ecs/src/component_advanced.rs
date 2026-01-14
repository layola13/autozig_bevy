//! Component Advanced API - 组件高级API

use crate::component::Component;
use crate::archetype::Archetype;
use std::marker::PhantomData;
use std::any::TypeId;

// Re-export HookContext
pub use crate::component::hooks::HookContext;

// ============================================================================
// Component ID and For - 组件ID和关联
// ============================================================================

/// ComponentIdFor<T> - 组件ID关联类型
pub struct ComponentIdFor<T: Component> {
    id: u32,
    _phantom: PhantomData<T>,
}

impl<T: Component> ComponentIdFor<T> {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl<T: Component> Clone for ComponentIdFor<T> {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<T: Component> Copy for ComponentIdFor<T> {}

// ============================================================================
// Component Registration - 组件注册
// ============================================================================

/// ComponentsRegistrator - 组件注册器
pub struct ComponentsRegistrator<'a> {
    components: &'a mut Components,
}

impl<'a> ComponentsRegistrator<'a> {
    /// Creates a new ComponentsRegistrator
    ///
    /// # Safety
    ///
    /// The caller must ensure exclusive access to the Components registry
    pub unsafe fn new(components: &'a mut Components) -> Self {
        Self { components }
    }
    
    /// Registers a component type and returns its ComponentId
    pub fn register_component<T: Component>(&mut self) -> crate::component::ComponentId {
        self.components.register::<T>(T::STORAGE_TYPE)
    }
    
    /// Registers a resource type and returns its ComponentId
    pub fn register_resource<T: 'static + Send + Sync + crate::resource::Resource>(&mut self) -> crate::component::ComponentId {
        self.components.register_resource_type::<T>()
    }
    
    pub fn register<T: Component>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.register::<T>(T::STORAGE_TYPE);
    }
    
    pub fn is_registered<T: Component>(&self) -> bool {
        self.components.indices.read().unwrap().contains_key(&TypeId::of::<T>())
    }
    
    pub fn len(&self) -> usize {
        self.components.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

/// ComponentsQueuedRegistrator - 组件队列注册器
pub struct ComponentsQueuedRegistrator {
    queue: Vec<TypeId>,
}

impl ComponentsQueuedRegistrator {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }
    
    pub fn enqueue<T: Component>(&mut self) {
        self.queue.push(TypeId::of::<T>());
    }
    
    pub fn drain(&mut self) -> Vec<TypeId> {
        std::mem::take(&mut self.queue)
    }
    
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl Default for ComponentsQueuedRegistrator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Archetype Generation - 原型代数
// ============================================================================

/// ArchetypeGeneration - 原型代数（用于追踪原型变化）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchetypeGeneration(pub u64);

impl ArchetypeGeneration {
    pub const fn initial() -> Self {
        Self(0)
    }
    
    pub fn next(&mut self) -> Self {
        let current = *self;
        self.0 += 1;
        current
    }
    
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Default for ArchetypeGeneration {
    fn default() -> Self {
        Self::initial()
    }
}

// ============================================================================
// Storage Types - 存储类型
// ============================================================================

/// TableStorage - 表存储标记
pub struct TableStorage;

/// SparseStorage - 稀疏存储标记
pub struct SparseStorage;

/// StorageType - 存储类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Table,
    SparseSet,
}

impl StorageType {
    pub fn is_table(&self) -> bool {
        matches!(self, Self::Table)
    }
    
    pub fn is_sparse_set(&self) -> bool {
        matches!(self, Self::SparseSet)
    }
}

// ============================================================================
// Component Reflect Errors - 组件反射错误
// ============================================================================

/// GetComponentReflectError - 获取组件反射错误
#[derive(Debug, Clone)]
pub enum GetComponentReflectError {
    ComponentNotFound(TypeId),
    NotRegistered(TypeId),
    NoReflect(TypeId),
}

impl std::fmt::Display for GetComponentReflectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComponentNotFound(id) => write!(f, "Component {:?} not found", id),
            Self::NotRegistered(id) => write!(f, "Component {:?} not registered", id),
            Self::NoReflect(id) => write!(f, "Component {:?} does not implement Reflect", id),
        }
    }
}

impl std::error::Error for GetComponentReflectError {}

// ============================================================================
// Component Hooks - 组件钩子
// ============================================================================


use crate::world::DeferredWorld;
use crate::entity::Entity;
use crate::component::ComponentId;
use std::sync::Arc;

/// ComponentHooks - 组件生命周期钩子
#[derive(Clone)]
pub struct ComponentHooks {
    pub(crate) on_add: Option<Arc<dyn Fn(DeferredWorld, HookContext) + Send + Sync>>,
    pub(crate) on_insert: Option<Arc<dyn Fn(DeferredWorld, HookContext) + Send + Sync>>,
    pub(crate) on_replace: Option<Arc<dyn Fn(DeferredWorld, HookContext) + Send + Sync>>,
    pub(crate) on_remove: Option<Arc<dyn Fn(DeferredWorld, HookContext) + Send + Sync>>,
}

impl std::fmt::Debug for ComponentHooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComponentHooks")
            .field("on_add", &self.on_add.is_some())
            .field("on_add", &self.on_add.is_some())
            .field("on_insert", &self.on_insert.is_some())
            .field("on_replace", &self.on_replace.is_some())
            .field("on_remove", &self.on_remove.is_some())
            .finish()
    }
}

impl ComponentHooks {
    pub fn new() -> Self {
        Self {
            on_add: None,
            on_insert: None,
            on_replace: None,
            on_remove: None,
        }
    }
    
    pub fn has_on_add(&self) -> bool {
        self.on_add.is_some()
    }
    
    pub fn has_on_insert(&self) -> bool {
        self.on_insert.is_some()
    }

    pub fn has_on_replace(&self) -> bool {
        self.on_replace.is_some()
    }
    
    pub fn has_on_remove(&self) -> bool {
        self.on_remove.is_some()
    }
    
    pub fn on_add<F>(&mut self, hook: F) -> &mut Self
    where
        F: Fn(DeferredWorld, HookContext) + Send + Sync + 'static,
    {
        self.on_add = Some(Arc::new(hook));
        self
    }
    
    pub fn on_insert<F>(&mut self, hook: F) -> &mut Self
    where
        F: Fn(DeferredWorld, HookContext) + Send + Sync + 'static,
    {
        self.on_insert = Some(Arc::new(hook));
        self
    }

    pub fn on_replace<F>(&mut self, hook: F) -> &mut Self
    where
        F: Fn(DeferredWorld, HookContext) + Send + Sync + 'static,
    {
        self.on_replace = Some(Arc::new(hook));
        self
    }
    
    pub fn on_remove<F>(&mut self, hook: F) -> &mut Self
    where
        F: Fn(DeferredWorld, HookContext) + Send + Sync + 'static,
    {
        self.on_remove = Some(Arc::new(hook));
        self
    }
    
    pub fn trigger_add(&self, world: DeferredWorld, context: HookContext) {
        if let Some(hook) = &self.on_add {
            hook(world, context);
        }
    }
    
    pub fn trigger_insert(&self, world: DeferredWorld, context: HookContext) {
        if let Some(hook) = &self.on_insert {
            hook(world, context);
        }
    }

    pub fn trigger_replace(&self, world: DeferredWorld, context: HookContext) {
        if let Some(hook) = &self.on_replace {
            hook(world, context);
        }
    }
    
    pub fn trigger_remove(&self, world: DeferredWorld, context: HookContext) {
        if let Some(hook) = &self.on_remove {
            hook(world, context);
        }
    }
}

impl Default for ComponentHooks {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Component Descriptor - 组件描述符
// ============================================================================

/// ComponentDescriptor - 组件描述符
pub struct ComponentDescriptor {
    name: String,
    type_id: TypeId,
    storage_type: StorageType,
    is_send_sync: bool,
}

impl ComponentDescriptor {
    pub fn new<T: Component>(storage_type: StorageType) -> Self {
        Self {
            name: std::any::type_name::<T>().to_string(),
            type_id: TypeId::of::<T>(),
            storage_type,
            is_send_sync: true,
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }
    
    pub fn storage_type(&self) -> StorageType {
        self.storage_type
    }
    
    pub fn is_send_sync(&self) -> bool {
        self.is_send_sync
    }
}

// ============================================================================
// Component Info - 组件信息
// ============================================================================

/// ComponentInfo - 组件信息
pub struct ComponentInfo {
    descriptor: ComponentDescriptor,
    hooks: ComponentHooks,
}

impl ComponentInfo {
    pub fn new(descriptor: ComponentDescriptor) -> Self {
        Self {
            descriptor,
            hooks: ComponentHooks::new(),
        }
    }
    
    pub fn descriptor(&self) -> &ComponentDescriptor {
        &self.descriptor
    }
    
    pub fn hooks(&self) -> &ComponentHooks {
        &self.hooks
    }
    
    pub fn hooks_mut(&mut self) -> &mut ComponentHooks {
        &mut self.hooks
    }
}

// ============================================================================
// Components Registry - 组件注册表
// ============================================================================

use crate::component::Components;

// Components struct removed (use crate::component::Components)

// ============================================================================
// Component Ticks - 组件时钟
// ============================================================================

/// Tick - 变化检测时钟
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(pub u32);

impl Tick {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
    
    pub fn get(&self) -> u32 {
        self.0
    }
    
    pub fn increment(&mut self) {
        self.0 = self.0.wrapping_add(1);
    }
}

/// ComponentTicks - 组件时钟（用于变化检测）
#[derive(Debug, Clone, Copy)]
pub struct ComponentTicks {
    pub added: Tick,
    pub changed: Tick,
}

impl ComponentTicks {
    pub fn new(tick: Tick) -> Self {
        Self {
            added: tick,
            changed: tick,
        }
    }
    
    pub fn is_added(&self, last_change_tick: Tick, change_tick: Tick) -> bool {
        self.added.get() > last_change_tick.get() && self.added.get() <= change_tick.get()
    }
    
    pub fn is_changed(&self, last_change_tick: Tick, change_tick: Tick) -> bool {
        self.changed.get() > last_change_tick.get() && self.changed.get() <= change_tick.get()
    }
    
    pub fn set_changed(&mut self, change_tick: Tick) {
        self.changed = change_tick;
    }
}

// ============================================================================
// Default Clone Behaviors - 默认克隆行为
// ============================================================================

/// DefaultCloneBehaviorBase - 默认克隆行为基础trait
pub trait DefaultCloneBehaviorBase {
    fn clone_value(&self) -> Box<dyn std::any::Any>;
}

/// DefaultCloneBehaviorViaClone - 通过Clone trait的默认克隆行为
pub trait DefaultCloneBehaviorViaClone: Clone + 'static {
    fn clone_boxed(&self) -> Box<dyn std::any::Any> {
        Box::new(self.clone())
    }
}

impl<T: Clone + 'static> DefaultCloneBehaviorViaClone for T {}

// ============================================================================
// DynEq and DynHash - 动态相等和哈希
// ============================================================================

/// DynEq - 动态相等trait
pub trait DynEq {
    fn dyn_eq(&self, other: &dyn std::any::Any) -> bool;
}

impl<T: PartialEq + 'static> DynEq for T {
    fn dyn_eq(&self, other: &dyn std::any::Any) -> bool {
        if let Some(other) = other.downcast_ref::<T>() {
            self == other
        } else {
            false
        }
    }
}

/// DynHash - 动态哈希trait
pub trait DynHash {
    fn dyn_hash(&self, state: &mut dyn std::hash::Hasher);
}

impl<T: std::hash::Hash + 'static> DynHash for T {
    fn dyn_hash(&self, mut state: &mut dyn std::hash::Hasher) {
        self.hash(&mut state);
    }
}

// ============================================================================
// No Bundle Effect - 无Bundle效果
// ============================================================================

/// NoBundleEffect - 无Bundle效果标记trait
pub trait NoBundleEffect {}