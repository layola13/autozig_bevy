//! EntityMut - mutable entity access
//!
//! This module provides mutable access to entities in the World.
//! It's a re-export/wrapper around the EntityWorldMut defined in entity module.

// Re-export EntityMut and EntityWorldMut from entity module
pub use crate::entity::{EntityMut, EntityWorldMut};

// Additional entity access types that may be defined later
pub struct EntityMutExcept<'w> {
    entity: crate::entity::Entity,
    world: &'w mut crate::world::World,
}

impl<'w> EntityMutExcept<'w> {
    pub fn new(entity: crate::entity::Entity, world: &'w mut crate::world::World) -> Self {
        Self { entity, world }
    }
    
    pub fn entity(&self) -> crate::entity::Entity {
        self.entity
    }
}

pub struct FilteredEntityMut<'w> {
    entity: crate::entity::Entity,
    world: &'w mut crate::world::World,
}

impl<'w> FilteredEntityMut<'w> {
    pub fn new(entity: crate::entity::Entity, world: &'w mut crate::world::World) -> Self {
        Self { entity, world }
    }
    
    pub fn entity(&self) -> crate::entity::Entity {
        self.entity
    }
}

pub struct UnsafeFilteredEntityMut<'w> {
    entity: crate::entity::Entity,
    world: &'w mut crate::world::World,
}

impl<'w> UnsafeFilteredEntityMut<'w> {
    pub fn new(entity: crate::entity::Entity, world: &'w mut crate::world::World) -> Self {
        Self { entity, world }
    }
    
    pub fn entity(&self) -> crate::entity::Entity {
        self.entity
    }
}