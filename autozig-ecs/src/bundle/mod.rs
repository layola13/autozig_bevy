//! Bundle module - organizing all bundle-related functionality

// Sub-modules
pub mod info;
pub mod remove;
pub mod spawner;

// Re-export core bundle types from info module
pub use info::{
    Bundle, BundleId, BundleInfo, Bundles, BundleInserter,
    InsertMode,
};

// Re-export with aliases for convenience
pub use info::{
    BundleId as Id, BundleInfo as Info, Bundles as Registry,
};

pub use remove::{
    empty_pre_remove, BundleRemover, PreRemoveHook, PreRemoveHooks,
    RemoveBundle,
};

pub use spawner::{
    BundleSpawner, SpawnResult, BatchSpawner,
};

/// DynamicBundle trait - allows runtime construction of bundles
/// 
/// This trait extends the static Bundle trait to support dynamic bundle creation
/// where component types may not be known at compile time.
pub trait DynamicBundle: Send + Sync + 'static {
    /// Get the component IDs in this bundle
    fn component_ids(&self) -> Vec<crate::component::ComponentId>;
    
    /// Get the components as type-erased pointers
    /// Returns (component_id, data_ptr, data_size) tuples
    fn get_components(&self) -> Vec<(crate::component::ComponentId, *const u8, usize)>;
    
    /// Clone this dynamic bundle
    fn clone_dynamic(&self) -> Box<dyn DynamicBundle>;
}

/// Implementation of DynamicBundle for static bundles
impl<T: Bundle + Clone> DynamicBundle for T {
    fn component_ids(&self) -> Vec<crate::component::ComponentId> {
        // Convert u32 IDs to ComponentId
        T::component_ids()
            .into_iter()
            .map(|id| crate::component::ComponentId(id as usize))
            .collect()
    }
    
    fn get_components(&self) -> Vec<(crate::component::ComponentId, *const u8, usize)> {
        // Convert component data
        Bundle::get_components(self)
            .into_iter()
            .map(|(id, ptr, size)| (crate::component::ComponentId(id as usize), ptr, size))
            .collect()
    }
    
    fn clone_dynamic(&self) -> Box<dyn DynamicBundle> {
        Box::new(self.clone())
    }
}