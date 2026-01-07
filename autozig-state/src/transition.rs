//! State transition system - OnEnter/OnExit/OnTransition schedules

use crate::state::{States, State, NextState};
use std::marker::PhantomData;

/// Schedule set that runs when entering a state
pub struct OnEnter<S: States>(pub S);

/// Schedule set that runs when exiting a state  
pub struct OnExit<S: States>(pub S);

/// Schedule set that runs on any state transition
pub struct OnTransition<S: States> {
    pub from: Option<S>,
    pub to: Option<S>,
}

/// State transition event
#[derive(Debug, Clone)]
pub struct StateTransitionEvent<S: States> {
    pub exited: Option<S>,
    pub entered: Option<S>,
}

impl<S: States> StateTransitionEvent<S> {
    pub fn new(exited: Option<S>, entered: Option<S>) -> Self {
        Self { exited, entered }
    }
}

/// Apply pending state transitions
pub fn apply_state_transition<S: States>(
    mut current_state: Option<&mut State<S>>,
    next_state: &mut NextState<S>,
) -> Option<StateTransitionEvent<S>> {
    if let Some(new_state) = next_state.take() {
        let old_state = current_state.as_ref().map(|s| s.get().clone());
        
        if let Some(current) = current_state.as_mut() {
            current.set(new_state.clone());
        }
        
        Some(StateTransitionEvent::new(old_state, Some(new_state)))
    } else {
        None
    }
}
