//! Run conditions for state-based system scheduling

use crate::state::{States, State};

/// Run condition that returns true if the state matches
pub fn in_state<S: States>(target: S) -> impl Fn(&State<S>) -> bool {
    move |state: &State<S>| state.get() == &target
}

/// Run condition that returns true if the state has changed
pub fn state_changed<S: States>() -> impl FnMut(Option<&State<S>>) -> bool {
    let mut last_state: Option<S> = None;
    move |state: Option<&State<S>>| {
        if let Some(current) = state {
            let current_value = current.get().clone();
            let changed = last_state.as_ref() != Some(&current_value);
            last_state = Some(current_value);
            changed
        } else {
            false
        }
    }
}

/// Run condition that returns true if entering the specified state
pub fn on_enter<S: States>(target: S) -> impl FnMut(Option<&State<S>>) -> bool {
    let mut last_state: Option<S> = None;
    move |state: Option<&State<S>>| {
        if let Some(current) = state {
            let current_value = current.get().clone();
            let entering = last_state.as_ref() != Some(&current_value) 
                && current_value == target;
            last_state = Some(current_value);
            entering
        } else {
            false
        }
    }
}

/// Run condition that returns true if exiting the specified state
pub fn on_exit<S: States>(target: S) -> impl FnMut(Option<&State<S>>) -> bool {
    let mut last_state: Option<S> = None;
    move |state: Option<&State<S>>| {
        if let Some(current) = state {
            let current_value = current.get().clone();
            let exiting = last_state.as_ref() == Some(&target) 
                && current_value != target;
            last_state = Some(current_value);
            exiting
        } else {
            false
        }
    }
}
