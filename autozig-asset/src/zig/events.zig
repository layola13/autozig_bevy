const std = @import("std");
const handle = @import("handle.zig");

const HandleId = handle.HandleId;
const AssetId = handle.AssetId;

// ============================================================================
// Asset Event - 资产事件
// ============================================================================
pub const AssetEventType = enum(u32) {
    Created = 0,
    Modified = 1,
    Removed = 2,
    LoadingStarted = 3,
    LoadingFinished = 4,
    LoadingFailed = 5,
};

pub const AssetEvent = extern struct {
    handle_id: HandleId,
    event_type: AssetEventType,
    timestamp: i64,

    pub fn init(handle_id: HandleId, event_type: AssetEventType) AssetEvent {
        return .{
            .handle_id = handle_id,
            .event_type = event_type,
            .timestamp = std.time.milliTimestamp(),
        };
    }

    pub fn created(handle_id: HandleId) AssetEvent {
        return init(handle_id, .Created);
    }

    pub fn modified(handle_id: HandleId) AssetEvent {
        return init(handle_id, .Modified);
    }

    pub fn removed(handle_id: HandleId) AssetEvent {
        return init(handle_id, .Removed);
    }

    pub fn loadingStarted(handle_id: HandleId) AssetEvent {
        return init(handle_id, .LoadingStarted);
    }

    pub fn loadingFinished(handle_id: HandleId) AssetEvent {
        return init(handle_id, .LoadingFinished);
    }

    pub fn loadingFailed(handle_id: HandleId) AssetEvent {
        return init(handle_id, .LoadingFailed);
    }
};

// ============================================================================
// Event Queue - 事件队列
// ============================================================================
pub const EventQueue = struct {
    allocator: std.mem.Allocator,
    events: std.ArrayList(AssetEvent),
    mutex: std.Thread.Mutex,

    pub fn init(allocator: std.mem.Allocator) EventQueue {
        return .{
            .allocator = allocator,
            .events = std.ArrayList(AssetEvent).init(allocator),
            .mutex = .{},
        };
    }

    pub fn deinit(self: *EventQueue) void {
        self.events.deinit();
    }

    pub fn push(self: *EventQueue, event: AssetEvent) !void {
        self.mutex.lock();
        defer self.mutex.unlock();
        try self.events.append(event);
    }

    pub fn drain(self: *EventQueue) []AssetEvent {
        self.mutex.lock();
        defer self.mutex.unlock();

        const events = self.events.items;
        const owned = self.allocator.dupe(AssetEvent, events) catch return &[_]AssetEvent{};
        self.events.clearRetainingCapacity();
        return owned;
    }

    pub fn clear(self: *EventQueue) void {
        self.mutex.lock();
        defer self.mutex.unlock();
        self.events.clearRetainingCapacity();
    }

    pub fn len(self: *EventQueue) usize {
        self.mutex.lock();
        defer self.mutex.unlock();
        return self.events.items.len;
    }

    pub fn isEmpty(self: *const EventQueue) bool {
        return self.events.items.len == 0;
    }
};

// ============================================================================
// Event Listener - 事件监听器
// ============================================================================
pub const EventListener = struct {
    callback: *const fn (AssetEvent) callconv(.C) void,
    filter_type: ?AssetEventType,

    pub fn init(callback: *const fn (AssetEvent) callconv(.C) void, filter_type: ?AssetEventType) EventListener {
        return .{
            .callback = callback,
            .filter_type = filter_type,
        };
    }

    pub fn matches(self: EventListener, event: AssetEvent) bool {
        if (self.filter_type) |filter| {
            return event.event_type == filter;
        }
        return true;
    }

    pub fn notify(self: EventListener, event: AssetEvent) void {
        if (self.matches(event)) {
            self.callback(event);
        }
    }
};

// ============================================================================
// Event System - 事件系统
// ============================================================================
pub const EventSystem = struct {
    allocator: std.mem.Allocator,
    queue: EventQueue,
    listeners: std.ArrayList(EventListener),

    pub fn init(allocator: std.mem.Allocator) EventSystem {
        return .{
            .allocator = allocator,
            .queue = EventQueue.init(allocator),
            .listeners = std.ArrayList(EventListener).init(allocator),
        };
    }

    pub fn deinit(self: *EventSystem) void {
        self.queue.deinit();
        self.listeners.deinit();
    }

    pub fn send(self: *EventSystem, event: AssetEvent) !void {
        try self.queue.push(event);
    }

    pub fn addListener(self: *EventSystem, listener: EventListener) !void {
        try self.listeners.append(listener);
    }

    pub fn removeListener(self: *EventSystem, callback: *const fn (AssetEvent) callconv(.C) void) void {
        var i: usize = 0;
        while (i < self.listeners.items.len) {
            if (self.listeners.items[i].callback == callback) {
                _ = self.listeners.swapRemove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn processEvents(self: *EventSystem) void {
        const events = self.queue.drain();
        defer self.allocator.free(events);

        for (events) |event| {
            for (self.listeners.items) |listener| {
                listener.notify(event);
            }
        }
    }

    pub fn clear(self: *EventSystem) void {
        self.queue.clear();
    }
};

// ============================================================================
// FFI exports
// ============================================================================

export fn asset_event_created(handle_id: HandleId) AssetEvent {
    return AssetEvent.created(handle_id);
}

export fn asset_event_modified(handle_id: HandleId) AssetEvent {
    return AssetEvent.modified(handle_id);
}

export fn asset_event_removed(handle_id: HandleId) AssetEvent {
    return AssetEvent.removed(handle_id);
}

export fn event_queue_create() ?*EventQueue {
    const allocator = std.heap.c_allocator;
    const queue = allocator.create(EventQueue) catch return null;
    queue.* = EventQueue.init(allocator);
    return queue;
}

export fn event_queue_destroy(queue: *EventQueue) void {
    const allocator = queue.allocator;
    queue.deinit();
    allocator.destroy(queue);
}

export fn event_queue_push(queue: *EventQueue, event: AssetEvent) bool {
    queue.push(event) catch return false;
    return true;
}

export fn event_queue_len(queue: *EventQueue) usize {
    return queue.len();
}

export fn event_queue_clear(queue: *EventQueue) void {
    queue.clear();
}

export fn event_system_create() ?*EventSystem {
    const allocator = std.heap.c_allocator;
    const system = allocator.create(EventSystem) catch return null;
    system.* = EventSystem.init(allocator);
    return system;
}

export fn event_system_destroy(system: *EventSystem) void {
    const allocator = system.allocator;
    system.deinit();
    allocator.destroy(system);
}

export fn event_system_send(system: *EventSystem, event: AssetEvent) bool {
    system.send(event) catch return false;
    return true;
}

export fn event_system_add_listener(
    system: *EventSystem,
    callback: *const fn (AssetEvent) callconv(.C) void,
    filter_type: i32,
) bool {
    const filter: ?AssetEventType = if (filter_type < 0) null else @enumFromInt(@as(u32, @intCast(filter_type)));
    const listener = EventListener.init(callback, filter);
    system.addListener(listener) catch return false;
    return true;
}

export fn event_system_process_events(system: *EventSystem) void {
    system.processEvents();
}

export fn event_system_clear(system: *EventSystem) void {
    system.clear();
}
