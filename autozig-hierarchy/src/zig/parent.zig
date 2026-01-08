// Parent component - stores parent entity reference
// WASM-optimized: stack-allocated, no dynamic memory

/// Parent component holds a reference to the parent entity
pub const Parent = extern struct {
    entity: u32,

    /// Create a new Parent component
    pub fn create(parent_entity: u32) Parent {
        return Parent{ .entity = parent_entity };
    }

    /// Get the parent entity ID
    pub fn get(self: *const Parent) u32 {
        return self.entity;
    }

    /// Set the parent entity ID
    pub fn set(self: *Parent, parent_entity: u32) void {
        self.entity = parent_entity;
    }
};

// Export functions for FFI
export fn parent_create(parent_entity: u32) Parent {
    return Parent.create(parent_entity);
}

export fn parent_get(self: *const Parent) u32 {
    return self.get();
}

export fn parent_set(self: *Parent, parent_entity: u32) void {
    self.set(parent_entity);
}
