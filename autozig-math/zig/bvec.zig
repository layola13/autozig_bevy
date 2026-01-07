const std = @import("std");

pub const BVec2 = extern struct {
    x: bool,
    y: bool,

    pub fn new(x: bool, y: bool) BVec2 {
        return .{ .x = x, .y = y };
    }

    pub fn all(self: BVec2) bool {
        return self.x and self.y;
    }

    pub fn any(self: BVec2) bool {
        return self.x or self.y;
    }

    pub fn not(self: BVec2) BVec2 {
        return .{ .x = !self.x, .y = !self.y };
    }
};

pub const BVec3 = extern struct {
    x: bool,
    y: bool,
    z: bool,

    pub fn new(x: bool, y: bool, z: bool) BVec3 {
        return .{ .x = x, .y = y, .z = z };
    }

    pub fn all(self: BVec3) bool {
        return self.x and self.y and self.z;
    }

    pub fn any(self: BVec3) bool {
        return self.x or self.y or self.z;
    }

    pub fn not(self: BVec3) BVec3 {
        return .{ .x = !self.x, .y = !self.y, .z = !self.z };
    }
};

pub const BVec4 = extern struct {
    x: bool,
    y: bool,
    z: bool,
    w: bool,

    pub fn new(x: bool, y: bool, z: bool, w: bool) BVec4 {
        return .{ .x = x, .y = y, .z = z, .w = w };
    }

    pub fn all(self: BVec4) bool {
        return self.x and self.y and self.z and self.w;
    }

    pub fn any(self: BVec4) bool {
        return self.x or self.y or self.z or self.w;
    }

    pub fn not(self: BVec4) BVec4 {
        return .{ .x = !self.x, .y = !self.y, .z = !self.z, .w = !self.w };
    }
};

export fn bvec2_new(x: bool, y: bool) BVec2 {
    return BVec2.new(x, y);
}

export fn bvec2_all(self: BVec2) bool {
    return self.all();
}

export fn bvec2_any(self: BVec2) bool {
    return self.any();
}

export fn bvec3_new(x: bool, y: bool, z: bool) BVec3 {
    return BVec3.new(x, y, z);
}

export fn bvec3_all(self: BVec3) bool {
    return self.all();
}

export fn bvec3_any(self: BVec3) bool {
    return self.any();
}

export fn bvec4_new(x: bool, y: bool, z: bool, w: bool) BVec4 {
    return BVec4.new(x, y, z, w);
}

export fn bvec4_all(self: BVec4) bool {
    return self.all();
}

export fn bvec4_any(self: BVec4) bool {
    return self.any();
}
