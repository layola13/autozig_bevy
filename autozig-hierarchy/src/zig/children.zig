// Children component - stores child entity references
// WASM-optimized: fixed-size array (64 children), stack-allocated

/// Children component holds up to 64 child entity references
pub const Children = extern struct {
    entities: [64]u32,
    count: u32,

    /// Create a new empty Children component
    pub fn create() Children {
        return Children{
            .entities = [_]u32{0} ** 64,
            .count = 0,
        };
    }

    /// Add a child entity to the list
    /// Returns true if added successfully, false if list is full
    pub fn add(self: *Children, child_entity: u32) bool {
        if (self.count >= 64) {
            return false;
        }
        self.entities[self.count] = child_entity;
        self.count += 1;
        return true;
    }

    /// Remove a child entity from the list
    /// Returns true if removed successfully, false if not found
    pub fn remove(self: *Children, child_entity: u32) bool {
        var i: u32 = 0;
        while (i < self.count) : (i += 1) {
            if (self.entities[i] == child_entity) {
                // Shift remaining elements left
                var j: u32 = i;
                while (j < self.count - 1) : (j += 1) {
                    self.entities[j] = self.entities[j + 1];
                }
                self.count -= 1;
                return true;
            }
        }
        return false;
    }

    /// Check if a child entity exists in the list
    pub fn contains(self: *const Children, child_entity: u32) bool {
        var i: u32 = 0;
        while (i < self.count) : (i += 1) {
            if (self.entities[i] == child_entity) {
                return true;
            }
        }
        return false;
    }

    /// Get child entity at index
    /// Returns 0 if index is out of bounds
    pub fn get_at(self: *const Children, index: u32) u32 {
        if (index >= self.count) {
            return 0;
        }
        return self.entities[index];
    }

    /// Get the number of children
    pub fn get_count(self: *const Children) u32 {
        return self.count;
    }

    /// Clear all children
    pub fn clear(self: *Children) void {
        self.count = 0;
    }

    /// Check if the children list is full
    pub fn is_full(self: *const Children) bool {
        return self.count >= 64;
    }
};

// Export functions for FFI
export fn children_create() Children {
    return Children.create();
}

export fn children_add(self: *Children, child_entity: u32) bool {
    return self.add(child_entity);
}

export fn children_remove(self: *Children, child_entity: u32) bool {
    return self.remove(child_entity);
}

export fn children_contains(self: *const Children, child_entity: u32) bool {
    return self.contains(child_entity);
}

export fn children_get_at(self: *const Children, index: u32) u32 {
    return self.get_at(index);
}

export fn children_get_count(self: *const Children) u32 {
    return self.get_count();
}

export fn children_clear(self: *Children) void {
    self.clear();
}

export fn children_is_full(self: *const Children) bool {
    return self.is_full();
}
