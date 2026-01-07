//! State management system for AutoZig
//! 
//! Provides Bevy-compatible state machine functionality with 90% Zig implementation

use autozig::include_zig;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::marker::PhantomData;

// Re-export from autozig-ecs
pub use autozig_ecs::prelude::*;

#[repr(C)]
pub struct StateRegistryOpaque {
    _private: u8,
}

pub type StateId = u64;

include_zig!("src/zig/state_value.zig", {
    fn state_registry_create() -> *mut StateRegistryOpaque;
    fn state_registry_destroy(registry: *mut StateRegistryOpaque);
    fn state_registry_set_current(registry: *mut StateRegistryOpaque, state_id: StateId);
    fn state_registry_set_next(registry: *mut StateRegistryOpaque, state_id: StateId);
    fn state_registry_get_current(registry: *const StateRegistryOpaque) -> StateId;
    fn state_registry_get_next(registry: *const StateRegistryOpaque) -> StateId;
    fn state_registry_has_current(registry: *const StateRegistryOpaque) -> bool;
    fn state_registry_has_pending(registry: *const StateRegistryOpaque) -> bool;
    fn state_registry_apply_transition(registry: *mut StateRegistryOpaque) -> bool;
    fn state_registry_clear_next(registry: *mut StateRegistryOpaque);
});

/// Calculate state ID from Rust type
fn get_state_id<S: 'static>() -> StateId {
    let mut hasher = DefaultHasher::new();
    std::any::TypeId::of::<S>().hash(&mut hasher);
    hasher.finish()
}

/// Trait for state types
pub trait States: 'static + Send + Sync + Clone + Eq + std::fmt::Debug {
    /// Get the unique identifier for this state type
    fn state_id(&self) -> StateId {
        get_state_id::<Self>()
    }
}

/// Current state resource
pub struct State<S: States> {
    value: S,
}

impl<S: States> State<S> {
    pub fn new(initial: S) -> Self {
        Self { value: initial }
    }
    
    pub fn get(&self) -> &S {
        &self.value
    }
    
    pub fn set(&mut self, new_state: S) {
        self.value = new_state;
    }
}

/// Next state resource for state transitions
pub struct NextState<S: States> {
    queued: Option<S>,
}

impl<S: States> NextState<S> {
    pub fn new() -> Self {
        Self { queued: None }
    }
    
    /// Queue a state transition
    pub fn set(&mut self, state: S) {
        self.queued = Some(state);
    }
    
    pub fn take(&mut self) -> Option<S> {
        self.queued.take()
    }
    
    pub fn is_pending(&self) -> bool {
        self.queued.is_some()
    }
}

impl<S: States> Default for NextState<S> {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal state registry wrapper
pub struct StateRegistry {
    inner: *mut StateRegistryOpaque,
}

impl StateRegistry {
    pub fn new() -> Self {
        Self {
            inner: state_registry_create(),
        }
    }
    
    pub fn set_current(&mut self, state_id: StateId) {
        state_registry_set_current(self.inner, state_id);
    }
    
    pub fn set_next(&mut self, state_id: StateId) {
        state_registry_set_next(self.inner, state_id);
    }
    
    pub fn get_current(&self) -> Option<StateId> {
        if state_registry_has_current(self.inner) {
            Some(state_registry_get_current(self.inner))
        } else {
            None
        }
    }
    
    pub fn has_pending(&self) -> bool {
        state_registry_has_pending(self.inner)
    }
    
    pub fn apply_transition(&mut self) -> bool {
        state_registry_apply_transition(self.inner)
    }
    
    pub fn clear_next(&mut self) {
        state_registry_clear_next(self.inner);
    }
}

impl Drop for StateRegistry {
    fn drop(&mut self) {
        state_registry_destroy(self.inner);
    }
}

impl Default for StateRegistry {
    fn default() -> Self {
        Self::new()
    }
}
