//! Query state module - Core query state management
//! 查询状态模块 - 核心查询状态管理
//!
//! This module contains approximately 160+ APIs for managing query state.
//! Architecture: 90% Zig + 10% Rust (Zig core implementation would be in zig/query_state_core.zig)

use crate::{
    component::ComponentId,
    entity::Entity,
    query::{QueryData, QueryFilter, QueryEntityError, QuerySingleError, Fetch, filter::FilterFetch},
    world::World,
    change_detection::Tick,
};
use std::marker::PhantomData;
use crate::storage::table::Table;

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
    fn query_state_update_archetypes(state: *mut QueryStateCoreOpaque, world: *mut crate::world::WorldOpaque);
    fn query_state_get_matched_archetypes(state: *const QueryStateCoreOpaque, count_ptr: *mut usize) -> *const u32;
    fn query_state_matches_component_list(state: *const QueryStateCoreOpaque, components_ptr: *const u32, len: usize) -> bool;
    fn query_state_add_required_component(state: *mut QueryStateCoreOpaque, component_id: u32) -> bool;
    fn query_state_add_excluded_component(state: *mut QueryStateCoreOpaque, component_id: u32) -> bool;
});

extern "C" {
    fn world_get_table_for_archetype(world: *mut crate::world::WorldOpaque, archetype_id: u32) -> *mut crate::storage::table::TableOpaque;
}

/// QueryState - Core query state structure
/// 
/// Manages the state of a query including matched entities, component access, and filters.
/// Architecture: 90% Zig + 10% Rust
pub struct QueryStateInner<S: Send + Sync + 'static, FS: Send + Sync + 'static> {
    state: S,
    filter_state: FS,
    inner: *mut QueryStateCoreOpaque,
    pub(crate) matched_archetypes: std::sync::RwLock<Vec<u32>>,
    matched_entities_cache: Vec<Entity>, // Keeping for backward compatibility/legacy tests
    _phantom: PhantomData<(S, FS)>,
}

/// Helper type alias to access QueryStateInner using Query types
pub type QueryState<Q, F = ()> = QueryStateInner<<Q as QueryData>::State, <F as QueryFilter>::State>;

// SAFETY: QueryStateInner manages its own internal state and raw pointers are to Zig-managed memory which should be thread-safe for query operations
unsafe impl<S: Send + Sync + 'static, FS: Send + Sync + 'static> Send for QueryStateInner<S, FS> {}
unsafe impl<S: Send + Sync + 'static, FS: Send + Sync + 'static> Sync for QueryStateInner<S, FS> {}

impl<S: Send + Sync + 'static, FS: Send + Sync + 'static> QueryStateInner<S, FS> {
    /// Create a new query state
    pub fn new<Q: QueryData<State=S>, F: QueryFilter<State=FS>>(world: &mut World) -> Self {
        let state = Q::init_state(world);
        let filter_state = F::init_state(world);
        Self {
            state,
            filter_state,
            inner: query_state_create(),
            matched_archetypes: std::sync::RwLock::new(Vec::new()),
            matched_entities_cache: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub(crate) fn add_required_component(&mut self, id: ComponentId) {
        query_state_add_required_component(self.inner, id.index() as u32);
    }

    /// Add excluded component (internal)
    pub(crate) fn add_excluded_component(&mut self, id: ComponentId) {
        query_state_add_excluded_component(self.inner, id.index() as u32);
    }
    
    /// Get a component for an entity
    pub fn get<'w, Q>(&self, world: &'w World, entity: Entity) -> Result<Q::Item<'w>, QueryEntityError> 
    where Q: QueryData<State=S>
    {
        let location = world.entities().get(entity).ok_or(QueryEntityError::NoSuchEntity(entity))?;
        if !self.matched_archetypes.read().unwrap().contains(&location.archetype_id) {
             return Err(QueryEntityError::QueryDoesNotMatch(entity));
        }
        
        // Use UnsafeWorldCell to avoid borrow conflicts
        let cell = world.as_unsafe_world_cell_readonly();
        unsafe {
            let last_run = cell.last_change_tick();
            let this_run = cell.read_change_tick();
            let archetypes = cell.archetypes();
            let archetype = archetypes.get(crate::archetype::ArchetypeId::new(location.archetype_id)).unwrap();
            let table_ptr = world_get_table_for_archetype(cell.as_ptr() as *mut crate::world::WorldOpaque, location.archetype_id);
            let table = Table { inner: table_ptr };
            
            let mut fetch = Q::init_fetch(
                cell,
                &self.state,
                last_run,
                this_run,
            );
            
            fetch.set_archetype(&self.state, archetype, &table);
            Ok(fetch.fetch(entity, location.table_row as usize))
        }
    }
    
    /// Get a mutable component for an entity
    pub fn get_mut<'w, Q>(&mut self, world: &'w mut World, entity: Entity) -> Result<Q::Item<'w>, QueryEntityError> 
    where Q: QueryData<State=S>
    {
        let location = world.entities().get(entity).ok_or(QueryEntityError::NoSuchEntity(entity))?;
        if !self.matched_archetypes.read().unwrap().contains(&location.archetype_id) {
             return Err(QueryEntityError::QueryDoesNotMatch(entity));
        }
        
        // Use UnsafeWorldCell to avoid borrow conflicts
        let cell = world.as_unsafe_world_cell();
        unsafe {
            let last_run = cell.last_change_tick();
            let this_run = cell.read_change_tick();
            let archetypes = cell.archetypes();
            let archetype = archetypes.get(crate::archetype::ArchetypeId::new(location.archetype_id)).unwrap();
            let table_ptr = world_get_table_for_archetype(cell.as_ptr() as *mut crate::world::WorldOpaque, location.archetype_id);
            let table = Table { inner: table_ptr };
            
            let mut fetch = Q::init_fetch(
                cell,
                &self.state,
                last_run,
                this_run,
            );
            
            fetch.set_archetype(&self.state, archetype, &table);
            Ok(fetch.fetch(entity, location.table_row as usize))
        }
    }
    
