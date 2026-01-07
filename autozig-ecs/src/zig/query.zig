const std = @import("std");

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
    
    pub fn init(allocator: std.mem.Allocator) !*QueryState {
        const state = try allocator.create(QueryState);
        state.* = QueryState{
            .allocator = allocator,
            .matched_entities = std.ArrayList(u32){},
        };
        return state;
    }
    
    pub fn deinit(self: *QueryState) void {
        self.matched_entities.deinit(self.allocator);
        self.allocator.destroy(self);
    }
    
    pub fn addEntity(self: *QueryState, entity_idx: u32) !void {
        try self.matched_entities.append(self.allocator, entity_idx);
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
