//! Hierarchy management for autozig-bevy
//! 
//! This module provides parent-child relationship management for ECS entities.
//! Optimized for WebGPU/WASM platform with fixed-size arrays and no dynamic allocation.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use autozig::include_zig;

// Include Zig modules with function signatures
include_zig!("src/zig/parent.zig", {
    fn parent_create(parent_entity: u32) -> Parent;
    fn parent_get(parent: *const Parent) -> u32;
    fn parent_set(parent: *mut Parent, parent_entity: u32);
});

include_zig!("src/zig/children.zig", {
    fn children_create() -> Children;
    fn children_add(children: *mut Children, child_entity: u32) -> bool;
    fn children_remove(children: *mut Children, child_entity: u32) -> bool;
    fn children_contains(children: *const Children, child_entity: u32) -> bool;
    fn children_get_at(children: *const Children, index: u32) -> u32;
    fn children_get_count(children: *const Children) -> u32;
    fn children_clear(children: *mut Children);
    fn children_is_full(children: *const Children) -> bool;
});

include_zig!("src/zig/event.zig", {
    fn hierarchy_event_child_added(parent_entity: u32, child_entity: u32) -> HierarchyEvent;
    fn hierarchy_event_child_removed(parent_entity: u32, child_entity: u32) -> HierarchyEvent;
    fn hierarchy_event_parent_changed(new_parent_entity: u32, child_entity: u32) -> HierarchyEvent;
    fn hierarchy_event_get_type(event: *const HierarchyEvent) -> HierarchyEventType;
    fn hierarchy_event_get_parent(event: *const HierarchyEvent) -> u32;
    fn hierarchy_event_get_child(event: *const HierarchyEvent) -> u32;
});

include_zig!("src/zig/system.zig", {
    fn hierarchy_system_add_child(parent: *mut Children, child_id: u32, child_parent: *mut Parent) -> bool;
    fn hierarchy_system_remove_child(parent: *mut Children, child_id: u32, child_parent: *mut Parent) -> bool;
    fn hierarchy_system_set_parent(
        child_parent: *mut Parent,
        old_parent: *mut Children,
        new_parent: *mut Children,
        child_id: u32
    ) -> bool;
    fn hierarchy_system_clear_children(parent: *mut Children);
});

// ============================================================================
// Parent Component
// ============================================================================

/// Parent component - stores reference to parent entity
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parent {
    pub entity: u32,
}

impl Parent {
    /// Create a new Parent component
    pub fn new(parent_entity: u32) -> Self {
        parent_create(parent_entity)
    }

    /// Get the parent entity ID
    pub fn get(&self) -> u32 {
        parent_get(self)
    }

    /// Set the parent entity ID
    pub fn set(&mut self, parent_entity: u32) {
        parent_set(self, parent_entity)
    }
}

// ============================================================================
// Children Component
// ============================================================================

/// Children component - stores up to 64 child entity references
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Children {
    pub entities: [u32; 64],
    pub count: u32,
}

impl Children {
    /// Create a new empty Children component
    pub fn new() -> Self {
        children_create()
    }

    /// Add a child entity
    /// Returns true if added successfully, false if list is full
    pub fn add(&mut self, child_entity: u32) -> bool {
        children_add(self, child_entity)
    }

    /// Remove a child entity
    /// Returns true if removed successfully, false if not found
    pub fn remove(&mut self, child_entity: u32) -> bool {
        children_remove(self, child_entity)
    }

    /// Check if a child entity exists
    pub fn contains(&self, child_entity: u32) -> bool {
        children_contains(self, child_entity)
    }

    /// Get child entity at index
    /// Returns None if index is out of bounds
    pub fn get(&self, index: u32) -> Option<u32> {
        let entity = children_get_at(self, index);
        if entity == 0 && index >= self.count {
            None
        } else {
            Some(entity)
        }
    }

    /// Get the number of children
    pub fn len(&self) -> usize {
        children_get_count(self) as usize
    }

    /// Check if the children list is empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Clear all children
    pub fn clear(&mut self) {
        children_clear(self)
    }

    /// Check if the children list is full
    pub fn is_full(&self) -> bool {
        children_is_full(self)
    }

    /// Get an iterator over the children
    pub fn iter(&self) -> ChildrenIter {
        ChildrenIter {
            children: self,
            index: 0,
        }
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator over children
pub struct ChildrenIter<'a> {
    children: &'a Children,
    index: u32,
}

impl<'a> Iterator for ChildrenIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.children.count {
            let entity = self.children.entities[self.index as usize];
            self.index += 1;
            Some(entity)
        } else {
            None
        }
    }
}

// ============================================================================
// Hierarchy Event
// ============================================================================

/// Hierarchy event types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HierarchyEventType {
    ChildAdded = 0,
    ChildRemoved = 1,
    ParentChanged = 2,
}

/// Hierarchy event - represents changes in parent-child relationships
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HierarchyEvent {
    pub event_type: HierarchyEventType,
    pub parent_entity: u32,
    pub child_entity: u32,
}

impl HierarchyEvent {
    /// Create a ChildAdded event
    pub fn child_added(parent_entity: u32, child_entity: u32) -> Self {
        hierarchy_event_child_added(parent_entity, child_entity)
    }

    /// Create a ChildRemoved event
    pub fn child_removed(parent_entity: u32, child_entity: u32) -> Self {
        hierarchy_event_child_removed(parent_entity, child_entity)
    }

    /// Create a ParentChanged event
    pub fn parent_changed(new_parent_entity: u32, child_entity: u32) -> Self {
        hierarchy_event_parent_changed(new_parent_entity, child_entity)
    }
}

// ============================================================================
// Hierarchy System
// ============================================================================

/// Hierarchy system operations
pub struct HierarchySystem;

impl HierarchySystem {
    /// Add a child to a parent, maintaining bidirectional consistency
    pub fn add_child(parent: &mut Children, child_id: u32, child_parent: &mut Parent) -> bool {
        hierarchy_system_add_child(parent, child_id, child_parent)
    }

    /// Remove a child from a parent, maintaining bidirectional consistency
    pub fn remove_child(parent: &mut Children, child_id: u32, child_parent: &mut Parent) -> bool {
        hierarchy_system_remove_child(parent, child_id, child_parent)
    }

    /// Set a new parent for a child, updating both old and new parent
    pub fn set_parent(
        child_parent: &mut Parent,
        old_parent: Option<&mut Children>,
        new_parent: &mut Children,
        child_id: u32,
    ) -> bool {
        match old_parent {
            Some(old) => hierarchy_system_set_parent(child_parent, old, new_parent, child_id),
            None => {
                // No old parent, just add to new parent
                hierarchy_system_add_child(new_parent, child_id, child_parent)
            }
        }
    }

    /// Clear all children from a parent
    pub fn clear_children(parent: &mut Children) {
        hierarchy_system_clear_children(parent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent_basic() {
        let mut parent = Parent::new(42);
        assert_eq!(parent.get(), 42);
        
        parent.set(100);
        assert_eq!(parent.get(), 100);
    }

    #[test]
    fn test_children_basic() {
        let mut children = Children::new();
        assert_eq!(children.len(), 0);
        assert!(children.is_empty());
        
        assert!(children.add(1));
        assert_eq!(children.len(), 1);
        assert!(!children.is_empty());
        
        assert!(children.contains(1));
        assert!(!children.contains(2));
    }
}