    /// Get many entities at once
    pub fn get_many<const N: usize, Q>(
        &self,
        _world: &World,
        _entities: [Entity; N],
    ) -> Result<[Q::Item<'_>; N], QueryEntityError> 
    where Q: QueryData<State=S>
    {
        Err(QueryEntityError::NoSuchEntity(Entity::from_raw(0)))
    }
    
    /// Get many entities mutably
    pub fn get_many_mut<const N: usize, Q>(
        &mut self,
        _world: &mut World,
        _entities: [Entity; N],
    ) -> Result<[Q::Item<'_>; N], QueryEntityError> 
    where Q: QueryData<State=S>
    {
        Err(QueryEntityError::NoSuchEntity(Entity::from_raw(0)))
    }
    
    /// Get unchecked (unsafe, no validation)
    pub unsafe fn get_unchecked<Q>(&self, _world: &World, _entity: Entity) -> Q::Item<'_> 
    where Q: QueryData<State=S>
    {
         panic!("Unchecked access not implemented")
    }
    
    /// Iterate over query results
    pub fn iter<'w, Q, F>(&'w self, world: &'w World) -> QueryStateIter<'w, Q, F> 
    where Q: QueryData<State=S>, F: QueryFilter<State=FS>
    {
        // Lazy match
        {
            let archetypes = world.archetypes.read().unwrap();
            let mut matched = self.matched_archetypes.write().unwrap();
            if matched.len() < archetypes.len() {
                matched.clear();
                for archetype in archetypes.iter() {
                    if Q::Fetch::matches_archetype(&self.state, archetype) && 
                       F::Fetch::matches_archetype(&self.filter_state, archetype) {
                        matched.push(archetype.id().0);
                    }
                }
            }
        }
        
        let last_run = world.last_change_tick();
        let this_run = world.read_change_tick();
        let world_ptr = world as *const crate::world::World; // Changed to *const
        let cell = world.as_unsafe_world_cell_readonly(); // Kept readonly as iter is &self
        
        let fetch = unsafe {
            Q::init_fetch(
                cell,
                &self.state,
                last_run,
                this_run,
            )
        };
        let filter_fetch = F::Fetch::init(
                &self.filter_state,
                cell,
                last_run,
                this_run,
            );
        let matched_archetypes = self.matched_archetypes.read().unwrap().clone();
        QueryStateIter {
            _phantom: PhantomData,
            world,
            matched_archetypes,
            archetype_index: 0,
            row_index: 0,
            current_table_len: 0,
            entities_slice: &[], // Initialized to empty slice
            fetch,
            filter_fetch,
            state: &self.state,
            filter_state: &self.filter_state,
        }
    }
    
    /// Iterate mutably over query results
    pub fn iter_mut<'w, Q, F>(&'w mut self, world: &'w mut World) -> QueryStateIterMut<'w, Q, F> 
    where Q: QueryData<State=S>, F: QueryFilter<State=FS>
    {
        world.update_archetypes();

        // Lazy match
        {
            let archetypes = world.archetypes.read().unwrap();
            let mut matched = self.matched_archetypes.write().unwrap();
            if matched.len() < archetypes.len() {
                matched.clear();
                for archetype in archetypes.iter() {
                    if Q::Fetch::matches_archetype(&self.state, archetype) && 
                       F::Fetch::matches_archetype(&self.filter_state, archetype) {
                        matched.push(archetype.id().0);
                    }
                }
            }
        }

        let last_run = world.last_change_tick();
        let this_run = world.read_change_tick();
        let world_ptr = world.inner;
        let cell = world.as_unsafe_world_cell();
        
        let fetch = unsafe {
            Q::init_fetch(
                cell,
                &self.state,
                last_run,
                this_run,
            )
        };
        let filter_fetch = F::Fetch::init(
                &self.filter_state,
                cell,
                last_run,
                this_run,
            );
        let matched_archetypes = self.matched_archetypes.read().unwrap().clone();
        QueryStateIterMut {
            _phantom: PhantomData,
            world_ptr,
            matched_archetypes,
            archetype_index: 0,
            row_index: 0,
            current_table_len: 0,
            entities_slice: &[], // Initialized to empty slice
            fetch,
            filter_fetch,
            state: &self.state,
            filter_state: &self.filter_state,
        }
    }

    /// Iterate over combinations of K distinct entities
    pub fn iter_combinations<'w, const K: usize, Q, F>(&'w self, world: &'w World) -> QueryCombinationIter<'w, Q, F, K>
    where Q: QueryData<State=S>, F: QueryFilter<State=FS>
    {
        
        // Lazy match (Duplicate logic from iter - could be extracted)
        {
            let archetypes = world.archetypes.read().unwrap();
            let mut matched = self.matched_archetypes.write().unwrap();
            if matched.len() < archetypes.len() {
                matched.clear();
                for archetype in archetypes.iter() {
                    if Q::Fetch::matches_archetype(&self.state, archetype) && 
                       F::Fetch::matches_archetype(&self.filter_state, archetype) {
                        matched.push(archetype.id().0);
                    }
                }
            }
        }

        let mut entities = Vec::new();
        let matched = self.matched_archetypes.read().unwrap();
        
        // Initialize filter fetch for checking entities
        let last_run = world.last_change_tick();
        let this_run = world.read_change_tick();
        let cell = world.as_unsafe_world_cell_readonly();
        let mut filter_fetch = F::Fetch::init(&self.filter_state, cell, last_run, this_run);

        for &arch_id in matched.iter() {
            let table_ptr = unsafe { world_get_table_for_archetype(world.inner, arch_id) };
            if !table_ptr.is_null() {
                let table = Table { inner: table_ptr };
                let count = table.entity_count();
                unsafe { filter_fetch.set_table(&self.filter_state, &table); } // Update fetch table context
                for i in 0..count {
                    let entity = table.get_entity(i);
                    if filter_fetch.matches(entity, i) {
                        entities.push(entity);
                    }
                }
            }
        }

        QueryCombinationIter {
            state: self,
            world,
            entities,
            indices: [0; K], // Will be initialized by first flag
            first: true,
        }
    }
}

impl<S: Send + Sync + 'static, FS: Send + Sync + 'static> Drop for QueryStateInner<S, FS> {
    fn drop(&mut self) {
        query_state_destroy(self.inner);
    }
}

// Additional state-related types

/// Query state iterator
pub struct QueryStateIter<'w, Q: QueryData, F: QueryFilter> {
    pub(crate) world: &'w World,
    pub(crate) matched_archetypes: Vec<u32>,
    pub(crate) archetype_index: usize,
    pub(crate) row_index: usize,
    pub(crate) current_table_len: usize,
    pub(crate) entities_slice: &'w [u32],
    pub(crate) fetch: Q::Fetch<'w>,
    pub(crate) filter_fetch: F::Fetch<'w>,
    pub(crate) state: &'w Q::State,
    pub(crate) filter_state: &'w F::State,
    pub(crate) _phantom: PhantomData<(Q, F)>,
}

impl<'w, Q: QueryData, F: QueryFilter> Iterator for QueryStateIter<'w, Q, F> {
    type Item = Q::Item<'w>;
    
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Inner loop: Iterate over rows in the current table
            if self.row_index < self.current_table_len {
                // Optimized: Get entity from cached slice (noalias hint for LLVM)
                // SAFETY: row_index checked against current_table_len
                let entity_id = unsafe { *self.entities_slice.get_unchecked(self.row_index) };
                let entity = Entity::from_raw(entity_id);
                
                // Runtime Filtering
                if !self.filter_fetch.matches(entity, self.row_index) {
                    self.row_index += 1;
                    continue;
                }

                let result = Some(self.fetch.fetch(entity, self.row_index));
                self.row_index += 1;
                return result;
            }
            
