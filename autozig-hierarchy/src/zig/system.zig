// Hierarchy system - maintains parent-child consistency
// WASM-optimized: no dynamic allocation, stack-only operations

const Parent = @import("parent.zig").Parent;
const Children = @import("children.zig").Children;

/// Hierarchy system operations
pub const HierarchySystem = extern struct {
    /// Add a child to a parent, maintaining bidirectional consistency
    /// Returns true if successful, false if children list is full
    pub fn add_child(parent: *Children, child_id: u32, child_parent: *Parent) bool {
        // Check if parent's children list is full
        if (parent.is_full()) {
            return false;
        }

        // Check if child is already in the list
        if (parent.contains(child_id)) {
            return true; // Already added
        }

        // Add child to parent's children list
        if (!parent.add(child_id)) {
            return false;
        }

        // Set parent reference in child
        child_parent.set(child_id);

        return true;
    }

    /// Remove a child from a parent, maintaining bidirectional consistency
    /// Returns true if successful, false if child not found
    pub fn remove_child(parent: *Children, child_id: u32, child_parent: *Parent) bool {
        // Remove child from parent's children list
        if (!parent.remove(child_id)) {
            return false;
        }

        // Clear parent reference in child (set to 0)
        child_parent.set(0);

        return true;
    }

    /// Set a new parent for a child, updating both old and new parent
    /// old_parent can be null if the child had no previous parent
    /// Returns true if successful, false if new parent's children list is full
    pub fn set_parent(
        child_parent: *Parent,
        old_parent: ?*Children,
        new_parent: *Children,
        child_id: u32,
    ) bool {
        // Remove from old parent if it exists
        if (old_parent) |old| {
            _ = old.remove(child_id);
        }

        // Add to new parent
        if (!new_parent.add(child_id)) {
            // If adding fails, restore to old parent if possible
            if (old_parent) |old| {
                _ = old.add(child_id);
            }
            return false;
        }

        // Update child's parent reference
        child_parent.set(child_id);

        return true;
    }

    /// Clear all children from a parent
    pub fn clear_children(parent: *Children) void {
        parent.clear();
    }
};

// Export functions for FFI
export fn hierarchy_system_add_child(parent: *Children, child_id: u32, child_parent: *Parent) bool {
    return HierarchySystem.add_child(parent, child_id, child_parent);
}

export fn hierarchy_system_remove_child(parent: *Children, child_id: u32, child_parent: *Parent) bool {
    return HierarchySystem.remove_child(parent, child_id, child_parent);
}

export fn hierarchy_system_set_parent(
    child_parent: *Parent,
    old_parent: ?*Children,
    new_parent: *Children,
    child_id: u32,
) bool {
    return HierarchySystem.set_parent(child_parent, old_parent, new_parent, child_id);
}

export fn hierarchy_system_clear_children(parent: *Children) void {
    HierarchySystem.clear_children(parent);
}
