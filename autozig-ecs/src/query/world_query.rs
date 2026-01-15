//! WorldQuery trait and implementations
//! WorldQuery trait及其实现

use crate::{
    component::{Component, ComponentId},
    entity::Entity,
    query::{access::{Access, FilteredAccess}, fetch::Fetch},
};

/// Core trait for types that can be used in queries
pub trait QueryData: Send + Sync {
    type Item<'w>;
    type Fetch<'w>: Fetch<'w, Item = Self::Item<'w>, State = Self::State>;
    /// The state type used to maintain persistent data for this query
    type State: Send + Sync + 'static;
    type ReadOnly: ReadOnlyWorldQuery;
    const IS_READ_ONLY: bool;

    fn init_state(world: &mut crate::world::World) -> Self::State;

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

pub trait ReadOnlyWorldQuery: QueryData {}

pub use super::fetch::{EntityFetch, ReadFetch, WriteFetch, OptionFetch, RefFetch};
pub use QueryData as WorldQuery;
pub use ReadOnlyWorldQuery as ReadOnlyQueryData;

// Entity implementation
impl QueryData for Entity {
    type Item<'w> = Entity;
    type Fetch<'w> = EntityFetch;
    type State = ();
    type ReadOnly = Entity;
    const IS_READ_ONLY: bool = true;
    fn init_state(_world: &mut crate::world::World) -> <Self as QueryData>::State { () }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &<Self as QueryData>::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> <Self as QueryData>::Fetch<'w> { EntityFetch::init(state, world, last_run, this_run) }
    unsafe fn set_archetype<'w>(fetch: &mut <Self as QueryData>::Fetch<'w>, state: &<Self as QueryData>::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { fetch.set_archetype(state, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut <Self as QueryData>::Fetch<'w>, state: &<Self as QueryData>::State, table: &crate::storage::Table) { fetch.set_table(state, table); }
    fn get_access(_state: &<Self as QueryData>::State) -> Access { Access::new() }
    fn update_component_access(_state: &<Self as QueryData>::State, _access: &mut FilteredAccess) {}
    fn matches_component_set(_state: &<Self as QueryData>::State, _set: &[ComponentId]) -> bool { true }
}
impl ReadOnlyWorldQuery for Entity {}

// &T implementation
impl<'a, T: Component> QueryData for &'a T {
    type Item<'w> = &'w T;
    type Fetch<'w> = ReadFetch<T>;
    type State = ComponentId;
    type ReadOnly = &'static T;
    const IS_READ_ONLY: bool = true;
    fn init_state(world: &mut crate::world::World) -> <Self as QueryData>::State { world.register_component::<T>() }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &<Self as QueryData>::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> <Self as QueryData>::Fetch<'w> { ReadFetch::init(state, world, last_run, this_run) }
    unsafe fn set_archetype<'w>(fetch: &mut <Self as QueryData>::Fetch<'w>, state: &<Self as QueryData>::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { fetch.set_archetype(state, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut <Self as QueryData>::Fetch<'w>, state: &<Self as QueryData>::State, table: &crate::storage::Table) { fetch.set_table(state, table); }
    fn get_access(state: &<Self as QueryData>::State) -> Access { let mut access = Access::new(); access.add_component_read(*state); access }
    fn update_component_access(state: &<Self as QueryData>::State, access: &mut FilteredAccess) { access.add_component_read(*state); }
    fn matches_component_set(state: &<Self as QueryData>::State, set: &[ComponentId]) -> bool { set.contains(state) }
}
impl<'a, T: Component> ReadOnlyWorldQuery for &'a T {}

// &mut T implementation
impl<'a, T: Component> QueryData for &'a mut T {
    type Item<'w> = crate::change_detection::Mut<'w, T>;
    type Fetch<'w> = WriteFetch<T>;
    type State = ComponentId;
    type ReadOnly = &'static T;
    const IS_READ_ONLY: bool = false;
    fn init_state(world: &mut crate::world::World) -> <Self as QueryData>::State { world.register_component::<T>() }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &<Self as QueryData>::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> <Self as QueryData>::Fetch<'w> { WriteFetch::init(state, world, last_run, this_run) }
    unsafe fn set_archetype<'w>(fetch: &mut <Self as QueryData>::Fetch<'w>, state: &<Self as QueryData>::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { fetch.set_archetype(state, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut <Self as QueryData>::Fetch<'w>, state: &<Self as QueryData>::State, table: &crate::storage::Table) { fetch.set_table(state, table); }
    fn get_access(state: &<Self as QueryData>::State) -> Access { let mut access = Access::new(); access.add_component_write(*state); access }
    fn update_component_access(state: &<Self as QueryData>::State, access: &mut FilteredAccess) { access.add_component_write(*state); }
    fn matches_component_set(state: &<Self as QueryData>::State, set: &[ComponentId]) -> bool { set.contains(state) }
}

// Ref<'static, T> implementation
impl<'a, T: Component> QueryData for crate::change_detection::Ref<'a, T> {
    type Item<'w> = crate::change_detection::Ref<'w, T>;
    type Fetch<'w> = RefFetch<T>;
    type State = ComponentId;
    type ReadOnly = Self;
    const IS_READ_ONLY: bool = true;
    fn init_state(world: &mut crate::world::World) -> <Self as QueryData>::State { world.register_component::<T>() }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &<Self as QueryData>::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> <Self as QueryData>::Fetch<'w> { RefFetch::init(state, world, last_run, this_run) }
    unsafe fn set_archetype<'w>(fetch: &mut <Self as QueryData>::Fetch<'w>, state: &<Self as QueryData>::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { fetch.set_archetype(state, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut <Self as QueryData>::Fetch<'w>, state: &<Self as QueryData>::State, table: &crate::storage::Table) { fetch.set_table(state, table); }
    fn get_access(state: &<Self as QueryData>::State) -> Access { let mut access = Access::new(); access.add_component_read(*state); access }
    fn update_component_access(state: &<Self as QueryData>::State, access: &mut FilteredAccess) { access.add_component_read(*state); }
    fn matches_component_set(state: &<Self as QueryData>::State, set: &[ComponentId]) -> bool { set.contains(state) }
}
impl<'a, T: Component> ReadOnlyWorldQuery for crate::change_detection::Ref<'a, T> {}

// Option<T> implementation
impl<T: QueryData> QueryData for Option<T> {
    type Item<'w> = Option<T::Item<'w>>;
    type Fetch<'w> = OptionFetch<T::Fetch<'w>>;
    type State = T::State;
    type ReadOnly = Option<T::ReadOnly>;
    const IS_READ_ONLY: bool = T::IS_READ_ONLY;
    fn init_state(world: &mut crate::world::World) -> <Self as QueryData>::State { T::init_state(world) }
    unsafe fn init_fetch<'w>(world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, state: &<Self as QueryData>::State, last_run: crate::change_detection::Tick, this_run: crate::change_detection::Tick) -> <Self as QueryData>::Fetch<'w> { OptionFetch::new(T::init_fetch(world, state, last_run, this_run)) }
    unsafe fn set_archetype<'w>(fetch: &mut <Self as QueryData>::Fetch<'w>, state: &<Self as QueryData>::State, archetype: &crate::archetype::Archetype, table: &crate::storage::Table) { fetch.set_archetype(state, archetype, table); }
    unsafe fn set_table<'w>(fetch: &mut <Self as QueryData>::Fetch<'w>, state: &<Self as QueryData>::State, table: &crate::storage::Table) { fetch.set_table(state, table); }
    fn get_access(state: &<Self as QueryData>::State) -> Access { T::get_access(state) }
    fn update_component_access(state: &<Self as QueryData>::State, access: &mut FilteredAccess) { T::update_component_access(state, access); }
    fn matches_component_set(_state: &<Self as QueryData>::State, _set: &[ComponentId]) -> bool { true }
}
impl<T: ReadOnlyWorldQuery> ReadOnlyWorldQuery for Option<T> {}

impl QueryData for () {
    type Item<'w> = ();
    type Fetch<'w> = ();
    type State = ();
    type ReadOnly = ();
    const IS_READ_ONLY: bool = true;
    fn init_state(_: &mut crate::world::World) -> <Self as QueryData>::State { () }
    unsafe fn init_fetch<'w>(_: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>, _: &<Self as QueryData>::State, _: crate::change_detection::Tick, _: crate::change_detection::Tick) -> <Self as QueryData>::Fetch<'w> { () }
    unsafe fn set_archetype<'w>(_: &mut <Self as QueryData>::Fetch<'w>, _: &<Self as QueryData>::State, _: &crate::archetype::Archetype, _: &crate::storage::Table) {}
    unsafe fn set_table<'w>(_: &mut <Self as QueryData>::Fetch<'w>, _: &<Self as QueryData>::State, _: &crate::storage::Table) {}
    fn get_access(_: &<Self as QueryData>::State) -> Access { Access::new() }
    fn update_component_access(_: &<Self as QueryData>::State, _: &mut FilteredAccess) {}
    fn matches_component_set(_: &<Self as QueryData>::State, _: &[ComponentId]) -> bool { true }
}
impl ReadOnlyWorldQuery for () {}

// Tuple implementations follow using the correct macro with 3 identifiers
// (A, s_a, f_a)

macro_rules! impl_tuple_world_query {
    ($(($name:ident, $state_var:ident, $fetch_var:ident)),*) => {
        #[allow(non_snake_case)]
        impl<$($name: QueryData),*> QueryData for ($($name,)*) {
            type Item<'w> = ($($name::Item<'w>,)*);
            type Fetch<'w> = ($($name::Fetch<'w>,)*);
            type State = ($($name::State,)*);
            type ReadOnly = ($($name::ReadOnly,)*);
            const IS_READ_ONLY: bool = true $(&& $name::IS_READ_ONLY)*;

            fn init_state(world: &mut crate::world::World) -> <Self as QueryData>::State {
                ($($name::init_state(world),)*)
            }

            unsafe fn init_fetch<'w>(
                world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>,
                state: &<Self as QueryData>::State,
                last_run: crate::change_detection::Tick,
                this_run: crate::change_detection::Tick,
            ) -> <Self as QueryData>::Fetch<'w> {
                let ($($state_var,)*) = state;
                ($($name::init_fetch(world, $state_var, last_run, this_run),)*)
            }

