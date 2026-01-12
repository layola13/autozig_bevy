//! Entity Advanced API - 实体高级API

use crate::entity::Entity;
use crate::component::Component;
use std::collections::HashMap;
use std::marker::PhantomData;

// ============================================================================
// Access Control - 访问控制
// ============================================================================

/// AccessConflictError - 访问冲突错误
#[derive(Debug, Clone)]
pub struct AccessConflictError {
    pub message: String,
}

impl AccessConflictError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for AccessConflictError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Access conflict: {}", self.message)
    }
}

impl std::error::Error for AccessConflictError {}

/// ComponentAccessKind - 组件访问类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentAccessKind {
    Read,
    Write,
}

/// EcsAccessLevel - ECS访问级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EcsAccessLevel {
    None,
    Read,
    Write,
    Exclusive,
}

/// EcsAccessType - ECS访问类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcsAccessType {
    Component,
    Resource,
    World,
}

// ============================================================================
// Entity Allocation - 实体分配
// ============================================================================

/// EntityAllocator - 实体分配器
pub struct EntityAllocator {
    next_index: u32,
    generation: u32,
}

impl EntityAllocator {
    pub fn new() -> Self {
        Self {
            next_index: 0,
            generation: 1,
        }
    }
    
    pub fn allocate(&mut self) -> Entity {
        let index = self.next_index;
        self.next_index += 1;
        Entity::new(index, self.generation)
    }
    
    pub fn reserve(&mut self, count: usize) -> Vec<Entity> {
        (0..count).map(|_| self.allocate()).collect()
    }
}

impl Default for EntityAllocator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Entity Hashing - 实体哈希
// ============================================================================

/// EntityHasher - 实体哈希器
pub struct EntityHasher {
    state: u64,
}

impl EntityHasher {
    pub fn new() -> Self {
        Self { state: 0 }
    }
    
    pub fn write_entity(&mut self, entity: Entity) {
        self.state ^= entity.index() as u64;
        self.state = self.state.wrapping_mul(0x9e3779b97f4a7c15);
        self.state ^= entity.generation() as u64;
    }
    
    pub fn finish(&self) -> u64 {
        self.state
    }
}

impl Default for EntityHasher {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Entity Cloning - 实体克隆
// ============================================================================

/// EntityClonerBuilder - 实体克隆器构建器
pub struct EntityClonerBuilder {
    clone_components: bool,
    filter: Option<Box<dyn Fn(&Entity) -> bool + Send + Sync>>,
}

impl EntityClonerBuilder {
    pub fn new() -> Self {
        Self {
            clone_components: true,
            filter: None,
        }
    }
    
    pub fn with_components(mut self, clone: bool) -> Self {
        self.clone_components = clone;
        self
    }
    
    pub fn with_filter<F>(mut self, filter: F) -> Self
    where
        F: Fn(&Entity) -> bool + Send + Sync + 'static,
    {
        self.filter = Some(Box::new(filter));
        self
    }
    
    pub fn should_clone(&self, entity: &Entity) -> bool {
        if let Some(filter) = &self.filter {
            filter(entity)
        } else {
            true
        }
    }
}

impl Default for EntityClonerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// EntityCloneCtx - 实体克隆上下文
pub struct EntityCloneCtx {
    source_to_clone: HashMap<Entity, Entity>,
}

impl EntityCloneCtx {
    pub fn new() -> Self {
        Self {
            source_to_clone: HashMap::new(),
        }
    }
    
    pub fn map_entity(&mut self, source: Entity, clone: Entity) {
        self.source_to_clone.insert(source, clone);
    }
    
    pub fn get_clone(&self, source: Entity) -> Option<Entity> {
        self.source_to_clone.get(&source).copied()
    }
}

impl Default for EntityCloneCtx {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Component Clone Behavior - 组件克隆行为
// ============================================================================

/// ComponentCloneBehavior - 组件克隆行为
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentCloneBehavior {
    /// 通过Clone trait克隆
    Clone,
    /// 通过Reflect克隆
    Reflect,
    /// 不克隆
    Ignore,
}

/// ComponentCloneHandler - 组件克隆处理器
pub struct ComponentCloneHandler {
    behavior: ComponentCloneBehavior,
}

impl ComponentCloneHandler {
    pub fn new(behavior: ComponentCloneBehavior) -> Self {
        Self { behavior }
    }
    
    pub fn behavior(&self) -> ComponentCloneBehavior {
        self.behavior
    }
}

/// ComponentDropHandler - 组件销毁处理器
pub struct ComponentDropHandler {
    drop_fn: Box<dyn Fn() + Send + Sync>,
}

impl ComponentDropHandler {
    pub fn new<F>(drop_fn: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self {
            drop_fn: Box::new(drop_fn),
        }
    }
    
