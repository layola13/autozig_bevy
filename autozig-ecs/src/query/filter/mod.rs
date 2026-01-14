//! Query filter module
//! 查询过滤器模块
//!
//! Filters determine which entities match a query without fetching component data

use crate::{
    component::{Component, ComponentId},
    entity::Entity,
};
use std::marker::PhantomData;
use crate::change_detection::{Tick, ComponentTicks};
use crate::query::fetch::{Fetch, FetchState};

/// With filter - requires entity to have component
pub struct With<T: Component>(PhantomData<T>);

impl<T: Component> With<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> Default for With<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Without filter - requires entity to NOT have component
pub struct Without<T: Component>(PhantomData<T>);

impl<T: Component> Without<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> Default for Without<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Or filter - matches if any of the filters match
pub struct Or<T>(pub PhantomData<T>);

impl<T> Or<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for Or<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Changed filter - matches entities with changed components
pub struct Changed<T: Component>(PhantomData<T>);

impl<T: Component> Changed<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> Default for Changed<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Added filter - matches entities with newly added components
pub struct Added<T: Component>(PhantomData<T>);

impl<T: Component> Added<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> Default for Added<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Allow filter - allows all entities (no-op filter)
pub struct Allow;

impl Allow {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Allow {
    fn default() -> Self {
        Self::new()
    }
}

/// Spawned filter - matches entities that were just spawned
pub struct Spawned;

/// QueryFilter trait - marker trait for query filters
pub trait QueryFilter: Send + Sync + 'static {
    type State: FetchState;
    type Fetch<'w>: FilterFetch<'w, State = Self::State>;

    fn init_state(world: &crate::world::World) -> Self::State;
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool;
}

/// FilterFetch trait - internal fetch for filters
pub trait FilterFetch<'w>: Send + Sync {
    type State;
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: Tick, this_run: Tick) -> Self;
    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table);
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table);
    fn matches(&mut self, entity: Entity, index: usize) -> bool;
    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool;
}

impl QueryFilter for () {
    type State = ();
    type Fetch<'w> = ();
    fn init_state(_: &crate::world::World) -> Self::State { () }
    fn matches_component_set(_: &Self::State, _: &[ComponentId]) -> bool { true }
}

impl<'w> FilterFetch<'w> for () {
    type State = ();
    fn init(_: &Self::State, _: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _: Tick, _: Tick) -> Self { () }
    unsafe fn set_table(&mut self, _: &Self::State, _: &crate::storage::Table) {}
    unsafe fn set_archetype(&mut self, _: &Self::State, _: &crate::archetype::Archetype, _: &crate::storage::Table) {}
    fn matches(&mut self, _: Entity, _: usize) -> bool { true }
    fn matches_archetype(_: &Self::State, _: &crate::archetype::Archetype) -> bool { true }
}

/// With filter implementation
impl<T: Component> QueryFilter for With<T> {
    type State = ComponentId;
    type Fetch<'w> = WithFetch;
    fn init_state(world: &crate::world::World) -> Self::State {
        world.component_id::<T>().expect("Component not registered")
    }
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool {
        set.contains(state)
    }
}

pub struct WithFetch;
impl<'w> FilterFetch<'w> for WithFetch {
    type State = ComponentId;
    fn init(_: &Self::State, _: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _: Tick, _: Tick) -> Self { WithFetch }
    unsafe fn set_table(&mut self, _: &Self::State, _: &crate::storage::Table) {}
    unsafe fn set_archetype(&mut self, _: &Self::State, _: &crate::archetype::Archetype, _: &crate::storage::Table) {}
    fn matches(&mut self, _: Entity, _: usize) -> bool { true }
    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
        archetype.components().contains(state)
    }
}

/// Without filter implementation
impl<T: Component> QueryFilter for Without<T> {
    type State = ComponentId;
    type Fetch<'w> = WithoutFetch;
    fn init_state(world: &crate::world::World) -> Self::State {
        world.component_id::<T>().expect("Component not registered")
    }
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool {
        !set.contains(state)
    }
}

pub struct WithoutFetch;
impl<'w> FilterFetch<'w> for WithoutFetch {
    type State = ComponentId;
    fn init(_: &Self::State, _: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _: Tick, _: Tick) -> Self { WithoutFetch }
    unsafe fn set_table(&mut self, _: &Self::State, _: &crate::storage::Table) {}
    unsafe fn set_archetype(&mut self, _: &Self::State, _: &crate::archetype::Archetype, _: &crate::storage::Table) {}
    fn matches(&mut self, _: Entity, _: usize) -> bool { true }
    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
        !archetype.components().contains(state)
    }
}

/// Marker component for tick tracking in filters
pub(crate) struct TickMarker;
impl crate::component::Component for TickMarker {}

/// Changed filter implementation
impl<T: Component> QueryFilter for Changed<T> {
    type State = ComponentId;
    type Fetch<'w> = ChangedFetch;
    fn init_state(world: &crate::world::World) -> Self::State {
        world.component_id::<T>().expect("Component not registered")
    }
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool {
        set.contains(state)
    }
}

