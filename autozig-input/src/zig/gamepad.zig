const std = @import("std");

/// GamepadButton enumeration
pub const GamepadButton = enum(u8) {
    South = 0,
    East = 1,
    North = 2,
    West = 3,
    DPadUp = 4,
    DPadDown = 5,
    DPadLeft = 6,
    DPadRight = 7,
    LeftShoulder = 8,
    RightShoulder = 9,
    LeftTrigger = 10,
    RightTrigger = 11,
    LeftThumb = 12,
    RightThumb = 13,
    Select = 14,
    Start = 15,
    Mode = 16,
};

/// GamepadAxis enumeration
pub const GamepadAxis = enum(u8) {
    LeftStickX = 0,
    LeftStickY = 1,
    RightStickX = 2,
    RightStickY = 3,
    LeftTrigger = 4,
    RightTrigger = 5,
};

// Maximum number of gamepad buttons and axes
const MAX_GAMEPAD_BUTTONS = 17;
const MAX_GAMEPAD_AXES = 6;

/// GamepadButtonInput - Fixed-size input state (no heap allocation)
pub const GamepadButtonInput = extern struct {
    pressed: [MAX_GAMEPAD_BUTTONS]bool,
    just_pressed: [MAX_GAMEPAD_BUTTONS]bool,
    just_released: [MAX_GAMEPAD_BUTTONS]bool,

    pub fn init() GamepadButtonInput {
        return GamepadButtonInput{
            .pressed = [_]bool{false} ** MAX_GAMEPAD_BUTTONS,
            .just_pressed = [_]bool{false} ** MAX_GAMEPAD_BUTTONS,
            .just_released = [_]bool{false} ** MAX_GAMEPAD_BUTTONS,
        };
    }

    pub fn press(self: *GamepadButtonInput, button: GamepadButton) void {
        const idx = @intFromEnum(button);
        if (idx >= MAX_GAMEPAD_BUTTONS) return;
        if (!self.pressed[idx]) {
            self.just_pressed[idx] = true;
        }
        self.pressed[idx] = true;
    }

    pub fn release(self: *GamepadButtonInput, button: GamepadButton) void {
        const idx = @intFromEnum(button);
        if (idx >= MAX_GAMEPAD_BUTTONS) return;
        if (self.pressed[idx]) {
            self.just_released[idx] = true;
        }
        self.pressed[idx] = false;
    }

    pub fn isPressed(self: *const GamepadButtonInput, button: GamepadButton) bool {
        const idx = @intFromEnum(button);
        if (idx >= MAX_GAMEPAD_BUTTONS) return false;
        return self.pressed[idx];
    }

    pub fn isJustPressed(self: *const GamepadButtonInput, button: GamepadButton) bool {
        const idx = @intFromEnum(button);
        if (idx >= MAX_GAMEPAD_BUTTONS) return false;
        return self.just_pressed[idx];
    }

    pub fn isJustReleased(self: *const GamepadButtonInput, button: GamepadButton) bool {
        const idx = @intFromEnum(button);
        if (idx >= MAX_GAMEPAD_BUTTONS) return false;
        return self.just_released[idx];
    }

    pub fn clear(self: *GamepadButtonInput) void {
        for (0..MAX_GAMEPAD_BUTTONS) |i| {
            self.just_pressed[i] = false;
            self.just_released[i] = false;
        }
    }

    pub fn resetAll(self: *GamepadButtonInput) void {
        for (0..MAX_GAMEPAD_BUTTONS) |i| {
            self.pressed[i] = false;
            self.just_pressed[i] = false;
            self.just_released[i] = false;
        }
    }
};

/// GamepadAxisState - Fixed-size axis state (no heap allocation)
pub const GamepadAxisState = extern struct {
    values: [MAX_GAMEPAD_AXES]f32,

    pub fn init() GamepadAxisState {
        return GamepadAxisState{
            .values = [_]f32{0.0} ** MAX_GAMEPAD_AXES,
        };
    }

    pub fn set(self: *GamepadAxisState, axis: GamepadAxis, value: f32) bool {
        const idx = @intFromEnum(axis);
        if (idx >= MAX_GAMEPAD_AXES) return false;
        self.values[idx] = value;
        return true;
    }

    pub fn get(self: *const GamepadAxisState, axis: GamepadAxis) f32 {
        const idx = @intFromEnum(axis);
        if (idx >= MAX_GAMEPAD_AXES) return 0.0;
        return self.values[idx];
    }

    pub fn resetAll(self: *GamepadAxisState) void {
        for (0..MAX_GAMEPAD_AXES) |i| {
            self.values[i] = 0.0;
        }
    }
};

