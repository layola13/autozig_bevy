const std = @import("std");

// ComponentId - unique identifier for component types
pub const ComponentId = u32;

// ComponentInfo - metadata about a component type
pub const ComponentInfo = struct {
    id: ComponentId,
    name: []const u8,
    size: usize,
    align_: usize,
};

// SparseSet storage for components
pub const SparseSet = struct {
    allocator: std.mem.Allocator,
    sparse: std.ArrayList(?usize), // entity_index -> dense_index
    dense_entities: std.ArrayList(u32), // dense_index -> entity_index
    dense_components: std.ArrayList(u8), // component data (type-erased)
    component_size: usize,
    
    pub fn init(allocator: std.mem.Allocator, component_size: usize) !*SparseSet {
        const set = try allocator.create(SparseSet);
        set.* = SparseSet{
            .allocator = allocator,
            .sparse = std.ArrayList(?usize){},
            .dense_entities = std.ArrayList(u32){},
            .dense_components = std.ArrayList(u8){},
            .component_size = component_size,
        };
        return set;
    }
    
    pub fn deinit(self: *SparseSet) void {
        self.sparse.deinit(self.allocator);
        self.dense_entities.deinit(self.allocator);
        self.dense_components.deinit(self.allocator);
        self.allocator.destroy(self);
    }
    
    pub fn insert(self: *SparseSet, entity_idx: u32, component_data: []const u8) !void {
        // Ensure sparse array is large enough
        while (self.sparse.items.len <= entity_idx) {
            try self.sparse.append(self.allocator, null);
        }
        
        // Check if entity already has this component
        if (self.sparse.items[entity_idx]) |_| {
            return error.ComponentAlreadyExists;
        }
        
        // Add to dense arrays
        const dense_idx = self.dense_entities.items.len;
        try self.dense_entities.append(self.allocator, entity_idx);
        
        // Add component data
        for (component_data) |byte| {
            try self.dense_components.append(self.allocator, byte);
        }
        
        // Update sparse array
        self.sparse.items[entity_idx] = dense_idx;
    }
    
    pub fn remove(self: *SparseSet, entity_idx: u32) bool {
        if (entity_idx >= self.sparse.items.len) return false;
        
        const dense_idx = self.sparse.items[entity_idx] orelse return false;
        
        // Swap-remove from dense arrays
        const last_dense_idx = self.dense_entities.items.len - 1;
        if (dense_idx != last_dense_idx) {
            const last_entity = self.dense_entities.items[last_dense_idx];
            self.dense_entities.items[dense_idx] = last_entity;
            self.sparse.items[last_entity] = dense_idx;
            
            // Move component data
            const start = dense_idx * self.component_size;
            const last_start = last_dense_idx * self.component_size;
            for (0..self.component_size) |i| {
                self.dense_components.items[start + i] = 
                    self.dense_components.items[last_start + i];
            }
        }
        
        // Remove last elements
        _ = self.dense_entities.pop();
        for (0..self.component_size) |_| {
            _ = self.dense_components.pop();
        }
        
        self.sparse.items[entity_idx] = null;
        return true;
    }
    
    pub fn contains(self: *const SparseSet, entity_idx: u32) bool {
        if (entity_idx >= self.sparse.items.len) return false;
        return self.sparse.items[entity_idx] != null;
    }
    
    pub fn len(self: *const SparseSet) usize {
        return self.dense_entities.items.len;
    }
};

// 全局allocator在entity.zig中定义，合并后直接可用

// Exported C API
export fn sparse_set_create(component_size: usize) ?*SparseSet {
    return SparseSet.init(g_allocator, component_size) catch null;
}

export fn sparse_set_destroy(set: *SparseSet) void {
    set.deinit();
}

export fn sparse_set_insert(set: *SparseSet, entity_idx: u32, data_ptr: [*]const u8, data_len: usize) bool {
    const data = data_ptr[0..data_len];
    set.insert(entity_idx, data) catch return false;
    return true;
}

export fn sparse_set_remove(set: *SparseSet, entity_idx: u32) bool {
    return set.remove(entity_idx);
}

export fn sparse_set_contains(set: *const SparseSet, entity_idx: u32) bool {
    return set.contains(entity_idx);
}

export fn sparse_set_len(set: *const SparseSet) usize {
    return set.len();
}

export fn sparse_set_get_entity(set: *const SparseSet, dense_index: usize) u32 {
    if (dense_index >= set.dense_entities.items.len) return 0xFFFFFFFF;
    return set.dense_entities.items[dense_index];
}
