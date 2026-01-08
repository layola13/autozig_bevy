// Hierarchy event system
// WASM-optimized: simple structs, no complex enums

/// Hierarchy event types
pub const HierarchyEventType = enum(u8) {
    ChildAdded = 0,
    ChildRemoved = 1,
    ParentChanged = 2,
};

/// Hierarchy event - represents changes in parent-child relationships
pub const HierarchyEvent = extern struct {
    event_type: HierarchyEventType,
    parent_entity: u32,
    child_entity: u32,

    /// Create a ChildAdded event
    pub fn child_added(parent_entity: u32, child_entity: u32) HierarchyEvent {
        return HierarchyEvent{
            .event_type = HierarchyEventType.ChildAdded,
            .parent_entity = parent_entity,
            .child_entity = child_entity,
        };
    }

    /// Create a ChildRemoved event
    pub fn child_removed(parent_entity: u32, child_entity: u32) HierarchyEvent {
        return HierarchyEvent{
            .event_type = HierarchyEventType.ChildRemoved,
            .parent_entity = parent_entity,
            .child_entity = child_entity,
        };
    }

    /// Create a ParentChanged event
    pub fn parent_changed(new_parent_entity: u32, child_entity: u32) HierarchyEvent {
        return HierarchyEvent{
            .event_type = HierarchyEventType.ParentChanged,
            .parent_entity = new_parent_entity,
            .child_entity = child_entity,
        };
    }

    /// Get the event type
    pub fn get_type(self: *const HierarchyEvent) HierarchyEventType {
        return self.event_type;
    }

    /// Get the parent entity
    pub fn get_parent(self: *const HierarchyEvent) u32 {
        return self.parent_entity;
    }

    /// Get the child entity
    pub fn get_child(self: *const HierarchyEvent) u32 {
        return self.child_entity;
    }
};

// Export functions for FFI
export fn hierarchy_event_child_added(parent_entity: u32, child_entity: u32) HierarchyEvent {
    return HierarchyEvent.child_added(parent_entity, child_entity);
}

export fn hierarchy_event_child_removed(parent_entity: u32, child_entity: u32) HierarchyEvent {
    return HierarchyEvent.child_removed(parent_entity, child_entity);
}

export fn hierarchy_event_parent_changed(new_parent_entity: u32, child_entity: u32) HierarchyEvent {
    return HierarchyEvent.parent_changed(new_parent_entity, child_entity);
}

export fn hierarchy_event_get_type(self: *const HierarchyEvent) HierarchyEventType {
    return self.get_type();
}

export fn hierarchy_event_get_parent(self: *const HierarchyEvent) u32 {
    return self.get_parent();
}

export fn hierarchy_event_get_child(self: *const HierarchyEvent) u32 {
    return self.get_child();
}
