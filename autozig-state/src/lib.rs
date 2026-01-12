pub mod state;
pub mod transition;
pub mod plugin;
pub mod condition;
pub mod scoped;

// New modules for complete API
pub mod computed_states;
pub mod sub_states;
pub mod freely_mutable_state;
pub mod state_set;
pub mod app;
pub mod commands;
pub mod reflect;
pub mod state_scoped_events;

pub mod prelude {
    // Core state types
    pub use crate::state::{States, State, NextState, PreviousState, StateRegistry};
    
    // Transition types
    pub use crate::transition::{
        OnEnter, OnExit, OnTransition, StateTransitionEvent, apply_state_transition,
        StateTransition, StateTransitionSystems,
        EnterSchedules, ExitSchedules, TransitionSchedules,
    };
    
    // Plugin and app extension
    pub use crate::plugin::{StatePlugin, AppStateExt};
    pub use crate::app::{AppExtStates, StatesPlugin};
    
    // Conditions
    pub use crate::condition::{in_state, state_changed, on_enter, on_exit};
    
    // Scoped entities
    pub use crate::scoped::{DespawnOnExit, DespawnOnEnter, StateScoped};
    
    // Advanced state types
    pub use crate::computed_states::ComputedStates;
    pub use crate::sub_states::SubStates;
    pub use crate::freely_mutable_state::FreelyMutableState;
    pub use crate::state_set::{StateSet, StateSetSealed};
    
    // Commands extension
    pub use crate::commands::CommandsStatesExt;
    
    // Reflection support
    pub use crate::reflect::{ReflectState, ReflectStateFns, ReflectFreelyMutableState, ReflectFreelyMutableStateFns};
    
    // State-scoped events
    pub use crate::state_scoped_events::StateScopedMessagesAppExt;
    
    // Re-export from autozig-ecs
    pub use autozig_ecs::prelude::*;
}
