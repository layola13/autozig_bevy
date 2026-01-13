//! System parameters - Types that can be used as system function arguments

use crate::world::World;
use crate::resource::{Res, ResMut, Resource};
use crate::command::Commands;
use crate::query::{Query, QueryData, QueryFilter, QueryState, QueryStateInner};
use crate::event::{Events, EventReader, EventWriter, Event};
use crate::removal_detection::{RemovedComponentEvents, RemovedComponentEntity, RemovedComponents, RemovedComponentReader};
use std::marker::PhantomData;

/// World access flags for system scheduling
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct WorldAccessFlags {
    pub reads_resources: bool,
    pub writes_resources: bool,
    pub reads_components: bool,
    pub writes_components: bool,
}

impl WorldAccessFlags {
    pub fn merge(others: &[Self]) -> Self {
        let mut flags = Self::default();
        for other in others {
            flags.reads_resources |= other.reads_resources;
            flags.writes_resources |= other.writes_resources;
            flags.reads_components |= other.reads_components;
            flags.writes_components |= other.writes_components;
        }
        flags
    }

    pub fn to_u8(&self) -> u8 {
        let mut res = 0;
        if self.reads_resources { res |= 1; }
        if self.writes_resources { res |= 2; }
        if self.reads_components { res |= 4; }
        if self.writes_components { res |= 8; }
        res
    }
}

use crate::system::SystemMeta;

/// SystemParam - trait for types that can be used as system parameters
pub trait SystemParam: Sized {
    /// The state type used to maintain persistent data for this parameter across system runs
    type State: Send + Sync + 'static;

    /// The item type that will be passed to the system function
    type Item<'w>;

    /// Initialize the parameter state
    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State;

    // TODO: Add support for change_tick
    // fn get_param<'w, 's>(
    //     state: &'s mut Self::State,
    //     system_meta: &SystemMeta,
    //     world: &'w World,
    //     change_tick: u32,
    // ) -> Self::Item<'w>;

    /// Fetch the parameter from the world using state
    fn get_param<'w, 's>(
        state: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w>;

    /// Apply any deferred operations from this parameter
    fn apply(_state: &mut Self::State, _system_meta: &SystemMeta, _world: &mut World) {}
}

/// ReadOnlySystemParam - Marker trait for read-only parameters
pub trait ReadOnlySystemParam: SystemParam {}

/// StaticSystemParam - Marker trait for parameters that don't depend on World
pub trait StaticSystemParam: SystemParam {}

// Implement for ()
impl SystemParam for () {
    type State = ();
    type Item<'w> = ();

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        ()
    }

    fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        _world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        ()
    }
}
impl ReadOnlySystemParam for () {}
impl StaticSystemParam for () {}

// Implement for Res<'static, T>
impl<T: Resource> SystemParam for Res<'static, T> {
    type State = ();
    type Item<'w> = Res<'w, T>;

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        ()
    }

    fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        world.resource::<T>()
    }
}
impl<T: Resource> ReadOnlySystemParam for Res<'static, T> {}

// Implement for ResMut<'static, T>
impl<T: Resource> SystemParam for ResMut<'static, T> {
    type State = ();
    type Item<'w> = ResMut<'w, T>;

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        ()
    }

    fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        // SAFETY: System access validation ensures this is exclusive
        let world_ptr = world as *const World as usize as *mut World;
        unsafe { (*world_ptr).resource_mut::<T>() }
    }
}

// Implement for Commands<'static>
impl SystemParam for Commands<'static> {
    type State = crate::command::CommandBuffer; 
    type Item<'w> = Commands<'w>;

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        crate::command::CommandBuffer::new()
    }

    fn get_param<'w, 's>(
        state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        _world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        // SAFETY: The Commands lifetime is tied to the state which lives beyond this call
        unsafe { std::mem::transmute::<Commands<'s>, Commands<'w>>(state.commands()) }
    }

    fn apply(state: &mut Self::State, _system_meta: &SystemMeta, world: &mut World) {
        state.apply_with_world(world);
    }
}

// Implement for Query
impl<Q: QueryData, F: QueryFilter> SystemParam for Query<'static, Q, F> {
    type State = QueryState<Q, F>;
    type Item<'w> = Query<'w, Q, F>;

    fn init_state(world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        QueryStateInner::new::<Q, F>(world)
    }

    fn get_param<'w, 's>(
        state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        unsafe { Query::new(world, state as *const QueryState<Q, F>) }
    }
}

// Implement for Events
impl<E: Event> SystemParam for Events<E> {
    type State = ();
    type Item<'w> = Res<'w, Events<E>>;

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        ()
    }

    fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
         world.resource::<Events<E>>()
    }
}

