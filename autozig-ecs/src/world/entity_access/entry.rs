//! Component entry types for entity access

use crate::component::Component;
use crate::entity::Entity;

/// Entry API for component access
pub enum ComponentEntry<'a, T: Component> {
    Occupied(OccupiedComponentEntry<'a, T>),
    Vacant(VacantComponentEntry<'a, T>),
}

/// Occupied component entry
pub struct OccupiedComponentEntry<'a, T: Component> {
    entity: Entity,
    component: &'a mut T,
}

impl<'a, T: Component> OccupiedComponentEntry<'a, T> {
    pub fn new(entity: Entity, component: &'a mut T) -> Self {
        Self { entity, component }
    }
    
    pub fn get(&self) -> &T {
        self.component
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        self.component
    }
    
    pub fn into_mut(self) -> &'a mut T {
        self.component
    }
}

/// Vacant component entry
pub struct VacantComponentEntry<'a, T: Component> {
    entity: Entity,
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T: Component> VacantComponentEntry<'a, T> {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            _marker: std::marker::PhantomData,
        }
    }
    
    pub fn insert(self, component: T) -> &'a mut T {
        // This is a placeholder implementation
        // In reality, would insert into world and return mutable reference
        unimplemented!("VacantComponentEntry::insert requires full World integration")
    }
}

// Legacy entry types for compatibility
pub use ComponentEntry as Entry;
pub use OccupiedComponentEntry as OccupiedEntry;
pub use VacantComponentEntry as VacantEntry;