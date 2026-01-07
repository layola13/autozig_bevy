const std = @import("std");

pub const UVec4 = extern struct {
    x: u32,
    y: u32,
    z: u32,
    w: u32,

    pub fn new(x: u32, y: u32, z: u32, w: u32) UVec4 {
        return .{ .x = x, .y = y, .z = z, .w = w };
    }

    pub fn splat(value: u32) UVec4 {
        return .{ .x = value, .y = value, .z = value, .w = value };
    }

    pub fn add(self: UVec4, other: UVec4) UVec4 {
        return .{
            .x = self.x + other.x,
            .y = self.y + other.y,
            .z = self.z + other.z,
            .w = self.w + other.w,
        };
    }

    pub fn sub(self: UVec4, other: UVec4) UVec4 {
        return .{
            .x = self.x - other.x,
            .y = self.y - other.y,
            .z = self.z - other.z,
            .w = self.w - other.w,
        };
    }

    pub fn mul(self: UVec4, other: UVec4) UVec4 {
        return .{
            .x = self.x * other.x,
            .y = self.y * other.y,
            .z = self.z * other.z,
            .w = self.w * other.w,
        };
    }

    pub fn dot(self: UVec4, other: UVec4) u32 {
        return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
    }

    pub fn min(self: UVec4, other: UVec4) UVec4 {
        return .{
            .x = @min(self.x, other.x),
            .y = @min(self.y, other.y),
            .z = @min(self.z, other.z),
            .w = @min(self.w, other.w),
        };
    }

    pub fn max(self: UVec4, other: UVec4) UVec4 {
        return .{
            .x = @max(self.x, other.x),
            .y = @max(self.y, other.y),
            .z = @max(self.z, other.z),
            .w = @max(self.w, other.w),
        };
    }
};

export fn uvec4_new(x: u32, y: u32, z: u32, w: u32) UVec4 {
    return UVec4.new(x, y, z, w);
}

export fn uvec4_splat(value: u32) UVec4 {
    return UVec4.splat(value);
}

export fn uvec4_add(self: UVec4, other: UVec4) UVec4 {
    return self.add(other);
}

export fn uvec4_sub(self: UVec4, other: UVec4) UVec4 {
    return self.sub(other);
}

export fn uvec4_dot(self: UVec4, other: UVec4) u32 {
    return self.dot(other);
}

export fn uvec4_min(self: UVec4, other: UVec4) UVec4 {
    return self.min(other);
}

export fn uvec4_max(self: UVec4, other: UVec4) UVec4 {
    return self.max(other);
}
