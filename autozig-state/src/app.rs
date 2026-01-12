//! App extension trait for state management

use crate::state::States;
use crate::freely_mutable_state::FreelyMutableState;
use crate::computed_states::ComputedStates;
use crate::sub_states::SubStates;

/// State installation methods for [`App`].
///
/// Note: This is a simplified version. In a full implementation with bevy_app integration,
/// this would contain the actual implementation methods.
pub trait AppExtStates {
    /// Initializes a [`State`](crate::state::State) with standard starting values.
    ///
    /// Adds [`State<S>`](crate::state::State) and [`NextState<S>`](crate::state::NextState) resources,
    /// and enables use of the [`OnEnter`](crate::state::OnEnter), [`OnTransition`](crate::state::OnTransition)
    /// and [`OnExit`](crate::state::OnExit) schedules.
    fn init_state<S: FreelyMutableState + Default>(&mut self) -> &mut Self;

    /// Inserts a specific [`State`](crate::state::State) to the current [`App`].
    ///
    /// Adds [`State<S>`](crate::state::State) and [`NextState<S>`](crate::state::NextState) resources.
    fn insert_state<S: FreelyMutableState>(&mut self, state: S) -> &mut Self;

    /// Sets up a type implementing [`ComputedStates`].
    ///
    /// This method is idempotent: it has no effect when called again using the same generic type.
    fn add_computed_state<S: ComputedStates>(&mut self) -> &mut Self;

    /// Sets up a type implementing [`SubStates`].
    ///
    /// This method is idempotent: it has no effect when called again using the same generic type.
    fn add_sub_state<S: SubStates>(&mut self) -> &mut Self;
}

/// Plugin that adds state management to an App.
///
/// This registers the [`StateTransition`](crate::state::StateTransition) schedule
/// to enable state processing.
pub struct StatesPlugin;

impl StatesPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StatesPlugin {
    fn default() -> Self {
        Self::new()
    }
}