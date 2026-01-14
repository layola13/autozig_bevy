//! Observer - Reactive system for responding to component changes (Bevy 0.14+)

use autozig_macro::{include_zig, Resource, Component};
use std::marker::PhantomData;
use crate::world::World;
use crate::entity::Entity;
use crate::component::ComponentId;
use crate::into_system::RawClosure;
use std::ffi::c_void;

include_zig!("src/zig/system.zig", {
    fn observer_create(
        data: *mut c_void, 
        vtable: *mut c_void, 
        trampoline: unsafe extern "C" fn(*mut c_void, Entity, *mut c_void)
    ) -> *mut u8;
    fn observer_trigger(observer: *mut u8, entity: Entity, world: *mut c_void);
    fn observer_destroy(observer: *mut u8);
});

unsafe extern "C" fn observer_trampoline<E: Default + TriggerEvent>(
    closure_ptr: *mut c_void, 
    entity: Entity, 
    world_ptr: *mut c_void
) {
    let closure = closure_ptr as *mut RawClosure;
    // We assume the stored closure is `Box<dyn ObserverSystem<Event=E>>`.
    // Reconstructing trait object pointer
    let system: *mut dyn ObserverSystem<Event=E> = std::mem::transmute(((*closure).data, (*closure).vtable));
    let world = &mut *(world_ptr as *mut World);
    
    // Construct trigger
    let trigger = Trigger {
        entity,
        event: E::default(),
        _marker: PhantomData,
    };
    
    (*system).run(trigger, world);
}

/// Observer that watches for specific events
#[repr(C)]
#[repr(C)]
pub struct Observer<E: TriggerEvent> {
    inner: *mut u8,
    _marker: PhantomData<E>,
}

unsafe impl<E: TriggerEvent> Send for Observer<E> {}
unsafe impl<E: TriggerEvent> Sync for Observer<E> {}
impl<E: TriggerEvent> crate::component::Component for Observer<E> {}

impl<E: TriggerEvent> Observer<E> {
    pub fn new<S>(system: S) -> Self 
    where 
        E: TriggerEvent + Default,
        S: ObserverSystem<Event = E> + 'static
    {
        let boxed_system: Box<dyn ObserverSystem<Event=E>> = Box::new(system);
        let ptr = Box::into_raw(boxed_system);
        let (data, vtable): (*mut c_void, *mut c_void) = unsafe { std::mem::transmute(ptr) };
        
        Self {
            inner: observer_create(data, vtable, observer_trampoline::<E>),
            _marker: PhantomData,
        }
    }
}

impl<E: TriggerEvent> Drop for Observer<E> {
    fn drop(&mut self) {
        observer_destroy(self.inner);
    }
}


/// ObserverList - List of observers for a specific event type
pub struct ObserverList<E: TriggerEvent> {
    pub observers: Vec<Observer<E>>,
}

impl<E: TriggerEvent> ObserverList<E> {
    pub fn trigger(&mut self, entity: Entity, world: &mut World) {
        for observer in &mut self.observers {
            // Unsafe: calling FFI function with opaque pointer
            observer_trigger(observer.inner, entity, world as *mut World as *mut c_void);
        }
    }
}

impl<E: TriggerEvent> Default for ObserverList<E> {
    fn default() -> Self {
        Self { observers: Vec::new() }
    }
}

unsafe impl<E: TriggerEvent> Send for ObserverList<E> {}
unsafe impl<E: TriggerEvent> Sync for ObserverList<E> {}
impl<E: TriggerEvent + 'static> crate::resource::Resource for ObserverList<E> {}

// Remove previously defined CachedComponentObservers and CachedObservers as they are superseded by generic ObserverList
#[repr(C)]
pub struct Trigger<E> {
    pub entity: Entity,
    pub event: E,
    pub(crate) _marker: PhantomData<E>,
}

