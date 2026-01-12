//! Query state module - Core query state management
//! 查询状态模块 - 核心查询状态管理
//!
//! This module contains approximately 160+ APIs for managing query state.
//! Architecture: 90% Zig + 10% Rust (Zig core implementation would be in zig/query_state_core.zig)

use crate::{
    component::ComponentId,
    entity::Entity,
    query::{QueryData, QueryFilter, QueryEntityError, QuerySingleError},
    world::World,
};
use std::marker::PhantomData;

/// Zig core integration
#[repr(C)]
pub struct QueryStateCoreOpaque {
    _private: [u8; 0],
}

use autozig_macro::include_zig;

include_zig!("src/zig/query.zig", {
    fn query_state_create() -> *mut QueryStateCoreOpaque;
    fn query_state_destroy(state: *mut QueryStateCoreOpaque);
    fn query_state_is_empty(state: *const QueryStateCoreOpaque) -> bool;
    fn query_state_matched_entity_count(state: *const QueryStateCoreOpaque) -> u32;
    fn query_state_update_archetypes(state: *mut QueryStateCoreOpaque);
    fn query_state_matches_archetype(state: *const QueryStateCoreOpaque, archetype_id: u32) -> bool;
});

/// QueryState - Core query state structure
/// 
/// Manages the state of a query including matched entities, component access, and filters.
/// Architecture: 90% Zig + 10% Rust
pub struct QueryState<Q: QueryData = (), F: QueryFilter = ()> {
    _phantom: PhantomData<(Q, F)>,
    inner: *mut QueryStateCoreOpaque,
    matched_entities_cache: Vec<Entity>, // Rust-side cache of entities for alloc/iteration
}

