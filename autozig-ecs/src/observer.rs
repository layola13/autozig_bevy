//! Observer - Reactive system for responding to component changes (Bevy 0.14+)

use autozig_macro::include_zig;
use std::marker::PhantomData;
use crate::world::World;
use crate::entity::Entity;
use crate::component::ComponentId;

include_zig!("src/zig/system.zig", {
    fn observer_create() -> *mut u8;
    fn observer_trigger(observer: *mut u8, entity: Entity);
});

/// Observer that watches for specific events
#[repr(C)]
pub struct Observer {
    inner: *mut u8,
}

impl Observer {
    pub fn new() -> Self {
        unsafe {
            Self {
                inner: observer_create(),
            }
        }
    }
}

/// State of an observer
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObserverState {
    Active,
    Inactive,
    Disabled,
}

/// Descriptor for creating observers
pub struct ObserverDescriptor {
    pub component_id: ComponentId,
    pub event_type: ObserverEventType,
}

/// Types of events observers can watch
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObserverEventType {
    OnAdd,
    OnInsert,
    OnRemove,
    OnReplace,
}

/// Trigger for observer execution
#[repr(C)]
pub struct Trigger<'w, E> {
    pub entity: Entity,
    pub event: E,
    _marker: PhantomData<&'w ()>,
}

impl<'w, E> Trigger<'w, E> {
    pub fn entity(&self) -> Entity {
        self.entity
    }
    
    pub fn event(&self) -> &E {
        &self.event
    }
}

/// Event that triggers observers
pub trait TriggerEvent: Send + Sync + 'static {}

/// Targets for observer triggers
pub struct TriggerTargets {
    entities: Vec<Entity>,
}

impl TriggerTargets {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
    
    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
    
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
}

/// Trait for observer systems
pub trait ObserverSystem: Send + Sync + 'static {
    type Event: TriggerEvent;
    
    fn run(&mut self, trigger: Trigger<Self::Event>, world: &mut World);
}

/// Runner for executing observers
pub struct ObserverRunner {
    observers: Vec<Observer>,
}

impl ObserverRunner {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }
    
    pub fn add_observer(&mut self, observer: Observer) {
        self.observers.push(observer);
    }
    
    pub fn trigger(&mut self, entity: Entity) {
        for observer in &mut self.observers {
            unsafe {
                observer_trigger(observer.inner, entity);
            }
        }
    }
}

/// Marker for OnAdd events
#[derive(Clone, Copy, Debug)]
pub struct OnAdd;
impl TriggerEvent for OnAdd {}

/// Marker for OnInsert events
#[derive(Clone, Copy, Debug)]
pub struct OnInsert;
impl TriggerEvent for OnInsert {}

/// Marker for OnRemove events
#[derive(Clone, Copy, Debug)]
pub struct OnRemove;
impl TriggerEvent for OnRemove {}

/// Marker for OnReplace events
#[derive(Clone, Copy, Debug)]
pub struct OnReplace;
impl TriggerEvent for OnReplace {}

/// Observer attached to a specific entity
pub struct EntityObserver {
    entity: Entity,
    observer: Observer,
}

impl EntityObserver {
    pub fn new(entity: Entity, observer: Observer) -> Self {
        Self { entity, observer }
    }
    
    pub fn entity(&self) -> Entity {
        self.entity
    }
}

/// Observer attached to a component type
pub struct ComponentObserver<T> {
    observer: Observer,
    _marker: PhantomData<T>,
}

impl<T> ComponentObserver<T> {
    pub fn new(observer: Observer) -> Self {
        Self {
            observer,
            _marker: PhantomData,
        }
    }
}
// ============================================================================
// Observer Advanced Types - Observer高级类型
// ============================================================================

/// CachedComponentObservers - 缓存的组件观察者
pub struct CachedComponentObservers {
    observers: std::collections::HashMap<ComponentId, Vec<Observer>>,
}

impl CachedComponentObservers {
    pub fn new() -> Self {
        Self {
            observers: std::collections::HashMap::new(),
        }
    }
    
    pub fn add_observer(&mut self, component_id: ComponentId, observer: Observer) {
        self.observers
            .entry(component_id)
            .or_insert_with(Vec::new)
            .push(observer);
    }
    
    pub fn get_observers(&self, component_id: ComponentId) -> Option<&[Observer]> {
        self.observers.get(&component_id).map(|v| v.as_slice())
    }
    
    pub fn remove_observers(&mut self, component_id: ComponentId) -> Option<Vec<Observer>> {
        self.observers.remove(&component_id)
    }
    
    pub fn clear(&mut self) {
        self.observers.clear();
    }
}

impl Default for CachedComponentObservers {
    fn default() -> Self {
        Self::new()
    }
}

/// CachedObservers - 通用缓存的观察者集合
pub struct CachedObservers {
    on_add: Vec<Observer>,
    on_insert: Vec<Observer>,
    on_remove: Vec<Observer>,
    on_replace: Vec<Observer>,
}

impl CachedObservers {
    pub fn new() -> Self {
        Self {
            on_add: Vec::new(),
            on_insert: Vec::new(),
            on_remove: Vec::new(),
            on_replace: Vec::new(),
        }
    }
    
    pub fn add_on_add(&mut self, observer: Observer) {
        self.on_add.push(observer);
    }
    
    pub fn add_on_insert(&mut self, observer: Observer) {
        self.on_insert.push(observer);
    }
    
    pub fn add_on_remove(&mut self, observer: Observer) {
        self.on_remove.push(observer);
    }
    
    pub fn add_on_replace(&mut self, observer: Observer) {
        self.on_replace.push(observer);
    }
    
    pub fn get_on_add(&self) -> &[Observer] {
        &self.on_add
    }
    
    pub fn get_on_insert(&self) -> &[Observer] {
        &self.on_insert
    }
    
    pub fn get_on_remove(&self) -> &[Observer] {
        &self.on_remove
    }
    
    pub fn get_on_replace(&self) -> &[Observer] {
        &self.on_replace
    }
    
    pub fn clear(&mut self) {
        self.on_add.clear();
        self.on_insert.clear();
        self.on_remove.clear();
        self.on_replace.clear();
    }
}

impl Default for CachedObservers {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Observer Traits - Observer trait扩展
// ============================================================================

/// IntoObserverSystem - 转换为观察者系统trait
pub trait IntoObserverSystem<E: TriggerEvent, Marker> {
    type System: ObserverSystem<Event = E>;
    
    fn into_system(self) -> Self::System;
}

// 为函数实现IntoObserverSystem
impl<E, F, Marker> IntoObserverSystem<E, Marker> for F
where
    E: TriggerEvent,
    F: FnMut(Trigger<E>, &mut World) + Send + Sync + 'static,
{
    type System = FunctionObserverSystem<E, F>;
    
    fn into_system(self) -> Self::System {
        FunctionObserverSystem::new(self)
    }
}

/// FunctionObserverSystem - 函数观察者系统
pub struct FunctionObserverSystem<E: TriggerEvent, F> {
    func: F,
    _phantom: PhantomData<E>,
}

impl<E: TriggerEvent, F> FunctionObserverSystem<E, F> {
    pub fn new(func: F) -> Self {
        Self {
            func,
            _phantom: PhantomData,
        }
    }
}

impl<E, F> ObserverSystem for FunctionObserverSystem<E, F>
where
    E: TriggerEvent,
    F: FnMut(Trigger<E>, &mut World) + Send + Sync + 'static,
{
    type Event = E;
    
    fn run(&mut self, trigger: Trigger<Self::Event>, world: &mut World) {
        (self.func)(trigger, world);
    }
}