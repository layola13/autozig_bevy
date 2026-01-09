//! Query system - 泛型查询系统，支持类型安全的组件访问
//! 
//! 实现类似 Bevy 的 Query<(&Transform, &mut Velocity), With<Player>> 语法

use autozig::include_zig;
use crate::{component::Component, entity::Entity, world::World};
use std::marker::PhantomData;

// ============================================================================
// Core Traits - 核心 trait 定义
// ============================================================================

/// QueryData trait - 定义可以从 Query 中获取的数据类型
/// 
/// 实现此 trait 的类型可以在 Query 中使用
pub trait QueryData: Send + Sync + 'static {}

/// 只读 QueryData 的标记 trait
pub trait ReadOnlyQueryData: QueryData {}

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

impl<T: Component> QueryData for Read<T> {}
impl<T: Component> ReadOnlyQueryData for Read<T> {}

/// Write - 可变组件访问包装器
#[derive(Debug, Clone, Copy)]
pub struct Write<T: Component>(PhantomData<T>);

impl<T: Component> QueryData for Write<T> {}

// ============================================================================
// Tuple Implementations - 元组实现（支持多组件查询）
// ============================================================================

// 空元组实现
impl QueryData for () {}
impl ReadOnlyQueryData for () {}
impl QueryFilter for () {}

// 2元组实现
impl<A: QueryData, B: QueryData> QueryData for (A, B) {}
impl<A: ReadOnlyQueryData, B: ReadOnlyQueryData> ReadOnlyQueryData for (A, B) {}
impl<A: QueryFilter, B: QueryFilter> QueryFilter for (A, B) {}

// 3元组实现
impl<A: QueryData, B: QueryData, C: QueryData> QueryData for (A, B, C) {}
impl<A: ReadOnlyQueryData, B: ReadOnlyQueryData, C: ReadOnlyQueryData> ReadOnlyQueryData for (A, B, C) {}
impl<A: QueryFilter, B: QueryFilter, C: QueryFilter> QueryFilter for (A, B, C) {}

// 4元组实现
impl<A: QueryData, B: QueryData, C: QueryData, D: QueryData> QueryData for (A, B, C, D) {}
impl<A: ReadOnlyQueryData, B: ReadOnlyQueryData, C: ReadOnlyQueryData, D: ReadOnlyQueryData> 
    ReadOnlyQueryData for (A, B, C, D) {}
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

// ============================================================================
// QueryState - 查询状态管理（使用 Zig 实现）
// ============================================================================

#[repr(C)]
pub struct QueryStateOpaque {
    _private: u8,
}

include_zig!("src/zig/query.zig", {
    fn query_state_create() -> *mut QueryStateOpaque;
    fn query_state_destroy(state: *mut QueryStateOpaque);
    fn query_state_add_entity(state: *mut QueryStateOpaque, entity_index: u32) -> bool;
    fn query_state_clear(state: *mut QueryStateOpaque);
    fn query_state_count(state: *const QueryStateOpaque) -> usize;
    fn query_state_get_entity(state: *const QueryStateOpaque, index: usize) -> u32;
});

/// QueryState - 内部查询状态管理
pub struct QueryState<Q: QueryData = (), F: QueryFilter = ()> {
    inner: *mut QueryStateOpaque,
    _phantom: PhantomData<(Q, F)>,
}

impl<Q: QueryData, F: QueryFilter> QueryState<Q, F> {
    pub fn new() -> Self {
        let inner = query_state_create();
        Self { 
            inner,
            _phantom: PhantomData,
        }
    }
    
    pub fn add_entity(&mut self, entity_index: u32) -> bool {
        query_state_add_entity(self.inner, entity_index)
    }
    
    pub fn clear(&mut self) {
        query_state_clear(self.inner);
    }
    
    pub fn count(&self) -> usize {
        query_state_count(self.inner)
    }
    
    fn get_entity(&self, index: usize) -> u32 {
        query_state_get_entity(self.inner, index)
    }
    
    pub fn iter(&self) -> QueryIter {
        QueryIter {
            state: self.inner,
            index: 0,
            len: self.count(),
        }
    }
}

