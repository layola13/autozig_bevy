//! Hierarchy and Relationship System - 层次结构和关系系统

use crate::entity::Entity;
use crate::component::Component;
use std::marker::PhantomData;

// ============================================================================
// Relationship Types - 关系类型
// ============================================================================

/// ChildOf<T> - 子关系组件
pub struct ChildOf<T> {
    pub parent: Entity,
    _phantom: PhantomData<T>,
}

impl<T> ChildOf<T> {
    pub fn new(parent: Entity) -> Self {
        Self {
            parent,
            _phantom: PhantomData,
        }
    }
    
    pub fn parent(&self) -> Entity {
        self.parent
    }
}

impl<T: Send + Sync + 'static> Component for ChildOf<T> {}

/// Children - 子实体列表组件
#[derive(Clone)]
pub struct Children {
    pub entities: Vec<Entity>,
}

impl Children {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }
    
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entities: Vec::with_capacity(capacity),
        }
    }
    
    pub fn push(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
    
    pub fn remove(&mut self, entity: Entity) -> bool {
        if let Some(pos) = self.entities.iter().position(|&e| e == entity) {
            self.entities.remove(pos);
            true
        } else {
            false
        }
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.entities.iter()
    }
    
    pub fn len(&self) -> usize {
        self.entities.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Children {}

/// Parent - 父实体引用组件
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parent {
    pub entity: Entity,
}

impl Parent {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
    
    pub fn get(&self) -> Entity {
        self.entity
    }
}

impl Component for Parent {}

// ============================================================================
// Hierarchy Iterators - 层次结构迭代器
// ============================================================================

/// AncestorIter - 祖先迭代器
pub struct AncestorIter<'w> {
    current: Option<Entity>,
    world: &'w crate::world::World,
}

impl<'w> AncestorIter<'w> {
    pub fn new(entity: Entity, world: &'w crate::world::World) -> Self {
        Self {
            current: Some(entity),
            world,
        }
    }
}

impl<'w> Iterator for AncestorIter<'w> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        // 简化实现：实际需要查询Parent组件
        self.current = None;
        Some(current)
    }
}

/// DescendantIter - 后代迭代器（广度优先）
pub struct DescendantIter<'w> {
    queue: Vec<Entity>,
    world: &'w crate::world::World,
}

impl<'w> DescendantIter<'w> {
    pub fn new(entity: Entity, world: &'w crate::world::World) -> Self {
        Self {
            queue: vec![entity],
            world,
        }
    }
}

impl<'w> Iterator for DescendantIter<'w> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.queue.pop()?;
        // 简化实现：实际需要查询Children组件并加入队列
        Some(current)
    }
}

/// DescendantDepthFirstIter - 后代迭代器（深度优先）
pub struct DescendantDepthFirstIter<'w> {
    stack: Vec<Entity>,
    world: &'w crate::world::World,
}

impl<'w> DescendantDepthFirstIter<'w> {
    pub fn new(entity: Entity, world: &'w crate::world::World) -> Self {
        Self {
            stack: vec![entity],
            world,
        }
    }
}

impl<'w> Iterator for DescendantDepthFirstIter<'w> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;
        // 简化实现：实际需要查询Children组件并加入栈
        Some(current)
    }
}

// ============================================================================
// Relationship Enums - 关系枚举
// ============================================================================

/// RelationshipAccessor - 关系访问器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationshipAccessor {
    Source,
    Target,
    Both,
}

/// RelationshipHookMode - 关系钩子模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationshipHookMode {
    OnAdd,
    OnRemove,
    OnReplace,
}

// ============================================================================
// Relationship Traits - 关系trait
// ============================================================================

/// Relationship - 关系trait
pub trait Relationship: Component {
    type Source;
    type Target;
    
    fn source(&self) -> Entity;
    fn target(&self) -> Entity;
}

/// RelationshipSourceCollection - 关系源集合trait
pub trait RelationshipSourceCollection {
    type Source;
    fn sources(&self) -> Vec<Entity>;
}

/// RelationshipTarget - 关系目标trait
pub trait RelationshipTarget {
    fn target(&self) -> Entity;
}

/// RelationshipCloneBehaviorBase - 关系克隆行为基础trait
pub trait RelationshipCloneBehaviorBase {
    fn clone_relationship(&self, source: Entity, target: Entity) -> Self;
}

/// RelationshipCloneBehaviorViaClone - 通过Clone的关系克隆行为
pub trait RelationshipCloneBehaviorViaClone: Clone + Relationship {
    fn clone_via_clone(&self) -> Self {
        self.clone()
    }
}

/// RelationshipCloneBehaviorViaReflect - 通过Reflect的关系克隆行为
pub trait RelationshipCloneBehaviorViaReflect: Relationship {
    fn clone_via_reflect(&self) -> Self;
}

/// RelationshipTargetCloneBehaviorHierarchy - 层次结构目标克隆行为
pub trait RelationshipTargetCloneBehaviorHierarchy: RelationshipTarget {
    fn clone_hierarchy(&self, new_target: Entity) -> Self;
}

/// RelationshipTargetCloneBehaviorViaClone - 通过Clone的目标克隆行为
pub trait RelationshipTargetCloneBehaviorViaClone: Clone + RelationshipTarget {
    fn clone_target_via_clone(&self) -> Self {
        self.clone()
    }
}

/// RelationshipTargetCloneBehaviorViaReflect - 通过Reflect的目标克隆行为
pub trait RelationshipTargetCloneBehaviorViaReflect: RelationshipTarget {
    fn clone_target_via_reflect(&self) -> Self;
}

/// OrderedRelationshipSourceCollection - 有序关系源集合trait
pub trait OrderedRelationshipSourceCollection: RelationshipSourceCollection {
    fn ordered_sources(&self) -> Vec<Entity>;
}

/// SpawnRelated - 生成关联实体trait
pub trait SpawnRelated {
    fn spawn_related(&mut self, source: Entity, target: Entity);
}

/// SpawnableList - 可生成列表trait
pub trait SpawnableList {
    type Item;
    fn spawn_list(&mut self, items: Vec<Self::Item>);
}