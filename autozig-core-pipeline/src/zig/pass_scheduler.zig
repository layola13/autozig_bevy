//! Pass Scheduler
//! Manages render pass scheduling and execution order

const std = @import("std");

/// Pass type enumeration
pub const PassType = enum(u32) {
    ClearPass = 0,
    MainOpaquePass = 1,
    MainTransparentPass = 2,
    PostProcessPass = 3,
    TonemappingPass = 4,
};

/// Pass priority (higher values execute first)
pub const PassPriority = enum(u32) {
    Early = 0,
    Normal = 100,
    Late = 200,
    PostProcess = 300,
};

/// Render pass descriptor
pub const Pass = extern struct {
    name: [64]u8,
    name_len: u32,
    pass_type: u32, // PassType
    priority: u32, // PassPriority
    inputs: [8]u32, // Resource IDs
    input_count: u32,
    outputs: [8]u32, // Resource IDs
    output_count: u32,
    execute_fn: ?*const fn (*anyopaque, *anyopaque) callconv(.c) void,
    user_data: ?*anyopaque,
    is_enabled: bool,
    is_dirty: bool,
};

/// Pass scheduler manages pass execution order
pub const PassScheduler = extern struct {
    passes: [64]Pass,
    pass_count: u32,
    execution_order: [64]u32,
    is_dirty: bool,
};

/// Create empty pass
export fn pass_create() Pass {
    return Pass{
        .name = [_]u8{0} ** 64,
        .name_len = 0,
        .pass_type = @intFromEnum(PassType.MainOpaquePass),
        .priority = @intFromEnum(PassPriority.Normal),
        .inputs = [_]u32{0} ** 8,
        .input_count = 0,
        .outputs = [_]u32{0} ** 8,
        .output_count = 0,
        .execute_fn = null,
        .user_data = null,
        .is_enabled = true,
        .is_dirty = false,
    };
}

/// Set pass name
export fn pass_set_name(pass: *Pass, name: [*]const u8, len: u32) void {
    const copy_len = @min(len, 63);
    @memcpy(pass.name[0..copy_len], name[0..copy_len]);
    pass.name[copy_len] = 0;
    pass.name_len = copy_len;
}

/// Set pass type
export fn pass_set_type(pass: *Pass, pass_type: u32) void {
    pass.pass_type = pass_type;
    pass.is_dirty = true;
}

/// Set pass priority
export fn pass_set_priority(pass: *Pass, priority: u32) void {
    pass.priority = priority;
    pass.is_dirty = true;
}

/// Add input resource
export fn pass_add_input(pass: *Pass, resource_id: u32) bool {
    if (pass.input_count >= 8) return false;
    pass.inputs[pass.input_count] = resource_id;
    pass.input_count += 1;
    pass.is_dirty = true;
    return true;
}

/// Add output resource
export fn pass_add_output(pass: *Pass, resource_id: u32) bool {
    if (pass.output_count >= 8) return false;
    pass.outputs[pass.output_count] = resource_id;
    pass.output_count += 1;
    pass.is_dirty = true;
    return true;
}

/// Set execute function
export fn pass_set_execute_fn(
    pass: *Pass,
    execute_fn: ?*const fn (*anyopaque, *anyopaque) callconv(.c) void,
) void {
    pass.execute_fn = execute_fn;
}

/// Set user data
export fn pass_set_user_data(pass: *Pass, user_data: ?*anyopaque) void {
    pass.user_data = user_data;
}

/// Enable/disable pass
export fn pass_set_enabled(pass: *Pass, enabled: bool) void {
    pass.is_enabled = enabled;
}

/// Check if pass is enabled
export fn pass_is_enabled(pass: *const Pass) bool {
    return pass.is_enabled;
}

/// Create empty pass scheduler
export fn pass_scheduler_create() PassScheduler {
    return PassScheduler{
        .passes = [_]Pass{pass_create()} ** 64,
        .pass_count = 0,
        .execution_order = [_]u32{0} ** 64,
        .is_dirty = true,
    };
}

