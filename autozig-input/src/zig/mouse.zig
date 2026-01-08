const std = @import("std");

/// MouseButton enumeration
pub const MouseButton = enum(u8) {
    Left = 0,
    Right = 1,
    Middle = 2,
    Other = 3,
};

/// MouseScrollUnit - Unit for mouse wheel scrolling
pub const MouseScrollUnit = enum(u8) {
    Line = 0,
    Pixel = 1,
};

/// MouseMotion - Mouse movement delta
pub const MouseMotion = extern struct {
    delta_x: f32,
    delta_y: f32,
};

/// MouseWheel - Mouse wheel scroll event
pub const MouseWheel = extern struct {
    unit: MouseScrollUnit,
    delta_x: f32,
    delta_y: f32,
};

// Maximum number of mouse buttons
const MAX_MOUSE_BUTTONS = 4;

/// MouseButtonInput - Fixed-size input state (no heap allocation)
pub const MouseButtonInput = extern struct {
    pressed: [MAX_MOUSE_BUTTONS]bool,
    just_pressed: [MAX_MOUSE_BUTTONS]bool,
    just_released: [MAX_MOUSE_BUTTONS]bool,

    pub fn init() MouseButtonInput {
        return MouseButtonInput{
            .pressed = [_]bool{false} ** MAX_MOUSE_BUTTONS,
            .just_pressed = [_]bool{false} ** MAX_MOUSE_BUTTONS,
            .just_released = [_]bool{false} ** MAX_MOUSE_BUTTONS,
        };
    }

    pub fn press(self: *MouseButtonInput, button: MouseButton) void {
        const idx = @intFromEnum(button);
        if (idx >= MAX_MOUSE_BUTTONS) return;
        if (!self.pressed[idx]) {
            self.just_pressed[idx] = true;
        }
        self.pressed[idx] = true;
    }

    pub fn release(self: *MouseButtonInput, button: MouseButton) void {
        const idx = @intFromEnum(button);
        if (idx >= MAX_MOUSE_BUTTONS) return;
        if (self.pressed[idx]) {
            self.just_released[idx] = true;
        }
        self.pressed[idx] = false;
    }

    pub fn isPressed(self: *const MouseButtonInput, button: MouseButton) bool {
        const idx = @intFromEnum(button);
        if (idx >= MAX_MOUSE_BUTTONS) return false;
        return self.pressed[idx];
    }

    pub fn isJustPressed(self: *const MouseButtonInput, button: MouseButton) bool {
        const idx = @intFromEnum(button);
        if (idx >= MAX_MOUSE_BUTTONS) return false;
        return self.just_pressed[idx];
    }

    pub fn isJustReleased(self: *const MouseButtonInput, button: MouseButton) bool {
        const idx = @intFromEnum(button);
        if (idx >= MAX_MOUSE_BUTTONS) return false;
        return self.just_released[idx];
    }

    pub fn clear(self: *MouseButtonInput) void {
        for (0..MAX_MOUSE_BUTTONS) |i| {
            self.just_pressed[i] = false;
            self.just_released[i] = false;
        }
    }

    pub fn resetAll(self: *MouseButtonInput) void {
        for (0..MAX_MOUSE_BUTTONS) |i| {
            self.pressed[i] = false;
            self.just_pressed[i] = false;
            self.just_released[i] = false;
        }
    }
};

// FFI exports for mouse button input

export fn mouse_button_input_create() MouseButtonInput {
    return MouseButtonInput.init();
}

export fn mouse_button_input_press(mouse: *MouseButtonInput, button: MouseButton) bool {
    mouse.press(button);
    return true;
}

export fn mouse_button_input_release(mouse: *MouseButtonInput, button: MouseButton) bool {
    mouse.release(button);
    return true;
}

export fn mouse_button_input_pressed(mouse: *const MouseButtonInput, button: MouseButton) bool {
    return mouse.isPressed(button);
}

export fn mouse_button_input_just_pressed(mouse: *const MouseButtonInput, button: MouseButton) bool {
    return mouse.isJustPressed(button);
}

export fn mouse_button_input_just_released(mouse: *const MouseButtonInput, button: MouseButton) bool {
    return mouse.isJustReleased(button);
}

export fn mouse_button_input_clear(mouse: *MouseButtonInput) void {
    mouse.clear();
}

export fn mouse_button_input_reset(mouse: *MouseButtonInput) void {
    mouse.resetAll();
}

// FFI exports for MouseMotion and MouseWheel

export fn mouse_motion_create(delta_x: f32, delta_y: f32) MouseMotion {
    return MouseMotion{ .delta_x = delta_x, .delta_y = delta_y };
}

export fn mouse_wheel_create(unit: MouseScrollUnit, delta_x: f32, delta_y: f32) MouseWheel {
    return MouseWheel{ .unit = unit, .delta_x = delta_x, .delta_y = delta_y };
}

// Tests

test "Mouse button basic operations" {
    const testing = std.testing;

    var mouse = mouse_button_input_create();

    // Initially not pressed
    try testing.expect(!mouse_button_input_pressed(&mouse, MouseButton.Left));

    // Press button
    try testing.expect(mouse_button_input_press(&mouse, MouseButton.Left));
    try testing.expect(mouse_button_input_pressed(&mouse, MouseButton.Left));
    try testing.expect(mouse_button_input_just_pressed(&mouse, MouseButton.Left));

    // Clear frame states
    mouse_button_input_clear(&mouse);
    try testing.expect(mouse_button_input_pressed(&mouse, MouseButton.Left));
    try testing.expect(!mouse_button_input_just_pressed(&mouse, MouseButton.Left));

    // Release button
    try testing.expect(mouse_button_input_release(&mouse, MouseButton.Left));
    try testing.expect(!mouse_button_input_pressed(&mouse, MouseButton.Left));
    try testing.expect(mouse_button_input_just_released(&mouse, MouseButton.Left));
}
