//! EntityRef - immutable entity access

// Re-export EntityRef from entity module
pub use crate::entity::EntityRef;

// Additional entity access types
pub struct EntityRefExcept<'w> {
    entity: crate::entity::Entity,
    world: &'w crate::world::World,
}

impl<'w> EntityRefExcept<'w> {
    pub fn new(entity: crate::entity::Entity, world: &'w crate::world::World) -> Self {
        Self { entity, world }
    }
    
    pub fn entity(&self) -> crate::entity::Entity {
        self.entity
    }
}

pub struct FilteredEntityRef<'w> {
    entity: crate::entity::Entity,
    world: &'w crate::world::World,
}

impl<'w> FilteredEntityRef<'w> {
    pub fn new(entity: crate::entity::Entity, world: &'w crate::world::World) -> Self {
        Self { entity, world }
    }
    
    pub fn entity(&self) -> crate::entity::Entity {
        self.entity
    }
}