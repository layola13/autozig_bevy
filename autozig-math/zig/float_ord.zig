const std = @import("std");

/// A wrapper around `f32` that provides ordering and hashing
pub const FloatOrd = extern struct {
    value: f32,

    pub fn new(value: f32) FloatOrd {
        return .{ .value = value };
    }

    /// Compare for ordering (handles NaN by treating it as greater than all other values)
    pub fn cmp(self: FloatOrd, other: FloatOrd) i32 {
        const a = self.value;
        const b = other.value;

        // Handle NaN cases
        const a_nan = std.math.isNan(a);
        const b_nan = std.math.isNan(b);

        if (a_nan and b_nan) return 0;
        if (a_nan) return 1;
        if (b_nan) return -1;

        // Normal comparison
        if (a < b) return -1;
        if (a > b) return 1;

        // Handle -0.0 vs 0.0
        const a_bits = @as(u32, @bitCast(a));
        const b_bits = @as(u32, @bitCast(b));
        if (a_bits < b_bits) return -1;
        if (a_bits > b_bits) return 1;
        return 0;
    }

    pub fn eq(self: FloatOrd, other: FloatOrd) bool {
        return self.cmp(other) == 0;
    }

    /// Hash the float value
    pub fn hash(self: FloatOrd) u32 {
        return @as(u32, @bitCast(self.value));
    }
};

export fn float_ord_new(value: f32) FloatOrd {
    return FloatOrd.new(value);
}

export fn float_ord_cmp(self: FloatOrd, other: FloatOrd) i32 {
    return self.cmp(other);
}

export fn float_ord_hash(self: FloatOrd) u32 {
    return self.hash();
}
