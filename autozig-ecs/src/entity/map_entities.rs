//! Entity mapping support for scenes and prefabs

use super::Entity;
use std::collections::HashMap;

/// EntityMapper - Maps entities from one world/scene to another
pub trait EntityMapper {
    /// Maps an entity to its corresponding entity in the target world
    fn map_entity(&mut self, entity: Entity) -> Entity;
    
    /// Maps multiple entities
    fn map_entities(&mut self, entities: &[Entity]) -> Vec<Entity> {
        entities.iter().map(|&e| self.map_entity(e)).collect()
    }
}

/// SceneEntityMapper - Entity mapper specifically for scene loading
pub trait SceneEntityMapper: EntityMapper {
    /// Maps an entity from a scene to the world
    fn map_scene_entity(&mut self, scene_entity: Entity) -> Entity {
        self.map_entity(scene_entity)
    }
    
    /// Checks if a scene entity has been mapped
    fn contains_scene_entity(&self, scene_entity: Entity) -> bool;
    
    /// Gets the mapped entity if it exists
    fn get_mapped(&self, scene_entity: Entity) -> Option<Entity>;
}

/// SimpleEntityMapper - Basic implementation of EntityMapper using a HashMap
pub struct SimpleEntityMapper {
    map: HashMap<Entity, Entity>,
}

impl SimpleEntityMapper {
    /// Creates a new SimpleEntityMapper
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Adds a mapping
    pub fn insert(&mut self, from: Entity, to: Entity) {
        self.map.insert(from, to);
    }

    /// Removes a mapping
    pub fn remove(&mut self, from: Entity) -> Option<Entity> {
        self.map.remove(&from)
    }

    /// Clears all mappings
    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Returns the number of mappings
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns true if there are no mappings
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Default for SimpleEntityMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityMapper for SimpleEntityMapper {
    fn map_entity(&mut self, entity: Entity) -> Entity {
        *self.map.get(&entity).unwrap_or(&entity)
    }
}

impl SceneEntityMapper for SimpleEntityMapper {
    fn contains_scene_entity(&self, scene_entity: Entity) -> bool {
        self.map.contains_key(&scene_entity)
    }

    fn get_mapped(&self, scene_entity: Entity) -> Option<Entity> {
        self.map.get(&scene_entity).copied()
    }
}

/// EntityMapperBuilder - Builder for creating entity mappers with predefined mappings
pub struct EntityMapperBuilder {
    mappings: Vec<(Entity, Entity)>,
}

impl EntityMapperBuilder {
    /// Creates a new EntityMapperBuilder
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    /// Adds a mapping
    pub fn add(mut self, from: Entity, to: Entity) -> Self {
        self.mappings.push((from, to));
        self
    }

    /// Builds a SimpleEntityMapper
    pub fn build(self) -> SimpleEntityMapper {
        let mut mapper = SimpleEntityMapper::new();
        for (from, to) in self.mappings {
            mapper.insert(from, to);
        }
        mapper
    }
}

impl Default for EntityMapperBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// MapEntities - Trait for types that contain entities that need to be mapped
pub trait MapEntities {
    /// Maps all entities in this type using the provided mapper
    fn map_entities(&mut self, mapper: &mut dyn EntityMapper);
}

impl MapEntities for Entity {
    fn map_entities(&mut self, mapper: &mut dyn EntityMapper) {
        *self = mapper.map_entity(*self);
    }
}

impl MapEntities for Vec<Entity> {
    fn map_entities(&mut self, mapper: &mut dyn EntityMapper) {
        for entity in self.iter_mut() {
            *entity = mapper.map_entity(*entity);
        }
    }
}

impl<const N: usize> MapEntities for [Entity; N] {
    fn map_entities(&mut self, mapper: &mut dyn EntityMapper) {
        for entity in self.iter_mut() {
            *entity = mapper.map_entity(*entity);
        }
    }
}

impl MapEntities for Option<Entity> {
    fn map_entities(&mut self, mapper: &mut dyn EntityMapper) {
        if let Some(entity) = self {
            *entity = mapper.map_entity(*entity);
        }
    }
}