/// Add pass to scheduler
export fn pass_scheduler_add_pass(scheduler: *PassScheduler, pass: Pass) bool {
    if (scheduler.pass_count >= 64) return false;
    scheduler.passes[scheduler.pass_count] = pass;
    scheduler.pass_count += 1;
    scheduler.is_dirty = true;
    return true;
}

/// Get pass by index
export fn pass_scheduler_get_pass(scheduler: *PassScheduler, index: u32) ?*Pass {
    if (index >= scheduler.pass_count) return null;
    return &scheduler.passes[index];
}

/// Find pass by name
export fn pass_scheduler_find_pass(
    scheduler: *PassScheduler,
    name: [*]const u8,
    len: u32,
) ?*Pass {
    var i: u32 = 0;
    while (i < scheduler.pass_count) : (i += 1) {
        const pass = &scheduler.passes[i];
        if (pass.name_len == len) {
            if (std.mem.eql(u8, pass.name[0..len], name[0..len])) {
                return pass;
            }
        }
    }
    return null;
}

/// Remove pass by index
export fn pass_scheduler_remove_pass(scheduler: *PassScheduler, index: u32) bool {
    if (index >= scheduler.pass_count) return false;

    // Shift passes down
    var i = index;
    while (i < scheduler.pass_count - 1) : (i += 1) {
        scheduler.passes[i] = scheduler.passes[i + 1];
    }
    scheduler.pass_count -= 1;
    scheduler.is_dirty = true;
    return true;
}

/// Clear all passes
export fn pass_scheduler_clear(scheduler: *PassScheduler) void {
    scheduler.pass_count = 0;
    scheduler.is_dirty = true;
}

/// Sort passes by priority (higher priority first, then by pass type)
export fn pass_scheduler_update_execution_order(scheduler: *PassScheduler) void {
    // Initialize execution order
    var i: u32 = 0;
    while (i < scheduler.pass_count) : (i += 1) {
        scheduler.execution_order[i] = i;
    }

    // Simple bubble sort by priority (higher first)
    i = 0;
    while (i < scheduler.pass_count) : (i += 1) {
        var j: u32 = 0;
        while (j < scheduler.pass_count - 1) : (j += 1) {
            const idx_a = scheduler.execution_order[j];
            const idx_b = scheduler.execution_order[j + 1];
            const pass_a = &scheduler.passes[idx_a];
            const pass_b = &scheduler.passes[idx_b];

            // Higher priority first
            if (pass_a.priority < pass_b.priority) {
                scheduler.execution_order[j] = idx_b;
                scheduler.execution_order[j + 1] = idx_a;
            }
        }
    }

    scheduler.is_dirty = false;
}

/// Execute all passes in order
export fn pass_scheduler_execute(scheduler: *PassScheduler, context: *anyopaque) void {
    if (scheduler.is_dirty) {
        pass_scheduler_update_execution_order(scheduler);
    }

    var i: u32 = 0;
    while (i < scheduler.pass_count) : (i += 1) {
        const pass_index = scheduler.execution_order[i];
        const pass = &scheduler.passes[pass_index];

        if (pass.is_enabled and pass.execute_fn != null) {
            if (pass.execute_fn) |execute_fn| {
                execute_fn(pass.user_data orelse context, context);
            }
        }
    }
}

/// Get pass count
export fn pass_scheduler_get_pass_count(scheduler: *const PassScheduler) u32 {
    return scheduler.pass_count;
}

/// Check if scheduler is dirty
export fn pass_scheduler_is_dirty(scheduler: *const PassScheduler) bool {
    return scheduler.is_dirty;
}

/// Mark scheduler as dirty
export fn pass_scheduler_mark_dirty(scheduler: *PassScheduler) void {
    scheduler.is_dirty = true;
}

/// Get execution order
export fn pass_scheduler_get_execution_order(
    scheduler: *const PassScheduler,
    out_order: [*]u32,
    max_count: u32,
) u32 {
    const count = @min(scheduler.pass_count, max_count);
    @memcpy(out_order[0..count], scheduler.execution_order[0..count]);
    return count;
}
