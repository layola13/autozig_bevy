//! Component registration system

use crate::component::{Component, ComponentDescriptor, ComponentId, ComponentInfo, Components, QueuedComponents};
use crate::resource::Resource;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

impl Components {
    /// Register a component with a descriptor
    pub fn register_component_with_descriptor(
        &mut self,
        descriptor: ComponentDescriptor,
    ) -> ComponentId {
        let id = ComponentId(self.components.len());
        let mut info = ComponentInfo {
            id,
            name: descriptor.name,
            type_id: descriptor.type_id,
            layout: descriptor.layout,
            storage_type: descriptor.storage_type,
            is_send_and_sync: descriptor.is_send_and_sync,
            mutable: descriptor.mutable,
            clone_behavior: descriptor.clone_behavior,
            hooks: crate::component_advanced::ComponentHooks::new(),
        };
        info.id = id;

        if let Some(type_id) = descriptor.type_id {
            self.indices.insert(type_id, id);
        }

        self.components.push(Some(info));
        id
    }

    /// Register a component type
    pub fn register_component<T: Component>(&mut self) -> ComponentId {
        let type_id = TypeId::of::<T>();
        
        // Check if already registered
        if let Some(&id) = self.indices.get(&type_id) {
            return id;
        }

        let descriptor = ComponentDescriptor::new::<T>();
        self.register_component_with_descriptor(descriptor)
    }

    /// Register a non-Send component
    pub fn register_non_send<T: Component>(&mut self) -> ComponentId {
        let type_id = TypeId::of::<T>();
        
        if let Some(&id) = self.indices.get(&type_id) {
            return id;
        }

        let mut descriptor = ComponentDescriptor::new::<T>();
        descriptor.is_send_and_sync = false;
        self.register_component_with_descriptor(descriptor)
    }

    /// Register a resource with a descriptor
    pub fn register_resource_with_descriptor(
        &mut self,
        descriptor: ComponentDescriptor,
    ) -> ComponentId {
        let id = ComponentId(self.components.len());
        let mut info = ComponentInfo {
            id,
            name: descriptor.name,
            type_id: descriptor.type_id,
            layout: descriptor.layout,
            storage_type: descriptor.storage_type,
            is_send_and_sync: descriptor.is_send_and_sync,
            mutable: descriptor.mutable,
            clone_behavior: descriptor.clone_behavior,
            hooks: crate::component_advanced::ComponentHooks::new(),
        };
        info.id = id;

        if let Some(type_id) = descriptor.type_id {
            self.resource_indices.insert(type_id, id);
        }

        self.components.push(Some(info));
        id
    }

    /// Register a resource type
    pub fn register_resource<T: Resource>(&mut self) -> ComponentId {
        let type_id = TypeId::of::<T>();
        
        if let Some(&id) = self.resource_indices.get(&type_id) {
            return id;
        }

        let descriptor = ComponentDescriptor::new_resource::<T>();
        self.register_resource_with_descriptor(descriptor)
    }

    /// Queue a component registration with descriptor
    pub fn queue_register_component_with_descriptor(
        &self,
        descriptor: ComponentDescriptor,
    ) {
        let mut queued = self.queued.write().unwrap();
        if let Some(type_id) = descriptor.type_id {
            let id = ComponentId(0); // Will be assigned during apply
            let info = ComponentInfo {
                id,
                name: descriptor.name,
                type_id: Some(type_id),
                layout: descriptor.layout,
                storage_type: descriptor.storage_type,
                is_send_and_sync: descriptor.is_send_and_sync,
                mutable: descriptor.mutable,
                clone_behavior: descriptor.clone_behavior,
                hooks: crate::component_advanced::ComponentHooks::new(),
            };
            queued.components.insert(type_id, info);
        }
    }

    /// Queue a component registration
    pub fn queue_register_component<T: Component>(&self) {
        let descriptor = ComponentDescriptor::new::<T>();
        self.queue_register_component_with_descriptor(descriptor);
    }