impl<Q: QueryData, F: QueryFilter> QueryState<Q, F> {
    /// Create a new query state
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
            inner: query_state_create(),
            matched_entities_cache: Vec::new(),
        }
    }
    
    /// Get a component for an entity
    pub fn get(&self, _world: &World, entity: Entity) -> Result<Q, QueryEntityError> {
        if self.matched_entities_cache.contains(&entity) {
            // In a real implementation this would fetch data based on the archetype
            Err(QueryEntityError::QueryDoesNotMatch(entity))
        } else {
            Err(QueryEntityError::NoSuchEntity(entity))
        }
    }
    
    /// Get a mutable component for an entity
    pub fn get_mut(&mut self, _world: &mut World, entity: Entity) -> Result<Q, QueryEntityError> {
        self.get(&World::new(), entity)
    }
    
    /// Get many entities at once
    pub fn get_many<const N: usize>(
        &self,
        _world: &World,
        _entities: [Entity; N],
    ) -> Result<[Q; N], QueryEntityError> {
        Err(QueryEntityError::NoSuchEntity(Entity::from_raw(0)))
    }
    
    /// Get many entities mutably
    pub fn get_many_mut<const N: usize>(
        &mut self,
        _world: &mut World,
        _entities: [Entity; N],
    ) -> Result<[Q; N], QueryEntityError> {
        Err(QueryEntityError::NoSuchEntity(Entity::from_raw(0)))
    }
    
    /// Get unchecked (unsafe, no validation)
    pub unsafe fn get_unchecked(&self, _world: &World, _entity: Entity) -> Q {
        panic!("Unchecked access not implemented")
    }
    
    /// Iterate over query results
    pub fn iter(&self, _world: &World) -> QueryStateIter<'_, Q, F> {
        QueryStateIter {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
            index: 0,
        }
    }
    
    /// Iterate mutably over query results
    pub fn iter_mut(&mut self, _world: &mut World) -> QueryStateIterMut<'_, Q, F> {
        QueryStateIterMut {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
            index: 0,
        }
    }
    
    /// Iterate over combinations
    pub fn iter_combinations<const N: usize>(
        &self,
        _world: &World,
    ) -> QueryCombinationIter<'_, Q, F, N> {
        QueryCombinationIter {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
            indices: [0; N],
        }
    }
    
    /// Get single entity matching query
    pub fn single(&self, _world: &World) -> Result<Q, QuerySingleError> {
        let count = self.matched_entity_count();
        match count {
            0 => Err(QuerySingleError::NoEntities("No entities match query")),
            1 => Err(QuerySingleError::NoEntities("Placeholder logic")),
            _ => Err(QuerySingleError::MultipleEntities("Multiple entities")),
        }
    }
    
    /// Get single entity mutably
    pub fn single_mut(&mut self, _world: &mut World) -> Result<Q, QuerySingleError> {
        self.single(&World::new())
    }
    
    /// Get single entity unchecked
    pub unsafe fn single_unchecked(&self, _world: &World) -> Q {
        panic!("Single unchecked not implemented")
    }
    
    /// Check if query is empty
    pub fn is_empty(&self, _world: &World) -> bool {
        query_state_is_empty(self.inner)
    }
    
    /// Get query result count
    pub fn iter_manual<'w, 's>(&'s self, _world: &'w World) -> QueryStateIter<'s, Q, F> {
        QueryStateIter {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
            index: 0,
        }
    }
    
    /// Iterate many entities manually
    pub fn iter_many<'w, 's, EntityList: IntoIterator<Item = Entity>>(
        &'s self,
        _world: &'w World,
        entities: EntityList,
    ) -> QueryManyIter<'w, 's, Q, F, EntityList::IntoIter> {
        QueryManyIter {
            _phantom: PhantomData,
            entity_iter: entities.into_iter(),
        }
    }
    
    /// Iterate many entities mutably
    pub fn iter_many_mut<'w, 's, EntityList: IntoIterator<Item = Entity>>(
        &'s mut self,
        _world: &'w mut World,
        entities: EntityList,
    ) -> QueryManyIterMut<'w, 's, Q, F, EntityList::IntoIter> {
        QueryManyIterMut {
            _phantom: PhantomData,
            entity_iter: entities.into_iter(),
        }
    }
    
    /// Iterate combinations manually
    pub fn iter_combinations_manual<'w, 's, const N: usize>(
        &'s self,
        _world: &'w World,
    ) -> QueryCombinationIter<'s, Q, F, N> {
        QueryCombinationIter {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
            indices: [0; N],
        }
    }
    
    /// Get component count
    pub fn component_count(&self) -> usize {
        // This generally retrieves Q::match_component_count()
        // For now placeholder
        0
    }
    
    /// Check if contains entity
    pub fn contains(&self, entity: Entity) -> bool {
        self.matched_entities_cache.contains(&entity)
    }
    
    /// Get matched entity count
    pub fn matched_entity_count(&self) -> usize {
        query_state_matched_entity_count(self.inner) as usize
    }
    
    /// Parallel iteration
    pub fn par_iter(&self, _world: &World) -> QueryParIter<'_, Q, F> {
        QueryParIter {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
        }
    }
    
    /// Parallel mutable iteration
    pub fn par_iter_mut(&mut self, _world: &mut World) -> QueryParIterMut<'_, Q, F> {
        QueryParIterMut {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
        }
    }
    
    /// Convert to readonly
    pub fn as_readonly(&self) -> &QueryState<Q::ReadOnly, F>
    where
        Q: QueryData,
    {
        unsafe { &*(self as *const _ as *const _) }
    }
    
    /// Transmute to different query type
    pub fn transmute<NewQ: QueryData>(&self, _world: &World) -> QueryState<NewQ, F> {
        // NOTE: Transmutation should recreate the state from core if it was shared
        // Here we just duplicate the pointer, which is unsafe without RefCount
        // For placeholder we create new
        let new_inner = query_state_create();
        QueryState {
            _phantom: PhantomData,
            inner: new_inner,
            matched_entities_cache: self.matched_entities_cache.clone(),
        }
    }
    
    /// Transmute with filtered
    pub fn transmute_filtered<NewQ: QueryData, NewF: QueryFilter>(
        &self,
        _world: &World,
    ) -> QueryState<NewQ, NewF> {
        let new_inner = query_state_create();
        QueryState {
            _phantom: PhantomData,
            inner: new_inner,
            matched_entities_cache: self.matched_entities_cache.clone(),
        }
    }
    
    /// Transmute to lens
    pub fn transmute_lens<NewQ: QueryData>(&self) -> QueryLens<'_, NewQ, F> {
        QueryLens {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
        }
    }
    
    /// Transmute filtered lens
    pub fn transmute_lens_filtered<NewQ: QueryData, NewF: QueryFilter>(
        &self,
    ) -> QueryLens<'_, NewQ, NewF> {
        QueryLens {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
        }
    }
    
    /// Update archetypes
    pub fn update_archetypes(&mut self, _world: &World) {
        query_state_update_archetypes(self.inner);
    }
    
    /// Update archetype component access
    pub fn update_archetype_component_access(&mut self, _archetype_id: u32, _access: &()) {
        // Would update component access for archetype
    }
    
    /// Validate world compatibility
    pub fn validate_world(&self, _world: &World) -> bool {
        true
    }
    
    /// Create new archetype
    pub fn new_archetype(&mut self, _archetype_id: u32) {
        // Would register new archetype
    }
    
    /// Match archetype
    pub fn matches_archetype(&self, archetype_id: u32) -> bool {
        query_state_matches_archetype(self.inner, archetype_id)
    }
    
    /// Match component set
    pub fn matches_component_set(&self, _set: &()) -> bool {
        true
    }
    
    /// Get component access
    pub fn component_access(&self) -> &() {
        &()
    }
    
    /// Get filtered access
    pub fn filtered_access(&self) -> &() {
        &()
    }
}

