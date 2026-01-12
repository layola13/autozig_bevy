//! State transition system - OnEnter/OnExit/OnTransition schedules

use crate::state::{States, State, NextState};
use std::marker::PhantomData;

/// The label of a [`Schedule`] that **only** runs whenever [`State<S>`] enters the provided state.
///
/// This schedule ignores identity transitions.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct OnEnter<S: States>(pub S);

/// The label of a [`Schedule`] that **only** runs whenever [`State<S>`] exits the provided state.
///
/// This schedule ignores identity transitions.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct OnExit<S: States>(pub S);

/// The label of a [`Schedule`] that **only** runs whenever [`State<S>`]
/// exits AND enters the provided `exited` and `entered` states.
///
/// Systems added to this schedule are always ran *after* [`OnExit`], and *before* [`OnEnter`].
///
/// This schedule will run on identity transitions.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct OnTransition<S: States> {
    /// The state being exited.
    pub exited: S,
    /// The state being entered.
    pub entered: S,
}

/// Runs [state transitions](States).
///
/// This schedule is split up into four phases, as described in [`StateTransitionSystems`].
#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct StateTransition;

/// A [`Message`] sent when any state transition of `S` happens.
/// This includes identity transitions, where `exited` and `entered` have the same value.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StateTransitionEvent<S: States> {
    /// The state being exited.
    pub exited: Option<S>,
    /// The state being entered.
    pub entered: Option<S>,
}

impl<S: States> StateTransitionEvent<S> {
    pub fn new(exited: Option<S>, entered: Option<S>) -> Self {
        Self { exited, entered }
    }
}

/// Applies state transitions and runs transitions schedules in order.
///
/// These system sets are run sequentially, in the order of the enum variants.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StateTransitionSystems {
    /// States apply their transitions from [`NextState`]
    /// and compute functions based on their parent states.
    DependentTransitions,
    /// Exit schedules are executed in leaf to root order
    ExitSchedules,
    /// Transition schedules are executed in arbitrary order.
    TransitionSchedules,
    /// Enter schedules are executed in root to leaf order.
    EnterSchedules,
}

/// System set that runs exit schedule(s) for state `S`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExitSchedules<S: States>(PhantomData<S>);

impl<S: States> Default for ExitSchedules<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// System set that runs transition schedule(s) for state `S`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TransitionSchedules<S: States>(PhantomData<S>);

impl<S: States> Default for TransitionSchedules<S> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// System set that runs enter schedule(s) for state `S`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnterSchedules<S: States>(PhantomData<S>);

impl<S: States> Default for EnterSchedules<S> {
    fn default() -> Self {
        Self(PhantomData)
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
