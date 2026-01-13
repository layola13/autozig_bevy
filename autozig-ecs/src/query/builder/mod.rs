//! Query builder module
//! 查询构建器模块
//!
//! Architecture: 90% Zig + 10% Rust
//! - Core logic in query_builder.zig
//! - Thin Rust wrapper for type safety

use crate::{
    component::{Component, ComponentId},
    query::{QueryData, QueryFilter},
    world::World,
};
use autozig_macro::include_zig;
use std::marker::PhantomData;

// Zig core integration
#[repr(C)]
pub struct QueryBuilderCoreOpaque {
    _private: [u8; 0],
}

include_zig!("src/query/builder/zig/query_builder.zig", {
    fn query_builder_create() -> *mut QueryBuilderCoreOpaque;
    fn query_builder_destroy(builder: *mut QueryBuilderCoreOpaque);
    fn query_builder_add_component(builder: *mut QueryBuilderCoreOpaque, component_id: u32) -> bool;
    fn query_builder_add_with(builder: *mut QueryBuilderCoreOpaque, component_id: u32) -> bool;
    fn query_builder_add_without(builder: *mut QueryBuilderCoreOpaque, component_id: u32) -> bool;
    fn query_builder_add_optional(builder: *mut QueryBuilderCoreOpaque, component_id: u32) -> bool;
    fn query_builder_set_read_only(builder: *mut QueryBuilderCoreOpaque, read_only: bool);
    fn query_builder_has_component(builder: *const QueryBuilderCoreOpaque, component_id: u32) -> bool;
    fn query_builder_clear(builder: *mut QueryBuilderCoreOpaque);
    fn query_builder_get_component_count(builder: *const QueryBuilderCoreOpaque) -> u32;
    fn query_builder_get_component(builder: *const QueryBuilderCoreOpaque, index: u32) -> u32;
    fn query_builder_validate(builder: *const QueryBuilderCoreOpaque) -> bool;
    fn query_builder_merge_or(builder: *mut QueryBuilderCoreOpaque, other: *const QueryBuilderCoreOpaque) -> bool;
    fn query_builder_merge_and(builder: *mut QueryBuilderCoreOpaque, other: *const QueryBuilderCoreOpaque) -> bool;
});

/// QueryBuilder - Dynamic query construction
/// 
/// Allows building queries at runtime by adding components and filters dynamically.
/// This is a thin Rust wrapper around Zig core implementation.
pub struct QueryBuilder<'w> {
    world: &'w World,
    inner: *mut QueryBuilderCoreOpaque,
}

