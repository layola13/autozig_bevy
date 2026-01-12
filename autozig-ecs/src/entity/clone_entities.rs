//! Entity cloning support - Clone entities with their components

use super::Entity;
use crate::entity::hash_set::EntityHashSet;
use std::marker::PhantomData;

/// EntityCloner - Clones entities and their components
pub struct EntityCloner {
    source: Entity,
    target: Entity,
    filter: CloneFilter,
}

impl EntityCloner {
    /// Creates a new EntityCloner
    pub fn new(source: Entity, target: Entity) -> Self {
        Self {
            source,
            target,
            filter: CloneFilter::None,
        }
    }

    /// Creates a new EntityCloner with a filter
    pub fn with_filter(source: Entity, target: Entity, filter: CloneFilter) -> Self {
        Self {
            source,
            target,
            filter,
        }
    }

    /// Gets the source entity
    pub fn source(&self) -> Entity {
        self.source
    }

    /// Gets the target entity
    pub fn target(&self) -> Entity {
        self.target
    }

    /// Gets the filter
    pub fn filter(&self) -> &CloneFilter {
        &self.filter
    }

    /// Sets the filter
    pub fn set_filter(&mut self, filter: CloneFilter) {
        self.filter = filter;
    }

    /// Clones the entity (requires world access, simplified here)
    pub fn clone_entity(&self) -> Entity {
        self.target
    }
}

/// CloneFilter - Filter for which components to clone
#[derive(Clone)]
pub enum CloneFilter {
    /// Clone all components
    None,
    /// Only clone allowed components (OptIn)
    Allow(EntityHashSet),
    /// Clone all except denied components (OptOut)
    Deny(EntityHashSet),
}

impl CloneFilter {
    /// Creates an empty Allow filter
    pub fn allow_empty() -> Self {
        Self::Allow(EntityHashSet::new())
    }

    /// Creates an empty Deny filter
    pub fn deny_empty() -> Self {
        Self::Deny(EntityHashSet::new())
    }

    /// Adds an entity to the allow list
    pub fn allow(mut self, entity: Entity) -> Self {
        match &mut self {
            Self::Allow(set) => {
                set.insert(entity);
            }
            _ => {}
        }
        self
    }

    /// Adds an entity to the deny list
    pub fn deny(mut self, entity: Entity) -> Self {
        match &mut self {
            Self::Deny(set) => {
                set.insert(entity);
            }
            _ => {}
        }
        self
    }

    /// Checks if an entity should be cloned
    pub fn should_clone(&self, entity: Entity) -> bool {
        match self {
            Self::None => true,
            Self::Allow(set) => set.contains(&entity),
            Self::Deny(set) => !set.contains(&entity),
        }
    }
}

/// EntityClonerFilter - Type alias for CloneFilter
pub type EntityClonerFilter = CloneFilter;

/// ComponentCloneCtx - Context for cloning components
pub struct ComponentCloneCtx<'w> {
    source: Entity,
    target: Entity,
    _marker: PhantomData<&'w ()>,
}

impl<'w> ComponentCloneCtx<'w> {
    /// Creates a new ComponentCloneCtx
    pub fn new(source: Entity, target: Entity) -> Self {
        Self {
            source,
            target,
            _marker: PhantomData,
        }
    }

    /// Gets the source entity
    pub fn source(&self) -> Entity {
        self.source
    }

    /// Gets the target entity
    pub fn target(&self) -> Entity {
        self.target
    }

    /// Reads a component from the source entity (simplified)
    pub fn read_source<T>(&self) -> Option<&T> {
        None
    }

    /// Writes a component to the target entity (simplified)
    pub fn write_target<T>(&mut self, _component: T) {
        // Simplified implementation
    }

    /// Clones a component from source to target
    pub fn clone_component<T: Clone>(&mut self) {
        // Simplified implementation
    }
}

/// EntityCloneBuilder - Builder for entity cloning
pub struct EntityCloneBuilder {
    filter: CloneFilter,
}

impl EntityCloneBuilder {
    /// Creates a new EntityCloneBuilder
    pub fn new() -> Self {
        Self {
            filter: CloneFilter::None,
        }
    }

    /// Sets the filter to OptIn
    pub fn opt_in(mut self) -> Self {
        self.filter = CloneFilter::allow_empty();
        self
    }

    /// Sets the filter to OptOut
    pub fn opt_out(mut self) -> Self {
        self.filter = CloneFilter::deny_empty();
        self
    }

    /// Allows a component type (for OptIn)
    pub fn allow<T: 'static>(self) -> Self {
        // Simplified: would use TypeId in real implementation
        self
    }

    /// Denies a component type (for OptOut)
    pub fn deny<T: 'static>(self) -> Self {
        // Simplified: would use TypeId in real implementation
        self
    }

    /// Builds the clone filter
    pub fn build(self) -> CloneFilter {
        self.filter
    }
}

impl Default for EntityCloneBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// ComponentCloneHandler - Handler for cloning specific component types
pub struct ComponentCloneHandler<T> {
    _marker: PhantomData<T>,
}

impl<T> ComponentCloneHandler<T> {
    /// Creates a new ComponentCloneHandler
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Clones a component
    pub fn clone(&self, source: &T) -> T
    where
        T: Clone,
    {
        source.clone()
    }
}

impl<T> Default for ComponentCloneHandler<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// EntityCloneRegistry - Registry for entity clone handlers
pub struct EntityCloneRegistry {
    handlers: Vec<Box<dyn Fn(Entity, Entity) + Send + Sync>>,
}

impl EntityCloneRegistry {
    /// Creates a new EntityCloneRegistry
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Registers a clone handler
    pub fn register<F>(&mut self, handler: F)
    where
        F: Fn(Entity, Entity) + Send + Sync + 'static,
    {
        self.handlers.push(Box::new(handler));
    }

    /// Clones an entity using registered handlers
    pub fn clone_entity(&self, source: Entity, target: Entity) {
        for handler in &self.handlers {
            handler(source, target);
        }
    }
}

impl Default for EntityCloneRegistry {
    fn default() -> Self {
        Self::new()
    }
}