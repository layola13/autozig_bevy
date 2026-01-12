//! Query Builder core implementation in Zig (90% of logic)
//! 查询构建器核心实现 - Zig

const std = @import("std");

/// ComponentId type matching Rust side
pub const ComponentId = u32;

/// Query builder structure - 查询构建器结构
pub const QueryBuilderCore = extern struct {
    component_ids: [32]ComponentId, // Up to 32 components
    component_count: u32,
    with_ids: [32]ComponentId, // Filter: with components
    with_count: u32,
    without_ids: [32]ComponentId, // Filter: without components
    without_count: u32,
    optional_ids: [32]ComponentId, // Optional components
    optional_count: u32,
    read_only: bool,

    pub fn init() QueryBuilderCore {
        return .{
            .component_ids = [_]ComponentId{0} ** 32,
            .component_count = 0,
            .with_ids = [_]ComponentId{0} ** 32,
            .with_count = 0,
            .without_ids = [_]ComponentId{0} ** 32,
            .without_count = 0,
            .optional_ids = [_]ComponentId{0} ** 32,
            .optional_count = 0,
            .read_only = false,
        };
    }

    /// Add a component to the query data
    pub fn addComponent(self: *QueryBuilderCore, component_id: ComponentId) bool {
        if (self.component_count >= 32) return false;
        self.component_ids[self.component_count] = component_id;
        self.component_count += 1;
        return true;
    }

    /// Add a "with" filter
    pub fn addWith(self: *QueryBuilderCore, component_id: ComponentId) bool {
        if (self.with_count >= 32) return false;
        self.with_ids[self.with_count] = component_id;
        self.with_count += 1;
        return true;
    }

    /// Add a "without" filter
    pub fn addWithout(self: *QueryBuilderCore, component_id: ComponentId) bool {
        if (self.without_count >= 32) return false;
        self.without_ids[self.without_count] = component_id;
        self.without_count += 1;
        return true;
    }

    /// Add an optional component
    pub fn addOptional(self: *QueryBuilderCore, component_id: ComponentId) bool {
        if (self.optional_count >= 32) return false;
        self.optional_ids[self.optional_count] = component_id;
        self.optional_count += 1;
        return true;
    }

    /// Set read-only mode
    pub fn setReadOnly(self: *QueryBuilderCore, read_only: bool) void {
        self.read_only = read_only;
    }

    /// Check if a component is in the query
    pub fn hasComponent(self: *const QueryBuilderCore, component_id: ComponentId) bool {
        var i: u32 = 0;
        while (i < self.component_count) : (i += 1) {
            if (self.component_ids[i] == component_id) return true;
        }
        return false;
    }

    /// Check if a "with" filter exists
    pub fn hasWith(self: *const QueryBuilderCore, component_id: ComponentId) bool {
        var i: u32 = 0;
        while (i < self.with_count) : (i += 1) {
            if (self.with_ids[i] == component_id) return true;
        }
        return false;
    }

    /// Check if a "without" filter exists
    pub fn hasWithout(self: *const QueryBuilderCore, component_id: ComponentId) bool {
        var i: u32 = 0;
        while (i < self.without_count) : (i += 1) {
            if (self.without_ids[i] == component_id) return true;
        }
        return false;
    }

    /// Check if an optional component exists
    pub fn hasOptional(self: *const QueryBuilderCore, component_id: ComponentId) bool {
        var i: u32 = 0;
        while (i < self.optional_count) : (i += 1) {
            if (self.optional_ids[i] == component_id) return true;
        }
        return false;
    }

    /// Clear all components
    pub fn clearComponents(self: *QueryBuilderCore) void {
        self.component_count = 0;
    }

    /// Clear all filters
    pub fn clearFilters(self: *QueryBuilderCore) void {
        self.with_count = 0;
        self.without_count = 0;
        self.optional_count = 0;
    }

    /// Clear everything
    pub fn clear(self: *QueryBuilderCore) void {
        self.clearComponents();
        self.clearFilters();
        self.read_only = false;
    }

    /// Get component count
    pub fn getComponentCount(self: *const QueryBuilderCore) u32 {
        return self.component_count;
    }

    /// Get component at index
    pub fn getComponent(self: *const QueryBuilderCore, index: u32) ComponentId {
        if (index >= self.component_count) return 0;
        return self.component_ids[index];
    }

    /// Validate the query builder state
    pub fn validate(self: *const QueryBuilderCore) bool {
        // Check for conflicts between with and without
        var i: u32 = 0;
        while (i < self.with_count) : (i += 1) {
            if (self.hasWithout(self.with_ids[i])) return false;
        }
        return true;
    }

    /// Merge with another query builder (OR logic)
    pub fn mergeOr(self: *QueryBuilderCore, other: *const QueryBuilderCore) bool {
        // Add all components from other
        var i: u32 = 0;
        while (i < other.component_count) : (i += 1) {
            if (!self.addComponent(other.component_ids[i])) return false;
        }

        // Add all filters from other
        i = 0;
        while (i < other.with_count) : (i += 1) {
            if (!self.addWith(other.with_ids[i])) return false;
        }

        i = 0;
        while (i < other.without_count) : (i += 1) {
            if (!self.addWithout(other.without_ids[i])) return false;
        }

        return true;
    }

    /// Merge with another query builder (AND logic)
    pub fn mergeAnd(self: *QueryBuilderCore, other: *const QueryBuilderCore) bool {
        // For AND, we need components from both
        var i: u32 = 0;
        while (i < other.component_count) : (i += 1) {
            const comp_id = other.component_ids[i];
            if (!self.hasComponent(comp_id)) {
                if (!self.addComponent(comp_id)) return false;
            }
        }
        return true;
    }
};

