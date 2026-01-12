//! Table storage for components
//!
//! Wraps Zig Table implementation.

pub mod column;

use crate::component::ComponentId;
use crate::entity::Entity;
use autozig_macro::include_zig;

pub use column::Column;

/// Opaque pointer to Zig Table
#[repr(C)]
pub struct TableOpaque {
    _private: [u8; 0],
}

include_zig!("src/zig/table.zig", {
    fn table_create() -> *mut TableOpaque;
    fn table_destroy(table: *mut TableOpaque);
    fn table_push_row(table: *mut TableOpaque, entity: u32) -> usize;
    fn table_entity_count(table: *const TableOpaque) -> usize;
    fn table_get_entity_row(table: *const TableOpaque, entity: u32) -> i64;
    fn table_get_entity(table: *const TableOpaque, row: usize) -> Entity;
    fn table_clear(table: *mut TableOpaque);
});

/// TableId - 表ID
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TableId(u32);

impl TableId {
    pub fn new(id: u32) -> Self { Self(id) }
    pub fn as_u32(&self) -> u32 { self.0 }
    pub fn as_usize(&self) -> usize { self.0 as usize }
    pub fn from_u32(id: u32) -> Self { Self(id) }
    pub fn from_usize(id: usize) -> Self { Self(id as u32) }
    pub const EMPTY: Self = Self(0);
    pub const INVALID: Self = Self(u32::MAX);
}

/// Table - Wrapper around Zig Table
#[derive(Debug)]
pub struct Table {
    pub(crate) inner: *mut TableOpaque,
}

impl Table {
    /// Creates a new empty Table (Allocated in Zig)
    pub fn new() -> Self {
        Self {
            inner: table_create(),
        }
    }
    
    pub fn entity_count(&self) -> usize {
        table_entity_count(self.inner)
    }

    pub fn reserve(&mut self, _additional: usize) {}
    
    pub fn is_empty(&self) -> bool {
        self.entity_count() == 0
    }
    
    pub fn get_entity(&self, row: usize) -> Entity {
        table_get_entity(self.inner, row)
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

/// Tables - Wrapper for Zig Tables collection
/// Since access is via World FFI, this might not be needed or just a helper.
#[derive(Default)]
pub struct Tables {
    tables: Vec<Table>,
}

impl Tables {
    pub fn new() -> Self { Self { tables: Vec::new() } }
    pub fn clear(&mut self) {}
    pub fn len(&self) -> usize { self.tables.len() }
    pub fn is_empty(&self) -> bool { self.tables.is_empty() }
    pub fn iter(&self) -> impl Iterator<Item = &Table> { self.tables.iter() }
}