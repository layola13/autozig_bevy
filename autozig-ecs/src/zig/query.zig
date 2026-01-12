const std = @import("std");
const common = @import("common.zig");
const Entity = common.Entity;
const g_allocator = common.g_allocator;

const Archetype = @import("archetype.zig").Archetype;

// Query iterator for entities with specific components
pub const QueryIter = struct {
    entities: []const u32,
    current: usize,

    pub fn next(self: *QueryIter) ?u32 {
        if (self.current >= self.entities.len) {
            return null;
        }
        const entity = self.entities[self.current];
        self.current += 1;
        return entity;
    }
};

// Query state - manages iteration over entities
pub const QueryState = struct {
    allocator: std.mem.Allocator,
    matched_entities: std.ArrayList(u32),
    required_components: std.ArrayList(u32),
    excluded_components: std.ArrayList(u32),

    pub fn init(allocator: std.mem.Allocator) !*QueryState {
        const state = try allocator.create(QueryState);
        state.* = QueryState{
            .allocator = allocator,
            .matched_entities = std.ArrayList(u32){},
            .required_components = std.ArrayList(u32){},
            .excluded_components = std.ArrayList(u32){},
        };
        return state;
    }

    pub fn deinit(self: *QueryState) void {
        self.matched_entities.deinit(self.allocator);
        self.required_components.deinit(self.allocator);
        self.excluded_components.deinit(self.allocator);
        self.allocator.destroy(self);
    }

    pub fn addEntity(self: *QueryState, entity_idx: u32) !void {
        try self.matched_entities.append(self.allocator, entity_idx);
    }

    pub fn addRequiredComponent(self: *QueryState, component_id: u32) !void {
        for (self.required_components.items) |id| {
            if (id == component_id) return;
        }
        try self.required_components.append(self.allocator, component_id);
    }

    pub fn addExcludedComponent(self: *QueryState, component_id: u32) !void {
        for (self.excluded_components.items) |id| {
            if (id == component_id) return;
        }
        try self.excluded_components.append(self.allocator, component_id);
    }

    pub fn clear(self: *QueryState) void {
        self.matched_entities.clearRetainingCapacity();
    }

    pub fn getIter(self: *const QueryState) QueryIter {
        return QueryIter{
            .entities = self.matched_entities.items,
            .current = 0,
        };
    }

    pub fn count(self: *const QueryState) usize {
        return self.matched_entities.items.len;
    }

    pub fn matchesArchetype(self: *const QueryState, archetype: *const Archetype) bool {
        // Check exclusions first
        for (self.excluded_components.items) |id| {
            if (archetype.hasComponent(id)) return false;
        }

        // Check requirements
        for (self.required_components.items) |id| {
            if (!archetype.hasComponent(id)) return false;
        }

        return true;
    }
};

// 全局allocator在entity.zig中定义，合并后直接可用

// Exported C API
export fn query_state_create() ?*QueryState {
    return QueryState.init(g_allocator) catch null;
}

export fn query_state_destroy(state: *QueryState) void {
    state.deinit();
}

export fn query_state_add_entity(state: *QueryState, entity_idx: u32) bool {
    state.addEntity(entity_idx) catch return false;
    return true;
}

export fn query_state_add_required_component(state: *QueryState, component_id: u32) bool {
    state.addRequiredComponent(component_id) catch return false;
    return true;
}

export fn query_state_add_excluded_component(state: *QueryState, component_id: u32) bool {
    state.addExcludedComponent(component_id) catch return false;
    return true;
}

export fn query_state_clear(state: *QueryState) void {
    state.clear();
}

export fn query_state_count(state: *const QueryState) usize {
    return state.count();
}

export fn query_state_get_entity(state: *const QueryState, index: usize) u32 {
    if (index >= state.matched_entities.items.len) return 0xFFFFFFFF;
    return state.matched_entities.items[index];
}

export fn query_state_is_empty(state: *const QueryState) bool {
    return state.matched_entities.items.len == 0;
}

export fn query_state_matched_entity_count(state: *const QueryState) u32 {
    return @intCast(state.matched_entities.items.len);
}

export fn query_state_update_archetypes(state: *QueryState) void {
    // Placeholder: In real implementation this would iterate World archetypes
    _ = state;
}

export fn query_state_matches_archetype(state: *const QueryState, archetype_ptr: *const anyopaque) bool {
    if (archetype_ptr == @as(*const anyopaque, @ptrFromInt(0))) return false;
    const archetype = @as(*const Archetype, @ptrCast(@alignCast(archetype_ptr)));
    return state.matchesArchetype(archetype);
}