            unsafe fn set_archetype<'w>(
                fetch: &mut <Self as QueryData>::Fetch<'w>,
                state: &<Self as QueryData>::State,
                archetype: &crate::archetype::Archetype,
                table: &crate::storage::Table,
            ) {
                let ($($state_var,)*) = state;
                let ($($fetch_var,)*) = fetch;
                $($name::set_archetype($fetch_var, $state_var, archetype, table);)*
            }

            unsafe fn set_table<'w>(
                fetch: &mut <Self as QueryData>::Fetch<'w>,
                state: &<Self as QueryData>::State,
                table: &crate::storage::Table,
            ) {
                let ($($state_var,)*) = state;
                let ($($fetch_var,)*) = fetch;
                $($name::set_table($fetch_var, $state_var, table);)*
            }

            fn get_access(state: &<Self as QueryData>::State) -> Access {
                let ($($state_var,)*) = state;
                let mut access = Access::new();
                $(
                    access.extend(&$name::get_access($state_var));
                )*
                access
            }

            fn update_component_access(state: &<Self as QueryData>::State, access: &mut FilteredAccess) {
                let ($($state_var,)*) = state;
                $($name::update_component_access($state_var, access);)*
            }

            fn matches_component_set(state: &<Self as QueryData>::State, set: &[ComponentId]) -> bool {
                let ($($state_var,)*) = state;
                true $(&& $name::matches_component_set($state_var, set))*
            }
        }

        impl<$($name: ReadOnlyWorldQuery),*> ReadOnlyWorldQuery for ($($name,)*) {}
    };
}

