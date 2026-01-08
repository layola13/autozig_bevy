const std = @import("std");

/// KeyCode enumeration - Maps to Web KeyboardEvent.code
/// Reference: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
pub const KeyCode = enum(u32) {
    // Letters A-Z
    KeyA = 0,
    KeyB = 1,
    KeyC = 2,
    KeyD = 3,
    KeyE = 4,
    KeyF = 5,
    KeyG = 6,
    KeyH = 7,
    KeyI = 8,
    KeyJ = 9,
    KeyK = 10,
    KeyL = 11,
    KeyM = 12,
    KeyN = 13,
    KeyO = 14,
    KeyP = 15,
    KeyQ = 16,
    KeyR = 17,
    KeyS = 18,
    KeyT = 19,
    KeyU = 20,
    KeyV = 21,
    KeyW = 22,
    KeyX = 23,
    KeyY = 24,
    KeyZ = 25,

    // Digits 0-9
    Digit0 = 26,
    Digit1 = 27,
    Digit2 = 28,
    Digit3 = 29,
    Digit4 = 30,
    Digit5 = 31,
    Digit6 = 32,
    Digit7 = 33,
    Digit8 = 34,
    Digit9 = 35,

    // Function keys
    F1 = 36,
    F2 = 37,
    F3 = 38,
    F4 = 39,
    F5 = 40,
    F6 = 41,
    F7 = 42,
    F8 = 43,
    F9 = 44,
    F10 = 45,
    F11 = 46,
    F12 = 47,

    // Control keys
    Escape = 48,
    Tab = 49,
    CapsLock = 50,
    ShiftLeft = 51,
    ShiftRight = 52,
    ControlLeft = 53,
    ControlRight = 54,
    AltLeft = 55,
    AltRight = 56,
    MetaLeft = 57,
    MetaRight = 58,
    Space = 59,
    Enter = 60,
    Backspace = 61,

    // Arrow keys
    ArrowLeft = 62,
    ArrowRight = 63,
    ArrowUp = 64,
    ArrowDown = 65,

    // Editing keys
    Insert = 66,
    Delete = 67,
    Home = 68,
    End = 69,
    PageUp = 70,
    PageDown = 71,

    // Symbol keys
    Minus = 72,
    Equal = 73,
    BracketLeft = 74,
    BracketRight = 75,
    Backslash = 76,
    Semicolon = 77,
    Quote = 78,
    Comma = 79,
    Period = 80,
    Slash = 81,
    Backquote = 82,

    // Numpad keys
    Numpad0 = 83,
    Numpad1 = 84,
    Numpad2 = 85,
    Numpad3 = 86,
    Numpad4 = 87,
    Numpad5 = 88,
    Numpad6 = 89,
    Numpad7 = 90,
    Numpad8 = 91,
    Numpad9 = 92,
    NumpadAdd = 93,
    NumpadSubtract = 94,
    NumpadMultiply = 95,
    NumpadDivide = 96,
    NumpadDecimal = 97,
    NumpadEnter = 98,
    NumLock = 99,

    // Additional keys
    ScrollLock = 100,
    Pause = 101,
    PrintScreen = 102,
    ContextMenu = 103,

    // Unknown/Other
    Unknown = 255,
};

// Maximum number of key codes (256 to cover Unknown = 255)
const MAX_KEY_CODES = 256;

