//! WorldQuery trait and implementations
//! WorldQuery trait及其实现

use crate::{
    component::{Component, ComponentId},
    entity::Entity,
    query::access::{Access, FilteredAccess},
};
use std::marker::PhantomData;

/// Core trait for types that can be used in queries
/// 
/// This trait defines how a type interacts with the ECS world during queries.
/// Types implementing this trait can be used in Query<T>.
pub trait WorldQuery: Send + Sync {
    /// The item type returned by this query
    type Item<'w>;
    
    /// The fetch type used to retrieve data
    type Fetch<'w>;
    
    /// The state type that stores query metadata
    type State: Send + Sync + 'static;

    /// The read-only version of this query data
    type ReadOnly: ReadOnlyWorldQuery;

    /// Initialize the state for this query
    fn init_state(world: &crate::world::World) -> Self::State;

    /// Get the component access information
    fn get_access(state: &Self::State) -> Access;

    /// Update the component access based on state
    fn update_component_access(state: &Self::State, access: &mut FilteredAccess);

    /// Check if this query matches an archetype
    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool;
}

/// Marker trait for read-only world queries
pub trait ReadOnlyWorldQuery: WorldQuery {}

/// Impl WorldQuery for Entity
impl WorldQuery for Entity {
    type Item<'w> = Entity;
    type Fetch<'w> = EntityFetch;
    type State = ();
    type ReadOnly = Entity;

    fn init_state(_world: &crate::world::World) -> Self::State {
        ()
    }

    fn get_access(_state: &Self::State) -> Access {
        Access::new()
    }

    fn update_component_access(_state: &Self::State, _access: &mut FilteredAccess) {
        // Entity access doesn't modify component access
    }

    fn matches_component_set(_state: &Self::State, _set: &[ComponentId]) -> bool {
        true // Entity always matches
    }
}

impl ReadOnlyWorldQuery for Entity {}

/// Entity fetch implementation
pub struct EntityFetch;

/// Impl WorldQuery for &T (immutable component reference)
impl<'a, T: Component> WorldQuery for &'a T {
    type Item<'w> = &'w T;
    type Fetch<'w> = ReadFetch<T>;
    type State = ComponentId;
    type ReadOnly = &'a T;

    fn init_state(world: &crate::world::World) -> Self::State {
        world.component_id::<T>().expect("Component not registered")
    }

    fn get_access(state: &Self::State) -> Access {
        let mut access = Access::new();
        access.add_component_read(*state);
        access
    }

    fn update_component_access(state: &Self::State, access: &mut FilteredAccess) {
        access.add_component_read(*state);
    }

    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool {
        set.contains(state)
    }
}

impl<T: Component> ReadOnlyWorldQuery for &T {}

/// Read fetch implementation
pub struct ReadFetch<T> {
    _marker: PhantomData<T>,
}

/// Impl WorldQuery for &mut T (mutable component reference)
impl<'a, T: Component> WorldQuery for &'a mut T {
    type Item<'w> = &'w mut T;
    type Fetch<'w> = WriteFetch<T>;
    type State = ComponentId;
    type ReadOnly = &'a T;

    fn init_state(world: &crate::world::World) -> Self::State {
        world.component_id::<T>().expect("Component not registered")
    }

    fn get_access(state: &Self::State) -> Access {
        let mut access = Access::new();
        access.add_component_write(*state);
        access
    }

    fn update_component_access(state: &Self::State, access: &mut FilteredAccess) {
        access.add_component_write(*state);
    }

    fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool {
        set.contains(state)
    }
}

/// Write fetch implementation
pub struct WriteFetch<T> {
    _marker: PhantomData<T>,
}

/// Impl WorldQuery for Option<T> (optional component)
impl<T: WorldQuery> WorldQuery for Option<T> {
    type Item<'w> = Option<T::Item<'w>>;
    type Fetch<'w> = OptionFetch<T::Fetch<'w>>;
    type State = T::State;
    type ReadOnly = Option<T::ReadOnly>;

    fn init_state(world: &crate::world::World) -> Self::State {
        T::init_state(world)
    }

    fn get_access(state: &Self::State) -> Access {
        T::get_access(state)
    }

    fn update_component_access(state: &Self::State, access: &mut FilteredAccess) {
        T::update_component_access(state, access);
    }

