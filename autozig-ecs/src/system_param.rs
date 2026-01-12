//! System parameters - Types that can be used as system function arguments

use crate::world::World;
use crate::resource::{Res, ResMut, Resource};
use crate::command::Commands;
use crate::query::{Query, QueryData, QueryFilter, QueryState};
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

/// SystemParam - trait for types that can be used as system parameters
pub trait SystemParam: Sized {
    /// The item type that will be passed to the system function
    type Item<'w>;

    /// Fetch the parameter from the world
    fn fetch<'w>(world: &'w World) -> Self::Item<'w>;

    /// Get the access flags for this parameter
    fn access_flags() -> WorldAccessFlags;
}

/// ReadOnlySystemParam - Marker trait for read-only parameters
pub trait ReadOnlySystemParam: SystemParam {}

/// StaticSystemParam - Marker trait for parameters that don't depend on World
pub trait StaticSystemParam: SystemParam {}

// Implement for ()
impl SystemParam for () {
    type Item<'w> = ();
    fn fetch<'w>(_: &'w World) -> Self::Item<'w> { () }
    fn access_flags() -> WorldAccessFlags { WorldAccessFlags::default() }
}
impl ReadOnlySystemParam for () {}
impl StaticSystemParam for () {}

// Implement for Res<'_, T>
impl<T: Resource> SystemParam for Res<'_, T> {
    type Item<'w> = Res<'w, T>;

    fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
        world.resource::<T>()
    }

    fn access_flags() -> WorldAccessFlags {
        WorldAccessFlags {
            reads_resources: true,
            ..WorldAccessFlags::default()
        }
    }
}
impl<T: Resource> ReadOnlySystemParam for Res<'_, T> {}

// Implement for ResMut<'_, T>
impl<T: Resource> SystemParam for ResMut<'_, T> {
    type Item<'w> = ResMut<'w, T>;

    fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
        // SAFETY: We are calling it from a system which should have exclusive access if scheduled correctly
        let world_ptr = world as *const World as usize as *mut World;
        unsafe { (*world_ptr).resource_mut::<T>() }
    }

    fn access_flags() -> WorldAccessFlags {
        WorldAccessFlags {
            writes_resources: true,
            ..WorldAccessFlags::default()
        }
    }
}

// Implement for Commands<'_>
impl SystemParam for Commands<'_> {
    type Item<'w> = Commands<'w>;

    fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
        Commands::new(world)
    }

    fn access_flags() -> WorldAccessFlags {
        WorldAccessFlags::default()
    }
}

// Implement for Query
impl<Q: QueryData, F: QueryFilter> SystemParam for Query<'_, Q, F> {
    type Item<'w> = Query<'w, Q, F>;

    fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
        // This is inefficient as it creates a new QueryState every time
        // In a full implementation, QueryState would be cached in SystemState
        let state = QueryState::new(world);
        unsafe { Query::new(world, Box::leak(Box::new(state))) }
    }

    fn access_flags() -> WorldAccessFlags {
        WorldAccessFlags {
            reads_components: true,
            writes_components: Q::IS_READ_ONLY == false,
            ..WorldAccessFlags::default()
        }
    }
}

// Implement for Events
impl<E: Event> SystemParam for Events<E> {
    type Item<'w> = &'w Events<E>;

    fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
        let events = world.resource::<Events<E>>();
        events.ptr
    }

    fn access_flags() -> WorldAccessFlags {
        WorldAccessFlags {
            reads_resources: true,
            ..WorldAccessFlags::default()
        }
    }
}

// Implement for EventReader
impl<E: Event> SystemParam for EventReader<'_, E> {
    type Item<'w> = EventReader<'w, E>;

    fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
        let events = world.resource::<Events<E>>();
        EventReader::new(events.ptr.queue)
    }

    fn access_flags() -> WorldAccessFlags {
        WorldAccessFlags {
            reads_resources: true,
            ..WorldAccessFlags::default()
        }
    }
}

// Implement for EventWriter
impl<E: Event> SystemParam for EventWriter<'_, E> {
    type Item<'w> = EventWriter<'w, E>;

    fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
        let events = unsafe {
            let world_mut_ptr = world as *const World as *mut World;
            (*world_mut_ptr).resource_mut::<Events<E>>()
        };
        EventWriter::new(events.ptr.queue)
    }

    fn access_flags() -> WorldAccessFlags {
        WorldAccessFlags {
            writes_resources: true,
            ..WorldAccessFlags::default()
        }
    }
}

// Implement for RemovedComponents
impl<T: crate::component::Component> SystemParam for RemovedComponents<'_, T> {
    type Item<'w> = RemovedComponents<'w, T>;

    fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
        // In Bevy, these are typically stored in the world and accessed by ComponentId
        // Here we provide a simplified version that fetches from world's removed_components map
        RemovedComponents::new(world)
    }

    fn access_flags() -> WorldAccessFlags {
        WorldAccessFlags {
            reads_components: true,
            ..WorldAccessFlags::default()
        }
    }
}

// Macro to implement SystemParam for tuples
macro_rules! impl_system_param_tuple {
    ($($param:ident),*) => {
        impl<$($param: SystemParam),*> SystemParam for ($($param,)*) {
            type Item<'w> = ($($param::Item<'w>,)*);

            fn fetch<'w>(world: &'w World) -> Self::Item<'w> {
                ($($param::fetch(world),)*)
            }

            fn access_flags() -> WorldAccessFlags {
                let mut flags = WorldAccessFlags::default();
                $(
                    let f = $param::access_flags();
                    flags.reads_resources |= f.reads_resources;
                    flags.writes_resources |= f.writes_resources;
                    flags.reads_components |= f.reads_components;
                    flags.writes_components |= f.writes_components;
                )*
                flags
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
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8);
impl_system_param_tuple!(P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16);
