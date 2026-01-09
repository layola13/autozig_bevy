const std = @import("std");

/// EventLoopState - Tracks the state of the event loop
pub const EventLoopState = enum(u8) {
    Idle = 0,
    Running = 1,
    Exiting = 2,
};

/// EventLoop - Main event loop structure for WASM platform
/// Handles window events, input events, and requestAnimationFrame
pub const EventLoop = extern struct {
    state: EventLoopState,
    frame_count: u64,
    last_frame_time: f64,
    delta_time: f32,
    is_wasm: bool,

    pub fn init() EventLoop {
        return EventLoop{
            .state = .Idle,
            .frame_count = 0,
            .last_frame_time = -1.0, // -1 means uninitialized
            .delta_time = 0.0,
            .is_wasm = true,
        };
    }

    pub fn start(self: *EventLoop) void {
        self.state = .Running;
        self.frame_count = 0;
        self.last_frame_time = -1.0; // Reset on start
    }

    pub fn stop(self: *EventLoop) void {
        self.state = .Exiting;
    }

    pub fn update(self: *EventLoop, current_time: f64) void {
        if (self.last_frame_time >= 0.0) {
            const delta = current_time - self.last_frame_time;
            self.delta_time = @floatCast(delta / 1000.0); // Convert ms to seconds
        } else {
            self.delta_time = 0.0; // First frame
        }
        self.last_frame_time = current_time;
        self.frame_count += 1;
    }

    pub fn isRunning(self: *const EventLoop) bool {
        return self.state == .Running;
    }

    pub fn isExiting(self: *const EventLoop) bool {
        return self.state == .Exiting;
    }
};

// FFI exports for EventLoop

export fn event_loop_init() EventLoop {
    return EventLoop.init();
}

export fn event_loop_start(loop: *EventLoop) void {
    loop.start();
}

export fn event_loop_stop(loop: *EventLoop) void {
    loop.stop();
}

export fn event_loop_update(loop: *EventLoop, current_time: f64) void {
    loop.update(current_time);
}

export fn event_loop_is_running(loop: *const EventLoop) bool {
    return loop.isRunning();
}

export fn event_loop_is_exiting(loop: *const EventLoop) bool {
    return loop.isExiting();
}

export fn event_loop_get_delta_time(loop: *const EventLoop) f32 {
    return loop.delta_time;
}

export fn event_loop_get_frame_count(loop: *const EventLoop) u64 {
    return loop.frame_count;
}

export fn event_loop_get_state(loop: *const EventLoop) EventLoopState {
    return loop.state;
}

// Tests

test "EventLoop init" {
    const testing = std.testing;

    const loop = event_loop_init();

    try testing.expectEqual(EventLoopState.Idle, loop.state);
    try testing.expectEqual(@as(u64, 0), loop.frame_count);
    try testing.expect(!event_loop_is_running(&loop));
}

test "EventLoop start and stop" {
    const testing = std.testing;

    var loop = event_loop_init();
    try testing.expect(!event_loop_is_running(&loop));

    event_loop_start(&loop);
    try testing.expect(event_loop_is_running(&loop));
    try testing.expect(!event_loop_is_exiting(&loop));

    event_loop_stop(&loop);
    try testing.expect(!event_loop_is_running(&loop));
    try testing.expect(event_loop_is_exiting(&loop));
}

test "EventLoop update" {
    const testing = std.testing;

    var loop = event_loop_init();
    event_loop_start(&loop);

    // First frame at 0ms
    event_loop_update(&loop, 0.0);
    try testing.expectEqual(@as(u64, 1), loop.frame_count);
    try testing.expectEqual(@as(f32, 0.0), loop.delta_time);

    // Second frame at 16.67ms (60fps)
    event_loop_update(&loop, 16.67);
    try testing.expectEqual(@as(u64, 2), loop.frame_count);
    try testing.expectApproxEqAbs(@as(f32, 0.01667), loop.delta_time, 0.0001);

    // Third frame at 33.34ms
    event_loop_update(&loop, 33.34);
    try testing.expectEqual(@as(u64, 3), loop.frame_count);
    try testing.expectApproxEqAbs(@as(f32, 0.01667), loop.delta_time, 0.0001);
}

test "EventLoop state transitions" {
    const testing = std.testing;

    var loop = event_loop_init();
    try testing.expectEqual(EventLoopState.Idle, event_loop_get_state(&loop));

    event_loop_start(&loop);
    try testing.expectEqual(EventLoopState.Running, event_loop_get_state(&loop));

    event_loop_stop(&loop);
    try testing.expectEqual(EventLoopState.Exiting, event_loop_get_state(&loop));
}
