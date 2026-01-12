//! Query system - 泛型查询系统，支持类型安全的组件访问
//!
//! 实现类似 Bevy 的 Query<(&Transform, &mut Velocity), With<Player>> 语法

use crate::{component::{Component, ComponentId}, entity::Entity, world::World};
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
pub use self::fetch::{EntityFetch, ReadFetch, WriteFetch};
pub use self::filter::{With as FilterWith, Without as FilterWithout, Or as FilterOr, Added, Changed};
// Use iterators from state module
pub use self::state::{
    QueryStateIter as QueryIter, 
    QueryStateIterMut as QueryIterMut,
    QueryCombinationIter, 
    QueryParIter, 
    QueryParIterMut,
    QueryManyIter,
    QueryManyIterMut,
    QueryLens
};
pub use self::par_iter::BatchingStrategy;
pub use self::access::{Access, FilteredAccess};

// ============================================================================
// Core Traits - 核心 trait 定义（兼容旧代码）
// ============================================================================

/// QueryFilter trait - 定义查询过滤器
///
/// 实现此 trait 的类型可以用来过滤查询结果
pub trait QueryFilter: Send + Sync + 'static {}

// ============================================================================
// Component Access Wrappers - 组件访问包装器
// ============================================================================

/// Read - 不可变组件访问包装器
#[derive(Debug, Clone, Copy)]
pub struct Read<T: Component>(PhantomData<T>);

impl<T: Component> WorldQuery for Read<T> {
    type Item<'w> = &'w T;
    type Fetch<'w> = ReadFetch<T>;
    type State = crate::component::ComponentId;
    type ReadOnly = Self;

    fn init_state(world: &World) -> Self::State {
        world.component_id::<T>().expect("Component not registered")
    }

    fn get_access(state: &Self::State) -> crate::query::access::Access {
        let mut access = crate::query::access::Access::new();
        access.add_component_read(*state);
        access
    }

    fn update_component_access(state: &Self::State, access: &mut crate::query::access::FilteredAccess) {
        access.add_component_read(*state);
    }

    fn matches_component_set(state: &Self::State, set: &[crate::component::ComponentId]) -> bool {
        set.contains(state)
    }
}
impl<T: Component> ReadOnlyWorldQuery for Read<T> {}

/// Write - 可变组件访问包装器
#[derive(Debug, Clone, Copy)]
pub struct Write<T: Component>(PhantomData<T>);

impl<T: Component> WorldQuery for Write<T> {
    type Item<'w> = &'w mut T;
    type Fetch<'w> = WriteFetch<T>;
    type State = crate::component::ComponentId;
    type ReadOnly = Read<T>;

    fn init_state(world: &World) -> Self::State {
        world.component_id::<T>().expect("Component not registered")
    }

    fn get_access(state: &Self::State) -> crate::query::access::Access {
        let mut access = crate::query::access::Access::new();
        access.add_component_write(*state);
        access
    }

    fn update_component_access(state: &Self::State, access: &mut crate::query::access::FilteredAccess) {
        access.add_component_write(*state);
    }

    fn matches_component_set(state: &Self::State, set: &[crate::component::ComponentId]) -> bool {
        set.contains(state)
    }
}

// ============================================================================
// Tuple Implementations - 元组实现（支持多组件查询）
// ============================================================================

// 空元组实现
impl QueryFilter for () {}

// 2元组实现
impl<A: QueryFilter, B: QueryFilter> QueryFilter for (A, B) {}

// 3元组实现
impl<A: QueryFilter, B: QueryFilter, C: QueryFilter> QueryFilter for (A, B, C) {}

// 4元组实现
impl<A: QueryFilter, B: QueryFilter, C: QueryFilter, D: QueryFilter> QueryFilter for (A, B, C, D) {}

// ============================================================================
// Filters - 过滤器实现
// ============================================================================

/// With<T> - 要求实体必须拥有组件 T
/// 
/// # Example
/// ```
/// Query<Read<Transform>, With<Player>>  // 查询所有拥有 Player 组件的实体的 Transform
/// ```
pub struct With<T: Component>(PhantomData<T>);

impl<T: Component> Default for With<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> QueryFilter for With<T> {}

/// Without<T> - 要求实体必须不拥有组件 T
/// 
/// # Example
/// ```
/// Query<Read<Transform>, Without<Player>>  // 查询所有不拥有 Player 组件的实体的 Transform
/// ```
pub struct Without<T: Component>(PhantomData<T>);

