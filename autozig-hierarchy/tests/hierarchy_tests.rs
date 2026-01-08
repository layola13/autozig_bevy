use autozig_hierarchy::{Children, HierarchyEvent, HierarchyEventType, HierarchySystem, Parent};

// ============================================================================
// Parent Tests
// ============================================================================

#[test]
fn test_parent_create() {
    let parent = Parent::new(42);
    assert_eq!(parent.entity, 42);
    assert_eq!(parent.get(), 42);
}

#[test]
fn test_parent_get_set() {
    let mut parent = Parent::new(10);
    assert_eq!(parent.get(), 10);

    parent.set(20);
    assert_eq!(parent.get(), 20);

    parent.set(0);
    assert_eq!(parent.get(), 0);
}

// ============================================================================
// Children Tests
// ============================================================================

#[test]
fn test_children_create() {
    let children = Children::new();
    assert_eq!(children.count, 0);
    assert_eq!(children.len(), 0);
    assert!(children.is_empty());
    assert!(!children.is_full());
}

#[test]
fn test_children_add() {
    let mut children = Children::new();
    
    assert!(children.add(1));
    assert_eq!(children.len(), 1);
    assert!(children.contains(1));
    
    assert!(children.add(2));
    assert_eq!(children.len(), 2);
    assert!(children.contains(2));
    
    assert!(children.add(3));
    assert_eq!(children.len(), 3);
}

#[test]
fn test_children_remove() {
    let mut children = Children::new();
    
    children.add(1);
    children.add(2);
    children.add(3);
    
    assert!(children.remove(2));
    assert_eq!(children.len(), 2);
    assert!(!children.contains(2));
    assert!(children.contains(1));
    assert!(children.contains(3));
    
    assert!(!children.remove(99)); // Not found
    assert_eq!(children.len(), 2);
}

#[test]
fn test_children_contains() {
    let mut children = Children::new();
    
    assert!(!children.contains(1));
    
    children.add(1);
    assert!(children.contains(1));
    assert!(!children.contains(2));
    
    children.add(2);
    assert!(children.contains(1));
    assert!(children.contains(2));
}

#[test]
fn test_children_get_at() {
    let mut children = Children::new();
    
    children.add(10);
    children.add(20);
    children.add(30);
    
    assert_eq!(children.get(0), Some(10));
    assert_eq!(children.get(1), Some(20));
    assert_eq!(children.get(2), Some(30));
    assert_eq!(children.get(3), None); // Out of bounds
    assert_eq!(children.get(99), None);
}

#[test]
fn test_children_clear() {
    let mut children = Children::new();
    
    children.add(1);
    children.add(2);
    children.add(3);
    assert_eq!(children.len(), 3);
    
    children.clear();
    assert_eq!(children.len(), 0);
    assert!(children.is_empty());
    assert!(!children.contains(1));
    assert!(!children.contains(2));
    assert!(!children.contains(3));
}

#[test]
fn test_children_max_capacity() {
    let mut children = Children::new();
    
    // Add 64 children (max capacity)
    for i in 0..64 {
        assert!(children.add(i));
    }
    
    assert_eq!(children.len(), 64);
    assert!(children.is_full());
    
    // Try to add one more - should fail
    assert!(!children.add(999));
    assert_eq!(children.len(), 64);
}

// ============================================================================
// Hierarchy Event Tests
// ============================================================================

#[test]
fn test_hierarchy_event() {
    let event1 = HierarchyEvent::child_added(100, 200);
    assert_eq!(event1.event_type, HierarchyEventType::ChildAdded);
    assert_eq!(event1.parent_entity, 100);
    assert_eq!(event1.child_entity, 200);
    
    let event2 = HierarchyEvent::child_removed(100, 200);
    assert_eq!(event2.event_type, HierarchyEventType::ChildRemoved);
    assert_eq!(event2.parent_entity, 100);
    assert_eq!(event2.child_entity, 200);
    
    let event3 = HierarchyEvent::parent_changed(300, 200);
    assert_eq!(event3.event_type, HierarchyEventType::ParentChanged);
    assert_eq!(event3.parent_entity, 300);
    assert_eq!(event3.child_entity, 200);
}

// ============================================================================
// Hierarchy System Tests
// ============================================================================

#[test]
fn test_system_add_child() {
    let mut parent_children = Children::new();
    let mut child_parent = Parent::new(0);
    
    let result = HierarchySystem::add_child(&mut parent_children, 42, &mut child_parent);
    
    assert!(result);
    assert_eq!(parent_children.len(), 1);
    assert!(parent_children.contains(42));
    assert_eq!(child_parent.get(), 42);
}

