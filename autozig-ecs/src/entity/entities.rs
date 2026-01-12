//! Entities collection - manages all entity metadata

use super::Entity;
use std::collections::HashMap;

/// Collection that stores metadata for all entities in the world
pub struct Entities {
    meta: HashMap<u32, EntityMeta>,
    len: usize,
}

/// Metadata for a single entity
#[derive(Debug, Clone, Copy)]
struct EntityMeta {
    generation: u32,
    location: EntityLocation,
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
    }

    /// Reserves capacity for at least `additional` more entities
    pub fn reserve(&mut self, additional: usize) {
        self.meta.reserve(additional);
    }
}

impl Default for Entities {
    fn default() -> Self {
        Self::new()
    }
}