// Export C ABI functions for Rust FFI
export fn query_builder_create() *QueryBuilderCore {
    const allocator = std.heap.c_allocator;
    const builder = allocator.create(QueryBuilderCore) catch unreachable;
    builder.* = QueryBuilderCore.init();
    return builder;
}

export fn query_builder_destroy(builder: *QueryBuilderCore) void {
    const allocator = std.heap.c_allocator;
    allocator.destroy(builder);
}

export fn query_builder_add_component(builder: *QueryBuilderCore, component_id: ComponentId) bool {
    return builder.addComponent(component_id);
}

export fn query_builder_add_with(builder: *QueryBuilderCore, component_id: ComponentId) bool {
    return builder.addWith(component_id);
}

export fn query_builder_add_without(builder: *QueryBuilderCore, component_id: ComponentId) bool {
    return builder.addWithout(component_id);
}

export fn query_builder_add_optional(builder: *QueryBuilderCore, component_id: ComponentId) bool {
    return builder.addOptional(component_id);
}

export fn query_builder_set_read_only(builder: *QueryBuilderCore, read_only: bool) void {
    builder.setReadOnly(read_only);
}

export fn query_builder_has_component(builder: *const QueryBuilderCore, component_id: ComponentId) bool {
    return builder.hasComponent(component_id);
}

export fn query_builder_clear(builder: *QueryBuilderCore) void {
    builder.clear();
}

export fn query_builder_get_component_count(builder: *const QueryBuilderCore) u32 {
    return builder.getComponentCount();
}

export fn query_builder_get_component(builder: *const QueryBuilderCore, index: u32) ComponentId {
    return builder.getComponent(index);
}

export fn query_builder_validate(builder: *const QueryBuilderCore) bool {
    return builder.validate();
}

export fn query_builder_merge_or(builder: *QueryBuilderCore, other: *const QueryBuilderCore) bool {
    return builder.mergeOr(other);
}

export fn query_builder_merge_and(builder: *QueryBuilderCore, other: *const QueryBuilderCore) bool {
    return builder.mergeAnd(other);
}
