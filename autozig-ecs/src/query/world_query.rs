//! WorldQuery trait and implementations
//! WorldQuery trait及其实现

use crate::{
    component::{Component, ComponentId},
    entity::Entity,
    query::{access::{Access, FilteredAccess}, fetch::Fetch},
};

/// Core trait for types that can be used in queries
pub trait WorldQuery: Send + Sync {
    type Item<'w>;
    type Fetch<'w>: Fetch<'w, Item = Self::Item<'w>>;
    type State: Send + Sync + 'static;
    type ReadOnly: ReadOnlyWorldQuery;

    fn init_state(world: &crate::world::World) -> Self::State;

    unsafe fn init_fetch<'w>(
        world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>,
        state: &Self::State,
        last_run: crate::change_detection::Tick,
        this_run: crate::change_detection::Tick,
    ) -> Self::Fetch<'w>;

    unsafe fn set_archetype<'w>(
        fetch: &mut Self::Fetch<'w>,
        state: &Self::State,
        archetype: &crate::archetype::Archetype,
        table: &crate::storage::Table,
    );

    unsafe fn set_table<'w>(
        fetch: &mut Self::Fetch<'w>,
        state: &Self::State,
        table: &crate::storage::Table,
    );

    fn get_access(state: &Self::State) -> Access;
    fn update_component_access(state: &Self::State, access: &mut FilteredAccess);
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool;
}

pub trait ReadOnlyWorldQuery: WorldQuery {}

pub use super::fetch::{EntityFetch, ReadFetch, WriteFetch, OptionFetch};