    /// Queue a non-Send component registration
    pub fn queue_register_non_send<T: Component>(&self) {
        let mut descriptor = ComponentDescriptor::new::<T>();
        descriptor.is_send_and_sync = false;
        self.queue_register_component_with_descriptor(descriptor);
    }

    /// Queue a resource registration with descriptor
    pub fn queue_register_resource_with_descriptor(
        &self,
        descriptor: ComponentDescriptor,
    ) {
        let mut queued = self.queued.write().unwrap();
        if let Some(type_id) = descriptor.type_id {
            let id = ComponentId(0);
            let info = ComponentInfo {
                id,
                name: descriptor.name,
                type_id: Some(type_id),
                layout: descriptor.layout,
                storage_type: descriptor.storage_type,
                is_send_and_sync: descriptor.is_send_and_sync,
                mutable: descriptor.mutable,
                clone_behavior: descriptor.clone_behavior,
                hooks: crate::component_advanced::ComponentHooks::new(),
            };
            queued.resources.insert(type_id, info);
        }
    }

    /// Queue a resource registration
    pub fn queue_register_resource<T: Resource>(&self) {
        let descriptor = ComponentDescriptor::new_resource::<T>();
        self.queue_register_resource_with_descriptor(descriptor);
    }

    /// Apply all queued registrations
    pub fn apply_queued_registrations(&mut self) {
        let mut queued = self.queued.write().unwrap();

        // Register queued components
        for (type_id, mut info) in queued.components.drain() {
            let id = ComponentId(self.components.len());
            info.id = id;
            self.indices.insert(type_id, id);
            self.components.push(Some(info));
        }

        // Register queued resources
        for (type_id, mut info) in queued.resources.drain() {
            let id = ComponentId(self.components.len());
            info.id = id;
            self.resource_indices.insert(type_id, id);
            self.components.push(Some(info));
        }
    }

    /// Get queued components as read-only
    pub fn as_queued(&self) -> &Arc<RwLock<QueuedComponents>> {
        &self.queued
    }

    /// Peek at the next queued component without removing it
    pub fn peek(&self) -> Option<ComponentInfo> {
        let queued = self.queued.read().unwrap();
        queued.components.values().next().cloned()
            .or_else(|| queued.resources.values().next().cloned())
    }

    /// Peek at the next queued component (mutable version)
    pub fn peek_mut(&mut self) -> Option<ComponentInfo> {
        let queued = self.queued.write().unwrap();
        queued.components.values().next().cloned()
            .or_else(|| queued.resources.values().next().cloned())
    }

    /// Get and remove the next queued component
    pub fn next_mut(&mut self) -> Option<(TypeId, ComponentInfo)> {
        let mut queued = self.queued.write().unwrap();
        
        if let Some((&type_id, _)) = queued.components.iter().next() {
            let info = queued.components.remove(&type_id).unwrap();
            return Some((type_id, info));
        }
        
        if let Some((&type_id, _)) = queued.resources.iter().next() {
            let info = queued.resources.remove(&type_id).unwrap();
            return Some((type_id, info));
        }
        
        None
    }
}

/// A collection of component IDs.
#[derive(Debug, Default, Clone)]
pub struct ComponentIds {
    ids: Vec<ComponentId>,
}

impl ComponentIds {
    /// Create a new `ComponentIds`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a component ID.
    pub fn push(&mut self, id: ComponentId) {
        self.ids.push(id);
    }

    /// Get all component IDs.
    pub fn ids(&self) -> &[ComponentId] {
        &self.ids
    }

    /// Number of component IDs.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Iterate over component IDs.
    pub fn iter(&self) -> impl Iterator<Item = &ComponentId> {
        self.ids.iter()
    }
}

impl FromIterator<ComponentId> for ComponentIds {
    fn from_iter<T: IntoIterator<Item = ComponentId>>(iter: T) -> Self {
        Self {
            ids: iter.into_iter().collect(),
        }
    }
}