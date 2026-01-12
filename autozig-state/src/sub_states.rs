//! Sub-states - states that exist only when source states meet certain conditions

use crate::state::States;
use crate::state_set::StateSet;
use crate::freely_mutable_state::FreelyMutableState;

/// A sub-state is a state that exists only when the source state meets certain conditions.
///
/// Unlike [`ComputedStates`](crate::state::ComputedStates), while they exist they can be manually modified.
pub trait SubStates: States + FreelyMutableState {
    /// The set of states from which the [`Self`] is derived.
    ///
    /// This can either be a single type that implements [`States`], or a tuple
    /// containing multiple types that implement [`States`].
    type SourceStates: StateSet;

    /// This function gets called whenever one of the [`SourceStates`](Self::SourceStates) changes.
    /// The result is used to determine the existence of [`State<Self>`](crate::state::State).
    ///
    /// If the result is [`None`], the [`State<Self>`](crate::state::State) resource will be removed from the world.
    /// Otherwise if the [`State<Self>`](crate::state::State) resource doesn't exist,
    /// it will be created from the returned [`Some`] as the initial state.
    ///
    /// Value within [`Some`] is ignored if the state already exists in the world
    /// and only symbolizes that the state should still exist.
    fn should_exist(sources: Self::SourceStates) -> Option<Self>;
}