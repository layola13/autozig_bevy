const std = @import("std");

pub const IVec4 = extern struct {
    x: i32,
    y: i32,
    z: i32,
    w: i32,

    pub fn new(x: i32, y: i32, z: i32, w: i32) IVec4 {
        return .{ .x = x, .y = y, .z = z, .w = w };
    }

    pub fn splat(value: i32) IVec4 {
        return .{ .x = value, .y = value, .z = value, .w = value };
    }

    pub fn add(self: IVec4, other: IVec4) IVec4 {
        return .{
            .x = self.x + other.x,
            .y = self.y + other.y,
            .z = self.z + other.z,
            .w = self.w + other.w,
        };
    }

    pub fn sub(self: IVec4, other: IVec4) IVec4 {
        return .{
            .x = self.x - other.x,
            .y = self.y - other.y,
            .z = self.z - other.z,
            .w = self.w - other.w,
        };
    }

    pub fn mul(self: IVec4, other: IVec4) IVec4 {
        return .{
            .x = self.x * other.x,
            .y = self.y * other.y,
            .z = self.z * other.z,
            .w = self.w * other.w,
        };
    }

    pub fn dot(self: IVec4, other: IVec4) i32 {
        return self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
    }

    pub fn min(self: IVec4, other: IVec4) IVec4 {
        return .{
            .x = @min(self.x, other.x),
            .y = @min(self.y, other.y),
            .z = @min(self.z, other.z),
            .w = @min(self.w, other.w),
        };
    }

    pub fn max(self: IVec4, other: IVec4) IVec4 {
        return .{
            .x = @max(self.x, other.x),
            .y = @max(self.y, other.y),
            .z = @max(self.z, other.z),
            .w = @max(self.w, other.w),
        };
    }

    pub fn abs(self: IVec4) IVec4 {
        return .{
            .x = if (self.x < 0) -self.x else self.x,
            .y = if (self.y < 0) -self.y else self.y,
            .z = if (self.z < 0) -self.z else self.z,
            .w = if (self.w < 0) -self.w else self.w,
        };
    }
};

export fn ivec4_new(x: i32, y: i32, z: i32, w: i32) IVec4 {
    return IVec4.new(x, y, z, w);
}

export fn ivec4_splat(value: i32) IVec4 {
    return IVec4.splat(value);
}

export fn ivec4_add(self: IVec4, other: IVec4) IVec4 {
    return self.add(other);
}

export fn ivec4_sub(self: IVec4, other: IVec4) IVec4 {
    return self.sub(other);
}

export fn ivec4_dot(self: IVec4, other: IVec4) i32 {
    return self.dot(other);
}

export fn ivec4_min(self: IVec4, other: IVec4) IVec4 {
    return self.min(other);
}

export fn ivec4_max(self: IVec4, other: IVec4) IVec4 {
    return self.max(other);
}

export fn ivec4_abs(self: IVec4) IVec4 {
    return self.abs();
}
