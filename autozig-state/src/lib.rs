pub mod state;
pub mod transition;
pub mod plugin;
pub mod condition;
pub mod scoped;

pub mod prelude {
    pub use crate::state::{States, State, NextState, StateRegistry};
    pub use crate::transition::{OnEnter, OnExit, OnTransition, StateTransitionEvent, apply_state_transition};
    pub use crate::plugin::{StatePlugin, AppStateExt};
    pub use crate::condition::{in_state, state_changed, on_enter, on_exit};
    pub use crate::scoped::{DespawnOnExit, DespawnOnEnter, StateScoped};
    pub use autozig_ecs::prelude::*;
}
