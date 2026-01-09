//! Resource Barrier Management
//! Tracks resource dependencies and manages GPU resource transitions

const std = @import("std");

/// Resource state for WebGPU
pub const ResourceState = enum(u32) {
    Undefined = 0,
    RenderTarget = 1,
    DepthWrite = 2,
    DepthRead = 3,
    ShaderResource = 4,
    UnorderedAccess = 5,
    CopySource = 6,
    CopyDest = 7,
    Present = 8,
};

/// Resource type
pub const ResourceType = enum(u32) {
    Buffer = 0,
    Texture = 1,
    TextureView = 2,
};

/// Resource descriptor
pub const Resource = extern struct {
    id: u32,
    resource_type: u32, // ResourceType
    handle: ?*anyopaque,
    current_state: u32, // ResourceState
    name: [64]u8,
    name_len: u32,
    is_valid: bool,
};

/// Resource barrier (state transition)
pub const ResourceBarrier = extern struct {
    resource_id: u32,
    state_before: u32, // ResourceState
    state_after: u32, // ResourceState
    is_executed: bool,
};

/// Resource tracker manages resource states and barriers
pub const ResourceTracker = extern struct {
    resources: [256]Resource,
    resource_count: u32,
    barriers: [512]ResourceBarrier,
    barrier_count: u32,
    next_resource_id: u32,
};

/// Create empty resource
export fn resource_create() Resource {
    return Resource{
        .id = 0,
        .resource_type = @intFromEnum(ResourceType.Texture),
        .handle = null,
        .current_state = @intFromEnum(ResourceState.Undefined),
        .name = [_]u8{0} ** 64,
        .name_len = 0,
        .is_valid = false,
    };
}

/// Set resource name
export fn resource_set_name(resource: *Resource, name: [*]const u8, len: u32) void {
    const copy_len = @min(len, 63);
    @memcpy(resource.name[0..copy_len], name[0..copy_len]);
    resource.name[copy_len] = 0;
    resource.name_len = copy_len;
}

/// Set resource type
export fn resource_set_type(resource: *Resource, resource_type: u32) void {
    resource.resource_type = resource_type;
}

/// Set resource handle
export fn resource_set_handle(resource: *Resource, handle: ?*anyopaque) void {
    resource.handle = handle;
    resource.is_valid = handle != null;
}

/// Set resource state
export fn resource_set_state(resource: *Resource, state: u32) void {
    resource.current_state = state;
}

/// Check if resource is valid
export fn resource_is_valid(resource: *const Resource) bool {
    return resource.is_valid and resource.handle != null;
}

/// Create empty barrier
export fn resource_barrier_create() ResourceBarrier {
    return ResourceBarrier{
        .resource_id = 0,
        .state_before = @intFromEnum(ResourceState.Undefined),
        .state_after = @intFromEnum(ResourceState.Undefined),
        .is_executed = false,
    };
}

/// Create resource tracker
export fn resource_tracker_create() ResourceTracker {
    return ResourceTracker{
        .resources = [_]Resource{resource_create()} ** 256,
        .resource_count = 0,
        .barriers = [_]ResourceBarrier{resource_barrier_create()} ** 512,
        .barrier_count = 0,
        .next_resource_id = 1,
    };
}

/// Register new resource
export fn resource_tracker_register(
    tracker: *ResourceTracker,
    resource_type: u32,
    handle: ?*anyopaque,
    initial_state: u32,
) u32 {
    if (tracker.resource_count >= 256) return 0;

    const resource_id = tracker.next_resource_id;
    const index = tracker.resource_count;

    tracker.next_resource_id += 1;
    tracker.resource_count += 1;

    var resource = &tracker.resources[index];
    resource.id = resource_id;
    resource.resource_type = resource_type;
    resource.handle = handle;
    resource.current_state = initial_state;
    resource.is_valid = true;
    resource.name_len = 0;

    return resource_id;
}

/// Unregister resource
export fn resource_tracker_unregister(tracker: *ResourceTracker, resource_id: u32) bool {
    var i: u32 = 0;
    while (i < tracker.resource_count) : (i += 1) {
        if (tracker.resources[i].id == resource_id) {
            // Shift resources down
            var j = i;
            while (j < tracker.resource_count - 1) : (j += 1) {
                tracker.resources[j] = tracker.resources[j + 1];
            }
            tracker.resource_count -= 1;
            return true;
        }
    }
    return false;
}

/// Find resource by ID
export fn resource_tracker_find(tracker: *ResourceTracker, resource_id: u32) ?*Resource {
    var i: u32 = 0;
    while (i < tracker.resource_count) : (i += 1) {
        if (tracker.resources[i].id == resource_id) {
            return &tracker.resources[i];
        }
    }
    return null;
}

/// Get resource by index
export fn resource_tracker_get(tracker: *ResourceTracker, index: u32) ?*Resource {
    if (index >= tracker.resource_count) return null;
    return &tracker.resources[index];
}

/// Add resource barrier
export fn resource_tracker_add_barrier(
    tracker: *ResourceTracker,
    resource_id: u32,
    state_after: u32,
) bool {
    if (tracker.barrier_count >= 512) return false;

    // Find resource
    const resource = resource_tracker_find(tracker, resource_id) orelse return false;

    // Skip if already in target state
    if (resource.current_state == state_after) return true;

    // Create barrier
    var barrier = &tracker.barriers[tracker.barrier_count];
    barrier.resource_id = resource_id;
    barrier.state_before = resource.current_state;
    barrier.state_after = state_after;
    barrier.is_executed = false;

    tracker.barrier_count += 1;
    return true;
}

/// Execute all pending barriers
export fn resource_tracker_execute_barriers(tracker: *ResourceTracker) void {
    var i: u32 = 0;
    while (i < tracker.barrier_count) : (i += 1) {
        const barrier = &tracker.barriers[i];
        if (!barrier.is_executed) {
            // Find resource and update its state
            if (resource_tracker_find(tracker, barrier.resource_id)) |resource| {
                resource.current_state = barrier.state_after;
            }
            barrier.is_executed = true;
        }
    }
    // Clear executed barriers
    tracker.barrier_count = 0;
}

/// Clear all barriers without executing
export fn resource_tracker_clear_barriers(tracker: *ResourceTracker) void {
    tracker.barrier_count = 0;
}

/// Get barrier count
export fn resource_tracker_get_barrier_count(tracker: *const ResourceTracker) u32 {
    return tracker.barrier_count;
}

/// Get resource count
export fn resource_tracker_get_resource_count(tracker: *const ResourceTracker) u32 {
    return tracker.resource_count;
}

/// Get resource state
export fn resource_tracker_get_state(tracker: *const ResourceTracker, resource_id: u32) u32 {
    var i: u32 = 0;
    while (i < tracker.resource_count) : (i += 1) {
        if (tracker.resources[i].id == resource_id) {
            return tracker.resources[i].current_state;
        }
    }
    return @intFromEnum(ResourceState.Undefined);
}

/// Check if resource needs barrier to target state
export fn resource_tracker_needs_barrier(
    tracker: *const ResourceTracker,
    resource_id: u32,
    target_state: u32,
) bool {
    const current_state = resource_tracker_get_state(tracker, resource_id);
    return current_state != target_state;
}

/// Clear all resources
export fn resource_tracker_clear(tracker: *ResourceTracker) void {
    tracker.resource_count = 0;
    tracker.barrier_count = 0;
    tracker.next_resource_id = 1;
}