    fn matches_component_set(_state: &Self::State, _set: &[ComponentId]) -> bool {
        true // Option always matches
    }
}

impl<T: ReadOnlyWorldQuery> ReadOnlyWorldQuery for Option<T> {}

/// Option fetch implementation
pub struct OptionFetch<F> {
    _marker: PhantomData<F>,
}

// Tuple implementations for WorldQuery
macro_rules! impl_world_query_tuple {
    ($($name:ident),*) => {
        #[allow(non_snake_case)]
        #[allow(clippy::unused_unit)]
        impl<$($name: WorldQuery),*> WorldQuery for ($($name,)*) {
            type Item<'w> = ($($name::Item<'w>,)*);
            type Fetch<'w> = ($($name::Fetch<'w>,)*);
            type State = ($($name::State,)*);
            type ReadOnly = ($($name::ReadOnly,)*);

            fn init_state(world: &crate::world::World) -> Self::State {
                ($($name::init_state(world),)*)
            }

            fn get_access(state: &Self::State) -> Access {
                #[allow(unused_mut)]
                let mut access = Access::new();
                let ($($name,)*) = state;
                $(
                    access.extend(&$name::get_access($name));
                )*
                access
            }

            fn update_component_access(state: &Self::State, access: &mut FilteredAccess) {
                let ($($name,)*) = state;
                $(
                    $name::update_component_access($name, access);
                )*
            }

            fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool {
                let ($($name,)*) = state;
                $(
                    $name::matches_component_set($name, set) &&
                )* true
            }
        }

        #[allow(non_snake_case)]
        #[allow(clippy::unused_unit)]
        impl<$($name: ReadOnlyWorldQuery),*> ReadOnlyWorldQuery for ($($name,)*) {}
    };
}

// Implement for tuples up to 15 elements
impl_world_query_tuple!();
impl_world_query_tuple!(A);
impl_world_query_tuple!(A, B);
impl_world_query_tuple!(A, B, C);
impl_world_query_tuple!(A, B, C, D);
impl_world_query_tuple!(A, B, C, D, E);
impl_world_query_tuple!(A, B, C, D, E, F);
impl_world_query_tuple!(A, B, C, D, E, F, G);
impl_world_query_tuple!(A, B, C, D, E, F, G, H);
impl_world_query_tuple!(A, B, C, D, E, F, G, H, I);
impl_world_query_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_world_query_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_world_query_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_world_query_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_world_query_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_world_query_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);

/// QueryData is an alias for WorldQuery
pub trait QueryData: WorldQuery {}

impl<T: WorldQuery> QueryData for T {}

/// ReadOnlyQueryData is an alias for ReadOnlyWorldQuery
pub trait ReadOnlyQueryData: ReadOnlyWorldQuery {}

impl<T: ReadOnlyWorldQuery> ReadOnlyQueryData for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct Position {
        x: f32,
        y: f32,
    }
    impl Component for Position {}

    #[derive(Debug, Clone, Copy)]
    struct Velocity {
        x: f32,
        y: f32,
    }
    impl Component for Velocity {}

    #[test]
    fn test_world_query_entity() {
        // Entity implements WorldQuery
        fn assert_world_query<T: WorldQuery>() {}
        assert_world_query::<Entity>();
    }

    #[test]
    fn test_world_query_ref() {
        // &T implements WorldQuery
        fn assert_world_query<T: WorldQuery>() {}
        assert_world_query::<&Position>();
    }

    #[test]
    fn test_world_query_mut() {
        // &mut T implements WorldQuery
        fn assert_world_query<T: WorldQuery>() {}
        assert_world_query::<&mut Position>();
    }

    #[test]
    fn test_world_query_tuple() {
        // Tuples implement WorldQuery
        fn assert_world_query<T: WorldQuery>() {}
        assert_world_query::<(&Position, &Velocity)>();
        assert_world_query::<(&Position, &mut Velocity)>();
    }

    #[test]
    fn test_read_only_world_query() {
        // Read-only queries
        fn assert_readonly<T: ReadOnlyWorldQuery>() {}
        assert_readonly::<Entity>();
        assert_readonly::<&Position>();
        assert_readonly::<(&Position, &Velocity)>();
    }
}