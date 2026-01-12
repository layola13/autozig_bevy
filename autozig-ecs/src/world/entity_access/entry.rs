//! Component entry types for entity access

use crate::component::Component;
use crate::entity::Entity;

use crate::world::World;

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
    world: &'a mut World,
    entity: Entity,
    _marker: std::marker::PhantomData<T>,
}

impl<'a, T: Component> VacantComponentEntry<'a, T> {
    pub fn new(world: &'a mut World, entity: Entity) -> Self {
        Self {
            world,
            entity,
            _marker: std::marker::PhantomData,
        }
    }
    
    pub fn insert(self, component: T) -> &'a mut T {
        self.world.entity_mut(self.entity).insert(component);
        // Unwrap is safe because we just inserted it
        self.world.get_mut::<T>(self.entity)
            .expect("Component should exist after insertion")
            .into_inner()
    }
}

// Legacy entry types for compatibility
pub use ComponentEntry as Entry;
pub use OccupiedComponentEntry as OccupiedEntry;
pub use VacantComponentEntry as VacantEntry;