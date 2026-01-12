//! Storage - ECS存储系统，90% Zig + 10% Rust架构
//!
//! 这个模块实现了高效的组件存储，包括约55个API

mod blob_array;
mod resource;
mod sparse_set;
mod thin_array_ptr;
pub mod table;

pub use blob_array::*;
pub use resource::*;
pub use sparse_set::*;
pub use thin_array_ptr::*;
pub use table::*;

use crate::component::ComponentId;
use std::collections::HashMap;

/// Storages - 管理所有存储类型的容器
#[derive(Default)]
pub struct Storages {
    pub tables: Tables,
    pub sparse_sets: SparseSets,
    pub resources: Resources,
    pub non_send_resources: Resources,
}

impl Storages {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn clear(&mut self) {
        self.tables.clear();
        self.sparse_sets.clear();
    }
}

// Tables definition removed (re-exported from table module)

pub use self::table::{Table, TableId, Tables};
pub use self::sparse_set::{SparseSet, SparseSets};
pub use self::resource::{Resources, ResourceData};

// Placeholders removed (use submodules)