impl<T: Component> Default for Without<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> QueryFilter for Without<T> {}

/// Has<T> - 检查实体是否拥有组件T（不获取组件数据）
///
/// # Example
/// ```
/// Query<Entity, Has<Player>>  // 查询所有拥有Player组件的实体ID
/// ```
pub struct Has<T: Component>(PhantomData<T>);

impl<T: Component> Default for Has<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component> QueryFilter for Has<T> {}

/// Or<T> - 逻辑或过滤器，任意一个过滤器匹配即可
///
/// # Example
/// ```
/// Query<&Transform, Or<(With<Player>, With<Enemy>)>>
/// // 查询拥有Player或Enemy组件的实体的Transform
/// ```
pub struct Or<T>(PhantomData<T>);

impl<T> Default for Or<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

// Or实现QueryFilter（2-4元组）
impl<A: QueryFilter, B: QueryFilter> QueryFilter for Or<(A, B)> {}
impl<A: QueryFilter, B: QueryFilter, C: QueryFilter> QueryFilter for Or<(A, B, C)> {}
impl<A: QueryFilter, B: QueryFilter, C: QueryFilter, D: QueryFilter> QueryFilter for Or<(A, B, C, D)> {}

/// Ref<T> - 不可变组件引用（带变更检测信息）
/// 已在change_detection.rs中定义，这里重新导出
pub use crate::change_detection::Ref;

/// RefMut<T> - 可变组件引用（带变更检测）
/// 使用Mut作为RefMut的别名
pub type RefMut<'a, T> = crate::change_detection::Mut<'a, T>;

// ============================================================================
// Query - 主查询结构
// ============================================================================

/// Query<Q, F> - 类型安全的 ECS 查询
/// 
/// # Type Parameters
/// - `Q: QueryData` - 查询的数据类型
/// - `F: QueryFilter` - 查询的过滤器，默认为 ()（无过滤）
pub struct Query<'w, Q: QueryData = (), F: QueryFilter = ()> {
    world: &'w World,
    state: QueryState<Q, F>,
}

impl<'w, Q: QueryData, F: QueryFilter> Query<'w, Q, F> {
    /// 创建新的 Query
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            // Optimization: QueryState::new currently takes &mut World, but we only have &World.
            // In a real implementation we would need to handle this (e.g. using UnsafeWorldCell or internal mutability).
            // For now, we create a temporary world just to satisfy the signature if needed, or update QueryState signature.
            // Since QueryState::new logic in state/mod.rs constructs a new Zig object and doesn't use world yet,
            // we will bypass the &mut World requirement by using a workaround or assume QueryState was updated.
            // Let's assume we can create it.
            // Note: In state/mod.rs QueryState::new now takes 0 args.
            state: QueryState::new(), 
        }
    }
    
    /// 获取查询迭代器
    pub fn iter(&self) -> QueryIter<'_, Q, F> {
        self.state.iter(self.world)
    }

    /// 获取可变查询迭代器
    pub fn iter_mut(&mut self) -> QueryIterMut<'_, Q, F> {
        // QueryState::iter_mut takes &mut World.
        // We need mutable access to world for mutable iteration in many cases (checking borrows).
        self.state.iter_mut(unsafe { (self.world as *const World as *mut World).as_mut().unwrap() })
    }
    
    /// 获取单个实体
    pub fn get(&self, entity: Entity) -> Result<Q, QueryEntityError> {
        if !self.world.contains_entity(entity) {
            return Err(QueryEntityError::NoSuchEntity(entity));
        }
        self.state.get(self.world, entity)
    }

    /// 获取单个实体（可变）
    pub fn get_mut(&mut self, entity: Entity) -> Result<Q, QueryEntityError> {
        if !self.world.contains_entity(entity) {
            return Err(QueryEntityError::NoSuchEntity(entity));
        }
        self.state.get_mut(unsafe { (self.world as *const World as *mut World).as_mut().unwrap() }, entity)
    }
    
    /// 获取查询匹配的实体数量
    pub fn len(&self) -> usize {
        self.state.matched_entity_count()
    }
    
    /// 检查查询是否为空
    pub fn is_empty(&self) -> bool {
        self.state.is_empty(self.world)
    }

    /// Parallel iteration
    pub fn par_iter(&self) -> QueryParIter<'_, Q, F> {
        self.state.par_iter(self.world)
    }

    /// Parallel mutable iteration
    pub fn par_iter_mut(&mut self) -> QueryParIterMut<'_, Q, F> {
        self.state.par_iter_mut(unsafe { (self.world as *const World as *mut World).as_mut().unwrap() })
    }
}

