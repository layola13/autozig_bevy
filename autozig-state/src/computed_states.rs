//! Computed states - states derived from other states

use crate::state::States;
use crate::state_set::StateSet;
use std::fmt::Debug;
use std::hash::Hash;

/// A state whose value is automatically computed based on the values of other [`States`].
///
/// A **computed state** is a state that is deterministically derived from a set of `SourceStates`.
pub trait ComputedStates: 'static + Send + Sync + Clone + PartialEq + Eq + Hash + Debug {
    /// The set of states from which the [`Self`] is derived.
    ///
    /// This can either be a single type that implements [`States`], an Option of a type
    /// that implements [`States`], or a tuple containing multiple types that implement [`States`].
    type SourceStates: StateSet;

    /// Whether state transition schedules should be run when the state changes to the same value.
    const ALLOW_SAME_STATE_TRANSITIONS: bool = true;

    /// Computes the next value of [`State<Self>`](crate::state::State).
    /// This function gets called whenever one of the [`SourceStates`](Self::SourceStates) changes.
    ///
    /// If the result is [`None`], the [`State<Self>`](crate::state::State) resource will be removed from the world.
    fn compute(sources: Self::SourceStates) -> Option<Self>;
}

impl<S: ComputedStates> States for S {
    const DEPENDENCY_DEPTH: usize = S::SourceStates::SET_DEPENDENCY_DEPTH + 1;
}