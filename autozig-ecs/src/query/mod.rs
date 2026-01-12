//! Query system - 泛型查询系统，支持类型安全的组件访问
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
///
/// # Type Parameters
/// - `Q: QueryData` - 查询的数据类型
/// - `F: QueryFilter` - 查询的过滤器，默认为 ()（无过滤）
pub struct Query<'w, Q: QueryData, F: QueryFilter = ()> {
    world: &'w World,
    state: QueryState<Q, F>,
}

/// QueryMut - 可变查询（需要可变World引用）
pub struct QueryMut<'w, Q: QueryData, F: QueryFilter = ()> {
    world: &'w mut World,
    state: QueryState<Q, F>,
}

impl<'w, Q: QueryData, F: QueryFilter> Query<'w, Q, F> {
    /// Create a new query
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            state: QueryState::new(world),
        }
    }

    /// Iterate over query results
    pub fn iter(&self) -> QueryIter<'_, Q, F> {
        self.state.iter(self.world)
    }

    /// Get single entity matching query
    pub fn single(&self) -> Result<Q::Item<'_>, QuerySingleError> {
        let mut iter = self.iter();
        let first = iter.next().ok_or(QuerySingleError::NoEntities("No entities match query"))?;
        if iter.next().is_some() {
            return Err(QuerySingleError::MultipleEntities("Multiple entities match query"));
        }
        Ok(first)
    }

    /// Get component data for a specific entity
    pub fn get(&self, entity: crate::entity::Entity) -> Result<Q::Item<'_>, QueryEntityError> {
        self.state.get(self.world, entity)
    }
}

impl<'w, Q: QueryData, F: QueryFilter> QueryMut<'w, Q, F> {
    /// Create a new mutable query
    pub fn new(world: &'w mut World) -> Self {
        let state = QueryState::new(world);
        Self {
            world,
            state,
        }
    }

    /// Iterate mutably over query results
    pub fn iter_mut(&mut self) -> QueryIterMut<'_, Q, F> {
        self.state.iter_mut(self.world)
    }

    /// Get mutable component data for a specific entity
    pub fn get_mut(&mut self, entity: crate::entity::Entity) -> Result<Q::Item<'_>, QueryEntityError> {
        self.state.get_mut(self.world, entity)
    }
}

/// Helper for Read and Write (legacy support if needed, but preferred to use &T and &mut T)
pub type Read<'a, T> = &'a T;
pub type Write<'a, T> = &'a mut T;