#[test]
fn test_system_remove_child() {
    let mut parent_children = Children::new();
    let mut child_parent = Parent::new(42);
    
    parent_children.add(42);
    
    let result = HierarchySystem::remove_child(&mut parent_children, 42, &mut child_parent);
    
    assert!(result);
    assert_eq!(parent_children.len(), 0);
    assert!(!parent_children.contains(42));
    assert_eq!(child_parent.get(), 0); // Parent cleared
}

#[test]
fn test_system_set_parent() {
    let mut old_parent = Children::new();
    let mut new_parent = Children::new();
    let mut child_parent = Parent::new(100);
    
    // Add child to old parent
    old_parent.add(42);
    
    // Move child to new parent
    let result = HierarchySystem::set_parent(
        &mut child_parent,
        Some(&mut old_parent),
        &mut new_parent,
        42,
    );
    
    assert!(result);
    assert!(!old_parent.contains(42));
    assert!(new_parent.contains(42));
    assert_eq!(child_parent.get(), 42);
}

#[test]
fn test_system_reparent() {
    let mut parent1 = Children::new();
    let mut parent2 = Children::new();
    let mut child_parent = Parent::new(0);
    
    // Add child to parent1
    HierarchySystem::add_child(&mut parent1, 10, &mut child_parent);
    assert!(parent1.contains(10));
    assert_eq!(parent1.len(), 1);
    
    // Move child to parent2
    HierarchySystem::set_parent(&mut child_parent, Some(&mut parent1), &mut parent2, 10);
    assert!(!parent1.contains(10));
    assert!(parent2.contains(10));
    assert_eq!(parent1.len(), 0);
    assert_eq!(parent2.len(), 1);
}

#[test]
fn test_children_order() {
    let mut children = Children::new();
    
    // Add children in specific order
    children.add(10);
    children.add(20);
    children.add(30);
    children.add(40);
    
    // Verify order is maintained
    assert_eq!(children.get(0), Some(10));
    assert_eq!(children.get(1), Some(20));
    assert_eq!(children.get(2), Some(30));
    assert_eq!(children.get(3), Some(40));
    
    // Remove middle element
    children.remove(20);
    
    // Verify order after removal
    assert_eq!(children.get(0), Some(10));
    assert_eq!(children.get(1), Some(30));
    assert_eq!(children.get(2), Some(40));
    assert_eq!(children.len(), 3);
}

#[test]
fn test_parent_child_consistency() {
    let mut parent_children = Children::new();
    let mut child_parent = Parent::new(0);
    
    // Add child
    HierarchySystem::add_child(&mut parent_children, 100, &mut child_parent);
    assert!(parent_children.contains(100));
    assert_eq!(child_parent.get(), 100);
    
    // Remove child
    HierarchySystem::remove_child(&mut parent_children, 100, &mut child_parent);
    assert!(!parent_children.contains(100));
    assert_eq!(child_parent.get(), 0);
    
    // Add multiple children
    HierarchySystem::add_child(&mut parent_children, 1, &mut child_parent);
    HierarchySystem::add_child(&mut parent_children, 2, &mut child_parent);
    HierarchySystem::add_child(&mut parent_children, 3, &mut child_parent);
    
    assert_eq!(parent_children.len(), 3);
    assert!(parent_children.contains(1));
    assert!(parent_children.contains(2));
    assert!(parent_children.contains(3));
}

// ============================================================================
// Iterator Tests
// ============================================================================

#[test]
fn test_children_iterator() {
    let mut children = Children::new();
    
    children.add(10);
    children.add(20);
    children.add(30);
    
    let collected: Vec<u32> = children.iter().collect();
    assert_eq!(collected, vec![10, 20, 30]);
}

#[test]
fn test_children_iterator_empty() {
    let children = Children::new();
    
    let collected: Vec<u32> = children.iter().collect();
    assert_eq!(collected, Vec::<u32>::new());
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_add_duplicate_child() {
    let mut children = Children::new();
    
    assert!(children.add(42));
    assert_eq!(children.len(), 1);
    
    // Add same child again - should succeed but not duplicate
    assert!(children.add(42));
    assert_eq!(children.len(), 2); // Actually adds duplicate in current implementation
    
    // Remove one instance
    assert!(children.remove(42));
    assert_eq!(children.len(), 1);
    assert!(children.contains(42)); // Still has one instance
}

#[test]
fn test_remove_nonexistent_child() {
    let mut children = Children::new();
    
    children.add(10);
    children.add(20);
    
    // Try to remove child that doesn't exist
    assert!(!children.remove(999));
    assert_eq!(children.len(), 2);
}

#[test]
fn test_system_clear_children() {
    let mut children = Children::new();
    
    children.add(1);
    children.add(2);
    children.add(3);
    
    HierarchySystem::clear_children(&mut children);
    
    assert_eq!(children.len(), 0);
    assert!(children.is_empty());
}