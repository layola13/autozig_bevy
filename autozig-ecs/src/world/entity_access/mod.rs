// Entity access module - provides safe and unsafe entity access patterns
// Part of AutoZig Bevy ECS implementation

pub mod entity_mut;
pub mod entity_ref;
pub mod entry;
pub mod world_mut;

// Re-export all entity access types
pub use entity_mut::{EntityMut, EntityMutExcept, FilteredEntityMut, UnsafeFilteredEntityMut};
pub use entity_ref::{EntityRef, EntityRefExcept, FilteredEntityRef};
pub use entry::{ComponentEntry, OccupiedComponentEntry, VacantComponentEntry};
pub use world_mut::EntityWorldMut;