// Implement for EventReader
impl<E: Event> SystemParam for EventReader<'static, E> {
    type State = ();
    type Item<'w> = EventReader<'w, E>;

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        ()
    }

    fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        let events = world.resource::<Events<E>>();
        EventReader::new(events.queue)
    }
}

// Implement for EventWriter
impl<E: Event> SystemParam for EventWriter<'static, E> {
    type State = ();
    type Item<'w> = EventWriter<'w, E>;

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        ()
    }

    fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
         let events = unsafe {
            let world_mut_ptr = world as *const World as *mut World;
            (*world_mut_ptr).resource_mut::<Events<E>>()
        };
        EventWriter::new(events.queue)
    }
}

// Implement for RemovedComponents
impl<T: crate::component::Component> SystemParam for RemovedComponents<'static, T> {
    type State = ();
    type Item<'w> = RemovedComponents<'w, T>;

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        ()
    }

    fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        RemovedComponents::new(world)
    }
}

// Macro to implement SystemParam for tuples
macro_rules! impl_system_param_tuple {
    ($($param:ident),*) => {
        impl<$($param: SystemParam),*> SystemParam for ($($param,)*) {
            type State = ($($param::State,)*);
            type Item<'w> = ($($param::Item<'w>,)*);

            fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
                (($($param::init_state(world, system_meta),)*))
            }

            fn get_param<'w, 's>(
                state: &'s mut Self::State,
                system_meta: &SystemMeta,
                world: &'w World,
                change_tick: u32,
            ) -> Self::Item<'w> {
                #[allow(non_snake_case)]
                let ($($param,)*) = state;
                ($($param::get_param($param, system_meta, world, change_tick),)*)
            }

            fn apply(state: &mut Self::State, system_meta: &SystemMeta, world: &mut World) {
                #[allow(non_snake_case)]
                let ($($param,)*) = state;
                ($($param::apply($param, system_meta, world),)*);
            }
        }
    }
}

impl_system_param_tuple!(P1);
impl_system_param_tuple!(P1, P2);
impl_system_param_tuple!(P1, P2, P3);
impl_system_param_tuple!(P1, P2, P3, P4);
impl_system_param_tuple!(P1, P2, P3, P4, P5);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16);

// ============================================================================
// P3 Advanced Features
// ============================================================================

/// Local<T> - System-local state that persists across runs
/// Each system instance gets its own copy of T
pub struct Local<'s, T: Default + Send + Sync + 'static> {
    pub inner: &'s mut T,
}

impl<'s, T: Default + Send + Sync + 'static> std::ops::Deref for Local<'s, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'s, T: Default + Send + Sync + 'static> std::ops::DerefMut for Local<'s, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl<T: Default + Send + Sync + 'static> SystemParam for Local<'static, T> {
    type State = T;
    type Item<'w> = Local<'w, T>;

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        T::default()
    }

    fn get_param<'w, 's>(
        state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        _world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        // SAFETY: State lifetime outlives the function call
        let state_ptr = state as *mut T;
        Local { inner: unsafe { &mut *state_ptr } }
    }
}

/// ParCommands - Parallel-safe command buffers for use in parallel iteration
pub struct ParCommands<'w> {
    inner: crate::command::CommandBuffer,
    _marker: PhantomData<&'w ()>,
}

impl<'w> ParCommands<'w> {
    pub fn commands(&mut self) -> Commands<'_> {
        self.inner.commands()
    }
}

impl SystemParam for ParCommands<'static> {
    type State = crate::command::CommandBuffer;
    type Item<'w> = ParCommands<'w>;

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        crate::command::CommandBuffer::new()
    }

    fn get_param<'w, 's>(
        state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        _world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        ParCommands {
            inner: crate::command::CommandBuffer::new(), // Each parallel scope gets its own buffer
            _marker: PhantomData,
        }
    }

    fn apply(state: &mut Self::State, _system_meta: &SystemMeta, world: &mut World) {
        state.apply_with_world(world);
    }
}

/// SystemChangeTick - Access to change detection ticks
pub struct SystemChangeTick {
    pub last_run: u32,
    pub this_run: u32,
}

impl SystemParam for SystemChangeTick {
    type State = ();
    type Item<'w> = SystemChangeTick;

    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        ()
    }

    fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        SystemChangeTick {
            last_run: system_meta.last_run().get(),
            this_run: world.read_change_tick().get(),
        }
    }
}

/// SystemName - Access to the current system's name
pub struct SystemName<'s>(pub &'s str);

impl<'s> std::ops::Deref for SystemName<'s> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl SystemParam for SystemName<'static> {
    type State = String;
    type Item<'w> = SystemName<'w>;

    fn init_state(_world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        system_meta.name().to_string()
    }

    fn get_param<'w, 's>(
        state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        _world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        // SAFETY: State lifetime outlives the function call  
        let state_ptr = state as *const String;
        unsafe { SystemName((*state_ptr).as_str()) }
    }
}
