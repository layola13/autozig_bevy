//! Query filter module
//! 查询过滤器模块
//!
//! Filters determine which entities match a query without fetching component data

use crate::{
    component::{Component, ComponentId},
    entity::Entity,
};
use std::marker::PhantomData;

/// Zig core integration
#[repr(C)]
pub struct FilterCoreOpaque {
    _private: [u8; 0],
}

use autozig_macro::include_zig;

include_zig!("src/query/filter/zig/filter.zig", {
    fn filter_create() -> *mut FilterCoreOpaque;
    fn filter_destroy(filter: *mut FilterCoreOpaque);
    fn filter_matches(filter: *const FilterCoreOpaque, entity: Entity) -> bool;
});

/// With filter - requires entity to have component
pub struct With<T: Component>(PhantomData<T>);

impl<T: Component> With<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> Default for With<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Without filter - requires entity to NOT have component
pub struct Without<T: Component>(PhantomData<T>);

impl<T: Component> Without<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> Default for Without<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Or filter - matches if any of the filters match
pub struct Or<T>(PhantomData<T>);

impl<T> Or<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T> Default for Or<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Changed filter - matches entities with changed components
pub struct Changed<T: Component>(PhantomData<T>);

impl<T: Component> Changed<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> Default for Changed<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Added filter - matches entities with newly added components
pub struct Added<T: Component>(PhantomData<T>);

impl<T: Component> Added<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> Default for Added<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Allow filter - allows all entities (no-op filter)
pub struct Allow {
    inner: *mut FilterCoreOpaque,
}

impl Allow {
    pub fn new() -> Self {
        Self {
            inner: filter_create(),
        }
    }
    
    pub fn matches(&self, entity: Entity) -> bool {
        filter_matches(self.inner, entity)
    }
}

impl Drop for Allow {
    fn drop(&mut self) {
        filter_destroy(self.inner);
    }
}

unsafe impl Send for Allow {}
unsafe impl Sync for Allow {}

impl Default for Allow {
    fn default() -> Self {
        Self::new()
    }
}

/// OrFetch - fetch for Or filter
pub struct OrFetch<T> {
    _marker: PhantomData<T>,
}

impl<T> OrFetch<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Default for OrFetch<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// SpawnedFetch - fetch for entities that were just spawned
pub struct SpawnedFetch {
    _marker: PhantomData<()>,
}

impl SpawnedFetch {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
    
    pub fn matches(&self, _entity: Entity) -> bool {
        // Placeholder - would check if entity was spawned in current frame
        false
    }
}

impl Default for SpawnedFetch {
    fn default() -> Self {
        Self::new()
    }
}

/// QueryFilter trait
pub trait QueryFilter: Send + Sync + 'static {
    fn matches(&self, entity: Entity) -> bool;
}

impl QueryFilter for Allow {
    fn matches(&self, _entity: Entity) -> bool {
        true
    }
}

impl<T: Component> QueryFilter for With<T> {
    fn matches(&self, _entity: Entity) -> bool {
        // Would check if entity has component T
        true
    }
}

impl<T: Component> QueryFilter for Without<T> {
    fn matches(&self, _entity: Entity) -> bool {
        // Would check if entity does NOT have component T
        true
    }
}

impl<T: Component> QueryFilter for Changed<T> {
    fn matches(&self, _entity: Entity) -> bool {
        // Would check if component T has changed
        false
    }
}

impl<T: Component> QueryFilter for Added<T> {
    fn matches(&self, _entity: Entity) -> bool {
        // Would check if component T was just added
        false
    }
}

// Tuple implementations for Or filter
impl<A: QueryFilter, B: QueryFilter> QueryFilter for Or<(A, B)> {
    fn matches(&self, _entity: Entity) -> bool {
        // Would check if A or B matches
        false
    }
}

impl<A: QueryFilter, B: QueryFilter, C: QueryFilter> QueryFilter for Or<(A, B, C)> {
    fn matches(&self, _entity: Entity) -> bool {
        // Would check if A, B, or C matches
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}

    #[derive(Debug, Clone, Copy)]
    struct Velocity { x: f32, y: f32 }
    impl Component for Velocity {}

    #[test]
    fn test_with_filter() {
        let _filter: With<Position> = With::default();
    }

    #[test]
    fn test_without_filter() {
        let _filter: Without<Velocity> = Without::default();
    }

    #[test]
    fn test_or_filter() {
        let _filter: Or<(With<Position>, With<Velocity>)> = Or::default();
    }

    #[test]
    fn test_changed_filter() {
        let _filter: Changed<Position> = Changed::default();
    }

    #[test]
    fn test_added_filter() {
        let _filter: Added<Position> = Added::default();
    }

    #[test]
    fn test_allow_filter() {
        let filter = Allow::new();
        let entity = Entity::from_raw(42);
        assert!(filter.matches(entity));
    }

    #[test]
    fn test_spawned_fetch() {
        let fetch = SpawnedFetch::new();
        let entity = Entity::from_raw(42);
        assert!(!fetch.matches(entity));
    }
}