/// KeyboardInput - Fixed-size input state (no heap allocation)
pub const KeyboardInput = extern struct {
    pressed: [MAX_KEY_CODES]bool,
    just_pressed: [MAX_KEY_CODES]bool,
    just_released: [MAX_KEY_CODES]bool,

    pub fn init() KeyboardInput {
        return KeyboardInput{
            .pressed = [_]bool{false} ** MAX_KEY_CODES,
            .just_pressed = [_]bool{false} ** MAX_KEY_CODES,
            .just_released = [_]bool{false} ** MAX_KEY_CODES,
        };
    }

    pub fn press(self: *KeyboardInput, key: KeyCode) void {
        const idx = @intFromEnum(key);
        if (idx >= MAX_KEY_CODES) return;
        if (!self.pressed[idx]) {
            self.just_pressed[idx] = true;
        }
        self.pressed[idx] = true;
    }

    pub fn release(self: *KeyboardInput, key: KeyCode) void {
        const idx = @intFromEnum(key);
        if (idx >= MAX_KEY_CODES) return;
        if (self.pressed[idx]) {
            self.just_released[idx] = true;
        }
        self.pressed[idx] = false;
    }

    pub fn isPressed(self: *const KeyboardInput, key: KeyCode) bool {
        const idx = @intFromEnum(key);
        if (idx >= MAX_KEY_CODES) return false;
        return self.pressed[idx];
    }

    pub fn isJustPressed(self: *const KeyboardInput, key: KeyCode) bool {
        const idx = @intFromEnum(key);
        if (idx >= MAX_KEY_CODES) return false;
        return self.just_pressed[idx];
    }

    pub fn isJustReleased(self: *const KeyboardInput, key: KeyCode) bool {
        const idx = @intFromEnum(key);
        if (idx >= MAX_KEY_CODES) return false;
        return self.just_released[idx];
    }

    pub fn clear(self: *KeyboardInput) void {
        for (0..MAX_KEY_CODES) |i| {
            self.just_pressed[i] = false;
            self.just_released[i] = false;
        }
    }

    pub fn resetAll(self: *KeyboardInput) void {
        for (0..MAX_KEY_CODES) |i| {
            self.pressed[i] = false;
            self.just_pressed[i] = false;
            self.just_released[i] = false;
        }
    }
};

// FFI exports for keyboard input

export fn keyboard_input_create() KeyboardInput {
    return KeyboardInput.init();
}

export fn keyboard_input_press(keyboard: *KeyboardInput, key_code: KeyCode) bool {
    keyboard.press(key_code);
    return true;
}

export fn keyboard_input_release(keyboard: *KeyboardInput, key_code: KeyCode) bool {
    keyboard.release(key_code);
    return true;
}

export fn keyboard_input_pressed(keyboard: *const KeyboardInput, key_code: KeyCode) bool {
    return keyboard.isPressed(key_code);
}

export fn keyboard_input_just_pressed(keyboard: *const KeyboardInput, key_code: KeyCode) bool {
    return keyboard.isJustPressed(key_code);
}

export fn keyboard_input_just_released(keyboard: *const KeyboardInput, key_code: KeyCode) bool {
    return keyboard.isJustReleased(key_code);
}

export fn keyboard_input_clear(keyboard: *KeyboardInput) void {
    keyboard.clear();
}

export fn keyboard_input_reset(keyboard: *KeyboardInput) void {
    keyboard.resetAll();
}

// Tests

test "KeyCode enum values" {
    const testing = std.testing;

    try testing.expectEqual(@as(u32, 0), @intFromEnum(KeyCode.KeyA));
    try testing.expectEqual(@as(u32, 25), @intFromEnum(KeyCode.KeyZ));
    try testing.expectEqual(@as(u32, 26), @intFromEnum(KeyCode.Digit0));
    try testing.expectEqual(@as(u32, 59), @intFromEnum(KeyCode.Space));
    try testing.expectEqual(@as(u32, 255), @intFromEnum(KeyCode.Unknown));
}

test "Keyboard input basic operations" {
    const testing = std.testing;

    var keyboard = keyboard_input_create();

    // Initially not pressed
    try testing.expect(!keyboard_input_pressed(&keyboard, KeyCode.KeyA));

    // Press key
    try testing.expect(keyboard_input_press(&keyboard, KeyCode.KeyA));
    try testing.expect(keyboard_input_pressed(&keyboard, KeyCode.KeyA));
    try testing.expect(keyboard_input_just_pressed(&keyboard, KeyCode.KeyA));

    // Clear frame states
    keyboard_input_clear(&keyboard);
    try testing.expect(keyboard_input_pressed(&keyboard, KeyCode.KeyA));
    try testing.expect(!keyboard_input_just_pressed(&keyboard, KeyCode.KeyA));

    // Release key
    try testing.expect(keyboard_input_release(&keyboard, KeyCode.KeyA));
    try testing.expect(!keyboard_input_pressed(&keyboard, KeyCode.KeyA));
    try testing.expect(keyboard_input_just_released(&keyboard, KeyCode.KeyA));
}
