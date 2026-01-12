//! Entity fetching utilities

use crate::entity::Entity;
use crate::world::World;

/// EntityFetcher - 实体获取器，用于从World中获取实体
pub struct EntityFetcher<'w> {
    world: &'w World,
}

impl<'w> EntityFetcher<'w> {
    pub fn new(world: &'w World) -> Self {
        Self { world }
    }
    
    /// 获取实体
    pub fn fetch(&self, entity: Entity) -> Option<crate::world::EntityRef<'w>> {
        if self.world.entities().contains(entity) {
            Some(crate::world::EntityRef::new(entity, self.world))
        } else {
            None
        }
    }
}

// Local EntityRef removed (use crate::world::EntityRef)

/// WorldEntityFetch - World实体获取器（用于从World中高效获取实体）
pub struct WorldEntityFetch<'w> {
    world: &'w World,
}

impl<'w> WorldEntityFetch<'w> {
    pub fn new(world: &'w World) -> Self {
        Self { world }
    }
    
    pub fn fetch(&self, entity: Entity) -> Option<crate::world::EntityRef<'w>> {
        if self.world.entities().contains(entity) {
            Some(crate::world::EntityRef::new(entity, self.world))
        } else {
            None
        }
    }
    
    pub fn world(&self) -> &World {
        self.world
    }
}