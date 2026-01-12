//! State set - trait for single states or tuples of states

use crate::state::States;

mod sealed {
    /// Sealed trait used to prevent external implementations of [`StateSet`](super::StateSet).
    pub trait StateSetSealed {}
}

pub use sealed::StateSetSealed;

/// A [`States`] type or tuple of types which implement [`States`].
///
/// This trait is used to allow implementors of [`States`], as well
/// as tuples containing exclusively implementors of [`States`], to
/// be used as [`ComputedStates::SourceStates`](crate::state::ComputedStates::SourceStates).
///
/// It is sealed, and auto implemented for all [`States`] types and
/// tuples containing them.
pub trait StateSet: StateSetSealed {
    /// The total [`DEPENDENCY_DEPTH`](States::DEPENDENCY_DEPTH) of all
    /// the states that are part of this [`StateSet`], added together.
    ///
    /// Used to de-duplicate computed state executions and prevent cyclic
    /// computed states.
    const SET_DEPENDENCY_DEPTH: usize;
}

// Implement for single state
impl<S: States> StateSetSealed for S {}

impl<S: States> StateSet for S {
    const SET_DEPENDENCY_DEPTH: usize = S::DEPENDENCY_DEPTH;
}

// Implement for Option<S>
impl<S: States> StateSetSealed for Option<S> {}

impl<S: States> StateSet for Option<S> {
    const SET_DEPENDENCY_DEPTH: usize = S::DEPENDENCY_DEPTH;
}

// Implement for tuples of states (up to 4 elements for simplicity)
impl<S1: States, S2: States> StateSetSealed for (S1, S2) {}

impl<S1: States, S2: States> StateSet for (S1, S2) {
    const SET_DEPENDENCY_DEPTH: usize = S1::DEPENDENCY_DEPTH + S2::DEPENDENCY_DEPTH;
}

impl<S1: States, S2: States, S3: States> StateSetSealed for (S1, S2, S3) {}

impl<S1: States, S2: States, S3: States> StateSet for (S1, S2, S3) {
    const SET_DEPENDENCY_DEPTH: usize = S1::DEPENDENCY_DEPTH + S2::DEPENDENCY_DEPTH + S3::DEPENDENCY_DEPTH;
}

impl<S1: States, S2: States, S3: States, S4: States> StateSetSealed for (S1, S2, S3, S4) {}

impl<S1: States, S2: States, S3: States, S4: States> StateSet for (S1, S2, S3, S4) {
    const SET_DEPENDENCY_DEPTH: usize = 
        S1::DEPENDENCY_DEPTH + S2::DEPENDENCY_DEPTH + S3::DEPENDENCY_DEPTH + S4::DEPENDENCY_DEPTH;
}

// Support for Option variants in tuples
impl<S1: States, S2: States> StateSetSealed for (Option<S1>, Option<S2>) {}

impl<S1: States, S2: States> StateSet for (Option<S1>, Option<S2>) {
    const SET_DEPENDENCY_DEPTH: usize = S1::DEPENDENCY_DEPTH + S2::DEPENDENCY_DEPTH;
}

impl<S1: States, S2: States> StateSetSealed for (S1, Option<S2>) {}

impl<S1: States, S2: States> StateSet for (S1, Option<S2>) {
    const SET_DEPENDENCY_DEPTH: usize = S1::DEPENDENCY_DEPTH + S2::DEPENDENCY_DEPTH;
}

impl<S1: States, S2: States> StateSetSealed for (Option<S1>, S2) {}

impl<S1: States, S2: States> StateSet for (Option<S1>, S2) {
    const SET_DEPENDENCY_DEPTH: usize = S1::DEPENDENCY_DEPTH + S2::DEPENDENCY_DEPTH;
}