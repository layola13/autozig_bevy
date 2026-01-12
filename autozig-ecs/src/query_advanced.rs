//! Query Advanced API - 查询高级API

use crate::query::{Query, QueryFilter};
use crate::entity::Entity;
use crate::component::Component;
use std::marker::PhantomData;

// ============================================================================
// Query Fetch Types - 查询获取类型
// ============================================================================

/// AddedFetch - 新增组件获取
pub struct AddedFetch<T: Component> {
    _phantom: PhantomData<T>,
}

/// ChangedFetch - 变化组件获取
pub struct ChangedFetch<T: Component> {
    _phantom: PhantomData<T>,
}

/// OptionFetch - 可选组件获取
pub struct OptionFetch<T: Component> {
    _phantom: PhantomData<T>,
}

/// ClientFetch - 客户端获取
pub struct ClientFetch<T: Component> {
    _phantom: PhantomData<T>,
}

// ============================================================================
// Query Combinators - 查询组合器
// ============================================================================

/// AnyOf - 任意一个匹配
pub struct AnyOf<T>(PhantomData<T>);

impl<T> Default for AnyOf<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

// ============================================================================
// Query Builders and Iterators - 查询构建器和迭代器
// ============================================================================

/// QueryBuilder - 查询构建器
pub struct QueryBuilder<Q, F = ()> 
where
    Q: Component,
    F: QueryFilter,
{
    _phantom: PhantomData<(Q, F)>,
}

impl<Q, F> QueryBuilder<Q, F>
where
    Q: Component,
    F: QueryFilter,
{
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<Q, F> Default for QueryBuilder<Q, F>
where
    Q: Component,
    F: QueryFilter,
{
    fn default() -> Self {
        Self::new()
    }
}

/// QueryCombinationIter - 查询组合迭代器
pub struct QueryCombinationIter<'w, 's, Q: Component, F: QueryFilter, const K: usize> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
}

/// QueryManyIter - 查询多个迭代器
pub struct QueryManyIter<'w, 's, Q: Component, F: QueryFilter> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
}

/// QueryParIter - 查询并行迭代器
pub struct QueryParIter<'w, 's, Q: Component, F: QueryFilter> {
    _phantom: PhantomData<(&'w (), &'s (), Q, F)>,
}

// ============================================================================
// Access Control - 访问控制
// ============================================================================

/// Access - 访问跟踪
pub struct Access {
    reads: Vec<std::any::TypeId>,
    writes: Vec<std::any::TypeId>,
}

impl Access {
    pub fn new() -> Self {
        Self {
            reads: Vec::new(),
            writes: Vec::new(),
        }
    }
    
    pub fn add_read(&mut self, type_id: std::any::TypeId) {
        if !self.reads.contains(&type_id) {
            self.reads.push(type_id);
        }
    }
    
    pub fn add_write(&mut self, type_id: std::any::TypeId) {
        if !self.writes.contains(&type_id) {
            self.writes.push(type_id);
        }
    }
    
    pub fn is_compatible_with(&self, other: &Access) -> bool {
        // Write-write conflicts
        for write in &self.writes {
            if other.writes.contains(write) {
                return false;
            }
        }
        // Write-read conflicts
        for write in &self.writes {
            if other.reads.contains(write) {
                return false;
            }
        }
        for write in &other.writes {
            if self.reads.contains(write) {
                return false;
            }
        }
        true
    }
}

impl Default for Access {
    fn default() -> Self {
        Self::new()
    }
}

/// FilteredAccess - 过滤访问
pub struct FilteredAccess {
    access: Access,
    required: Vec<std::any::TypeId>,
    excluded: Vec<std::any::TypeId>,
}

impl FilteredAccess {
    pub fn new() -> Self {
        Self {
            access: Access::new(),
            required: Vec::new(),
            excluded: Vec::new(),
        }
    }
    
    pub fn add_required(&mut self, type_id: std::any::TypeId) {
        if !self.required.contains(&type_id) {
            self.required.push(type_id);
        }
    }
    