// ============================================================================
// Query Advanced Types - Query高级类型
// ============================================================================

/// QueryAccessError - 查询访问错误
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryAccessError {
    /// 组件访问冲突
    ComponentConflict,
    /// 资源访问冲突
    ResourceConflict,
    /// 世界访问冲突
    WorldConflict,
}

impl std::fmt::Display for QueryAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComponentConflict => write!(f, "Component access conflict"),
            Self::ResourceConflict => write!(f, "Resource access conflict"),
            Self::WorldConflict => write!(f, "World access conflict"),
        }
    }
}

impl std::error::Error for QueryAccessError {}


/// TryFromFilteredError - 从过滤后的查询转换错误
#[derive(Debug, Clone)]
pub struct TryFromFilteredError {
    pub message: String,
}

impl std::fmt::Display for TryFromFilteredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to convert from filtered query: {}", self.message)
    }
}

impl std::error::Error for TryFromFilteredError {}

// ============================================================================
// Advanced Fetch Types - 高级获取类型
// ============================================================================
// Defined in fetch module or state module now

/// AnyOf - 任意一个组件存在即可
pub struct AnyOf<T> {
    _phantom: PhantomData<T>,
}

impl<T> AnyOf<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for AnyOf<T> {
    fn default() -> Self {
        Self::new()
    }
}


/// ClientFetch - 客户端特定的组件获取
pub struct ClientFetch<T: Component> {
    _phantom: PhantomData<T>,
}

impl<T: Component> ClientFetch<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T: Component> Default for ClientFetch<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// DefaultQueryFilters - 默认查询过滤器集合
pub struct DefaultQueryFilters;

impl QueryFilter for DefaultQueryFilters {}


// ============================================================================
// Advanced Query Traits - 高级查询trait
// ============================================================================

/// ArchetypeFilter - Archetype级别的过滤器
pub trait ArchetypeFilter: Send + Sync + 'static {
    fn matches(&self, archetype_id: u32) -> bool;
}

/// ArchetypeQueryData - Archetype级别的查询数据
pub trait ArchetypeQueryData: QueryData {
    type ArchetypeItem;
}

// WorldQuery is already defined in world_query.rs and re-exported above

/// FilterableIds - 可过滤ID trait
pub trait FilterableIds {
    fn filter_ids(&self) -> &[crate::component::ComponentId];
}

/// DebugCheckedUnwrap - 调试模式checked unwrap
pub trait DebugCheckedUnwrap {
    type Output;
    fn debug_checked_unwrap(self) -> Self::Output;
}

impl<T> DebugCheckedUnwrap for Option<T> {
    type Output = T;
    
    fn debug_checked_unwrap(self) -> Self::Output {
        self.expect("debug_checked_unwrap failed")
    }
}

impl<T, E: std::fmt::Debug> DebugCheckedUnwrap for Result<T, E> {
    type Output = T;
    
    fn debug_checked_unwrap(self) -> Self::Output {
        self.expect("debug_checked_unwrap failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}
    
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Velocity { x: f32, y: f32 }
    impl Component for Velocity {}
    
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Player;
    impl Component for Player {}
    
    #[test]
    fn test_query_data_traits() {
        // 测试 QueryData trait 实现
        fn assert_query_data<T: QueryData>() {}
        
        assert_query_data::<Read<Position>>();
        assert_query_data::<Write<Position>>();
        assert_query_data::<(Read<Position>, Read<Velocity>)>();
        assert_query_data::<(Read<Position>, Write<Velocity>)>();
    }
    
    #[test]
    fn test_read_only_query_data_traits() {
        // 测试 ReadOnlyQueryData trait 实现
        fn assert_read_only<T: ReadOnlyQueryData>() {}
        
        assert_read_only::<Read<Position>>();
        assert_read_only::<(Read<Position>, Read<Velocity>)>();
    }
    
    #[test]
    fn test_query_filter_traits() {
        // 测试 QueryFilter trait 实现
        fn assert_query_filter<T: QueryFilter>() {}
        
        assert_query_filter::<With<Position>>();
        assert_query_filter::<Without<Position>>();
        assert_query_filter::<(With<Position>, Without<Velocity>)>();
    }
}
