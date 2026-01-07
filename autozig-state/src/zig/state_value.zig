const std = @import("std");

// StateId - 状态唯一标识符 (使用u64以兼容Rust TypeId)
pub const StateId = u64;

// 全局allocator
var g_allocator: std.mem.Allocator = std.heap.page_allocator;

// StateValue - 状态值结构
pub const StateValue = struct {
    id: StateId,
    name: []const u8, // 可选：用于调试
};

// StateRegistry - 状态注册表（简化版，实际可扩展）
pub const StateRegistry = struct {
    current: ?StateId,
    next: ?StateId,

    pub fn init() StateRegistry {
        return StateRegistry{
            .current = null,
            .next = null,
        };
    }

    pub fn setCurrent(self: *StateRegistry, state_id: StateId) void {
        self.current = state_id;
    }

    pub fn setNext(self: *StateRegistry, state_id: StateId) void {
        self.next = state_id;
    }

    pub fn getCurrent(self: *const StateRegistry) ?StateId {
        return self.current;
    }

    pub fn getNext(self: *const StateRegistry) ?StateId {
        return self.next;
    }

    pub fn clearNext(self: *StateRegistry) void {
        self.next = null;
    }

    pub fn hasPending(self: *const StateRegistry) bool {
        return self.next != null;
    }

    // 应用挂起的状态转换
    pub fn applyTransition(self: *StateRegistry) bool {
        if (self.next) |next_state| {
            self.current = next_state;
            self.next = null;
            return true;
        }
        return false;
    }
};

// Exported C API
export fn state_registry_create() *StateRegistry {
    const registry = g_allocator.create(StateRegistry) catch unreachable;
    registry.* = StateRegistry.init();
    return registry;
}

export fn state_registry_destroy(registry: *StateRegistry) void {
    g_allocator.destroy(registry);
}

export fn state_registry_set_current(registry: *StateRegistry, state_id: StateId) void {
    registry.setCurrent(state_id);
}

export fn state_registry_set_next(registry: *StateRegistry, state_id: StateId) void {
    registry.setNext(state_id);
}

export fn state_registry_get_current(registry: *const StateRegistry) StateId {
    return registry.getCurrent() orelse 0;
}

export fn state_registry_get_next(registry: *const StateRegistry) StateId {
    return registry.getNext() orelse 0;
}

export fn state_registry_has_current(registry: *const StateRegistry) bool {
    return registry.current != null;
}

export fn state_registry_has_pending(registry: *const StateRegistry) bool {
    return registry.hasPending();
}

export fn state_registry_apply_transition(registry: *StateRegistry) bool {
    return registry.applyTransition();
}

export fn state_registry_clear_next(registry: *StateRegistry) void {
    registry.clearNext();
}
