//! Bundle spawning operations

use crate::bundle::BundleInfo;
use crate::component::ComponentId;
use crate::entity::Entity;
use crate::storage::Table;

/// Helper for spawning entities with bundles efficiently
#[derive(Debug)]
pub struct BundleSpawner<'w> {
    bundle_info: &'w BundleInfo,
    /// Storage tables for components
    tables: Option<&'w mut Vec<Table>>,
}

impl<'w> BundleSpawner<'w> {
    /// Create a new bundle spawner
    pub fn new(bundle_info: &'w BundleInfo) -> Self {
        Self {
            bundle_info,
            tables: None,
        }
    }

    /// Create with access to storage tables
    pub fn with_tables(bundle_info: &'w BundleInfo, tables: &'w mut Vec<Table>) -> Self {
        Self {
            bundle_info,
            tables: Some(tables),
        }
    }

    /// Reserve storage space for a number of entities
    /// 
    /// This pre-allocates space in the component storage to efficiently spawn
    /// multiple entities without repeated reallocations.
    pub fn reserve_storage(&mut self, additional: usize) {
        if let Some(tables) = &mut self.tables {
            // Reserve space in each table for the components in this bundle
            for &component_id in self.bundle_info.contributed_components() {
                let table_index = component_id.index();
                
                // Ensure we have enough tables
                while tables.len() <= table_index {
                    tables.push(Table::new());
                }
                
                // Reserve space in the appropriate table
                if let Some(table) = tables.get_mut(table_index) {
                    table.reserve(additional);
                }
            }
        }
    }

    /// Spawn an entity at a specific location
    /// 
    /// This allows spawning an entity with pre-allocated storage at a specific
    /// table row, which is useful for batch spawning operations.
    pub fn spawn_at(&mut self, entity: Entity, table_row: usize) -> SpawnResult<'w> {
        SpawnResult {
            entity,
            table_row,
            bundle_info: self.bundle_info,
        }
    }

    /// Get the bundle info
    pub fn bundle_info(&self) -> &BundleInfo {
        self.bundle_info
    }

    /// Spawn a single entity without specific location
    pub fn spawn(&mut self, entity: Entity) -> SpawnResult<'w> {
        // In a full implementation, this would allocate a new table row
        self.spawn_at(entity, 0)
    }

    /// Batch spawn multiple entities
    pub fn spawn_batch(&mut self, entities: &[Entity]) -> Vec<SpawnResult<'w>> {
        self.reserve_storage(entities.len());
        
        entities
            .iter()
            .enumerate()
            .map(|(row, &entity)| self.spawn_at(entity, row))
            .collect()
    }
}

/// Result of spawning an entity with a bundle
#[derive(Debug, Clone)]
pub struct SpawnResult<'a> {
    /// The spawned entity
    pub entity: Entity,
    /// The table row where the entity was placed
    pub table_row: usize,
    /// Information about the bundle that was spawned
    pub bundle_info: &'a BundleInfo,
}

impl<'a> SpawnResult<'a> {
    /// Get the entity that was spawned
    pub fn entity(&self) -> Entity {
        self.entity
    }

    /// Get the table row
    pub fn table_row(&self) -> usize {
        self.table_row
    }

    /// Get the bundle info
    pub fn bundle_info(&self) -> &BundleInfo {
        self.bundle_info
    }

    /// Get all component IDs in the spawned bundle
    pub fn component_ids(&self) -> &[ComponentId] {
        self.bundle_info.contributed_components()
    }
}

/// Batch spawning helper
#[derive(Debug)]
pub struct BatchSpawner<'w> {
    spawners: Vec<BundleSpawner<'w>>,
}

impl<'w> BatchSpawner<'w> {
    /// Create a new batch spawner
    pub fn new() -> Self {
        Self {
            spawners: Vec::new(),
        }
    }

    /// Add a bundle spawner to the batch
    pub fn add_spawner(&mut self, spawner: BundleSpawner<'w>) {
        self.spawners.push(spawner);
    }

    /// Reserve storage for all spawners
    pub fn reserve_storage(&mut self, additional: usize) {
        for spawner in &mut self.spawners {
            spawner.reserve_storage(additional);
        }
    }

    /// Spawn entities in batch across all spawners
    pub fn spawn_batch(&mut self, entities_per_spawner: &[&[Entity]]) -> Vec<Vec<SpawnResult<'_>>> {
        self.spawners
            .iter_mut()
            .zip(entities_per_spawner.iter())
            .map(|(spawner, entities)| spawner.spawn_batch(entities))
            .collect()
    }
}

impl<'w> Default for BatchSpawner<'w> {
    fn default() -> Self {
        Self::new()
    }
}