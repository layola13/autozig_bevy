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
    fn component_ids(&self) -> Vec<std::any::TypeId>;
    
    /// Get the components as type-erased pointers
    /// Returns (type_id, data_ptr, data_size) tuples
    fn get_components(&self) -> Vec<(std::any::TypeId, *const u8, usize)>;
    
    /// Clone this dynamic bundle
    fn clone_dynamic(&self) -> Box<dyn DynamicBundle>;
}

/// Implementation of DynamicBundle for static bundles
impl<T: Bundle + Clone> DynamicBundle for T {
    fn component_ids(&self) -> Vec<std::any::TypeId> {
        T::component_ids()
    }
    
    fn get_components(&self) -> Vec<(std::any::TypeId, *const u8, usize)> {
        Bundle::get_components(self)
    }
    
    fn clone_dynamic(&self) -> Box<dyn DynamicBundle> {
        Box::new(self.clone())
    }
}