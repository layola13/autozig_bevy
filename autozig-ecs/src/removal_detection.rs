//! Removal detection - Track removed components

use crate::entity::Entity;
use crate::component::ComponentId;
use std::collections::VecDeque;

/// Events for tracking removed components
#[derive(Default)]
pub struct RemovedComponentEvents {
    events: Vec<RemovedComponentEntity>,
}

impl RemovedComponentEvents {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn send(&mut self, entity: Entity, component_id: ComponentId) {
        self.events.push(RemovedComponentEntity {
            entity,
            component_id,
        });
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &RemovedComponentEntity> {
        self.events.iter()
    }
    
    pub fn clear(&mut self) {
        self.events.clear();
    }
}

/// Event for a removed component
#[derive(Clone, Copy, Debug)]
pub struct RemovedComponentEntity {
    pub entity: Entity,
    pub component_id: ComponentId,
}

/// System parameter for reading removed components
pub struct RemovedComponents<'w, T> {
    reader: RemovedComponentReader<'w>,
    _marker: std::marker::PhantomData<T>,
}

impl<'w, T: 'static> RemovedComponents<'w, T> {
    pub fn new(world: &'w crate::world::World) -> Self {
        let type_id = std::any::TypeId::of::<T>();
        let events = if let Some(events_any) = world.removed_components.get(&type_id) {
            if let Some(events) = events_any.downcast_ref::<RemovedComponentEvents>() {
                &events.events[..]
            } else {
                &[]
            }
        } else {
            &[]
        };
        
        Self {
            reader: RemovedComponentReader { events },
            _marker: std::marker::PhantomData,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.reader.iter().map(|e| e.entity)
    }
}

/// Reader for removed component events
pub struct RemovedComponentReader<'w> {
    events: &'w [RemovedComponentEntity],
}

impl<'w> RemovedComponentReader<'w> {
    pub fn new(events: &'w [RemovedComponentEntity]) -> Self {
        Self { events }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &RemovedComponentEntity> {
        self.events.iter()
    }
}