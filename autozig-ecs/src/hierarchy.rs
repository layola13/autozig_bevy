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
        let first = world.get::<Parent>(entity).map(|p| p.get());
        Self {
            current: first,
            world,
        }
    }
}

impl<'w> Iterator for AncestorIter<'w> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = self.world.get::<Parent>(current).map(|p| p.get());
        Some(current)
    }
}

/// DescendantIter - 后代迭代器（广度优先）
pub struct DescendantIter<'w> {
    queue: std::collections::VecDeque<Entity>,
    world: &'w crate::world::World,
}

impl<'w> DescendantIter<'w> {
    pub fn new(entity: Entity, world: &'w crate::world::World) -> Self {
        let mut queue = std::collections::VecDeque::new();
        // Add children of root
        if let Some(children) = world.get::<Children>(entity) {
             for child in &children.entities {
                 queue.push_back(*child);
             }
        }
        Self {
            queue,
            world,
        }
    }
}

impl<'w> Iterator for DescendantIter<'w> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.queue.pop_front()?;
        // Add children of current
        if let Some(children) = self.world.get::<Children>(current) {
            for child in &children.entities {
                self.queue.push_back(*child);
            }
        }
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
        let mut stack = Vec::new();
        // Add children of root (reverse order for stack to preserve order)
        if let Some(children) = world.get::<Children>(entity) {
             for child in children.entities.iter().rev() {
                 stack.push(*child);
             }
        }
        Self {
            stack,
            world,
        }
    }
}

impl<'w> Iterator for DescendantDepthFirstIter<'w> {
    type Item = Entity;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;
        // Add children of current (reverse)
        if let Some(children) = self.world.get::<Children>(current) {
            for child in children.entities.iter().rev() {
                self.stack.push(*child);
            }
        }
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
/// System to maintain Children component based on Parent component changes
pub fn hierarchy_maintenance_system(
    mut commands: crate::command::Commands,
    changed_parents: crate::query::Query<(crate::entity::Entity, &'static Parent), crate::query::filter::Changed<Parent>>,
    mut parents: crate::query::Query<&'static mut Children>,
) {
    // Handle changed/added Parent components
    let mut updates: Vec<(crate::entity::Entity, crate::entity::Entity)> = Vec::new();
    for (child, parent) in changed_parents.iter() {
        updates.push((child, parent.entity));
    }
    
    for (child, parent_entity) in updates {
        // Add child to new parent
        if let Ok(mut children) = parents.get_mut(parent_entity) {
            if !children.entities.contains(&child) {
                children.entities.push(child);
            }
        } else {
            // Parent doesn't have Children component, add it
            let mut children = Children::new();
            children.push(child);
            commands.entity(parent_entity).insert(children);
        }
    }
    
    // Handle removed Parent components
    // Note: We don't have easy access to the *old* parent entity here unless we track it or use Observers.
    // For full correctness, we need to know the old parent to remove the child.
    // Bevy uses `PreviousParent` logic or component hooks.
    // Without hooks/observers, this is tricky.
    // Fallback: This system only handles ADDITIONS. 
    // Removals require more complex tracking or observers.
    // Assuming P3 (Observers) is active, we should rely on that in the future.
    // For now, this system handles the `spawn().insert(Parent)` case for initialization.
}

use crate::command::EntityCommands;

pub trait BuildChildren {
    fn with_children(&mut self, spawn_children: impl FnOnce(&mut ChildBuilder) + Send + Sync + 'static) -> &mut Self;
    fn add_child(&mut self, child: Entity) -> &mut Self;
}

impl<'w> BuildChildren for EntityCommands<'w> {
    fn with_children(&mut self, spawn_children: impl FnOnce(&mut ChildBuilder) + Send + Sync + 'static) -> &mut Self {
        let parent = self.id();
        // Since we are in EntityCommands, we hold `queue` (resource queue).
        // We can't easily construct a ChildBuilder that shares this queue AND creates buffered commands?
        // ChildBuilder needs a `Commands` instance.
        // We don't have `Commands` instance here, only `queue` and `buffer` pointer.
        // Constructing `Commands` from raw pointer is unsafe/impossible (ownership).
        
        // Workaround: We queue a closure that creates a ChildBuilder from a fresh Commands.
        self.add(move |world: &mut crate::world::World| {
            let mut builder = ChildBuilder {
                world,
                parent,
            };
            spawn_children(&mut builder);
        });
        self
    }

    fn add_child(&mut self, child: Entity) -> &mut Self {
        let parent = self.id();
        self.add(move |world: &mut crate::world::World| {
             world.insert_bundle(child, Parent::new(parent));
             
             if let Some(mut children) = world.get_mut::<Children>(parent) {
                 children.push(child);
             } else {
                 let mut children = Children::new();
                 children.push(child);
                 world.insert_bundle(parent, children);
             }
        });
        self
    }
}

impl<'w> BuildChildren for crate::entity::EntityWorldMut<'w> {
    fn with_children(&mut self, spawn_children: impl FnOnce(&mut ChildBuilder) + Send + Sync + 'static) -> &mut Self {
        let parent = self.id();
        let world = self.world_mut();
        let mut builder = ChildBuilder {
            world,
            parent,
        };
        spawn_children(&mut builder);
        self
    }

    fn add_child(&mut self, child: Entity) -> &mut Self {
        let parent = self.id();
        let world = self.world_mut();
        world.insert_bundle(child, Parent::new(parent));
        if let Some(mut children) = world.get_mut::<Children>(parent) {
             children.push(child);
        } else {
             let mut children = Children::new();
             children.push(child);
             world.insert_bundle(parent, children);
        }
        self
    }
}

/// ChildBuilder - 子实体构建器
pub struct ChildBuilder<'a> {
    pub world: &'a mut crate::world::World,
    pub parent: Entity,
}

impl<'a> ChildBuilder<'a> {
    pub fn spawn(&mut self, bundle: impl crate::bundle::Bundle) -> crate::entity::EntityWorldMut<'_> {
        let entity = self.world.spawn(bundle).id();
        self.world.insert_bundle(entity, Parent::new(self.parent));
        
        if let Some(mut children) = self.world.get_mut::<Children>(self.parent) {
             children.push(entity);
        } else {
             let mut children = Children::new();
             children.push(entity);
             self.world.insert_bundle(self.parent, children);
        }
        
        self.world.entity_mut(entity)
    }
    
    pub fn spawn_empty(&mut self) -> crate::entity::EntityWorldMut<'_> {
        let entity = self.world.spawn_empty().id();
        self.world.insert_bundle(entity, Parent::new(self.parent));
        
        if let Some(mut children) = self.world.get_mut::<Children>(self.parent) {
             children.push(entity);
        } else {
             let mut children = Children::new();
             children.push(entity);
             self.world.insert_bundle(self.parent, children);
        }
        
        self.world.entity_mut(entity)
    }
    
    pub fn parent_entity(&self) -> Entity {
        self.parent
    }
}

pub struct HierarchyPlugin;
impl crate::plugin::Plugin for HierarchyPlugin {
    fn build(&self, app: &mut crate::plugin::App) {
        use crate::schedule::IntoSystemConfigs;
        use crate::into_system::IntoSystem;
        
        type HierarchySystemMarker = (
            (),
            crate::command::Commands<'static>,
            crate::query::Query<'static, (crate::entity::Entity, &'static Parent), crate::query::filter::Changed<Parent>>,
            crate::query::Query<'static, &'static mut Children>,
        );
        
        app.add_systems(
            crate::schedule::PostUpdate, 
            IntoSystem::<HierarchySystemMarker>::into_system(hierarchy_maintenance_system)
        );
    }
}