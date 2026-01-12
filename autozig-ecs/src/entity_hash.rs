//! Entity hashing utilities

use crate::entity::Entity;
use std::collections::{HashMap, HashSet};
use ahash::RandomState;

/// Hash implementation for Entity
#[derive(Clone, Copy, Debug)]
pub struct EntityHash;

impl EntityHash {
    pub fn hash(entity: Entity) -> u64 {
        entity.to_bits() as u64
    }
}

/// Hash set specialized for entities
pub type EntityHashSet = HashSet<Entity, RandomState>;

/// Hash map specialized for entities
pub type EntityHashMap<V> = HashMap<Entity, V, RandomState>;