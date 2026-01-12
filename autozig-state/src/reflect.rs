//! Reflection support for state types

use crate::state::States;
use crate::freely_mutable_state::FreelyMutableState;

/// A struct used to operate on the reflected [`States`] trait of a type.
///
/// A [`ReflectState`] for type `T` can be obtained via type registration data.
#[derive(Clone)]
pub struct ReflectState(pub ReflectStateFns);

/// The raw function pointers needed to make up a [`ReflectState`].
#[derive(Clone)]
pub struct ReflectStateFns {
    /// Function pointer implementing [`ReflectState::get()`].
    pub get: fn() -> Option<Box<dyn std::any::Any>>,
}

impl ReflectState {
    /// Gets the value of this [`States`] type as a trait object.
    pub fn get(&self) -> Option<Box<dyn std::any::Any>> {
        (self.0.get)()
    }
}

/// A struct used to operate on the reflected [`FreelyMutableState`] trait of a type.
///
/// A [`ReflectFreelyMutableState`] for type `T` can be obtained via type registration data.
#[derive(Clone)]
pub struct ReflectFreelyMutableState(pub ReflectFreelyMutableStateFns);

/// The raw function pointers needed to make up a [`ReflectFreelyMutableState`].
#[derive(Clone)]
pub struct ReflectFreelyMutableStateFns {
    /// Function pointer implementing [`ReflectFreelyMutableState::set_next_state()`].
    pub set_next_state: fn(state: Box<dyn std::any::Any>),
    /// Function pointer implementing [`ReflectFreelyMutableState::set_next_state_if_neq()`].
    pub set_next_state_if_neq: fn(state: Box<dyn std::any::Any>),
}

impl ReflectFreelyMutableState {
    /// Tentatively set a pending state transition to a reflected [`FreelyMutableState`].
    pub fn set_next_state(&self, state: Box<dyn std::any::Any>) {
        (self.0.set_next_state)(state);
    }
    
    /// Tentatively set a pending state transition to a reflected [`FreelyMutableState`],
    /// skipping state transitions if the target state is the same as the current state.
    pub fn set_next_state_if_neq(&self, state: Box<dyn std::any::Any>) {
        (self.0.set_next_state_if_neq)(state);
    }
}