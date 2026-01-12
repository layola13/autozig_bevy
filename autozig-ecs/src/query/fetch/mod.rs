//! Fetch module for query data retrieval
//! 查询数据获取模块
//!
//! Architecture: 90% Zig + 10% Rust
//! Core fetch logic implemented in Zig for performance

use crate::{
    component::{Component, ComponentId},
    entity::Entity,
};
use std::marker::PhantomData;

/// Zig core integration
#[repr(C)]
pub struct FetchCoreOpaque {
    _private: [u8; 0],
}

use autozig_macro::include_zig;

include_zig!("src/query/fetch/zig/fetch.zig", {
    fn fetch_create() -> *mut FetchCoreOpaque;
    fn fetch_destroy(fetch: *mut FetchCoreOpaque);
    fn fetch_next(fetch: *mut FetchCoreOpaque, entity_out: *mut Entity) -> bool;
});

/// Entity fetch - fetches entity IDs
pub struct EntityFetch {
    inner: *mut FetchCoreOpaque,
}

impl EntityFetch {
    pub fn new() -> Self {
        Self {
            inner: fetch_create(),
        }
    }
    
    pub fn fetch(&self, entity: Entity) -> Entity {
        // Placeholder delegation
        entity
    }
}

impl Drop for EntityFetch {
    fn drop(&mut self) {
        fetch_destroy(self.inner);
    }
}

unsafe impl Send for EntityFetch {}
unsafe impl Sync for EntityFetch {}

impl Default for EntityFetch {
    fn default() -> Self {
        Self::new()
    }
}

/// Read fetch - fetches immutable component data
pub struct ReadFetch<T: Component> {
    component_id: ComponentId,
    _phantom: PhantomData<T>,
}

impl<T: Component> ReadFetch<T> {
    pub fn new(component_id: ComponentId) -> Self {
        Self {
            component_id,
            _phantom: PhantomData,
        }
    }
    
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }
}

/// Write fetch - fetches mutable component data
pub struct WriteFetch<T: Component> {
    component_id: ComponentId,
    _phantom: PhantomData<T>,
}

impl<T: Component> WriteFetch<T> {
    pub fn new(component_id: ComponentId) -> Self {
        Self {
            component_id,
            _phantom: PhantomData,
        }
    }
    
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }
}

/// Option fetch - fetches optional component data
pub struct OptionFetch<F> {
    inner: F,
}

impl<F> OptionFetch<F> {
    pub fn new(inner: F) -> Self {
        Self { inner }
    }
    
    pub fn inner(&self) -> &F {
        &self.inner
    }
}

/// Changed fetch - fetches components that have changed
pub struct ChangedFetch<T: Component> {
    component_id: ComponentId,
    _phantom: PhantomData<T>,
}

impl<T: Component> ChangedFetch<T> {
    pub fn new(component_id: ComponentId) -> Self {
        Self {
            component_id,
            _phantom: PhantomData,
        }
    }
}

/// Added fetch - fetches newly added components
pub struct AddedFetch<T: Component> {
    component_id: ComponentId,
    _phantom: PhantomData<T>,
}

impl<T: Component> AddedFetch<T> {
    pub fn new(component_id: ComponentId) -> Self {
        Self {
            component_id,
            _phantom: PhantomData,
        }
    }
}

/// Spawn details fetch - fetches entity spawn information
pub struct SpawnDetailsFetch {
    _marker: PhantomData<()>,
}

impl SpawnDetailsFetch {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
    
    /// Get the tick when entity was spawned
    pub fn spawn_tick(&self, _entity: Entity) -> u32 {
        0 // Placeholder
    }
    
    /// Get the entity that spawned this entity
    pub fn spawned_by(&self, _entity: Entity) -> Option<Entity> {
        None // Placeholder
    }
}

impl Default for SpawnDetailsFetch {
    fn default() -> Self {
        Self::new()
    }
}

/// Ref fetch - fetch with change detection
pub struct RefFetch<T: Component> {
    fetch: ReadFetch<T>,
}

impl<T: Component> RefFetch<T> {
    pub fn new(component_id: ComponentId) -> Self {
        Self {
            fetch: ReadFetch::new(component_id),
        }
    }
}

/// RefMut fetch - mutable fetch with change detection
pub struct RefMutFetch<T: Component> {
    fetch: WriteFetch<T>,
}

impl<T: Component> RefMutFetch<T> {
    pub fn new(component_id: ComponentId) -> Self {
        Self {
            fetch: WriteFetch::new(component_id),
        }
    }
}

// Type aliases for convenience
pub type QueryItem<'w, Q> = Q;
pub type ROQueryItem<'w, Q> = Q;

/// Fetch state trait
pub trait FetchState: Send + Sync + 'static {
    fn init(world: &crate::world::World) -> Self;
}

/// Fetch trait for retrieving component data
pub trait Fetch<'w>: Sized {
    type Item;
    type State: FetchState;
    
    fn init(state: &Self::State, world: &'w crate::world::World) -> Self;
    fn fetch(&mut self, entity: Entity) -> Self::Item;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}

    #[test]
    fn test_entity_fetch() {
        let fetch = EntityFetch::new();
        let entity = Entity::from_raw(42);
        assert_eq!(fetch.fetch(entity), entity);
    }

    #[test]
    fn test_read_fetch() {
        let component_id = ComponentId::new(1);
        let fetch = ReadFetch::<Position>::new(component_id);
        assert_eq!(fetch.component_id(), component_id);
    }

    #[test]
    fn test_write_fetch() {
        let component_id = ComponentId::new(1);
        let fetch = WriteFetch::<Position>::new(component_id);
        assert_eq!(fetch.component_id(), component_id);
    }

    #[test]
    fn test_spawn_details_fetch() {
        let fetch = SpawnDetailsFetch::new();
        let entity = Entity::from_raw(42);
        assert_eq!(fetch.spawn_tick(entity), 0);
        assert_eq!(fetch.spawned_by(entity), None);
    }
}