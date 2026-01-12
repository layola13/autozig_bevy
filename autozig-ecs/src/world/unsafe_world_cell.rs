//! UnsafeWorldCell - unsafe but flexible World access
//!
//! This module provides UnsafeWorldCell, which allows multiple mutable references
//! to World data. It's the foundation for parallel query execution and advanced
//! ECS patterns. Safety is enforced by runtime checks and careful API design.

use crate::{
    archetype::Archetypes,
    bundle::Bundles,
    component::{Component, ComponentId, Components},
    entity::{Entities, Entity, EntityAllocator},
    storage::Storages,
    world::World,
    change_detection::Tick,
};
use std::marker::PhantomData;

/// Variant of the World where you can access everything immutably
///
/// # Safety
/// Must not be accessed while any mutable references to World data exist.
/// This includes EntityRef, EntityMut, Resources, etc.
#[derive(Clone, Copy)]
pub struct UnsafeWorldCell<'w> {
    world: *mut World,
    _marker: PhantomData<&'w World>,
}

impl<'w> UnsafeWorldCell<'w> {
    /// Creates a new UnsafeWorldCell with mutable access
    ///
    /// # Safety
    /// Caller must ensure no other references to World exist
    #[inline]
    pub(crate) fn new_mutable(world: &'w mut World) -> Self {
        Self {
            world: world as *mut World,
            _marker: PhantomData,
        }
    }

    /// Creates a new UnsafeWorldCell with readonly access
    ///
    /// # Safety
    /// Caller must ensure no mutable references to World exist
    #[inline]
    pub(crate) fn new_readonly(world: &'w World) -> Self {
        Self {
            world: world as *const World as *mut World,
            _marker: PhantomData,
        }
    }

    /// Gets a reference to the World (unsafe)
    ///
    /// # Safety
    /// Caller must ensure this doesn't violate Rust's aliasing rules
    #[inline]
    pub unsafe fn world(&self) -> &World {
        &*self.world
    }

    /// Gets a mutable reference to the World (unsafe)
    ///
    /// # Safety
    /// Caller must ensure this doesn't violate Rust's aliasing rules
    #[inline]
    pub unsafe fn world_mut(&self) -> &mut World {
        &mut *self.world
    }

    /// Gets a reference to the World's Entities collection
    ///
    /// # Safety
    /// Caller must ensure no mutable access to entities exists
    #[inline]
    pub unsafe fn entities(&self) -> &Entities {
        &(*self.world).entities
    }

    /// Gets a reference to the World's Archetypes collection
    ///
    /// # Safety
    /// Caller must ensure no mutable access to archetypes exists
    #[inline]
    pub unsafe fn archetypes(&self) -> &Archetypes {
        &(*self.world).archetypes
    }

    /// Gets a reference to the World's Components collection
    ///
    /// # Safety
    /// Caller must ensure no mutable access to components exists
    #[inline]
    pub unsafe fn components(&self) -> &Components {
        &(*self.world).components
    }

    /// Gets a reference to the World's Storages
    ///
    /// # Safety
    /// Caller must ensure no mutable access to storages exists
    #[inline]
    pub unsafe fn storages(&self) -> &Storages {
        &(*self.world).storages
    }

    /// Gets a reference to the World's Bundles collection
    ///
    /// # Safety
    /// Caller must ensure no mutable access to bundles exists
    #[inline]
    pub unsafe fn bundles(&self) -> &Bundles {
        &(*self.world).bundles
    }

    /// Gets a reference to the World's EntityAllocator
    ///
    /// # Safety
    /// Caller must ensure no mutable access to the allocator exists
    #[inline]
    pub unsafe fn entities_allocator(&self) -> &EntityAllocator {
        &(*self.world).allocator
    }

    /// Reads the current change tick
    ///
    /// # Safety
    /// This is safe because reading an atomic is always safe
    #[inline]
    pub unsafe fn read_change_tick(&self) -> Tick {
        (*self.world).read_change_tick()
    }

