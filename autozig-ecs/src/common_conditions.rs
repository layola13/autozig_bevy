//! Common conditions for system execution
//! 
//! This module provides standard Bevy-like run conditions.

use crate::prelude::*;

/// Returns `true` if the resource of type `T` exists.
pub fn resource_exists<T: Resource>(world: &World) -> bool {
    world.contains_resource::<T>()
}

/// Returns `true` if there are any entities with the given component.
pub fn any_with_component<T: Component>(mut query: Query<(), With<T>>) -> bool {
    !query.is_empty()
}

/// Returns `true` if the event of type `E` has any events.
pub fn has_event<E: Event>(events: Res<Events<E>>) -> bool {
    !events.is_empty()
}

/// Returns `true` if the current state is `T`.
pub fn state_equals<S: crate::state::States>(state: Res<crate::state::State<S>>, value: S) -> bool {
    *state.get() == value
}

/// Returns `true` if the current state is NOT `T`.
pub fn not_in_state<S: crate::state::States>(state: S) -> impl Condition {
    use crate::resource::Res;
    crate::condition::IntoCondition::<crate::into_system::FunctionMarker<(bool, Res<crate::state::State<S>>)> >::into_condition(move |current_state: Res<crate::state::State<S>>| {
        *current_state.get() != state
    })
}

/// Returns `true` if the current state is `T`.
pub fn in_state<S: crate::state::States>(state: S) -> impl Condition {
    crate::state::in_state(state)
}