pub struct ChangedFetch {
    fetch: crate::query::fetch::ReadFetch<TickMarker>,
}

impl<'w> FilterFetch<'w> for ChangedFetch {
    type State = ComponentId;
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: Tick, this_run: Tick) -> Self {
        Self {
            fetch: crate::query::fetch::ReadFetch::new(*state, last_run, this_run),
        }
    }
    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) {
        self.fetch.set_table(state, table);
    }
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
        self.fetch.set_archetype(state, archetype, table);
    }
    fn matches(&mut self, _entity: Entity, index: usize) -> bool {
        let ticks_ptr = crate::query::fetch::fetch_get_ticks_at(self.fetch.inner, index);
        if ticks_ptr.is_null() { return false; }
        unsafe { (*ticks_ptr).is_changed(self.fetch.last_run, self.fetch.this_run) }
    }
    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
        archetype.components().contains(state)
    }
}

/// Added filter implementation
impl<T: Component> QueryFilter for Added<T> {
    type State = ComponentId;
    type Fetch<'w> = AddedFetch;
    fn init_state(world: &crate::world::World) -> Self::State {
        world.component_id::<T>().expect("Component not registered")
    }
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool {
        set.contains(state)
    }
}

pub struct AddedFetch {
    fetch: crate::query::fetch::ReadFetch<TickMarker>,
}

impl<'w> FilterFetch<'w> for AddedFetch {
    type State = ComponentId;
    fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: Tick, this_run: Tick) -> Self {
        Self {
            fetch: crate::query::fetch::ReadFetch::new(*state, last_run, this_run),
        }
    }
    unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) {
        self.fetch.set_table(state, table);
    }
    unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
        self.fetch.set_archetype(state, archetype, table);
    }
    fn matches(&mut self, _entity: Entity, index: usize) -> bool {
        let ticks_ptr = crate::query::fetch::fetch_get_ticks_at(self.fetch.inner, index);
        if ticks_ptr.is_null() { return false; }
        unsafe { (*ticks_ptr).is_added(self.fetch.last_run, self.fetch.this_run) }
    }
    fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
        archetype.components().contains(state)
    }
}

unsafe impl Send for ChangedFetch {}
unsafe impl Sync for ChangedFetch {}
unsafe impl Send for AddedFetch {}
unsafe impl Sync for AddedFetch {}

/// Allow filter implementation
impl QueryFilter for Allow {
    type State = ();
    type Fetch<'w> = AllowFetch;
    fn init_state(_world: &crate::world::World) -> Self::State { () }
    fn matches_component_set(_state: &Self::State, _set: &[ComponentId]) -> bool { true }
}

pub struct AllowFetch;

impl<'w> FilterFetch<'w> for AllowFetch {
    type State = ();
    fn init(_state: &Self::State, _world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _last_run: Tick, _this_run: Tick) -> Self {
        Self
    }
    unsafe fn set_table(&mut self, _state: &Self::State, _table: &crate::storage::Table) {}
    unsafe fn set_archetype(&mut self, _state: &Self::State, _archetype: &crate::archetype::Archetype, _table: &crate::storage::Table) {}
    fn matches(&mut self, _entity: Entity, _index: usize) -> bool { true }
    fn matches_archetype(_: &Self::State, _: &crate::archetype::Archetype) -> bool { true }
}

/// Spawned filter implementation
impl QueryFilter for Spawned {
    type State = ();
    type Fetch<'w> = SpawnedFetch;
    fn init_state(_world: &crate::world::World) -> Self::State { () }
    fn matches_component_set(_state: &Self::State, _set: &[ComponentId]) -> bool { true }
}

pub struct SpawnedFetch;

impl<'w> FilterFetch<'w> for SpawnedFetch {
    type State = ();
    fn init(_state: &Self::State, _world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _last_run: Tick, _this_run: Tick) -> Self {
        Self
    }
    unsafe fn set_table(&mut self, _state: &Self::State, _table: &crate::storage::Table) {}
    unsafe fn set_archetype(&mut self, _state: &Self::State, _archetype: &crate::archetype::Archetype, _table: &crate::storage::Table) {}
    fn matches(&mut self, _entity: Entity, _index: usize) -> bool {
        true
    }
    fn matches_archetype(_: &Self::State, _: &crate::archetype::Archetype) -> bool { true }
}

// Redefine OrFetch to generic wrapper for tuples
pub struct OrFetch<T>(T);