    /// Gets the last change tick
    ///
    /// # Safety
    /// This is safe because it's just reading a value
    #[inline]
    pub unsafe fn last_change_tick(&self) -> Tick {
        (*self.world).last_change_tick()
    }

    /// Checks if an entity exists
    ///
    /// # Safety
    /// Caller must ensure no concurrent modifications to entities
    #[inline]
    pub unsafe fn contains_entity(&self, entity: Entity) -> bool {
        (*self.world).entities.contains(entity)
    }

    /// Gets the component for an entity
    ///
    /// # Safety
    /// Caller must ensure:
    /// - Entity is valid
    /// - No mutable access to this component exists
    /// - Component type T matches the stored component
    #[inline]
    pub unsafe fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        (*self.world).get(entity)
    }

    /// Gets a raw pointer to the World
    ///
    /// # Safety
    /// Caller must ensure proper usage of the raw pointer
    #[inline]
    pub fn as_ptr(&self) -> *mut World {
        self.world
    }
}

// UnsafeWorldCell can be sent between threads
unsafe impl<'w> Send for UnsafeWorldCell<'w> {}

// UnsafeWorldCell can be shared between threads (with care)
unsafe impl<'w> Sync for UnsafeWorldCell<'w> {}

impl<'w> std::fmt::Debug for UnsafeWorldCell<'w> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnsafeWorldCell")
            .field("world", &self.world)
            .finish()
    }
}

/// Immutable World reference with runtime borrow checking
///
/// Similar to `&World` but with additional safety guarantees.
/// Prefer using this over `&World` when you need to pass world
/// references around with runtime checks.
pub struct WorldCell<'w> {
    world: UnsafeWorldCell<'w>,
}

impl<'w> WorldCell<'w> {
    /// Creates a new WorldCell from a World reference
    #[inline]
    pub fn new(world: &'w World) -> Self {
        Self {
            world: UnsafeWorldCell::new_readonly(world),
        }
    }

    /// Gets the underlying UnsafeWorldCell
    #[inline]
    pub fn as_unsafe_world_cell(&self) -> UnsafeWorldCell<'w> {
        self.world
    }

    /// Gets a reference to Entities
    #[inline]
    pub fn entities(&self) -> &Entities {
        unsafe { self.world.entities() }
    }

    /// Gets a reference to Archetypes
    #[inline]
    pub fn archetypes(&self) -> &Archetypes {
        unsafe { self.world.archetypes() }
    }

    /// Gets a reference to Components
    #[inline]
    pub fn components(&self) -> &Components {
        unsafe { self.world.components() }
    }

    /// Gets a reference to Storages
    #[inline]
    pub fn storages(&self) -> &Storages {
        unsafe { self.world.storages() }
    }

    /// Gets a reference to Bundles
    #[inline]
    pub fn bundles(&self) -> &Bundles {
        unsafe { self.world.bundles() }
    }

    /// Reads the current change tick
    #[inline]
    pub fn read_change_tick(&self) -> Tick {
        unsafe { self.world.read_change_tick() }
    }

    /// Gets the last change tick
    #[inline]
    pub fn last_change_tick(&self) -> Tick {
        unsafe { self.world.last_change_tick() }
    }

    /// Checks if an entity exists
    #[inline]
    pub fn contains_entity(&self, entity: Entity) -> bool {
        unsafe { self.world.contains_entity(entity) }
    }
}

impl<'w> std::fmt::Debug for WorldCell<'w> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorldCell")
            .field("world", &self.world)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsafe_world_cell_creation() {
        let mut world = World::new();
        let _cell = world.as_unsafe_world_cell();
        // Test passes if no panic occurs
    }

    #[test]
    fn world_cell_creation() {
        let world = World::new();
        let cell = WorldCell::new(&world);
        assert_eq!(cell.entities().len(), 0);
    }

    #[test]
    fn unsafe_world_cell_readonly() {
        let world = World::new();
        let cell = world.as_unsafe_world_cell_readonly();
        unsafe {
            assert_eq!(cell.entities().len(), 0);
        }
    }
}