const std = @import("std");

// 全局allocator定义 - 所有模块共享
var gpa_instance = std.heap.GeneralPurposeAllocator(.{}){};
pub const g_allocator = gpa_instance.allocator();

// HashMap wrapper - 针对u64 key和value的优化实现
pub const HashMap = struct {
    map: std.AutoHashMap(u64, u64),

    pub fn init() HashMap {
        return HashMap{
            .map = std.AutoHashMap(u64, u64).init(g_allocator),
        };
    }

    pub fn deinit(self: *HashMap) void {
        self.map.deinit();
    }

    pub fn insert(self: *HashMap, key: u64, value: u64) !void {
        try self.map.put(key, value);
    }

    pub fn get(self: *HashMap, key: u64) ?u64 {
        return self.map.get(key);
    }

    pub fn remove(self: *HashMap, key: u64) bool {
        return self.map.remove(key);
    }

    pub fn contains(self: *HashMap, key: u64) bool {
        return self.map.contains(key);
    }

    pub fn len(self: *HashMap) usize {
        return self.map.count();
    }

    pub fn clear(self: *HashMap) void {
        self.map.clearRetainingCapacity();
    }

    pub fn capacity(self: *HashMap) usize {
        return self.map.capacity();
    }
};

// HashSet wrapper - 基于HashMap实现
pub const HashSet = struct {
    map: std.AutoHashMap(u64, void),

    pub fn init() HashSet {
        return HashSet{
            .map = std.AutoHashMap(u64, void).init(g_allocator),
        };
    }

    pub fn deinit(self: *HashSet) void {
        self.map.deinit();
    }

    pub fn insert(self: *HashSet, key: u64) !void {
        try self.map.put(key, {});
    }

    pub fn remove(self: *HashSet, key: u64) bool {
        return self.map.remove(key);
    }

    pub fn contains(self: *HashSet, key: u64) bool {
        return self.map.contains(key);
    }

    pub fn len(self: *HashSet) usize {
        return self.map.count();
    }

    pub fn clear(self: *HashSet) void {
        self.map.clearRetainingCapacity();
    }

    pub fn capacity(self: *HashSet) usize {
        return self.map.capacity();
    }
};

// FFI导出 - HashMap操作
export fn hashmap_create() *HashMap {
    const map = g_allocator.create(HashMap) catch unreachable;
    map.* = HashMap.init();
    return map;
}

export fn hashmap_destroy(map: *HashMap) void {
    map.deinit();
    g_allocator.destroy(map);
}

export fn hashmap_insert(map: *HashMap, key: u64, value: u64) bool {
    map.insert(key, value) catch return false;
    return true;
}

export fn hashmap_get(map: *HashMap, key: u64, out_value: *u64) bool {
    if (map.get(key)) |value| {
        out_value.* = value;
        return true;
    }
    return false;
}

export fn hashmap_remove(map: *HashMap, key: u64) bool {
    return map.remove(key);
}

export fn hashmap_contains(map: *HashMap, key: u64) bool {
    return map.contains(key);
}

export fn hashmap_len(map: *HashMap) usize {
    return map.len();
}

export fn hashmap_clear(map: *HashMap) void {
    map.clear();
}

export fn hashmap_capacity(map: *HashMap) usize {
    return map.capacity();
}

// FFI导出 - HashSet操作
export fn hashset_create() *HashSet {
    const set = g_allocator.create(HashSet) catch unreachable;
    set.* = HashSet.init();
    return set;
}

export fn hashset_destroy(set: *HashSet) void {
    set.deinit();
    g_allocator.destroy(set);
}

export fn hashset_insert(set: *HashSet, key: u64) bool {
    set.insert(key) catch return false;
    return true;
}

export fn hashset_remove(set: *HashSet, key: u64) bool {
    return set.remove(key);
}

export fn hashset_contains(set: *HashSet, key: u64) bool {
    return set.contains(key);
}

export fn hashset_len(set: *HashSet) usize {
    return set.len();
}

export fn hashset_clear(set: *HashSet) void {
    set.clear();
}

export fn hashset_capacity(set: *HashSet) usize {
    return set.capacity();
}

// 单元测试
test "HashMap basic operations" {
    var map = HashMap.init();
    defer map.deinit();

    // 测试插入
    try map.insert(1, 100);
    try map.insert(2, 200);
    try map.insert(3, 300);

    // 测试长度
    try std.testing.expectEqual(@as(usize, 3), map.len());

    // 测试获取
    try std.testing.expectEqual(@as(?u64, 100), map.get(1));
    try std.testing.expectEqual(@as(?u64, 200), map.get(2));
    try std.testing.expectEqual(@as(?u64, null), map.get(999));

    // 测试包含
    try std.testing.expect(map.contains(1));
    try std.testing.expect(!map.contains(999));

    // 测试删除
    try std.testing.expect(map.remove(2));
    try std.testing.expectEqual(@as(usize, 2), map.len());
    try std.testing.expect(!map.contains(2));

    // 测试清空
    map.clear();
    try std.testing.expectEqual(@as(usize, 0), map.len());
}

test "HashSet basic operations" {
    var set = HashSet.init();
    defer set.deinit();

    // 测试插入
    try set.insert(1);
    try set.insert(2);
    try set.insert(3);

    // 测试长度
    try std.testing.expectEqual(@as(usize, 3), set.len());

    // 测试包含
    try std.testing.expect(set.contains(1));
    try std.testing.expect(set.contains(2));
    try std.testing.expect(!set.contains(999));

    // 测试删除
    try std.testing.expect(set.remove(2));
    try std.testing.expectEqual(@as(usize, 2), set.len());
    try std.testing.expect(!set.contains(2));

    // 测试清空
    set.clear();
    try std.testing.expectEqual(@as(usize, 0), set.len());
}