impl<E> Trigger<E> {
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

/// Runner for executing observers - type-erased version
pub struct ObserverRunner {
    // Type-erased: stores raw function pointers
    _marker: std::marker::PhantomData<()>,
}

impl ObserverRunner {
    pub fn new() -> Self {
        Self { _marker: std::marker::PhantomData }
    }
}

/// Trait for types that can be converted to ObserverSystem
pub trait IntoObserverSystem<E, M>: Sized {
    type System: ObserverSystem<Event = E>;
    fn into_observer_system(self) -> Self::System;
}

use crate::component::Component;

/// Marker for OnAdd events
#[derive(Clone, Copy, Debug)]
pub struct OnAdd<C>(PhantomData<C>);
impl<C> Default for OnAdd<C> { fn default() -> Self { Self(PhantomData) } }
impl<C: Component> TriggerEvent for OnAdd<C> {}

/// Marker for OnInsert events
#[derive(Clone, Copy, Debug)]
pub struct OnInsert<C>(PhantomData<C>);
impl<C> Default for OnInsert<C> { fn default() -> Self { Self(PhantomData) } }
impl<C: Component> TriggerEvent for OnInsert<C> {}

/// Marker for OnRemove events
#[derive(Clone, Copy, Debug)]
pub struct OnRemove<C>(PhantomData<C>);
impl<C> Default for OnRemove<C> { fn default() -> Self { Self(PhantomData) } }
impl<C: Component> TriggerEvent for OnRemove<C> {}

/// Marker for OnReplace events
#[derive(Clone, Copy, Debug)]
pub struct OnReplace<C>(PhantomData<C>);
impl<C> Default for OnReplace<C> { fn default() -> Self { Self(PhantomData) } }
impl<C: Component> TriggerEvent for OnReplace<C> {}


// ============================================================================
// Observer Traits - Observer trait扩展
// ============================================================================

// ... imports ...
use crate::resource::Resource as ResourceTrait;
use crate::system_param::{SystemParam, ReadOnlySystemParam};
use crate::system::{SystemMeta, System};

// ...

struct CurrentTrigger<E: TriggerEvent>(Trigger<E>);
unsafe impl<E: TriggerEvent> Send for CurrentTrigger<E> {}
unsafe impl<E: TriggerEvent> Sync for CurrentTrigger<E> {}
impl<E: TriggerEvent + 'static> crate::resource::Resource for CurrentTrigger<E> {}

// Impl SystemParam for Trigger
impl<E: Component + Clone + TriggerEvent> crate::system_param::SystemParam for Trigger<E> {
    type State = ();
    type Item<'w> = Trigger<E>;
    
    fn init_state(_world: &mut World, _system_meta: &mut crate::system::SystemMeta) -> Self::State {
        ()
    }
    
    fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        _system_meta: &crate::system::SystemMeta,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
         // Access world resource "CurrentTrigger"
         match world.get_resource::<CurrentTrigger<E>>() {
             Some(current) => Trigger {
                 entity: current.0.entity,
                 event: current.0.event.clone(),
                 _marker: PhantomData,
             },
             None => {
                 // Return a dummy trigger or panic?
                 // Since SystemParam creation happens before system run,
                 // if we are outside observer, this should fail or panic.
                 panic!("Trigger<E> used outside of Observer or CurrentTrigger resource missing.");
             }
         }
    }
}

// ... ObserverSystem traits ...

/// GenericObserverSystem - Adapts a System to be an ObserverSystem
pub struct GenericObserverSystem<S, E> {
    system: S,
    _marker: PhantomData<E>,
}

impl<S, E> GenericObserverSystem<S, E> {
    pub fn new(system: S) -> Self {
        Self { system, _marker: PhantomData }
    }
}

impl<S, E> ObserverSystem for GenericObserverSystem<S, E>
where
    S: System<In=()> + 'static,
    E: TriggerEvent + Clone + 'static,
{
    type Event = E;
    fn run(&mut self, trigger: Trigger<E>, world: &mut World) {
        let prev = world.remove_resource::<CurrentTrigger<E>>();
        world.insert_resource(CurrentTrigger(trigger)); // Moves trigger
        
        self.system.run((), world);
        
        world.remove_resource::<CurrentTrigger<E>>();
        if let Some(p) = prev {
            world.insert_resource(p);
        }
    }
}

// Implement IntoObserverSystem for any S: IntoSystem where S::System is used
impl<E, M, S> IntoObserverSystem<E, M> for S
where
    E: TriggerEvent + Clone + 'static,
    S: crate::into_system::IntoSystem<M>,
{
    type System = GenericObserverSystem<crate::system::BoxedSystem, E>;
    
    fn into_observer_system(self) -> Self::System {
        GenericObserverSystem::new(self.into_system())
    }
}