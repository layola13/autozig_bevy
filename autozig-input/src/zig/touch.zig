const std = @import("std");

/// TouchPhase - Touch event lifecycle phases
pub const TouchPhase = enum(u8) {
    Started = 0,
    Moved = 1,
    Ended = 2,
    Cancelled = 3,
};

/// Touch - Represents a single touch point
pub const Touch = extern struct {
    id: u64,
    phase: TouchPhase,
    position_x: f32,
    position_y: f32,
};

// Maximum number of simultaneous touches
const MAX_TOUCHES = 10;

/// TouchInput - Fixed-size touch state (no heap allocation)
pub const TouchInput = extern struct {
    touches: [MAX_TOUCHES]Touch,
    active: [MAX_TOUCHES]bool,
    count: usize,

    pub fn init() TouchInput {
        return TouchInput{
            .touches = [_]Touch{Touch{ .id = 0, .phase = TouchPhase.Started, .position_x = 0, .position_y = 0 }} ** MAX_TOUCHES,
            .active = [_]bool{false} ** MAX_TOUCHES,
            .count = 0,
        };
    }

    /// Find slot for existing touch id
    fn findSlot(self: *const TouchInput, id: u64) ?usize {
        for (0..MAX_TOUCHES) |i| {
            if (self.active[i] and self.touches[i].id == id) {
                return i;
            }
        }
        return null;
    }

    /// Find first empty slot
    fn findEmptySlot(self: *const TouchInput) ?usize {
        for (0..MAX_TOUCHES) |i| {
            if (!self.active[i]) {
                return i;
            }
        }
        return null;
    }

    /// Update or add a touch
    pub fn update(self: *TouchInput, touch: Touch) bool {
        // Try to find existing touch
        if (self.findSlot(touch.id)) |slot| {
            self.touches[slot] = touch;
            return true;
        }
        // Try to find empty slot for new touch
        if (self.findEmptySlot()) |slot| {
            self.touches[slot] = touch;
            self.active[slot] = true;
            self.count += 1;
            return true;
        }
        return false; // No space
    }

    /// Remove a touch by id
    pub fn remove(self: *TouchInput, id: u64) void {
        if (self.findSlot(id)) |slot| {
            self.active[slot] = false;
            if (self.count > 0) {
                self.count -= 1;
            }
        }
    }

    /// Get a touch by id
    pub fn get(self: *const TouchInput, id: u64, out_touch: *Touch) bool {
        if (self.findSlot(id)) |slot| {
            out_touch.* = self.touches[slot];
            return true;
        }
        return false;
    }

    /// Clear all touches
    pub fn clearAll(self: *TouchInput) void {
        for (0..MAX_TOUCHES) |i| {
            self.active[i] = false;
        }
        self.count = 0;
    }
};

// FFI exports for touch input

export fn touch_input_create() TouchInput {
    return TouchInput.init();
}

export fn touch_create(id: u64, phase: TouchPhase, position_x: f32, position_y: f32) Touch {
    return Touch{
        .id = id,
        .phase = phase,
        .position_x = position_x,
        .position_y = position_y,
    };
}

export fn touch_input_update(touch_input: *TouchInput, touch: Touch) bool {
    return touch_input.update(touch);
}

export fn touch_input_remove(touch_input: *TouchInput, touch_id: u64) void {
    touch_input.remove(touch_id);
}

export fn touch_input_get(touch_input: *const TouchInput, touch_id: u64, out_touch: *Touch) bool {
    return touch_input.get(touch_id, out_touch);
}

export fn touch_input_count(touch_input: *const TouchInput) usize {
    return touch_input.count;
}

export fn touch_input_clear(touch_input: *TouchInput) void {
    touch_input.clearAll();
}

// Tests

test "Touch input basic operations" {
    const testing = std.testing;

    var touch_input = touch_input_create();

    // Initially no touches
    try testing.expectEqual(@as(usize, 0), touch_input_count(&touch_input));

    // Add a touch
    const touch = touch_create(1, TouchPhase.Started, 100.0, 200.0);
    try testing.expect(touch_input_update(&touch_input, touch));
    try testing.expectEqual(@as(usize, 1), touch_input_count(&touch_input));

    // Get the touch back
    var retrieved: Touch = undefined;
    try testing.expect(touch_input_get(&touch_input, 1, &retrieved));
    try testing.expectEqual(@as(u64, 1), retrieved.id);
    try testing.expectEqual(TouchPhase.Started, retrieved.phase);
    try testing.expectEqual(@as(f32, 100.0), retrieved.position_x);
    try testing.expectEqual(@as(f32, 200.0), retrieved.position_y);

    // Remove touch
    touch_input_remove(&touch_input, 1);
    try testing.expectEqual(@as(usize, 0), touch_input_count(&touch_input));
}
