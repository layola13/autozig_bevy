//! Sparse set storage for components

use crate::entity::Entity;
use crate::component::ComponentId;
use std::collections::HashMap;

/// SparseSet - 稀疏集合存储
pub struct SparseSet {
    sparse: Vec<Option<usize>>,
    dense: Vec<Entity>,
    data: Vec<u8>, // TODO: 使用proper类型擦除
}

impl SparseSet {
    pub fn new() -> Self {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            data: Vec::new(),
        }
    }
    
    pub fn len(&self) -> usize {
        self.dense.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.dense.is_empty()
    }
    
    pub fn contains(&self, entity: Entity) -> bool {
        let index = entity.index() as usize;
        index < self.sparse.len() && self.sparse[index].is_some()
    }
    
    pub fn clear(&mut self) {
        self.sparse.clear();
        self.dense.clear();
        self.data.clear();
    }
    
    pub fn get(&self, entity: Entity) -> Option<usize> {
        let index = entity.index() as usize;
        if index < self.sparse.len() {
            self.sparse[index]
        } else {
            None
        }
    }
    
    pub fn indices(&self) -> &[Entity] {
        &self.dense
    }
    
    // TODO: 实现剩余的~3个API (get_or_insert_with, values系列)
}

impl Default for SparseSet {
    fn default() -> Self {
        Self::new()
    }
}

/// SparseSets - 管理所有稀疏集合
#[derive(Default)]
pub struct SparseSets {
    sets: HashMap<ComponentId, SparseSet>,
}

impl SparseSets {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn get(&self, component_id: ComponentId) -> Option<&SparseSet> {
        self.sets.get(&component_id)
    }
    
    pub fn get_mut(&mut self, component_id: ComponentId) -> Option<&mut SparseSet> {
        self.sets.get_mut(&component_id)
    }
    
    pub fn clear(&mut self) {
        self.sets.clear()
    }
    
    pub fn clear_entities(&mut self) {
        for set in self.sets.values_mut() {
            set.clear();
        }
    }
}