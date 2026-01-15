//! State machine implementation

use autozig_macro::Resource;
use crate::resource::{Resource as ResourceTrait, Res, ResMut};
use crate::schedule::ScheduleLabel;
use crate::condition::Condition;
use std::borrow::Cow;
use std::fmt::Debug;
use std::hash::Hash;

/// States trait
pub trait States: 'static + Send + Sync + Clone + PartialEq + Eq + Hash + Debug + Default {}

impl<T> States for T where T: 'static + Send + Sync + Clone + PartialEq + Eq + Hash + Debug + Default {}

/// Current state resource
#[derive(Default, Debug)]
pub struct State<S: States>(pub S);

impl<S: States> State<S> {
    pub fn get(&self) -> &S {
        &self.0
    }
}

impl<S: States> crate::resource::Resource for State<S> {}

/// Next state resource
#[derive(Default, Debug)]
pub struct NextState<S: States>(pub Option<S>);

impl<S: States> NextState<S> {
    pub fn set(&mut self, state: S) {
        self.0 = Some(state);
    }
}

impl<S: States> crate::resource::Resource for NextState<S> {}

/// Schedule: Run when entering a state
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OnEnter<S: States>(pub S);

impl<S: States> ScheduleLabel for OnEnter<S> {
    fn label(&self) -> Cow<'static, str> {
         Cow::Owned(format!("OnEnter({:?})", self.0))
    }
}

/// Schedule: Run when exiting a state
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OnExit<S: States>(pub S);

impl<S: States> ScheduleLabel for OnExit<S> {
    fn label(&self) -> Cow<'static, str> {
         Cow::Owned(format!("OnExit({:?})", self.0))
    }
}

/// Condition: Check if in specific state
pub fn in_state<S: States>(state: S) -> impl crate::condition::Condition {
    // Closure automatically implements IntoCondition via SystemParamFunction logic
    // We need to return the Condition itself, not the closure.
    // The closure converted to system is the condition.
    use crate::condition::IntoCondition;
    use crate::resource::Res;
    crate::condition::IntoCondition::<crate::into_system::FunctionMarker<(bool, Res<State<S>>)> >::into_condition(move |current_state: Res<State<S>>| {
        *current_state.get() == state
    })
}