// Implement generic OrFetch logic via macro for tuples
macro_rules! impl_or_filter_tuple {
    ($(($name:ident, $state:ident, $fetch:ident, $idx:tt)),*) => {
        // Implement QueryFilter for Or<(A, B, ...)>
        impl<$($name: QueryFilter),*> QueryFilter for Or<($($name,)*)> {
            type State = ($($name::State,)*);
            type Fetch<'w> = OrFetch<($($name::Fetch<'w>,)*)>;
            
            fn init_state(world: &crate::world::World) -> Self::State {
                ($($name::init_state(world),)*)
            }
            
            fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool {
                let ($($state,)*) = state;
                false $(|| $name::matches_component_set($state, set))*
            }
        }
        
        // Implement FilterFetch for OrFetch<(FA, FB, ...)>
        impl<'w, $($name: FilterFetch<'w>),*> FilterFetch<'w> for OrFetch<($($name,)*)> {
            type State = ($($name::State,)*);
            
            fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: Tick, this_run: Tick) -> Self {
                let ($($state,)*) = state;
                OrFetch(($($name::init($state, world, last_run, this_run),)*))
            }
            
            unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) {
                let ($($state,)*) = state;
                let inner = &mut self.0;
                $(inner.$idx.set_table($state, table);)*
            }
            
            unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
                let ($($state,)*) = state;
                let inner = &mut self.0;
                $(inner.$idx.set_archetype($state, archetype, table);)*
            }
            
            fn matches(&mut self, entity: Entity, index: usize) -> bool {
                let inner = &mut self.0;
                false $(|| inner.$idx.matches(entity, index))*
            }
            
            fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
                let ($($state,)*) = state;
                false $(|| $name::matches_archetype($state, archetype))*
            }
        }
    };
}

impl_or_filter_tuple!((A, sa, fa, 0), (B, sb, fb, 1));
impl_or_filter_tuple!((A, sa, fa, 0), (B, sb, fb, 1), (C, sc, fc, 2));
impl_or_filter_tuple!((A, sa, fa, 0), (B, sb, fb, 1), (C, sc, fc, 2), (D, sd, fd, 3));
impl_or_filter_tuple!((A, sa, fa, 0), (B, sb, fb, 1), (C, sc, fc, 2), (D, sd, fd, 3), (E, se, fe, 4));

// Original Or<(A, B)> is covered by tuple macro (A, B) case above.
// So we can remove the manual implementation for Or<(A, B)>.
// And Or<T> for single T? Or is usually Or<(A, B)>.
// If Or<T> where T is single type, it's just T.
// Bevy provides Or<(A,)>? No.
// We only support Or of tuples.

macro_rules! impl_query_filter_tuple {
    ($(($name:ident, $state_var:ident, $fetch_var:ident)),*) => {
        impl<$($name: QueryFilter),*> QueryFilter for ($($name,)*) {
            type State = ($($name::State,)*);
            type Fetch<'w> = ($($name::Fetch<'w>,)*);

            fn init_state(world: &crate::world::World) -> Self::State {
                ($($name::init_state(world),)*)
            }

            fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool {
                let ($($state_var,)*) = state;
                true $(&& $name::matches_component_set($state_var, set))*
            }
        }

        impl<'w, $($name: FilterFetch<'w>),*> FilterFetch<'w> for ($($name,)*) {
            type State = ($($name::State,)*);

            fn init(state: &Self::State, world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, last_run: Tick, this_run: Tick) -> Self {
                let ($($state_var,)*) = state;
                ($($name::init($state_var, world, last_run, this_run),)*)
            }

            unsafe fn set_table(&mut self, state: &Self::State, table: &crate::storage::Table) {
                let ($($state_var,)*) = state;
                let ($($fetch_var,)*) = self;
                $($fetch_var.set_table($state_var, table);)*
            }

            unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) {
                let ($($state_var,)*) = state;
                let ($($fetch_var,)*) = self;
                $($fetch_var.set_archetype($state_var, archetype, table);)*
            }

            fn matches(&mut self, entity: Entity, index: usize) -> bool {
                let ($($fetch_var,)*) = self;
                true $(&& $fetch_var.matches(entity, index))*
            }

            fn matches_archetype(state: &Self::State, archetype: &crate::archetype::Archetype) -> bool {
                let ($($state_var,)*) = state;
                true $(&& $name::matches_archetype($state_var, archetype))*
            }
        }
    };
}

impl_query_filter_tuple!((A, sa, fa));
impl_query_filter_tuple!((A, sa, fa), (B, sb, fb));
impl_query_filter_tuple!((A, sa, fa), (B, sb, fb), (C, sc, fc));
impl_query_filter_tuple!((A, sa, fa), (B, sb, fb), (C, sc, fc), (D, sd, fd));
impl_query_filter_tuple!((A, sa, fa), (B, sb, fb), (C, sc, fc), (D, sd, fd), (E, se, fe));

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct Position { x: f32, y: f32 }
    impl crate::component::Component for Position {}

    #[derive(Debug, Clone, Copy)]
    struct Velocity { x: f32, y: f32 }
    impl crate::component::Component for Velocity {}

    #[test]
    fn test_with_filter() {
        let _filter: With<Position> = With::new();
    }

    #[test]
    fn test_without_filter() {
        let _filter: Without<Velocity> = Without::new();
    }

    #[test]
    fn test_or_filter() {
        let _filter: Or<(With<Position>, With<Velocity>)> = Or::new();
    }

    #[test]
    fn test_changed_filter() {
        let _filter: Changed<Position> = Changed::new();
    }

    #[test]
    fn test_added_filter() {
        let _filter: Added<Position> = Added::new();
    }
}