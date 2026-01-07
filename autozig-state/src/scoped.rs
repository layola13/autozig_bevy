//! State-scoped entity management
//! 
//! Entities marked with DespawnOnExit will be automatically despawned
//! when exiting the specified state

use crate::state::States;
use std::marker::PhantomData;

/// Marker component for entities that should be despawned when exiting a state
pub struct DespawnOnExit<S: States> {
    pub state: S,
    _phantom: PhantomData<S>,
}

impl<S: States> DespawnOnExit<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            _phantom: PhantomData,
        }
    }
}

/// Marker component for entities that should be despawned when entering a state
pub struct DespawnOnEnter<S: States> {
    pub state: S,
    _phantom: PhantomData<S>,
}

impl<S: States> DespawnOnEnter<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            _phantom: PhantomData,
        }
    }
}

/// Helper trait to spawn state-scoped entities
pub trait StateScoped<S: States> {
    /// Spawn an entity that will be despawned when exiting the current state
    fn spawn_scoped(&mut self, state: S) -> &mut Self;
}

// Note: This would be implemented on EntityCommands when integrating with autozig-ecs
// For now, we provide the trait definition
