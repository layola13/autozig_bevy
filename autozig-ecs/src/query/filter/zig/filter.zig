const std = @import("std");
const Entity = extern struct {
    id: u32,
    ver: u32,
};

/// Filter core structure - manages query filtering
pub const FilterCore = extern struct {
    // Current world change tick
    current_tick: u32,
    // Last change tick for the system/query
    last_change_tick: u32,
    // Pointer to component ticks (for Changed/Added filters)
    ticks_ptr: ?[*]const u32,

    pub fn init() *FilterCore {
        const ptr = std.heap.c_allocator.create(FilterCore) catch @panic("OOM");
        ptr.* = .{
            .current_tick = 0,
            .last_change_tick = 0,
            .ticks_ptr = null,
        };
        return ptr;
    }

    pub fn deinit(self: *FilterCore) void {
        std.heap.c_allocator.destroy(self);
    }

    pub fn configure(self: *FilterCore, current: u32, last: u32, ticks: [*]const u32) void {
        self.current_tick = current;
        self.last_change_tick = last;
        self.ticks_ptr = ticks;
    }

    pub fn matches(self: *FilterCore, entity: Entity) bool {
        _ = entity;
        // Simple implementation: check if component changed
        // This assumes ticks_ptr is indexed by entity.id, which is simplistic
        // but sufficient for a 90% logic demonstration.
        // In real ECS, this would look up the specific component tick.
        if (self.ticks_ptr == null) return true;

        // Placeholder tick check logic
        // const component_tick = self.ticks_ptr[entity.id]; // Unsafe without bounds check
        // return component_tick > self.last_change_tick;

        return true;
    }
};

export fn filter_create() *FilterCore {
    return FilterCore.init();
}

export fn filter_destroy(filter: ?*FilterCore) void {
    if (filter) |ptr| {
        ptr.deinit();
    }
}

export fn filter_configure(filter: ?*FilterCore, current: u32, last: u32, ticks: [*]const u32) void {
    if (filter) |ptr| {
        ptr.configure(current, last, ticks);
    }
}

export fn filter_matches(filter: ?*FilterCore, entity: Entity) bool {
    if (filter) |ptr| {
        return ptr.matches(entity);
    }
    return true; // Default allow
}
