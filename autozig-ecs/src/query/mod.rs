//! Query system - 泛型查询 system，支持类型安全的组件访问
//!
//! 实现类似 Bevy 的 Query<(&Transform, &mut Velocity), With<Player>> 语法

use crate::{world::World, component::Component};
use std::marker::PhantomData;

// ============================================================================
// Module Declarations - 模块声明
// ============================================================================

/// Access control and conflict detection
pub mod access;

/// Query error types
pub mod error;

/// WorldQuery trait and implementations
pub mod world_query;

/// Query builder for dynamic query construction
pub mod builder;

/// Data fetching system
pub mod fetch;

/// Query filters
pub mod filter;

/// Query iterators
pub mod iter;

/// Parallel query iteration
pub mod par_iter;

/// Query state management (core module with ~160+ APIs)
pub mod state;

// Re-export commonly used types
pub use self::error::{QueryEntityError, QuerySingleError, QueryComponentError, QueryBuildError, QueryIterError, QueryError};
pub use self::world_query::{QueryData, ReadOnlyQueryData, WorldQuery, ReadOnlyWorldQuery, OptionFetch};
pub use self::state::QueryState;
pub use self::state::QueryStateInner;
pub use self::builder::QueryBuilder;
pub use self::fetch::{EntityFetch, ReadFetch, WriteFetch, Fetch};
pub use self::filter::{With, Without, Or, Added, Changed, QueryFilter};
// Use iterators from state module
pub use self::state::{
    QueryStateIter as QueryIter, 
    QueryStateIterMut as QueryIterMut,
};
pub use self::par_iter::BatchingStrategy;
pub use self::access::{Access, FilteredAccess};

// ============================================================================
// Query - 主查询结构
// ============================================================================

/// Query<Q, F> - 类型安全的 ECS 查询
pub struct Query<'w, Q: QueryData, F: QueryFilter = ()> {
    world: &'w World,
    state: *const QueryState<Q, F>,
    _marker: PhantomData<Q>,
    _filter: PhantomData<F>,
}

/// QueryMut - 可变查询（需要可变World引用）
pub struct QueryMut<'w, Q: QueryData, F: QueryFilter = ()> {
    world: &'w mut World,
    state: *mut QueryState<Q, F>,
}

impl<'w, Q: QueryData, F: QueryFilter> Query<'w, Q, F> {
    /// Create a new query
    pub unsafe fn new(world: &'w World, state: *const QueryState<Q, F>) -> Self {
        Self {
            world,
            state,
            _marker: PhantomData,
            _filter: PhantomData,
        }
    }

    pub fn state(&self) -> &QueryState<Q, F> {
        unsafe { &*self.state }
    }

    /// Iterate over query results
    pub fn iter(&self) -> QueryIter<'w, Q, F> {
        unsafe { (*self.state).iter(self.world) }
    }

    /// Get single entity matching query
    pub fn single(&self) -> Result<Q::Item<'w>, QuerySingleError> {
        let mut iter = self.iter();
        let first = iter.next().ok_or(QuerySingleError::NoEntities("No entities match query"))?;
        if iter.next().is_some() {
            return Err(QuerySingleError::MultipleEntities("Multiple entities match query"));
        }
        Ok(first)
    }

    /// Returns true if the query is empty
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }

    /// Get component data for a specific entity
    pub fn get(&self, entity: crate::entity::Entity) -> Result<Q::Item<'w>, QueryEntityError> {
        unsafe { (*self.state).get::<Q>(self.world, entity) }
    }
    
    /// Get mutable component data for a specific entity
    pub fn get_mut(&mut self, entity: crate::entity::Entity) -> Result<Q::Item<'w>, QueryEntityError> {
        // Note: Check for mutable access in state?
        // QueryState::get handles it.
        unsafe { (*self.state).get::<Q>(self.world, entity) }
    }

    /// Iterate over combinations of K distinct entities
    pub fn iter_combinations<const K: usize>(&self) -> crate::query::state::QueryCombinationIter<'w, Q, F, K> {
        unsafe { (*self.state).iter_combinations(self.world) }
    }
}

impl<'w, Q: QueryData, F: QueryFilter> QueryMut<'w, Q, F> {
    /// Create a new mutable query
    pub unsafe fn new(world: &'w mut World, state: *mut QueryState<Q, F>) -> Self {
        Self {
            world,
            state,
        }
    }
    
    pub fn state(&self) -> &QueryState<Q, F> {
        unsafe { &*self.state }
    }
    
    pub fn state_mut(&mut self) -> &mut QueryState<Q, F> {
        unsafe { &mut *self.state }
    }

    /// Iterate mutably over query results
    pub fn iter_mut(&mut self) -> QueryIterMut<'w, Q, F> {
        let world_ptr = self.world as *mut World;
        unsafe { (*self.state).iter_mut(&mut *world_ptr) }
    }

    /// Get mutable component data for a specific entity
    pub fn get_mut(&mut self, entity: crate::entity::Entity) -> Result<Q::Item<'w>, QueryEntityError> {
        let world_ptr = self.world as *mut World;
        unsafe { (*self.state).get_mut::<Q>(&mut *world_ptr, entity) }
    }
}

unsafe impl<'w, Q: QueryData, F: QueryFilter> Send for Query<'w, Q, F> {}
unsafe impl<'w, Q: QueryData, F: QueryFilter> Sync for Query<'w, Q, F> {}

unsafe impl<'w, Q: QueryData, F: QueryFilter> Send for QueryMut<'w, Q, F> {}
unsafe impl<'w, Q: QueryData, F: QueryFilter> Sync for QueryMut<'w, Q, F> {}

/// Helper for Read and Write (legacy support if needed, but preferred to use &T and &mut T)
pub type Read<'a, T> = &'a T;
pub type Write<'a, T> = &'a mut T;

impl<'a, 'w, Q: QueryData, F: QueryFilter> IntoIterator for &'a Query<'w, Q, F> {
    type Item = Q::Item<'w>;
    type IntoIter = QueryIter<'w, Q, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, 'w, Q: QueryData, F: QueryFilter> IntoIterator for &'a mut Query<'w, Q, F> {
    type Item = Q::Item<'w>;
    type IntoIter = QueryIter<'w, Q, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, 'w, Q: QueryData, F: QueryFilter> IntoIterator for &'a mut QueryMut<'w, Q, F> {
    type Item = Q::Item<'w>;
    type IntoIter = QueryIterMut<'w, Q, F>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