    pub fn drop(&self) {
        (self.drop_fn)();
    }
}

// ============================================================================
// Component Relationships - 组件关系
// ============================================================================

/// ComponentRelationshipAccessor - 组件关系访问器
pub struct ComponentRelationshipAccessor<T: Component> {
    _phantom: PhantomData<T>,
}

impl<T: Component> ComponentRelationshipAccessor<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T: Component> Default for ComponentRelationshipAccessor<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Required Components - 必需组件
// ============================================================================

/// RequiredComponents - 必需组件集合
pub struct RequiredComponents {
    components: Vec<std::any::TypeId>,
}

impl RequiredComponents {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    
    pub fn add<T: Component>(&mut self) {
        self.components.push(std::any::TypeId::of::<T>());
    }
    
    pub fn contains<T: Component>(&self) -> bool {
        self.components.contains(&std::any::TypeId::of::<T>())
    }
    
    pub fn len(&self) -> usize {
        self.components.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }
}

impl Default for RequiredComponents {
    fn default() -> Self {
        Self::new()
    }
}

/// RequiredComponentsError - 必需组件错误
#[derive(Debug, Clone)]
pub struct RequiredComponentsError {
    pub message: String,
}

impl RequiredComponentsError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for RequiredComponentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Required components error: {}", self.message)
    }
}

impl std::error::Error for RequiredComponentsError {}

// ============================================================================
// Entity Errors - 实体错误
// ============================================================================

/// EntityNotSpawnedError - 实体未生成错误
#[derive(Debug, Clone)]
pub struct EntityNotSpawnedError {
    pub entity: Entity,
}

impl EntityNotSpawnedError {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}

impl std::fmt::Display for EntityNotSpawnedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity {:?} was not spawned", self.entity)
    }
}

impl std::error::Error for EntityNotSpawnedError {}

/// EntityMutableFetchError - 实体可变获取错误
#[derive(Debug, Clone)]
pub enum EntityMutableFetchError {
    NotFound(Entity),
    AccessConflict(Entity),
}

impl std::fmt::Display for EntityMutableFetchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(entity) => write!(f, "Entity {:?} not found", entity),
            Self::AccessConflict(entity) => write!(f, "Access conflict for entity {:?}", entity),
        }
    }
}

impl std::error::Error for EntityMutableFetchError {}

// ============================================================================
// Component Entry - 组件入口
// ============================================================================

/// ComponentEntry - 组件入口（类似HashMap的Entry）
pub enum ComponentEntry<'a, T: Component> {
    Occupied(OccupiedEntry<'a, T>),
    Vacant(VacantEntry<'a, T>),
}

pub struct OccupiedEntry<'a, T: Component> {
    _phantom: PhantomData<&'a mut T>,
}

pub struct VacantEntry<'a, T: Component> {
    _phantom: PhantomData<&'a mut T>,
}

impl<'a, T: Component> ComponentEntry<'a, T> {
    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            Self::Occupied(_) => unimplemented!("OccupiedEntry::get_mut"),
            Self::Vacant(_) => unimplemented!("VacantEntry::insert"),
        }
    }
    
    pub fn or_insert_with<F: FnOnce() -> T>(self, default: F) -> &'a mut T {
        match self {
            Self::Occupied(_) => unimplemented!("OccupiedEntry::get_mut"),
            Self::Vacant(_) => {
                let _ = default();
                unimplemented!("VacantEntry::insert")
            }
        }
    }
}

// ============================================================================
// Entity Set Traits - 实体集合trait
// ============================================================================

/// ContainsEntity - 包含实体trait
pub trait ContainsEntity {
    fn contains(&self, entity: Entity) -> bool;
}

/// EntitySet - 实体集合trait
pub trait EntitySet: ContainsEntity {
    fn insert(&mut self, entity: Entity) -> bool;
    fn remove(&mut self, entity: Entity) -> bool;
    fn clear(&mut self);
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

/// FromEntitySetIterator - 从实体集合迭代器构建
pub trait FromEntitySetIterator<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;
}

// ============================================================================
// Component Mutability - 组件可变性
// ============================================================================

/// ComponentMutability - 组件可变性trait
pub trait ComponentMutability {
    const IS_MUTABLE: bool;
}

/// Immutable marker
pub struct Immutable;

/// Mutable marker  
pub struct Mutable;

impl ComponentMutability for Immutable {
    const IS_MUTABLE: bool = false;
}

impl ComponentMutability for Mutable {
    const IS_MUTABLE: bool = true;
}