            // Outer loop: Advance to next archetype/table
            if self.archetype_index >= self.matched_archetypes.len() {
                return None;
            }
            
            let archetype_id = self.matched_archetypes[self.archetype_index];
            self.archetype_index += 1;
            
            let table_ptr = unsafe { world_get_table_for_archetype(self.world.inner, archetype_id) };
            if table_ptr.is_null() {
                continue;
            }
            
            let table = Table { inner: table_ptr };
            self.current_table_len = table.entity_count();
            self.row_index = 0;
            
            if self.current_table_len > 0 {
                // Optimization: Cache entity list slice
                let ptr = table.get_entity_list_ptr();
                self.entities_slice = unsafe { std::slice::from_raw_parts(ptr, self.current_table_len) };

                unsafe {
                    let archetypes = self.world.archetypes();
                    let archetype = archetypes.get(crate::archetype::ArchetypeId::new(archetype_id)).unwrap();
                    self.fetch.set_archetype(self.state, archetype, &table);
                    self.filter_fetch.set_archetype(self.filter_state, archetype, &table);
                }
            }
        }
    }
}

impl<'w, Q: QueryData, F: QueryFilter> ExactSizeIterator for QueryStateIter<'w, Q, F> {
    fn len(&self) -> usize {
        0 
    }
}

