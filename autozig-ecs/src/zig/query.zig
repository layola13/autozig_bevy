const std = @import("std");
const common = @import("common.zig");
const World = @import("world.zig").World;
const Entity = common.Entity;
const g_allocator = common.g_allocator;

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
    matched_entities: std.ArrayListUnmanaged(u32),
    matched_archetypes: std.ArrayListUnmanaged(u32),
    required_components: std.ArrayListUnmanaged(u32),
    excluded_components: std.ArrayListUnmanaged(u32),

    pub fn init(allocator: std.mem.Allocator) !*QueryState {
        const state = try allocator.create(QueryState);
        state.* = QueryState{
            .allocator = allocator,
            .matched_entities = .{},
            .matched_archetypes = .{},
            .required_components = .{},
            .excluded_components = .{},
        };
        return state;
    }

    pub fn deinit(self: *QueryState) void {
        self.matched_entities.deinit(self.allocator);
        self.matched_archetypes.deinit(self.allocator);
        self.required_components.deinit(self.allocator);
        self.excluded_components.deinit(self.allocator);
        self.allocator.destroy(self);
    }

    pub fn updateArchetypes(self: *QueryState, world: *World) !void {
        self.matched_archetypes.clearRetainingCapacity();
        for (world.archetypes.items) |arch| {
            if (self.matchesComponents(arch.table_components.items)) {
                try self.matched_archetypes.append(self.allocator, arch.id);
            }
        }
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
        self.matched_archetypes.clearRetainingCapacity();
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

    pub fn matchesComponents(self: *const QueryState, components: []const u32) bool {
        // Check exclusions first
        for (self.excluded_components.items) |id| {
            var found = false;
            for (components) |c_id| {
                if (c_id == id) {
                    found = true;
                    break;
                }
            }
            if (found) return false;
        }

        // Check requirements
        for (self.required_components.items) |id| {
            var found = false;
            for (components) |c_id| {
                if (c_id == id) {
                    found = true;
                    break;
                }
            }
            if (!found) return false;
        }

        return true;
    }
};

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

export fn query_state_update_archetypes(state: *QueryState, world: *World) void {
    state.updateArchetypes(world) catch {};
}

export fn query_state_get_matched_archetypes(state: *const QueryState, count_ptr: *usize) ?[*]const u32 {
    count_ptr.* = state.matched_archetypes.items.len;
    if (count_ptr.* == 0) return null;
    return state.matched_archetypes.items.ptr;
}

export fn query_state_matches_component_list(state: *const QueryState, components_ptr: [*]const u32, len: usize) bool {
    const components = components_ptr[0..len];
    return state.matchesComponents(components);
}
