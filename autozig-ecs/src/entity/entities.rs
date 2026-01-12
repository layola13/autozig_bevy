//! Entities collection - manages all entity metadata

use super::{Entity, EntityGeneration, EntityIndex};
use std::collections::HashMap;

/// Collection that stores metadata for all entities in the world
pub struct Entities {
    meta: HashMap<u32, EntityMeta>,
    len: usize,
    pending: Vec<u32>, // Pending entity spawns
}

/// Metadata for a single entity
#[derive(Debug, Clone, Copy)]
struct EntityMeta {
    generation: u32,
    location: EntityLocation,
    spawn_tick: u32,
    spawned_by: Option<u32>, // System that spawned this entity
}

/// Location of an entity in the archetype/table storage
#[derive(Debug, Clone, Copy, Default)]
pub struct EntityLocation {
    pub archetype_id: u32,
    pub archetype_row: usize,
    pub table_id: u32,
    pub table_row: usize,
}

impl Entities {
    /// Creates a new empty Entities collection
    pub fn new() -> Self {
        Self {
            meta: HashMap::new(),
            len: 0,
            pending: Vec::new(),
        }
    }

    /// Returns the number of entities
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks if there are no entities
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Checks if an entity exists
    pub fn contains(&self, entity: Entity) -> bool {
        self.meta.get(&entity.index)
            .map(|meta| meta.generation == entity.generation)
            .unwrap_or(false)
    }
    
    /// Checks if any entities are spawned
    pub fn any_spawned(&self) -> bool {
        !self.is_empty()
    }
    
    /// Checks if a specific entity is spawned
    pub fn contains_spawned(&self, entity: Entity) -> bool {
        self.contains(entity)
    }
    
    /// Returns the count of spawned entities
    pub fn count_spawned(&self) -> usize {
        self.len
    }
    
    /// Checks if an entity index is currently spawned
    pub fn is_index_spawned(&self, index: u32) -> bool {
        self.meta.contains_key(&index)
    }
    
    /// Gets a spawned entity by index, returning None if not spawned
    pub fn get_spawned(&self, index: u32) -> Option<Entity> {
        self.meta.get(&index).map(|meta| Entity {
            index,
            generation: meta.generation,
        })
    }
    
    /// Resolves an entity from just its index (uses current generation)
    pub fn resolve_from_index(&self, index: u32) -> Option<Entity> {
        self.get_spawned(index)
    }
    
    /// Checks if an entity can be spawned at the given position
    pub fn check_can_spawn_at(&self, entity: Entity) -> Result<(), super::SpawnError> {
        if let Some(meta) = self.meta.get(&entity.index) {
            if meta.generation == entity.generation {
                return Err(super::SpawnError::AlreadySpawned(entity));
            }
        }
        Ok(())
    }
    
    /// Gets the spawn tick for an entity
    pub fn entity_get_spawn_or_despawn_tick(&self, entity: Entity) -> Option<u32> {
        self.meta.get(&entity.index)
            .filter(|meta| meta.generation == entity.generation)
            .map(|meta| meta.spawn_tick)
    }
    
    /// Gets the system that spawned or despawned an entity
    pub fn entity_get_spawned_or_despawned_by(&self, entity: Entity) -> Option<Option<u32>> {
        self.meta.get(&entity.index)
            .filter(|meta| meta.generation == entity.generation)
            .map(|meta| meta.spawned_by)
    }

    /// Gets the location of an entity
    pub fn get(&self, entity: Entity) -> Option<EntityLocation> {
        self.meta.get(&entity.index)
            .filter(|meta| meta.generation == entity.generation)
            .map(|meta| meta.location)
    }

    /// Clears all entities
    pub fn clear(&mut self) {
        self.meta.clear();
        self.len = 0;
        self.pending.clear();
    }

    /// Reserves capacity for at least `additional` more entities
    pub fn reserve(&mut self, additional: usize) {
        self.meta.reserve(additional);
    }
    
    /// Spawns a new entity with the given metadata
    pub(crate) fn spawn(&mut self, entity: Entity, location: EntityLocation, spawn_tick: u32, spawned_by: Option<u32>) {
        self.meta.insert(entity.index, EntityMeta {
            generation: entity.generation,
            location,
            spawn_tick,
            spawned_by,
        });
        self.len += 1;
    }
    
    /// Despawns an entity
    pub(crate) fn despawn(&mut self, entity: Entity) -> bool {
        if let Some(meta) = self.meta.get(&entity.index) {
            if meta.generation == entity.generation {
                self.meta.remove(&entity.index);
                self.len -= 1;
                return true;
            }
        }
        false
    }
}

impl Default for Entities {
    fn default() -> Self {
        Self::new()
    }
}