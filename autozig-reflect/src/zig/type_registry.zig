// Type registry implementation in Zig (90%)

const std = @import("std");

// Type registration entry
pub const TypeEntry = struct {
    type_id: u64,
    type_name: []const u8,
    alloc: std.mem.Allocator,

    pub fn init(alloc: std.mem.Allocator, type_id: u64, type_name: []const u8) !TypeEntry {
        const name_copy = try alloc.dupe(u8, type_name);
        return TypeEntry{
            .type_id = type_id,
            .type_name = name_copy,
            .alloc = alloc,
        };
    }

    pub fn deinit(self: *TypeEntry) void {
        self.alloc.free(self.type_name);
    }
};

// Type registry structure
pub const TypeRegistry = struct {
    entries: std.AutoHashMap(u64, TypeEntry),
    alloc: std.mem.Allocator,

    pub fn init(alloc: std.mem.Allocator) !*TypeRegistry {
        const registry = try alloc.create(TypeRegistry);
        registry.* = TypeRegistry{
            .entries = std.AutoHashMap(u64, TypeEntry).init(alloc),
            .alloc = alloc,
        };
        return registry;
    }

    pub fn deinit(self: *TypeRegistry) void {
        var iter = self.entries.valueIterator();
        while (iter.next()) |entry| {
            var entry_mut = entry.*;
            entry_mut.deinit();
        }
        self.entries.deinit();
        self.alloc.destroy(self);
    }

    pub fn register(self: *TypeRegistry, type_id: u64, type_name: []const u8) !void {
        if (self.entries.contains(type_id)) {
            return; // Already registered
        }

        const entry = try TypeEntry.init(self.alloc, type_id, type_name);
        try self.entries.put(type_id, entry);
    }

    pub fn getTypeName(self: *const TypeRegistry, type_id: u64) ?[]const u8 {
        if (self.entries.get(type_id)) |entry| {
            return entry.type_name;
        }
        return null;
    }

    pub fn contains(self: *const TypeRegistry, type_id: u64) bool {
        return self.entries.contains(type_id);
    }

    pub fn len(self: *const TypeRegistry) usize {
        return self.entries.count();
    }
};

// Global allocator for type_registry module
var type_registry_gpa = std.heap.GeneralPurposeAllocator(.{}){};
const type_registry_allocator = type_registry_gpa.allocator();

// FFI exports
export fn type_registry_create() ?*TypeRegistry {
    return TypeRegistry.init(type_registry_allocator) catch null;
}

export fn type_registry_destroy(registry: *TypeRegistry) void {
    registry.deinit();
}

export fn type_registry_register(
    registry: *TypeRegistry,
    type_id: u64,
    type_name: [*]const u8,
    type_name_len: usize,
) bool {
    const name_slice = type_name[0..type_name_len];
    registry.register(type_id, name_slice) catch return false;
    return true;
}

export fn type_registry_get_type_name(
    registry: *const TypeRegistry,
    type_id: u64,
) [*]const u8 {
    if (registry.getTypeName(type_id)) |name| {
        return name.ptr;
    }
    return "";
}

export fn type_registry_get_type_name_len(
    registry: *const TypeRegistry,
    type_id: u64,
) usize {
    if (registry.getTypeName(type_id)) |name| {
        return name.len;
    }
    return 0;
}

export fn type_registry_contains(
    registry: *const TypeRegistry,
    type_id: u64,
) bool {
    return registry.contains(type_id);
}

export fn type_registry_len(registry: *const TypeRegistry) usize {
    return registry.len();
}

// Tests
test "type_registry_basic" {
    const test_alloc = std.testing.allocator;

    var registry = try TypeRegistry.init(test_alloc);
    defer registry.deinit();

    try std.testing.expectEqual(@as(usize, 0), registry.len());
}

test "type_registry_register" {
    const test_alloc = std.testing.allocator;

    var registry = try TypeRegistry.init(test_alloc);
    defer registry.deinit();

    try registry.register(1, "i32");
    try registry.register(2, "f64");

    try std.testing.expectEqual(@as(usize, 2), registry.len());
    try std.testing.expect(registry.contains(1));
    try std.testing.expect(registry.contains(2));
}

test "type_registry_get_name" {
    const test_alloc = std.testing.allocator;

    var registry = try TypeRegistry.init(test_alloc);
    defer registry.deinit();

    try registry.register(42, "TestType");

    const name = registry.getTypeName(42);
    try std.testing.expect(name != null);
    try std.testing.expectEqualStrings("TestType", name.?);
}
