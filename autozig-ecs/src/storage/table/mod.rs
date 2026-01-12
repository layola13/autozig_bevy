//! Table storage for components

pub mod column;

use crate::component::ComponentId;
use crate::entity::Entity;
use std::collections::HashMap;

pub use column::Column;

/// TableId - 表ID
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TableId(u32);

impl TableId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    
    pub fn as_u32(&self) -> u32 {
        self.0
    }
    
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
    
    pub fn from_u32(id: u32) -> Self {
        Self(id)
    }
    
    pub fn from_usize(id: usize) -> Self {
        Self(id as u32)
    }
    
    pub const EMPTY: Self = Self(0);
}

/// Table - 组件表，用于存储相同原型的实体
#[derive(Debug)]
pub struct Table {
    columns: HashMap<ComponentId, Column>,
    entities: Vec<Entity>,
}

impl Table {
    /// Creates a new empty Table
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            entities: Vec::new(),
        }
    }
    
    /// Reserves capacity for at least `additional` more entities
    pub fn reserve(&mut self, additional: usize) {
        self.entities.reserve(additional);
        for column in self.columns.values_mut() {
            column.reserve(additional);
        }
    }
    
    pub fn component_count(&self) -> usize {
        self.columns.len()
    }
    
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
    
    pub fn entity_capacity(&self) -> usize {
        self.entities.capacity()
    }
    
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
    
    pub fn has_column(&self, component_id: ComponentId) -> bool {
        self.columns.contains_key(&component_id)
    }
    
    pub fn get_column(&self, component_id: ComponentId) -> Option<&Column> {
        self.columns.get(&component_id)
    }
    
    pub fn get_column_mut(&mut self, component_id: ComponentId) -> Option<&mut Column> {
        self.columns.get_mut(&component_id)
    }
    
    pub fn iter_columns(&self) -> impl Iterator<Item = (&ComponentId, &Column)> {
        self.columns.iter()
    }
    
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
    
    // TODO: 实现剩余的table API (~20个)
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

/// Tables - 管理所有表
#[derive(Default)]
pub struct Tables {
    tables: Vec<Table>,
}

impl Tables {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn len(&self) -> usize {
        self.tables.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }
    
    pub fn clear(&mut self) {
        self.tables.clear();
    }
    
    pub fn get(&self, id: TableId) -> Option<&Table> {
        self.tables.get(id.as_usize())
    }
    
    pub fn get_mut(&mut self, id: TableId) -> Option<&mut Table> {
        self.tables.get_mut(id.as_usize())
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &Table> {
        self.tables.iter()
    }
}