    pub fn add_excluded(&mut self, type_id: std::any::TypeId) {
        if !self.excluded.contains(&type_id) {
            self.excluded.push(type_id);
        }
    }
    
    pub fn access(&self) -> &Access {
        &self.access
    }
    
    pub fn access_mut(&mut self) -> &mut Access {
        &mut self.access
    }
}

impl Default for FilteredAccess {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Batching Strategy - 批处理策略
// ============================================================================

/// BatchingStrategy - 批处理策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BatchingStrategy {
    /// 固定大小批次
    FixedBatchSize(usize),
    /// 每个线程一个批次
    PerThread,
    /// 自适应批次大小
    Adaptive,
}

impl Default for BatchingStrategy {
    fn default() -> Self {
        Self::PerThread
    }
}

// ============================================================================
// Query Filters - 查询过滤器
// ============================================================================

/// DefaultQueryFilters - 默认查询过滤器
pub struct DefaultQueryFilters;

// ============================================================================
// Query Errors - 查询错误
// ============================================================================

/// QueryAccessError - 查询访问错误
#[derive(Debug, Clone)]
pub enum QueryAccessError {
    AccessConflict(String),
    ComponentNotFound(std::any::TypeId),
}

impl std::fmt::Display for QueryAccessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AccessConflict(msg) => write!(f, "Query access conflict: {}", msg),
            Self::ComponentNotFound(id) => write!(f, "Component {:?} not found", id),
        }
    }
}

impl std::error::Error for QueryAccessError {}

/// QuerySingleError - 查询单个实体错误
#[derive(Debug, Clone)]
pub enum QuerySingleError {
    NoEntities,
    MultipleEntities(usize),
}

impl std::fmt::Display for QuerySingleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoEntities => write!(f, "No entities match the query"),
            Self::MultipleEntities(count) => write!(f, "Expected single entity but found {}", count),
        }
    }
}

impl std::error::Error for QuerySingleError {}

/// TryFromFilteredError - 从过滤转换错误
#[derive(Debug, Clone)]
pub struct TryFromFilteredError {
    pub message: String,
}

impl std::fmt::Display for TryFromFilteredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Try from filtered error: {}", self.message)
    }
}

impl std::error::Error for TryFromFilteredError {}

// ============================================================================
// Query Traits - 查询trait
// ============================================================================

/// ArchetypeFilter - 原型过滤器trait
pub trait ArchetypeFilter {
    fn filter_archetype(&self, archetype_id: u32) -> bool;
}

/// ArchetypeQueryData - 原型查询数据trait
pub trait ArchetypeQueryData {
    type Item;
    fn fetch(&self, entity: Entity) -> Option<Self::Item>;
}

/// ReadOnlyQueryData - 只读查询数据trait
pub trait ReadOnlyQueryData: ArchetypeQueryData {}

/// WorldQuery - 世界查询trait
pub trait WorldQuery {
    type Item<'w>;
    type State;
    
    fn init_state() -> Self::State;
    fn fetch<'w>(state: &'w Self::State, entity: Entity) -> Option<Self::Item<'w>>;
}

/// FilterableIds - 可过滤ID trait
pub trait FilterableIds {
    fn filter_ids(&self, entities: &[Entity]) -> Vec<Entity>;
}

/// DebugCheckedUnwrap - 调试检查解包trait
pub trait DebugCheckedUnwrap {
    type Item;
    fn debug_checked_unwrap(self) -> Self::Item;
}

impl<T> DebugCheckedUnwrap for Option<T> {
    type Item = T;
    
    fn debug_checked_unwrap(self) -> Self::Item {
        match self {
            Some(value) => value,
            None => panic!("Called debug_checked_unwrap on None"),
        }
    }
}

impl<T, E: std::fmt::Debug> DebugCheckedUnwrap for Result<T, E> {
    type Item = T;
    
    fn debug_checked_unwrap(self) -> Self::Item {
        match self {
            Ok(value) => value,
            Err(err) => panic!("Called debug_checked_unwrap on Err: {:?}", err),
        }
    }
}