impl_tuple_world_query!((A, a, fa));
impl_tuple_world_query!((A, a, fa), (B, b, fb));
impl_tuple_world_query!((A, a, fa), (B, b, fb), (C, c, fc));
impl_tuple_world_query!((A, a, fa), (B, b, fb), (C, c, fc), (D, d, fd));
impl_tuple_world_query!((A, a, fa), (B, b, fb), (C, c, fc), (D, d, fd), (E, e, fe));
impl_tuple_world_query!((A, a, fa), (B, b, fb), (C, c, fc), (D, d, fd), (E, e, fe), (F, f, ff));
impl_tuple_world_query!((A, a, fa), (B, b, fb), (C, c, fc), (D, d, fd), (E, e, fe), (F, f, ff), (G, g, fg));
impl_tuple_world_query!((A, a, fa), (B, b, fb), (C, c, fc), (D, d, fd), (E, e, fe), (F, f, ff), (G, g, fg), (H, h, fh));
impl_tuple_world_query!((A, a, fa), (B, b, fb), (C, c, fc), (D, d, fd), (E, e, fe), (F, f, ff), (G, g, fg), (H, h, fh), (I, i, fi));
impl_tuple_world_query!((A, a, fa), (B, b, fb), (C, c, fc), (D, d, fd), (E, e, fe), (F, f, ff), (G, g, fg), (H, h, fh), (I, i, fi), (J, j, fj));
impl_tuple_world_query!((A, a, fa), (B, b, fb), (C, c, fc), (D, d, fd), (E, e, fe), (F, f, ff), (G, g, fg), (H, h, fh), (I, i, fi), (J, j, fj), (K, k, fk));
impl_tuple_world_query!((A, a, fa), (B, b, fb), (C, c, fc), (D, d, fd), (E, e, fe), (F, f, ff), (G, g, fg), (H, h, fh), (I, i, fi), (J, j, fj), (K, k, fk), (L, l, fl));


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