//! State plugin for ECS integration

use crate::state::{States, State, NextState};
use crate::transition::{OnEnter, OnExit, StateTransitionEvent, apply_state_transition};
use autozig_ecs::prelude::*;

/// Plugin that adds state management to an App
pub struct StatePlugin<S: States> {
    pub initial_state: S,
}

impl<S: States> StatePlugin<S> {
    pub fn new(initial_state: S) -> Self {
        Self { initial_state }
    }
}

/// Extension trait for App to add state support
pub trait AppStateExt {
    /// Initialize a state with an initial value
    fn init_state<S: States>(&mut self, initial: S) -> &mut Self;
    
    /// Add a state without initial value
    fn add_state<S: States>(&mut self) -> &mut Self;
}

// Note: This would be implemented on autozig_ecs::App when we integrate with it
// For now, we provide the trait definition for future use
