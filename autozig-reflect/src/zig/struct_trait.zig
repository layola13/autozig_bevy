// Struct reflection implementation in Zig (90%)

const std = @import("std");

pub const StructData = struct {
    field_names: std.ArrayList([]const u8),
    alloc: std.mem.Allocator,

    pub fn init(alloc: std.mem.Allocator, field_count: usize) !*StructData {
        const data = try alloc.create(StructData);
        data.* = StructData{
            .field_names = .{},
            .alloc = alloc,
        };
        try data.field_names.ensureTotalCapacity(alloc, field_count);
        return data;
    }

    pub fn deinit(self: *StructData) void {
        for (self.field_names.items) |name| {
            self.alloc.free(name);
        }
        self.field_names.deinit(self.alloc);
        self.alloc.destroy(self);
    }

    pub fn addField(self: *StructData, name: []const u8) !void {
        const name_copy = try self.alloc.dupe(u8, name);
        try self.field_names.append(self.alloc, name_copy);
    }

    pub fn fieldCount(self: *const StructData) usize {
        return self.field_names.items.len;
    }

    pub fn getFieldName(self: *const StructData, index: usize) ?[]const u8 {
        if (index >= self.field_names.items.len) {
            return null;
        }
        return self.field_names.items[index];
    }
};

// Global allocator for struct_trait module
var struct_trait_gpa = std.heap.GeneralPurposeAllocator(.{}){};
const struct_trait_allocator = struct_trait_gpa.allocator();

// FFI exports
export fn struct_data_create(field_count: usize) ?*StructData {
    return StructData.init(struct_trait_allocator, field_count) catch null;
}

export fn struct_data_destroy(data: *StructData) void {
    data.deinit();
}

export fn struct_data_field_count(data: *const StructData) usize {
    return data.fieldCount();
}

export fn struct_data_get_field_name(data: *const StructData, index: usize) [*]const u8 {
    if (data.getFieldName(index)) |name| {
        return name.ptr;
    }
    return "";
}

export fn struct_data_get_field_name_len(data: *const StructData, index: usize) usize {
    if (data.getFieldName(index)) |name| {
        return name.len;
    }
    return 0;
}

// Tests
test "struct_data_basic" {
    const test_alloc = std.testing.allocator;

    var data = try StructData.init(test_alloc, 2);
    defer data.deinit();

    try data.addField("x");
    try data.addField("y");

    try std.testing.expectEqual(@as(usize, 2), data.fieldCount());
}

test "struct_data_get_field" {
    const test_alloc = std.testing.allocator;

    var data = try StructData.init(test_alloc, 1);
    defer data.deinit();

    try data.addField("test_field");

    const name = data.getFieldName(0);
    try std.testing.expect(name != null);
    try std.testing.expectEqualStrings("test_field", name.?);
}
