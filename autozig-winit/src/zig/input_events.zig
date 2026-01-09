const std = @import("std");

/// KeyCode - Keyboard key codes
pub const KeyCode = enum(u16) {
    Unknown = 0,
    // Letters
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    F = 6,
    G = 7,
    H = 8,
    I = 9,
    J = 10,
    K = 11,
    L = 12,
    M = 13,
    N = 14,
    O = 15,
    P = 16,
    Q = 17,
    R = 18,
    S = 19,
    T = 20,
    U = 21,
    V = 22,
    W = 23,
    X = 24,
    Y = 25,
    Z = 26,
    // Numbers
    Num0 = 30,
    Num1 = 31,
    Num2 = 32,
    Num3 = 33,
    Num4 = 34,
    Num5 = 35,
    Num6 = 36,
    Num7 = 37,
    Num8 = 38,
    Num9 = 39,
    // Function keys
    F1 = 40,
    F2 = 41,
    F3 = 42,
    F4 = 43,
    F5 = 44,
    F6 = 45,
    F7 = 46,
    F8 = 47,
    F9 = 48,
    F10 = 49,
    F11 = 50,
    F12 = 51,
    // Control keys
    Escape = 60,
    Space = 61,
    Enter = 62,
    Tab = 63,
    Backspace = 64,
    Delete = 65,
    Insert = 66,
    // Arrow keys
    Left = 70,
    Right = 71,
    Up = 72,
    Down = 73,
    // Modifier keys
    LShift = 80,
    RShift = 81,
    LControl = 82,
    RControl = 83,
    LAlt = 84,
    RAlt = 85,
};

/// MouseButton - Mouse button types
pub const MouseButton = enum(u8) {
    Left = 0,
    Right = 1,
    Middle = 2,
    Other = 3,
};

/// KeyboardEventType - Types of keyboard events
pub const KeyboardEventType = enum(u8) {
    KeyDown = 0,
    KeyUp = 1,
};

/// MouseEventType - Types of mouse events
pub const MouseEventType = enum(u8) {
    ButtonDown = 0,
    ButtonUp = 1,
    Move = 2,
    Wheel = 3,
};

/// TouchEventType - Types of touch events
pub const TouchEventType = enum(u8) {
    Start = 0,
    Move = 1,
    End = 2,
    Cancel = 3,
};

/// KeyboardEvent - Keyboard input event
pub const KeyboardEvent = extern struct {
    event_type: KeyboardEventType,
    key_code: KeyCode,
    shift: bool,
    ctrl: bool,
    alt: bool,
    meta: bool,
    repeat: bool,

    pub fn init(event_type: KeyboardEventType, key_code: KeyCode) KeyboardEvent {
        return KeyboardEvent{
            .event_type = event_type,
            .key_code = key_code,
            .shift = false,
            .ctrl = false,
            .alt = false,
            .meta = false,
            .repeat = false,
        };
    }

    pub fn isKeyDown(self: *const KeyboardEvent) bool {
        return self.event_type == .KeyDown;
    }

    pub fn isKeyUp(self: *const KeyboardEvent) bool {
        return self.event_type == .KeyUp;
    }
};

/// MouseEvent - Mouse input event
pub const MouseEvent = extern struct {
    event_type: MouseEventType,
    button: MouseButton,
    x: f32,
    y: f32,
    delta_x: f32,
    delta_y: f32,
    wheel_delta: f32,
    shift: bool,
    ctrl: bool,
    alt: bool,

    pub fn init(event_type: MouseEventType) MouseEvent {
        return MouseEvent{
            .event_type = event_type,
            .button = .Left,
            .x = 0.0,
            .y = 0.0,
            .delta_x = 0.0,
            .delta_y = 0.0,
            .wheel_delta = 0.0,
            .shift = false,
            .ctrl = false,
            .alt = false,
        };
    }

    pub fn isButtonDown(self: *const MouseEvent) bool {
        return self.event_type == .ButtonDown;
    }

    pub fn isButtonUp(self: *const MouseEvent) bool {
        return self.event_type == .ButtonUp;
    }

    pub fn isMove(self: *const MouseEvent) bool {
        return self.event_type == .Move;
    }

    pub fn isWheel(self: *const MouseEvent) bool {
        return self.event_type == .Wheel;
    }
};

/// TouchEvent - Touch input event
pub const TouchEvent = extern struct {
    event_type: TouchEventType,
    touch_id: u32,
    x: f32,
    y: f32,
    force: f32,

    pub fn init(event_type: TouchEventType, touch_id: u32) TouchEvent {
        return TouchEvent{
            .event_type = event_type,
            .touch_id = touch_id,
            .x = 0.0,
            .y = 0.0,
            .force = 1.0,
        };
    }

    pub fn isStart(self: *const TouchEvent) bool {
        return self.event_type == .Start;
    }

    pub fn isMove(self: *const TouchEvent) bool {
        return self.event_type == .Move;
    }

    pub fn isEnd(self: *const TouchEvent) bool {
        return self.event_type == .End;
    }

    pub fn isCancel(self: *const TouchEvent) bool {
        return self.event_type == .Cancel;
    }
};

// FFI exports for KeyboardEvent

