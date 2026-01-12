//! Freely mutable states - states that can be changed directly via NextState

use crate::state::States;

/// This trait allows a state to be mutated directly using the [`NextState<S>`](crate::state::NextState) resource.
///
/// While ordinary states are freely mutable (and implement this trait as part of their derive macro),
/// computed states are not: instead, they can *only* change when the states that drive them do.
pub trait FreelyMutableState: States {}