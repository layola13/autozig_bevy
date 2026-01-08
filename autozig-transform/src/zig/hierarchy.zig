// Hierarchy component for parent-child relationships
// Fixed-size array implementation (no allocator needed)

const std = @import("std");

/// Maximum number of children per node
pub const MAX_CHILDREN: u32 = 32;

/// Hierarchy component for managing parent-child relationships
/// Uses fixed-size array for WASM optimization
pub const Hierarchy = extern struct {
    parent: u32, // Parent node ID (0 means no parent)
    children: [MAX_CHILDREN]u32, // Fixed-size children array
    children_count: u32, // Number of active children

    /// Create a new hierarchy node with no parent or children
    pub fn create() Hierarchy {
        return Hierarchy{
            .parent = 0,
            .children = [_]u32{0} ** MAX_CHILDREN,
            .children_count = 0,
        };
    }

    /// Add a child to this hierarchy node
    /// Returns true if successful, false if children array is full
    pub fn add_child(self: *Hierarchy, child_id: u32) bool {
        if (self.children_count >= MAX_CHILDREN) {
            return false;
        }

        self.children[self.children_count] = child_id;
        self.children_count += 1;
        return true;
    }

    /// Remove a child from this hierarchy node
    /// Returns true if found and removed, false otherwise
    pub fn remove_child(self: *Hierarchy, child_id: u32) bool {
        var i: u32 = 0;
        while (i < self.children_count) : (i += 1) {
            if (self.children[i] == child_id) {
                // Shift remaining children left
                var j: u32 = i;
                while (j < self.children_count - 1) : (j += 1) {
                    self.children[j] = self.children[j + 1];
                }
                // Clear the last slot
                self.children[self.children_count - 1] = 0;
                self.children_count -= 1;
                return true;
            }
        }
        return false;
    }

    /// Get children as a slice
    pub fn get_children(self: *const Hierarchy) []const u32 {
        return self.children[0..self.children_count];
    }

    /// Check if this node has a parent
    pub fn has_parent(self: *const Hierarchy) bool {
        return self.parent != 0;
    }

    /// Check if this node has children
    pub fn has_children(self: *const Hierarchy) bool {
        return self.children_count > 0;
    }

    /// Clear all children
    pub fn clear_children(self: *Hierarchy) void {
        var i: u32 = 0;
        while (i < MAX_CHILDREN) : (i += 1) {
            self.children[i] = 0;
        }
        self.children_count = 0;
    }

    /// Set parent ID
    pub fn set_parent(self: *Hierarchy, parent_id: u32) void {
        self.parent = parent_id;
    }

    /// Clear parent
    pub fn clear_parent(self: *Hierarchy) void {
        self.parent = 0;
    }
};

// Export C-compatible functions for FFI
export fn hierarchy_create() Hierarchy {
    return Hierarchy.create();
}

export fn hierarchy_add_child(hierarchy: *Hierarchy, child_id: u32) bool {
    return hierarchy.add_child(child_id);
}

export fn hierarchy_remove_child(hierarchy: *Hierarchy, child_id: u32) bool {
    return hierarchy.remove_child(child_id);
}

export fn hierarchy_get_children_count(hierarchy: *const Hierarchy) u32 {
    return hierarchy.children_count;
}

export fn hierarchy_get_child(hierarchy: *const Hierarchy, index: u32) u32 {
    if (index >= hierarchy.children_count) {
        return 0;
    }
    return hierarchy.children[index];
}

export fn hierarchy_has_parent(hierarchy: *const Hierarchy) bool {
    return hierarchy.has_parent();
}

export fn hierarchy_has_children(hierarchy: *const Hierarchy) bool {
    return hierarchy.has_children();
}

export fn hierarchy_clear_children(hierarchy: *Hierarchy) void {
    hierarchy.clear_children();
}

export fn hierarchy_set_parent(hierarchy: *Hierarchy, parent_id: u32) void {
    hierarchy.set_parent(parent_id);
}

export fn hierarchy_clear_parent(hierarchy: *Hierarchy) void {
    hierarchy.clear_parent();
}

export fn hierarchy_get_parent(hierarchy: *const Hierarchy) u32 {
    return hierarchy.parent;
}
