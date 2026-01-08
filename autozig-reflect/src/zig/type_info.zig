// Type information storage and management - Zig implementation (90%)

const std = @import("std");

// Type info kind matching Rust enum
pub const TypeInfoKind = enum(u8) {
    Struct = 0,
    TupleStruct = 1,
    Tuple = 2,
    List = 3,
    Array = 4,
    Map = 5,
    Enum = 6,
    Value = 7,
};

// Field information
pub const FieldInfo = struct {
    name: []const u8,
    type_name: []const u8,
    offset: usize,

    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, name: []const u8, type_name: []const u8, offset: usize) !FieldInfo {
        const name_copy = try allocator.dupe(u8, name);
        const type_name_copy = try allocator.dupe(u8, type_name);

        return FieldInfo{
            .name = name_copy,
            .type_name = type_name_copy,
            .offset = offset,
            .allocator = allocator,
        };
    }

    pub fn deinit(self: *FieldInfo) void {
        self.allocator.free(self.name);
        self.allocator.free(self.type_name);
    }
};

// Main type info structure
pub const TypeInfo = struct {
    type_name: []const u8,
    type_id: u64,
    kind: TypeInfoKind,
    fields: std.ArrayList(FieldInfo),
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator, type_name: []const u8, type_id: u64, kind: TypeInfoKind) !*TypeInfo {
        const info = try allocator.create(TypeInfo);
        const name_copy = try allocator.dupe(u8, type_name);

        info.* = TypeInfo{
            .type_name = name_copy,
            .type_id = type_id,
            .kind = kind,
            .fields = .{},
            .allocator = allocator,
        };

        return info;
    }

    pub fn deinit(self: *TypeInfo) void {
        for (self.fields.items) |*field| {
            field.deinit();
        }
        self.fields.deinit(self.allocator);
        self.allocator.free(self.type_name);
        self.allocator.destroy(self);
    }

    pub fn addField(self: *TypeInfo, name: []const u8, type_name: []const u8, offset: usize) !void {
        const field = try FieldInfo.init(self.allocator, name, type_name, offset);
        try self.fields.append(self.allocator, field);
    }
};
// Global allocator for type_info module
var type_info_gpa = std.heap.GeneralPurposeAllocator(.{}){};
const type_info_allocator = type_info_gpa.allocator();

// FFI exports
export fn type_info_create(
    type_name: [*]const u8,
    type_name_len: usize,
    type_id: u64,
    kind: TypeInfoKind,
) ?*TypeInfo {
    const name_slice = type_name[0..type_name_len];
    return TypeInfo.init(type_info_allocator, name_slice, type_id, kind) catch null;
}

export fn type_info_destroy(info: *TypeInfo) void {
    info.deinit();
}

export fn type_info_get_name(info: *const TypeInfo) [*]const u8 {
    return info.type_name.ptr;
}

export fn type_info_get_name_len(info: *const TypeInfo) usize {
    return info.type_name.len;
}

export fn type_info_get_type_id(info: *const TypeInfo) u64 {
    return info.type_id;
}

export fn type_info_get_kind(info: *const TypeInfo) TypeInfoKind {
    return info.kind;
}

export fn type_info_set_field_count(info: *TypeInfo, count: usize) void {
    info.fields.ensureTotalCapacity(type_info_allocator, count) catch return;
}

export fn type_info_get_field_count(info: *const TypeInfo) usize {
    return info.fields.items.len;
}

export fn type_info_add_field(
    info: *TypeInfo,
    field_name: [*]const u8,
    field_name_len: usize,
    field_type_name: [*]const u8,
    field_type_name_len: usize,
    field_offset: usize,
) bool {
    const name_slice = field_name[0..field_name_len];
    const type_name_slice = field_type_name[0..field_type_name_len];

    info.addField(name_slice, type_name_slice, field_offset) catch return false;
    return true;
}

export fn type_info_get_field_name(info: *const TypeInfo, index: usize) [*]const u8 {
    if (index >= info.fields.items.len) return "";
    return info.fields.items[index].name.ptr;
}

export fn type_info_get_field_name_len(info: *const TypeInfo, index: usize) usize {
    if (index >= info.fields.items.len) return 0;
    return info.fields.items[index].name.len;
}

// Tests
test "type_info_basic" {
    const test_allocator = std.testing.allocator;

    var info = try TypeInfo.init(test_allocator, "TestStruct", 12345, .Struct);
    defer info.deinit();

    try std.testing.expectEqualStrings("TestStruct", info.type_name);
    try std.testing.expectEqual(@as(u64, 12345), info.type_id);
    try std.testing.expectEqual(TypeInfoKind.Struct, info.kind);
}

test "type_info_fields" {
    const test_allocator = std.testing.allocator;

    var info = try TypeInfo.init(test_allocator, "Point", 54321, .Struct);
    defer info.deinit();

    try info.addField("x", "f32", 0);
    try info.addField("y", "f32", 4);

    try std.testing.expectEqual(@as(usize, 2), info.fields.items.len);
    try std.testing.expectEqualStrings("x", info.fields.items[0].name);
    try std.testing.expectEqualStrings("y", info.fields.items[1].name);
}
