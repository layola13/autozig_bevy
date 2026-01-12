//! Component module - organizing all component-related functionality

// Re-export core component types from info module
pub use info::{
    ComponentId, ComponentInfo, ComponentDescriptor, Components,
    QueuedComponents,
};

// Re-export from component_advanced module
pub use crate::component_advanced::{
    StorageType, ComponentTicks,
    ComponentsQueuedRegistrator, ComponentsRegistrator,
    ComponentHooks, TableStorage, SparseStorage,
};

// Re-export from entity_advanced module
pub use crate::entity_advanced::{
    ComponentCloneHandler, ComponentDropHandler,
    Mutable, Immutable,
};

// Re-export from query_advanced module
pub use crate::query_advanced::Access;

// Re-export from change_detection module
pub use crate::change_detection::Tick;

// Re-export Component trait (defined elsewhere, likely in entity or as a standalone trait)
// For now, define it here to satisfy imports
pub trait Component: Send + Sync + 'static {
    const STORAGE_TYPE: StorageType = StorageType::Table;
}

// Sub-modules
pub mod clone;
pub mod info;
pub mod register;
pub mod required;

// Re-export commonly used types
pub use clone::{
    ComponentCloneFn, ComponentCloneBehavior,
    component_clone_ignore, component_clone_via_clone,
    DefaultCloneBehaviorSpecialization,
    global_default_fn, resolve,
};

// Type aliases for convenience
pub use ComponentId as Id;
pub use ComponentDescriptor as Descriptor;
pub use ComponentInfo as Info;
pub use Components as Registry;

pub use register::{
    ComponentIds,
};

pub use required::{
    RequiredComponent, RequiredComponentConstructor,
    RequiredComponents, RequiredComponentsError,
    RequiredComponentsRegistrator,
};