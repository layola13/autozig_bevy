//! Table - Column-based storage for components

use autozig_macro::include_zig;
use crate::entity::Entity;
use crate::component::ComponentId;
use std::ptr::NonNull;

include_zig!("src/zig/table.zig", {
    fn table_create() -> *mut u8;
});

/// Unique identifier for a table
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct TableId(pub u32);

impl TableId {
    pub const EMPTY: Self = Self(0);
    pub const INVALID: Self = Self(u32::MAX);
    
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    
    pub fn index(&self) -> usize {
        self.0 as usize
    }
}

/// Row index within a table
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct TableRow(pub usize);

impl TableRow {
    pub fn new(row: usize) -> Self {
        Self(row)
    }
    
    pub fn index(&self) -> usize {
        self.0
    }
}

/// A table storing entities and their components in columns
#[repr(C)]
pub struct Table {
    id: TableId,
    columns: Vec<Column>,
    entities: Vec<Entity>,
}

impl Table {
    pub fn new(id: TableId) -> Self {
        Self {
            id,
            columns: Vec::new(),
            entities: Vec::new(),
        }
    }
    
    pub fn id(&self) -> TableId {
        self.id
    }
    
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
    
    pub fn entities(&self) -> &[Entity] {
        &self.entities
    }
    
    pub fn add_column(&mut self, component_id: ComponentId) {
        self.columns.push(Column::new(component_id));
    }
    
    pub fn get_column(&self, component_id: ComponentId) -> Option<&Column> {
        self.columns.iter().find(|c| c.component_id == component_id)
    }
    
    pub fn get_column_mut(&mut self, component_id: ComponentId) -> Option<&mut Column> {
        self.columns.iter_mut().find(|c| c.component_id == component_id)
    }
    
    pub fn allocate(&mut self, entity: Entity) -> TableRow {
        let row = TableRow::new(self.entities.len());
        self.entities.push(entity);
        for column in &mut self.columns {
            column.data.push(0); // Placeholder
        }
        row
    }
    
    pub fn swap_remove(&mut self, row: TableRow) -> TableMoveResult {
        let last_row = TableRow::new(self.entities.len() - 1);
        let swapped_entity = if row.0 < last_row.0 {
            Some(self.entities.swap_remove(row.0))
        } else {
            self.entities.pop();
            None
        };
        
        for column in &mut self.columns {
            if row.0 < column.data.len() - 1 {
                column.data.swap_remove(row.0);
            } else {
                column.data.pop();
            }
        }
        
        TableMoveResult { swapped_entity }
    }
}

/// A column storing components of a single type
#[repr(C)]
pub struct Column {
    component_id: ComponentId,
    data: Vec<u8>,
    item_size: usize,
}

impl Column {
    pub fn new(component_id: ComponentId) -> Self {
        Self {
            component_id,
            data: Vec::new(),
            item_size: 0,
        }
    }
    
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }
    
    pub fn len(&self) -> usize {
        if self.item_size == 0 {
            0
        } else {
            self.data.len() / self.item_size
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Builder for constructing tables
pub struct TableBuilder {
    columns: Vec<ComponentId>,
}

impl TableBuilder {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }
    
    pub fn add_column(mut self, component_id: ComponentId) -> Self {
        self.columns.push(component_id);
        self
    }
    
    pub fn build(self, id: TableId) -> Table {
        let mut table = Table::new(id);
        for component_id in self.columns {
            table.add_column(component_id);
        }
        table
    }
}

/// Container for all tables
pub struct Tables {
    tables: Vec<Table>,
}

impl Tables {
    pub fn new() -> Self {
        let mut tables = Vec::new();
        // Add empty table
        tables.push(Table::new(TableId::EMPTY));
        
        Self { tables }
    }
    
    pub fn get(&self, id: TableId) -> Option<&Table> {
        self.tables.get(id.index())
    }
    
    pub fn get_mut(&mut self, id: TableId) -> Option<&mut Table> {
        self.tables.get_mut(id.index())
    }
    
    pub fn len(&self) -> usize {
        self.tables.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &Table> {
        self.tables.iter()
    }
}

/// Result of moving an entity from a table
pub struct TableMoveResult {
    pub swapped_entity: Option<Entity>,
}