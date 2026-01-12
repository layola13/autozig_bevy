//! Bundle removal operations

use crate::component::ComponentId;
use crate::entity::Entity;

/// Empty pre-remove hook that does nothing
/// 
/// This function serves as a placeholder for the pre-removal hook system.
/// In Bevy, hooks can be registered to run before components are removed from entities.
/// This empty implementation is used when no custom hook is needed.
#[inline]
pub fn empty_pre_remove(_entity: Entity, _component: ComponentId) {
    // Intentionally empty - this is a no-op hook
}

/// Trait for bundle removal operations
pub trait RemoveBundle {
    /// Remove this bundle from an entity
    fn remove_from(&self, entity: Entity);
}

/// Information needed to remove a bundle from an entity
#[derive(Debug, Clone)]
pub struct BundleRemover {
    /// Components to remove
    pub(crate) components: Vec<ComponentId>,
}

impl BundleRemover {
    /// Create a new bundle remover
    pub fn new(components: Vec<ComponentId>) -> Self {
        Self { components }
    }

    /// Get the components that will be removed
    pub fn components(&self) -> &[ComponentId] {
        &self.components
    }

    /// Execute the removal with a pre-remove hook
    pub fn remove_with_hook<F>(&self, entity: Entity, mut hook: F)
    where
        F: FnMut(Entity, ComponentId),
    {
        for &component in &self.components {
            hook(entity, component);
        }
    }

    /// Execute the removal without any hooks
    pub fn remove(&self, entity: Entity) {
        self.remove_with_hook(entity, empty_pre_remove);
    }
}

/// Type alias for pre-remove hooks
pub type PreRemoveHook = fn(Entity, ComponentId);

/// Registry for pre-remove hooks
#[derive(Debug, Default)]
pub struct PreRemoveHooks {
    hooks: Vec<(ComponentId, PreRemoveHook)>,
}

impl PreRemoveHooks {
    /// Create a new hooks registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a hook for a component
    pub fn register(&mut self, component: ComponentId, hook: PreRemoveHook) {
        self.hooks.push((component, hook));
    }

    /// Get all hooks for a component
    pub fn get_hooks(&self, component: ComponentId) -> impl Iterator<Item = &PreRemoveHook> {
        self.hooks
            .iter()
            .filter(move |(id, _)| *id == component)
            .map(|(_, hook)| hook)
    }

    /// Execute all hooks for a component
    pub fn execute(&self, entity: Entity, component: ComponentId) {
        for hook in self.get_hooks(component) {
            hook(entity, component);
        }
    }

    /// Clear all registered hooks
    pub fn clear(&mut self) {
        self.hooks.clear();
    }
}