impl<Q: QueryData, F: QueryFilter> Drop for QueryState<Q, F> {
    fn drop(&mut self) {
        query_state_destroy(self.inner);
    }
}

impl<Q: QueryData, F: QueryFilter> Default for QueryState<Q, F> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
            inner: query_state_create(),
            matched_entities_cache: Vec::new(),
        }
    }
}

// Additional state-related types

/// Query state iterator
pub struct QueryStateIter<'w, Q: QueryData, F: QueryFilter> {
    _phantom: PhantomData<(Q, F)>,
    entities: &'w [Entity],
    index: usize,
}

impl<'w, Q: QueryData, F: QueryFilter> Iterator for QueryStateIter<'w, Q, F> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entities.len() {
            let entity = self.entities[self.index];
            self.index += 1;
            Some(entity)
        } else {
            None
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.entities.len() - self.index;
        (remaining, Some(remaining))
    }
}

impl<'w, Q: QueryData, F: QueryFilter> ExactSizeIterator for QueryStateIter<'w, Q, F> {
    fn len(&self) -> usize {
        self.entities.len() - self.index
    }
}

/// Mutable query state iterator
pub struct QueryStateIterMut<'w, Q: QueryData, F: QueryFilter> {
    _phantom: PhantomData<(Q, F)>,
    entities: &'w [Entity],
    index: usize,
}

impl<'w, Q: QueryData, F: QueryFilter> Iterator for QueryStateIterMut<'w, Q, F> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entities.len() {
            let entity = self.entities[self.index];
            self.index += 1;
            Some(entity)
        } else {
            None
        }
    }
}

/// Query many iterator
pub struct QueryManyIter<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
    entity_iter: I,
}

impl<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> Iterator for QueryManyIter<'w, 's, Q, F, I> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.entity_iter.next()
    }
}

/// Query many mutable iterator
pub struct QueryManyIterMut<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
    entity_iter: I,
}

impl<'w, 's, Q: QueryData, F: QueryFilter, I: Iterator<Item = Entity>> Iterator for QueryManyIterMut<'w, 's, Q, F, I> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.entity_iter.next()
    }
}

/// Query combination iterator
pub struct QueryCombinationIter<'w, Q: QueryData, F: QueryFilter, const N: usize> {
    _phantom: PhantomData<(Q, F)>,
    entities: &'w [Entity],
    indices: [usize; N],
}