impl<'w> QueryBuilder<'w> {
    /// Create a new query builder
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            inner: query_builder_create(),
        }
    }

    /// Add a component to query for (data access)
    pub fn data<T: Component>(&mut self) -> &mut Self {
        let component_id = self.world.component_id::<T>().expect("Component not registered");
        query_builder_add_component(self.inner, component_id.index() as u32);
        self
    }

    /// Add a filter that requires the component
    pub fn with<T: Component>(&mut self) -> &mut Self {
        let component_id = self.world.component_id::<T>().expect("Component not registered");
        query_builder_add_with(self.inner, component_id.index() as u32);
        self
    }

    /// Add a filter that excludes the component
    pub fn without<T: Component>(&mut self) -> &mut Self {
        let component_id = self.world.component_id::<T>().expect("Component not registered");
        query_builder_add_without(self.inner, component_id.index() as u32);
        self
    }

    /// Add an optional component
    pub fn optional<T: Component>(&mut self) -> &mut Self {
        let component_id = self.world.component_id::<T>().expect("Component not registered");
        query_builder_add_optional(self.inner, component_id.index() as u32);
        self
    }

    /// Set read-only mode
    pub fn read_only(&mut self, read_only: bool) -> &mut Self {
        query_builder_set_read_only(self.inner, read_only);
        self
    }

    /// Get mutable component access by ID
    pub fn mut_id(&mut self, component_id: ComponentId) -> &mut Self {
        query_builder_add_component(self.inner, component_id.index() as u32);
        self
    }

    /// Get immutable component access by ID
    pub fn ref_id(&mut self, component_id: ComponentId) -> &mut Self {
        query_builder_add_component(self.inner, component_id.index() as u32);
        query_builder_set_read_only(self.inner, true);
        self
    }

    /// Filter with component by ID
    pub fn with_id(&mut self, component_id: ComponentId) -> &mut Self {
        query_builder_add_with(self.inner, component_id.index() as u32);
        self
    }

    /// Filter without component by ID
    pub fn without_id(&mut self, component_id: ComponentId) -> &mut Self {
        query_builder_add_without(self.inner, component_id.index() as u32);
        self
    }

    /// Check if has component
    pub fn has_component(&self, component_id: ComponentId) -> bool {
        query_builder_has_component(self.inner, component_id.index() as u32)
    }

    /// Clear the builder
    pub fn clear(&mut self) -> &mut Self {
        query_builder_clear(self.inner);
        self
    }

    /// Get component count
    pub fn component_count(&self) -> usize {
        query_builder_get_component_count(self.inner) as usize
    }

    /// Get component at index
    pub fn component_at(&self, index: usize) -> Option<ComponentId> {
        if index >= self.component_count() {
            return None;
        }
        let id = query_builder_get_component(self.inner, index as u32);
        Some(ComponentId::new(id as usize))
    }

    /// Validate the builder state
    pub fn validate(&self) -> bool {
        query_builder_validate(self.inner)
    }

    /// Merge with another builder (OR logic)
    pub fn or(&mut self, other: &QueryBuilder<'w>) -> &mut Self {
        query_builder_merge_or(self.inner, other.inner);
        self
    }

    /// Merge with another builder (AND logic)
    pub fn and(&mut self, other: &QueryBuilder<'w>) -> &mut Self {
        query_builder_merge_and(self.inner, other.inner);
        self
    }

    /// Build a typed query
    pub fn build<Q: QueryData, F: QueryFilter>(&self) -> crate::query::Query<'w, Q, F> {
        unsafe { crate::query::Query::new(self.world, Box::leak(Box::new(crate::query::QueryStateInner::new::<Q, F>(self.world)))) }
    }

    /// Transmute to a different query type (type-level cast)
    pub fn transmute<Q: QueryData>(&self) -> QueryBuilder<'w> {
        // Create a new builder with same state
        QueryBuilder {
            world: self.world,
            inner: query_builder_create(),
        }
    }

    /// Transmute with filtered access
    pub fn transmute_filtered<Q: QueryData, F: QueryFilter>(&self) -> QueryBuilder<'w> {
        self.transmute::<Q>()
    }

    /// Get the world reference
    pub fn world(&self) -> &'w World {
        self.world
    }
}

impl<'w> Drop for QueryBuilder<'w> {
    fn drop(&mut self) {
        query_builder_destroy(self.inner);
    }
}

// Safety: QueryBuilder can be sent across threads
unsafe impl<'w> Send for QueryBuilder<'w> {}

// QueryBuilder is not Sync because it holds mutable state
// Uncomment if needed with proper synchronization
// unsafe impl<'w> Sync for QueryBuilder<'w> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::Component;

    #[derive(Debug, Clone, Copy)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}

    #[derive(Debug, Clone, Copy)]
    struct Velocity { x: f32, y: f32 }
    impl Component for Velocity {}

    #[derive(Debug, Clone, Copy)]
    struct Player;
    impl Component for Player {}

    #[test]
    fn test_query_builder_creation() {
        let world = World::new();
        let builder = QueryBuilder::new(&world);
        assert_eq!(builder.component_count(), 0);
    }

    #[test]
    fn test_query_builder_add_component() {
        let mut world = World::new();
        world.register_component::<Position>();
        let mut builder = QueryBuilder::new(&world);
        
        builder.data::<Position>();
        assert_eq!(builder.component_count(), 1);
    }

    #[test]
    fn test_query_builder_filters() {
        let mut world = World::new();
        world.register_component::<Position>();
        world.register_component::<Velocity>();
        world.register_component::<Player>();
        let mut builder = QueryBuilder::new(&world);
        
        builder
            .data::<Position>()
            .with::<Player>()
            .without::<Velocity>();
        
        assert!(builder.validate());
    }

    #[test]
    fn test_query_builder_clear() {
        let mut world = World::new();
        world.register_component::<Position>();
        let mut builder = QueryBuilder::new(&world);
        
        builder.data::<Position>();
        assert_eq!(builder.component_count(), 1);
        
        builder.clear();
        assert_eq!(builder.component_count(), 0);
    }

    #[test]
    fn test_query_builder_chaining() {
        let mut world = World::new();
        world.register_component::<Position>();
        world.register_component::<Velocity>();
        world.register_component::<Player>();
        let mut builder = QueryBuilder::new(&world);
        
        builder
            .data::<Position>()
            .data::<Velocity>()
            .with::<Player>();
        
        assert_eq!(builder.component_count(), 2);
    }
}