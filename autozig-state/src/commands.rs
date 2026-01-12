//! Commands extension trait for state manipulation

use crate::freely_mutable_state::FreelyMutableState;

/// Extension trait for [`Commands`] adding `bevy_state` helpers.
///
/// Note: This is a trait definition. Actual implementation would require
/// integration with autozig_ecs::Commands.
pub trait CommandsStatesExt {
    /// Sets the next state the app should move to.
    ///
    /// Internally this schedules a command that updates the [`NextState<S>`](crate::state::NextState)
    /// resource with `state`.
    fn set_state<S: FreelyMutableState>(&mut self, state: S);

    /// Sets the next state the app should move to, skipping any state transitions
    /// if the next state is the same as the current state.
    ///
    /// Internally this schedules a command that updates the [`NextState<S>`](crate::state::NextState)
    /// resource with `state`.
    fn set_state_if_neq<S: FreelyMutableState>(&mut self, state: S);
}