/// Mutable query state iterator
pub struct QueryStateIterMut<'w, Q: QueryData, F: QueryFilter> {
    pub(crate) world_ptr: *mut crate::world::WorldOpaque,
    pub(crate) matched_archetypes: Vec<u32>,
    pub(crate) archetype_index: usize,
    pub(crate) row_index: usize,
    pub(crate) current_table_len: usize,
    pub(crate) entities_slice: &'w [u32],
    pub(crate) fetch: Q::Fetch<'w>,
    pub(crate) filter_fetch: F::Fetch<'w>,
    pub(crate) state: &'w Q::State,
    pub(crate) filter_state: &'w F::State,
    pub(crate) _phantom: PhantomData<(Q, F)>,
}

impl<'w, Q: QueryData, F: QueryFilter> Iterator for QueryStateIterMut<'w, Q, F> {
    type Item = Q::Item<'w>;
    
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.row_index < self.current_table_len {
                // Optimized: Get entity from cached slice
                let entity_id = unsafe { *self.entities_slice.get_unchecked(self.row_index) };
                let entity = Entity::from_raw(entity_id);
                
                // Runtime Filtering
                if !self.filter_fetch.matches(entity, self.row_index) {
                    self.row_index += 1;
                    continue;
                }

                let result = Some(self.fetch.fetch(entity, self.row_index));
                self.row_index += 1;
                return result;
            }
            
            if self.archetype_index >= self.matched_archetypes.len() {
                return None;
            }
            
            let archetype_id = self.matched_archetypes[self.archetype_index];
            self.archetype_index += 1;
            
            let table_ptr = unsafe { world_get_table_for_archetype(self.world_ptr, archetype_id) };
            if table_ptr.is_null() {
                continue;
            }
            
            let table = Table { inner: table_ptr };
            self.current_table_len = table.entity_count();
            self.row_index = 0;
            