export fn keyboard_event_init(event_type: KeyboardEventType, key_code: KeyCode) KeyboardEvent {
    return KeyboardEvent.init(event_type, key_code);
}

export fn keyboard_event_is_key_down(event: *const KeyboardEvent) bool {
    return event.isKeyDown();
}

export fn keyboard_event_is_key_up(event: *const KeyboardEvent) bool {
    return event.isKeyUp();
}

// FFI exports for MouseEvent

export fn mouse_event_init(event_type: MouseEventType) MouseEvent {
    return MouseEvent.init(event_type);
}

export fn mouse_event_set_position(event: *MouseEvent, x: f32, y: f32) void {
    event.x = x;
    event.y = y;
}

export fn mouse_event_set_delta(event: *MouseEvent, delta_x: f32, delta_y: f32) void {
    event.delta_x = delta_x;
    event.delta_y = delta_y;
}

export fn mouse_event_set_button(event: *MouseEvent, button: MouseButton) void {
    event.button = button;
}

export fn mouse_event_set_wheel_delta(event: *MouseEvent, delta: f32) void {
    event.wheel_delta = delta;
}

export fn mouse_event_is_button_down(event: *const MouseEvent) bool {
    return event.isButtonDown();
}

export fn mouse_event_is_button_up(event: *const MouseEvent) bool {
    return event.isButtonUp();
}

export fn mouse_event_is_move(event: *const MouseEvent) bool {
    return event.isMove();
}

export fn mouse_event_is_wheel(event: *const MouseEvent) bool {
    return event.isWheel();
}

// FFI exports for TouchEvent

export fn touch_event_init(event_type: TouchEventType, touch_id: u32) TouchEvent {
    return TouchEvent.init(event_type, touch_id);
}

export fn touch_event_set_position(event: *TouchEvent, x: f32, y: f32) void {
    event.x = x;
    event.y = y;
}

export fn touch_event_set_force(event: *TouchEvent, force: f32) void {
    event.force = force;
}

export fn touch_event_is_start(event: *const TouchEvent) bool {
    return event.isStart();
}

export fn touch_event_is_move(event: *const TouchEvent) bool {
    return event.isMove();
}

export fn touch_event_is_end(event: *const TouchEvent) bool {
    return event.isEnd();
}

export fn touch_event_is_cancel(event: *const TouchEvent) bool {
    return event.isCancel();
}

// Tests

test "KeyboardEvent init and queries" {
    const testing = std.testing;

    var event = keyboard_event_init(.KeyDown, .A);
    try testing.expect(keyboard_event_is_key_down(&event));
    try testing.expect(!keyboard_event_is_key_up(&event));
    try testing.expectEqual(KeyCode.A, event.key_code);

    event = keyboard_event_init(.KeyUp, .Space);
    try testing.expect(!keyboard_event_is_key_down(&event));
    try testing.expect(keyboard_event_is_key_up(&event));
    try testing.expectEqual(KeyCode.Space, event.key_code);
}

test "MouseEvent button events" {
    const testing = std.testing;

    var event = mouse_event_init(.ButtonDown);
    mouse_event_set_button(&event, .Left);
    mouse_event_set_position(&event, 100.0, 200.0);

    try testing.expect(mouse_event_is_button_down(&event));
    try testing.expect(!mouse_event_is_button_up(&event));
    try testing.expectEqual(MouseButton.Left, event.button);
    try testing.expectEqual(@as(f32, 100.0), event.x);
    try testing.expectEqual(@as(f32, 200.0), event.y);
}

test "MouseEvent move event" {
    const testing = std.testing;

    var event = mouse_event_init(.Move);
    mouse_event_set_position(&event, 50.0, 75.0);
    mouse_event_set_delta(&event, 10.0, -5.0);

    try testing.expect(mouse_event_is_move(&event));
    try testing.expectEqual(@as(f32, 50.0), event.x);
    try testing.expectEqual(@as(f32, 75.0), event.y);
    try testing.expectEqual(@as(f32, 10.0), event.delta_x);
    try testing.expectEqual(@as(f32, -5.0), event.delta_y);
}

test "MouseEvent wheel event" {
    const testing = std.testing;

    var event = mouse_event_init(.Wheel);
    mouse_event_set_wheel_delta(&event, 120.0);

    try testing.expect(mouse_event_is_wheel(&event));
    try testing.expectEqual(@as(f32, 120.0), event.wheel_delta);
}

test "TouchEvent lifecycle" {
    const testing = std.testing;

    var event = touch_event_init(.Start, 1);
    touch_event_set_position(&event, 100.0, 200.0);
    touch_event_set_force(&event, 0.8);

    try testing.expect(touch_event_is_start(&event));
    try testing.expectEqual(@as(u32, 1), event.touch_id);
    try testing.expectEqual(@as(f32, 100.0), event.x);
    try testing.expectEqual(@as(f32, 200.0), event.y);
    try testing.expectEqual(@as(f32, 0.8), event.force);

    event = touch_event_init(.Move, 1);
    try testing.expect(touch_event_is_move(&event));

    event = touch_event_init(.End, 1);
    try testing.expect(touch_event_is_end(&event));
}