// FFI exports for gamepad button input

export fn gamepad_button_input_create() GamepadButtonInput {
    return GamepadButtonInput.init();
}

export fn gamepad_button_input_press(gamepad: *GamepadButtonInput, button: GamepadButton) bool {
    gamepad.press(button);
    return true;
}

export fn gamepad_button_input_release(gamepad: *GamepadButtonInput, button: GamepadButton) bool {
    gamepad.release(button);
    return true;
}

export fn gamepad_button_input_pressed(gamepad: *const GamepadButtonInput, button: GamepadButton) bool {
    return gamepad.isPressed(button);
}

export fn gamepad_button_input_just_pressed(gamepad: *const GamepadButtonInput, button: GamepadButton) bool {
    return gamepad.isJustPressed(button);
}

export fn gamepad_button_input_just_released(gamepad: *const GamepadButtonInput, button: GamepadButton) bool {
    return gamepad.isJustReleased(button);
}

export fn gamepad_button_input_clear(gamepad: *GamepadButtonInput) void {
    gamepad.clear();
}

export fn gamepad_button_input_reset(gamepad: *GamepadButtonInput) void {
    gamepad.resetAll();
}

// FFI exports for gamepad axis state

export fn gamepad_axis_state_create() GamepadAxisState {
    return GamepadAxisState.init();
}

export fn gamepad_axis_state_set(axis_state: *GamepadAxisState, axis: GamepadAxis, value: f32) bool {
    return axis_state.set(axis, value);
}

export fn gamepad_axis_state_get(axis_state: *const GamepadAxisState, axis: GamepadAxis) f32 {
    return axis_state.get(axis);
}

export fn gamepad_axis_state_reset(axis_state: *GamepadAxisState) void {
    axis_state.resetAll();
}

// Tests

test "Gamepad button basic operations" {
    const testing = std.testing;

    var gamepad = gamepad_button_input_create();

    // Initially not pressed
    try testing.expect(!gamepad_button_input_pressed(&gamepad, GamepadButton.South));

    // Press button
    try testing.expect(gamepad_button_input_press(&gamepad, GamepadButton.South));
    try testing.expect(gamepad_button_input_pressed(&gamepad, GamepadButton.South));
    try testing.expect(gamepad_button_input_just_pressed(&gamepad, GamepadButton.South));

    // Clear frame states
    gamepad_button_input_clear(&gamepad);
    try testing.expect(gamepad_button_input_pressed(&gamepad, GamepadButton.South));
    try testing.expect(!gamepad_button_input_just_pressed(&gamepad, GamepadButton.South));

    // Release button
    try testing.expect(gamepad_button_input_release(&gamepad, GamepadButton.South));
    try testing.expect(!gamepad_button_input_pressed(&gamepad, GamepadButton.South));
    try testing.expect(gamepad_button_input_just_released(&gamepad, GamepadButton.South));
}

test "Gamepad axis basic operations" {
    const testing = std.testing;

    var axes = gamepad_axis_state_create();

    // Initially zero
    try testing.expectEqual(@as(f32, 0.0), gamepad_axis_state_get(&axes, GamepadAxis.LeftStickX));

    // Set values
    try testing.expect(gamepad_axis_state_set(&axes, GamepadAxis.LeftStickX, 0.75));
    try testing.expectEqual(@as(f32, 0.75), gamepad_axis_state_get(&axes, GamepadAxis.LeftStickX));

    try testing.expect(gamepad_axis_state_set(&axes, GamepadAxis.RightStickY, -0.5));
    try testing.expectEqual(@as(f32, -0.5), gamepad_axis_state_get(&axes, GamepadAxis.RightStickY));

    // Reset
    gamepad_axis_state_reset(&axes);
    try testing.expectEqual(@as(f32, 0.0), gamepad_axis_state_get(&axes, GamepadAxis.LeftStickX));
    try testing.expectEqual(@as(f32, 0.0), gamepad_axis_state_get(&axes, GamepadAxis.RightStickY));
}
