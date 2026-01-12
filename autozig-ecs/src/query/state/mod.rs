//! Query state module - Core query state management
//! 查询状态模块 - 核心查询状态管理

use crate::{
    component::ComponentId,
    entity::Entity,
    query::{QueryData, QueryFilter, QueryEntityError, QuerySingleError, world_query::WorldQuery, fetch::Fetch},
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
    fn query_state_matches_component_list(state: *const QueryStateCoreOpaque, components_ptr: *const u32, len: usize) -> bool;
    fn query_state_add_required_component(state: *mut QueryStateCoreOpaque, component_id: u32) -> bool;
    fn query_state_add_excluded_component(state: *mut QueryStateCoreOpaque, component_id: u32) -> bool;
});

/// QueryState - Core query state structure
pub struct QueryState<Q: QueryData = (), F: QueryFilter = ()> {
    pub(crate) state: Q::State,
    pub(crate) filter_state: F::State,
    pub(crate) inner: *mut QueryStateCoreOpaque,
    pub(crate) matched_entities_cache: Vec<Entity>,
    pub(crate) _phantom: PhantomData<(Q, F)>,
}

impl<Q: QueryData, F: QueryFilter> QueryState<Q, F> {
    pub fn new(world: &World) -> Self {
        let state = Q::init_state(world);
        let filter_state = F::init_state(world);
        Self {
            state,
            filter_state,
            inner: unsafe { query_state_create() },
            matched_entities_cache: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn get<'w>(&self, world: &'w World, entity: Entity) -> Result<Q::Item<'w>, QueryEntityError> {
        if self.matched_entities_cache.contains(&entity) {
            let last_run = world.last_change_tick();
            let this_run = world.read_change_tick();
            let mut fetch = unsafe {
                Q::init_fetch(world.as_unsafe_world_cell_readonly(), &self.state, last_run, this_run)
            };
            Ok(<Q::Fetch<'w> as Fetch<'w>>::fetch(&mut fetch, entity, entity.index() as usize))
        } else {
            Err(QueryEntityError::NoSuchEntity(entity))
        }
    }
    
    pub fn get_mut<'w>(&mut self, world: &'w mut World, entity: Entity) -> Result<Q::Item<'w>, QueryEntityError> {
        if self.matched_entities_cache.contains(&entity) {
            let last_run = world.last_change_tick();
            let this_run = world.read_change_tick();
            let mut fetch = unsafe {
                Q::init_fetch(world.as_unsafe_world_cell(), &self.state, last_run, this_run)
            };
            Ok(<Q::Fetch<'w> as Fetch<'w>>::fetch(&mut fetch, entity, entity.index() as usize))
        } else {
            Err(QueryEntityError::NoSuchEntity(entity))
        }
    }
    
    pub fn iter<'w>(&'w self, world: &'w World) -> QueryStateIter<'w, Q, F> {
        let last_run = world.last_change_tick();
        let this_run = world.read_change_tick();
        let fetch = unsafe {
            Q::init_fetch(world.as_unsafe_world_cell_readonly(), &self.state, last_run, this_run)
        };
        QueryStateIter {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
            index: 0,
            fetch,
            state: &self.state,
        }
    }
    
    pub fn iter_mut<'w>(&'w mut self, world: &'w mut World) -> QueryStateIterMut<'w, Q, F> {
        let last_run = world.last_change_tick();
        let this_run = world.read_change_tick();
        let fetch = unsafe {
            Q::init_fetch(world.as_unsafe_world_cell(), &self.state, last_run, this_run)
        };
        QueryStateIterMut {
            _phantom: PhantomData,
            entities: &self.matched_entities_cache,
            index: 0,
            fetch,
            state: &self.state,
        }
    }
    
    pub fn single<'w>(&'w self, world: &'w World) -> Result<Q::Item<'w>, QuerySingleError> {
        let mut iter = self.iter(world);
        let first = iter.next().ok_or(QuerySingleError::NoEntities("No entities match query"))?;
        if iter.next().is_some() { return Err(QuerySingleError::MultipleEntities("Multiple entities match query")); }
        Ok(first)
    }
    
    pub fn single_mut<'w>(&'w mut self, world: &'w mut World) -> Result<Q::Item<'w>, QuerySingleError> {
        let mut iter = self.iter_mut(world);
        let first = iter.next().ok_or(QuerySingleError::NoEntities("No entities match query"))?;
        if iter.next().is_some() { return Err(QuerySingleError::MultipleEntities("Multiple entities match query")); }
        Ok(first)
    }
    
    pub fn is_empty(&self, _world: &World) -> bool { unsafe { query_state_is_empty(self.inner) } }
    pub fn matched_entity_count(&self) -> usize { unsafe { query_state_matched_entity_count(self.inner) as usize } }
    pub fn contains(&self, entity: Entity) -> bool { self.matched_entities_cache.contains(&entity) }
    pub fn update_archetypes(&mut self, _world: &World) { unsafe { query_state_update_archetypes(self.inner) }; }
}

impl<Q: QueryData, F: QueryFilter> Drop for QueryState<Q, F> {
    fn drop(&mut self) { unsafe { query_state_destroy(self.inner) }; }
}

pub struct QueryStateIter<'w, Q: QueryData, F: QueryFilter> {
    pub(crate) entities: &'w [Entity],
    pub(crate) index: usize,
    pub(crate) fetch: Q::Fetch<'w>,
    pub(crate) state: &'w Q::State,
    pub(crate) _phantom: PhantomData<(Q, F)>,
}

impl<'w, Q: QueryData, F: QueryFilter> Iterator for QueryStateIter<'w, Q, F> {
    type Item = Q::Item<'w>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entities.len() {
            let entity = self.entities[self.index];
            self.index += 1;
            Some(<Q::Fetch<'w> as Fetch<'w>>::fetch(&mut self.fetch, entity, entity.index() as usize))
        } else { None }
    }
}

pub struct QueryStateIterMut<'w, Q: QueryData, F: QueryFilter> {
    pub(crate) entities: &'w [Entity],
    pub(crate) index: usize,
    pub(crate) fetch: Q::Fetch<'w>,
    pub(crate) state: &'w Q::State,
    pub(crate) _phantom: PhantomData<(Q, F)>,
}

impl<'w, Q: QueryData, F: QueryFilter> Iterator for QueryStateIterMut<'w, Q, F> {
    type Item = Q::Item<'w>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.entities.len() {
            let entity = self.entities[self.index];
            self.index += 1;
            Some(<Q::Fetch<'w> as Fetch<'w>>::fetch(&mut self.fetch, entity, entity.index() as usize))
        } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::Component;
    #[derive(Debug, Clone, Copy, PartialEq)] struct Position { x: f32, y: f32 }
    impl Component for Position {}
    #[test]
    fn test_query_state_basics() {
        let world = World::new();
        let state = QueryState::<(), ()>::new(&world);
        assert!(state.is_empty(&world));
    }
}