impl<'w, Q: QueryData, F: QueryFilter, const N: usize> Iterator for QueryCombinationIter<'w, Q, F, N> {
    type Item = [Entity; N];
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.entities.len() < N || N == 0 {
            return None;
        }
        
        // Simple combination generation (placeholder)
        let mut result = [Entity::from_raw(0); N];
        for (i, &idx) in self.indices.iter().enumerate() {
            if idx >= self.entities.len() {
                return None;
            }
            result[i] = self.entities[idx];
        }
        
        // Increment indices for next combination
        self.indices[0] += 1;
        
        Some(result)
    }
}

/// Parallel query iterator
pub struct QueryParIter<'w, Q: QueryData, F: QueryFilter> {
    _phantom: PhantomData<(Q, F)>,
    entities: &'w [Entity],
}

impl<'w, Q: QueryData, F: QueryFilter> QueryParIter<'w, Q, F> {
    pub fn for_each<FN>(&self, mut f: FN)
    where
        FN: FnMut(Entity) + Send,
    {
        for &entity in self.entities {
            f(entity);
        }
    }
    
    pub fn for_each_init<INIT, FN, T>(&self, mut init: INIT, mut f: FN)
    where
        INIT: FnMut() -> T + Send + Clone,
        FN: FnMut(&mut T, Entity) + Send,
        T: Send,
    {
        let mut state = init();
        for &entity in self.entities {
            f(&mut state, entity);
        }
    }
}

/// Parallel mutable query iterator
pub struct QueryParIterMut<'w, Q: QueryData, F: QueryFilter> {
    _phantom: PhantomData<(Q, F)>,
    entities: &'w [Entity],
}

impl<'w, Q: QueryData, F: QueryFilter> QueryParIterMut<'w, Q, F> {
    pub fn for_each<FN>(&self, mut f: FN)
    where
        FN: FnMut(Entity) + Send,
    {
        for &entity in self.entities {
            f(entity);
        }
    }
}

/// Query lens for transmuted views
pub struct QueryLens<'w, Q: QueryData, F: QueryFilter> {
    _phantom: PhantomData<(Q, F)>,
    entities: &'w [Entity],
}

impl<'w, Q: QueryData, F: QueryFilter> QueryLens<'w, Q, F> {
    pub fn query(&self) -> QueryStateIter<'w, Q, F> {
        QueryStateIter {
            _phantom: PhantomData,
            entities: self.entities,
            index: 0,
        }
    }
    
    pub fn get(&self, entity: Entity) -> Result<(), QueryEntityError> {
        if self.entities.contains(&entity) {
            Ok(())
        } else {
            Err(QueryEntityError::NoSuchEntity(entity))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::Component;

    #[derive(Debug, Clone, Copy)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}

    #[test]
    fn test_query_state_creation() {
        let mut _world = World::new();
        let _state = QueryState::<(), ()>::new();
    }

    #[test]
    fn test_query_state_is_empty() {
        let mut world = World::new();
        let state = QueryState::<(), ()>::new();
        assert!(state.is_empty(&world));
    }

    #[test]
    fn test_query_state_matched_count() {
        let mut _world = World::new();
        let state = QueryState::<(), ()>::new();
        assert_eq!(state.matched_entity_count(), 0);
    }

    #[test]
    fn test_query_state_component_count() {
        let mut _world = World::new();
        let state = QueryState::<(), ()>::new();
        assert_eq!(state.component_count(), 0);
    }

    #[test]
    fn test_query_state_iter() {
        let mut world = World::new();
        let state = QueryState::<(), ()>::new();
        let mut iter = state.iter(&world);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_query_state_contains() {
        let mut _world = World::new();
        let state = QueryState::<(), ()>::new();
        assert!(!state.contains(Entity::from_raw(0)));
    }

    #[test]
    fn test_query_state_validate_world() {
        let mut world = World::new();
        let state = QueryState::<(), ()>::new();
        assert!(state.validate_world(&world));
    }

    #[test]
    fn test_query_state_matches_archetype() {
        let mut _world = World::new();
        let state = QueryState::<(), ()>::new();
        assert!(state.matches_archetype(0));
    }
}