impl<Q: QueryData, F: QueryFilter> Drop for QueryState<Q, F> {
    fn drop(&mut self) {
        query_state_destroy(self.inner);
    }
}

impl<Q: QueryData, F: QueryFilter> Default for QueryState<Q, F> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Query - 主查询结构
// ============================================================================

/// Query<Q, F> - 类型安全的 ECS 查询
/// 
/// # Type Parameters
/// - `Q: QueryData` - 查询的数据类型
/// - `F: QueryFilter` - 查询的过滤器，默认为 ()（无过滤）
/// 
/// # Examples
/// ```
/// // 查询所有 Transform 组件
/// fn system(query: Query<Read<Transform>>) {
///     for entity in query.iter() {
///         // ...
///     }
/// }
/// 
/// // 查询有 Player 组件的实体
/// fn system(query: Query<Read<Transform>, With<Player>>) {
///     for entity in query.iter() {
///         // ...
///     }
/// }
/// ```
pub struct Query<'w, Q: QueryData = (), F: QueryFilter = ()> {
    world: &'w World,
    state: QueryState<Q, F>,
    _phantom: PhantomData<F>,
}

impl<'w, Q: QueryData, F: QueryFilter> Query<'w, Q, F> {
    /// 创建新的 Query
    pub fn new(world: &'w World) -> Self {
        Self {
            world,
            state: QueryState::new(),
            _phantom: PhantomData,
        }
    }
    
    /// 获取查询迭代器
    pub fn iter(&self) -> QueryIter {
        self.state.iter()
    }
    
    /// 获取单个实体
    pub fn get(&self, entity: Entity) -> Result<Entity, QueryEntityError> {
        // 检查实体是否存在
        if !self.world.contains(entity) {
            return Err(QueryEntityError::NoSuchEntity(entity));
        }
        
        Ok(entity)
    }
    
    /// 获取查询匹配的实体数量
    pub fn len(&self) -> usize {
        self.state.count()
    }
    
    /// 检查查询是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ============================================================================
// Query Iterators - 查询迭代器
// ============================================================================

/// 查询迭代器
pub struct QueryIter {
    state: *const QueryStateOpaque,
    index: usize,
    len: usize,
}

impl Iterator for QueryIter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        let entity = query_state_get_entity(self.state, self.index);
        self.index += 1;
        if entity == 0xFFFFFFFF {
            None
        } else {
            Some(entity)
        }
    }
    
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

// ============================================================================
// Error Types - 错误类型
// ============================================================================

/// Query 错误类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryEntityError {
    /// 实体不存在
    NoSuchEntity(Entity),
    /// 实体不匹配查询条件
    QueryDoesNotMatch(Entity),
    /// 组件访问冲突
    AliasedMutability(Entity),
}

impl std::fmt::Display for QueryEntityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchEntity(e) => write!(f, "Entity {:?} does not exist", e),
            Self::QueryDoesNotMatch(e) => write!(f, "Entity {:?} does not match query", e),
            Self::AliasedMutability(e) => write!(f, "Aliased mutability for entity {:?}", e),
        }
    }
}

impl std::error::Error for QueryEntityError {}

// ============================================================================
// Tests - 测试
// ============================================================================

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
    
    #[test]
    fn test_query_state_creation() {
        let state: QueryState<Read<Position>, With<Player>> = QueryState::new();
        assert_eq!(state.count(), 0);
    }
    
    #[test]
    fn test_query_state_add_entity() {
        let mut state: QueryState<Read<Position>, ()> = QueryState::new();
        assert!(state.add_entity(0));
        assert!(state.add_entity(1));
        assert_eq!(state.count(), 2);
    }
    
    #[test]
    fn test_with_filter_creation() {
        let _filter: With<Position> = With::default();
        let _filter: Without<Velocity> = Without::default();
    }
    
    #[test]
    fn test_query_iter() {
        let mut state: QueryState<Read<Position>, ()> = QueryState::new();
        state.add_entity(0);
        state.add_entity(1);
        state.add_entity(2);
        
        let entities: Vec<u32> = state.iter().collect();
        assert_eq!(entities.len(), 3);
        assert_eq!(entities, vec![0, 1, 2]);
    }
}