// Entity implementation
impl WorldQuery for Entity {
    type Item<'w> = Entity;
    type Fetch<'w> = EntityFetch;
    type State = ();
    type ReadOnly = Entity;
    fn init_state(_world: &crate::world::World) -> Self::State { () }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &Self::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self::Fetch<'w> { EntityFetch::init(state, world, last_run, this_run) }
    unsafe fn set_archetype<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { fetch.set_archetype(state, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &crate::storage::Table) { fetch.set_table(state, table); }
    fn get_access(_state: &Self::State) -> Access { Access::new() }
    fn update_component_access(_state: &Self::State, _access: &mut FilteredAccess) {}
    fn matches_component_set(_state: &Self::State, _set: &[ComponentId]) -> bool { true }
}
impl ReadOnlyWorldQuery for Entity {}

// &T implementation
impl<'a, T: Component> WorldQuery for &'a T {
    type Item<'w> = &'w T;
    type Fetch<'w> = ReadFetch<T>;
    type State = ComponentId;
    type ReadOnly = &'a T;
    fn init_state(world: &crate::world::World) -> Self::State { world.component_id::<T>().expect("Component not registered") }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &Self::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self::Fetch<'w> { ReadFetch::init(state, world, last_run, this_run) }
    unsafe fn set_archetype<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { fetch.set_archetype(state, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &crate::storage::Table) { fetch.set_table(state, table); }
    fn get_access(state: &Self::State) -> Access { let mut access = Access::new(); access.add_component_read(*state); access }
    fn update_component_access(state: &Self::State, access: &mut FilteredAccess) { access.add_component_read(*state); }
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool { set.contains(state) }
}
impl<'a, T: Component> ReadOnlyWorldQuery for &'a T {}

// &mut T implementation
impl<'a, T: Component> WorldQuery for &'a mut T {
    type Item<'w> = &'w mut T;
    type Fetch<'w> = WriteFetch<T>;
    type State = ComponentId;
    type ReadOnly = &'a T;
    fn init_state(world: &crate::world::World) -> Self::State { world.component_id::<T>().expect("Component not registered") }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &Self::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self::Fetch<'w> { WriteFetch::init(state, world, last_run, this_run) }
    unsafe fn set_archetype<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { fetch.set_archetype(state, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &crate::storage::Table) { fetch.set_table(state, table); }
    fn get_access(state: &Self::State) -> Access { let mut access = Access::new(); access.add_component_write(*state); access }
    fn update_component_access(state: &Self::State, access: &mut FilteredAccess) { access.add_component_write(*state); }
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool { set.contains(state) }
}

// Option<T> implementation
impl<T: WorldQuery> WorldQuery for Option<T> {
    type Item<'w> = Option<T::Item<'w>>;
    type Fetch<'w> = OptionFetch<T::Fetch<'w>>;
    type State = T::State;
    type ReadOnly = Option<T::ReadOnly>;
    fn init_state(world: &crate::world::World) -> Self::State { T::init_state(world) }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &Self::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self::Fetch<'w> { OptionFetch::new(T::init_fetch(world, state, last_run, this_run)) }
    unsafe fn set_archetype<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { T::set_archetype(&mut fetch.inner, state, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &crate::storage::Table) { T::set_table(&mut fetch.inner, state, table); }
    fn get_access(state: &Self::State) -> Access { T::get_access(state) }
    fn update_component_access(state: &Self::State, access: &mut FilteredAccess) { T::update_component_access(state, access); }
    fn matches_component_set(_state: &Self::State, _set: &[ComponentId]) -> bool { true }
}
impl<T: ReadOnlyWorldQuery> ReadOnlyWorldQuery for Option<T> {}

impl WorldQuery for () {
    type Item<'w> = ();
    type Fetch<'w> = ();
    type State = ();
    type ReadOnly = ();
    fn init_state(_: &crate::world::World) -> Self::State { () }
    unsafe fn init_fetch<'w>(_: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _: &Self::State, _: crate::change_detection::Tick, _: crate::change_detection::Tick) -> Self::Fetch<'w> { () }
    unsafe fn set_archetype<'w>(_: &mut Self::Fetch<'w>, _: &Self::State, _: &crate::archetype::Archetype, _: &crate::storage::Table) {}
    unsafe fn set_table<'w>(_: &mut Self::Fetch<'w>, _: &Self::State, _: &crate::storage::Table) {}
    fn get_access(_: &Self::State) -> Access { Access::new() }
    fn update_component_access(_: &Self::State, _: &mut FilteredAccess) {}
    fn matches_component_set(_: &Self::State, _: &[ComponentId]) -> bool { true }
}
impl ReadOnlyWorldQuery for () {}

// 1-tuple implementation
impl<A: WorldQuery> WorldQuery for (A,) {
    type Item<'w> = (A::Item<'w>,);
    type Fetch<'w> = (A::Fetch<'w>,);
    type State = (A::State,);
    type ReadOnly = (A::ReadOnly,);
    fn init_state(world: &crate::world::World) -> Self::State { (A::init_state(world),) }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &Self::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self::Fetch<'w> { (A::init_fetch(world, &state.0, last_run, this_run),) }
    unsafe fn set_archetype<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { A::set_archetype(&mut fetch.0, &state.0, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &crate::storage::Table) { A::set_table(&mut fetch.0, &state.0, table); }
    fn get_access(state: &Self::State) -> Access { A::get_access(&state.0) }
    fn update_component_access(state: &Self::State, access: &mut FilteredAccess) { A::update_component_access(&state.0, access); }
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool { A::matches_component_set(&state.0, set) }
}
impl<A: ReadOnlyWorldQuery> ReadOnlyWorldQuery for (A,) {}

// 2-tuple implementation
impl<A: WorldQuery, B: WorldQuery> WorldQuery for (A, B) {
    type Item<'w> = (A::Item<'w>, B::Item<'w>);
    type Fetch<'w> = (A::Fetch<'w>, B::Fetch<'w>);
    type State = (A::State, B::State);
    type ReadOnly = (A::ReadOnly, B::ReadOnly);
    fn init_state(world: &crate::world::World) -> Self::State { (A::init_state(world), B::init_state(world)) }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &Self::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> Self::Fetch<'w> { (A::init_fetch(world, &state.0, last_run, this_run), B::init_fetch(world, &state.1, last_run, this_run)) }
    unsafe fn set_archetype<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { A::set_archetype(&mut fetch.0, &state.0, archetype, table); B::set_archetype(&mut fetch.1, &state.1, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: &crate::storage::Table) { A::set_table(&mut fetch.0, &state.0, table); B::set_table(&mut fetch.1, &state.1, table); }
    fn get_access(state: &Self::State) -> Access { let mut access = A::get_access(&state.0); access.extend(&B::get_access(&state.1)); access }
    fn update_component_access(state: &Self::State, access: &mut FilteredAccess) { A::update_component_access(&state.0, access); B::update_component_access(&state.1, access); }
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool { A::matches_component_set(&state.0, set) && B::matches_component_set(&state.1, set) }
}
impl<A: ReadOnlyWorldQuery, B: ReadOnlyWorldQuery> ReadOnlyWorldQuery for (A, B) {}

pub trait QueryData: WorldQuery {}
pub trait ReadOnlyQueryData: QueryData + ReadOnlyWorldQuery {}
impl<T: WorldQuery> QueryData for T {}
impl<T: ReadOnlyWorldQuery> ReadOnlyQueryData for T {}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::world::World;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Position { pub x: f32, pub y: f32 }
    impl Component for Position {}

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Velocity { pub x: f32, pub y: f32 }
    impl Component for Velocity {}

    #[test]
    fn test_world_query_tuple() {
        fn assert_world_query<T: WorldQuery>() {}
        assert_world_query::<&Position>();
        assert_world_query::<(&Position, &Velocity)>();
    }
}