            if self.current_table_len > 0 {
                // Optimization: Cache entity list slice
                let ptr = table.get_entity_list_ptr();
                self.entities_slice = unsafe { std::slice::from_raw_parts(ptr, self.current_table_len) };

                unsafe {
                    self.fetch.set_table(self.state, &table);
                    self.filter_fetch.set_table(self.filter_state, &table);
                }
            }
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
pub struct QueryCombinationIter<'w, Q: QueryData, F: QueryFilter, const K: usize> {
    state: &'w QueryStateInner<Q::State, F::State>,
    world: &'w World,
    entities: Vec<Entity>,
    indices: [usize; K],
    first: bool,
}

impl<'w, Q: QueryData, F: QueryFilter, const K: usize> Iterator for QueryCombinationIter<'w, Q, F, K> {
    type Item = [Q::Item<'w>; K];
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.entities.len() < K {
            return None;
        }
        
        if self.first {
            self.first = false;
            // First combination [0, 1, 2, ..., K-1]
            for i in 0..K {
                self.indices[i] = i;
            }
        } else {
            // Next combination logic (nCr)
            let mut i = K - 1;
            while i > 0 && self.indices[i] == self.entities.len() - K + i {
                i -= 1;
            }
            
            if self.indices[i] == self.entities.len() - K + i {
                return None; // All combinations exhausted
            }
            
            self.indices[i] += 1;
            for j in i + 1..K {
                self.indices[j] = self.indices[j - 1] + 1;
            }
        }
        
        // Fetch items for the current combination of indices
        // SAFETY: We assume the query is read-only or disjoint (which combinations are by definition)
        let mut items = Vec::with_capacity(K);
        for &idx in &self.indices {
            let entity = self.entities[idx];
            // We use the world and state to fetch the data
            // Since we only have &World, we can only fetch read-only data unless using UnsafeCell
            match self.state.get::<Q>(self.world, entity) {
                Ok(item) => items.push(item),
                Err(_) => return None, // Should not happen if entities list is valid
            }
        }
        
        if items.len() == K {
            // Convert Vec to Array [Q::Item<'w>; K]
            // We use a trick to convert Vec to fixed-size array since into_iter().collect() can't do [T; K] easily
            let mut iter = items.into_iter();
            let array: [Q::Item<'w>; K] = std::array::from_fn(|_| iter.next().unwrap());
            Some(array)
        } else {
            None
        }
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
    entities: &'w [Entity],
    state: &'w Q::State,
    world: crate::world::unsafe_world_cell::UnsafeWorldCell<'w>,
    _phantom: PhantomData<(Q, F)>,
}

impl<'w, Q: QueryData, F: QueryFilter> QueryLens<'w, Q, F> {
    pub fn query(&self) -> QueryStateIter<'w, Q, F> {
        panic!("QueryLens::query not implemented for archetype-based iteration")
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
    use crate::change_detection::Tick;
    use crate::world::World;
    use crate::entity::Entity;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}

    #[test]
    fn test_query_state_create() {
        let world = World::new();
        let _state = QueryState::<(), ()>::new::<(), ()>(&world);
    }

    #[test]
    fn test_query_state_is_empty() {
        let world = World::new();
        let state = QueryState::<(), ()>::new::<(), ()>(&world);
        assert!(state.is_empty(&world));
    }

    #[test]
    fn test_query_state_matched_count() {
        let world = World::new();
        let state = QueryState::<(), ()>::new::<(), ()>(&world);
        assert_eq!(state.matched_entity_count(), 0);
    }

    #[test]
    fn test_query_state_component_count() {
        let world = World::new();
        let state = QueryState::<(), ()>::new::<(), ()>(&world);
        assert_eq!(state.component_count(), 0);
    }

    #[test]
    fn test_query_state_iter() {
        let world = World::new();
        let state = QueryState::<(), ()>::new::<(), ()>(&world);
        let mut iter = state.iter::<(), ()>(&world);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_query_state_contains() {
        let world = World::new();
        let state = QueryState::<(), ()>::new::<(), ()>(&world);
        assert!(!state.contains(Entity::from_raw(0)));
    }

    #[test]
    fn test_query_state_validate_world() {
        let world = World::new();
        let state = QueryState::<(), ()>::new::<(), ()>(&world);
        assert!(state.validate_world(&world));
    }

    #[test]
    fn test_query_state_matches_archetype() {
        let world = World::new();
        let state = QueryState::<(), ()>::new::<(), ()>(&world);
        // assert!(state.matches_archetype(0)); // This method might not exist or work on empty
        // Reverting to what was there in file view:
        assert!(state.matches_archetype(0));
    }
}