// Inspector implementation - generic accessor through pointer offsets - Zig implementation (90%)

const std = @import("std");
const schema = @import("schema.zig");

/// Field value union for runtime inspection
pub const FieldValue = union(schema.FieldType) {
    F32: f32,
    F64: f64,
    I8: i8,
    I16: i16,
    I32: i32,
    I64: i64,
    U8: u8,
    U16: u16,
    U32: u32,
    U64: u64,
    Bool: bool,
    String: []const u8,
    Struct: []const u8, // Raw bytes
    Array: []const u8, // Raw bytes
    List: []const u8, // Raw bytes
    Unknown: void,

    pub fn format(self: FieldValue, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;

        switch (self) {
            .F32 => |v| try writer.print("{d}", .{v}),
            .F64 => |v| try writer.print("{d}", .{v}),
            .I8 => |v| try writer.print("{d}", .{v}),
            .I16 => |v| try writer.print("{d}", .{v}),
            .I32 => |v| try writer.print("{d}", .{v}),
            .I64 => |v| try writer.print("{d}", .{v}),
            .U8 => |v| try writer.print("{d}", .{v}),
            .U16 => |v| try writer.print("{d}", .{v}),
            .U32 => |v| try writer.print("{d}", .{v}),
            .U64 => |v| try writer.print("{d}", .{v}),
            .Bool => |v| try writer.print("{}", .{v}),
            .String => |v| try writer.print("\"{s}\"", .{v}),
            .Struct => try writer.print("<struct>", .{}),
            .Array => try writer.print("<array>", .{}),
            .List => try writer.print("<list>", .{}),
            .Unknown => try writer.print("<unknown>", .{}),
        }
    }
};

/// Inspector for accessing struct fields at runtime
pub const Inspector = struct {
    data_ptr: [*]const u8,
    desc: *const schema.StructDesc,

    pub fn init(data_ptr: [*]const u8, desc: *const schema.StructDesc) Inspector {
        return Inspector{
            .data_ptr = data_ptr,
            .desc = desc,
        };
    }

    /// Get field value by index
    pub fn getFieldValue(self: *const Inspector, index: usize) ?FieldValue {
        const field = self.desc.getField(index) orelse return null;
        const field_ptr = self.data_ptr + field.offset;

        return switch (field.field_type) {
            .F32 => FieldValue{ .F32 = @as(*const f32, @ptrCast(@alignCast(field_ptr))).* },
            .F64 => FieldValue{ .F64 = @as(*const f64, @ptrCast(@alignCast(field_ptr))).* },
            .I8 => FieldValue{ .I8 = @as(*const i8, @ptrCast(@alignCast(field_ptr))).* },
            .I16 => FieldValue{ .I16 = @as(*const i16, @ptrCast(@alignCast(field_ptr))).* },
            .I32 => FieldValue{ .I32 = @as(*const i32, @ptrCast(@alignCast(field_ptr))).* },
            .I64 => FieldValue{ .I64 = @as(*const i64, @ptrCast(@alignCast(field_ptr))).* },
            .U8 => FieldValue{ .U8 = @as(*const u8, @ptrCast(@alignCast(field_ptr))).* },
            .U16 => FieldValue{ .U16 = @as(*const u16, @ptrCast(@alignCast(field_ptr))).* },
            .U32 => FieldValue{ .U32 = @as(*const u32, @ptrCast(@alignCast(field_ptr))).* },
            .U64 => FieldValue{ .U64 = @as(*const u64, @ptrCast(@alignCast(field_ptr))).* },
            .Bool => FieldValue{ .Bool = @as(*const bool, @ptrCast(@alignCast(field_ptr))).* },
            else => FieldValue{ .Unknown = {} },
        };
    }

    /// Get field value by name
    pub fn getFieldValueByName(self: *const Inspector, name: []const u8) ?FieldValue {
        const field = self.desc.findField(name) orelse return null;
        const field_ptr = self.data_ptr + field.offset;

        return switch (field.field_type) {
            .F32 => FieldValue{ .F32 = @as(*const f32, @ptrCast(@alignCast(field_ptr))).* },
            .F64 => FieldValue{ .F64 = @as(*const f64, @ptrCast(@alignCast(field_ptr))).* },
            .I8 => FieldValue{ .I8 = @as(*const i8, @ptrCast(@alignCast(field_ptr))).* },
            .I16 => FieldValue{ .I16 = @as(*const i16, @ptrCast(@alignCast(field_ptr))).* },
            .I32 => FieldValue{ .I32 = @as(*const i32, @ptrCast(@alignCast(field_ptr))).* },
            .I64 => FieldValue{ .I64 = @as(*const i64, @ptrCast(@alignCast(field_ptr))).* },
            .U8 => FieldValue{ .U8 = @as(*const u8, @ptrCast(@alignCast(field_ptr))).* },
            .U16 => FieldValue{ .U16 = @as(*const u16, @ptrCast(@alignCast(field_ptr))).* },
            .U32 => FieldValue{ .U32 = @as(*const u32, @ptrCast(@alignCast(field_ptr))).* },
            .U64 => FieldValue{ .U64 = @as(*const u64, @ptrCast(@alignCast(field_ptr))).* },
            .Bool => FieldValue{ .Bool = @as(*const bool, @ptrCast(@alignCast(field_ptr))).* },
            else => FieldValue{ .Unknown = {} },
        };
    }

    /// Print all fields for debugging
    pub fn printFields(self: *const Inspector, writer: anytype) !void {
        try writer.print("Struct: {s}\n", .{self.desc.getName()});
        const fields = self.desc.getFields();
        for (fields, 0..) |field, i| {
            const value = self.getFieldValue(i);
            try writer.print("  {s}: {s} = ", .{ field.getName(), field.getTypeName() });
            if (value) |v| {
                try writer.print("{}\n", .{v});
            } else {
                try writer.print("<error>\n", .{});
            }
        }
    }
};

// Global allocator for inspector module
var inspector_gpa = std.heap.GeneralPurposeAllocator(.{}){};
const inspector_allocator = inspector_gpa.allocator();

// FFI exports
export fn inspector_create(data_ptr: [*]const u8, desc: *const schema.StructDesc) ?*Inspector {
    const inspector = inspector_allocator.create(Inspector) catch return null;
    inspector.* = Inspector.init(data_ptr, desc);
    return inspector;
}

export fn inspector_destroy(inspector: *Inspector) void {
    inspector_allocator.destroy(inspector);
}

export fn inspector_get_field_f32(inspector: *const Inspector, index: usize) f32 {
    if (inspector.getFieldValue(index)) |value| {
        return switch (value) {
            .F32 => |v| v,
            else => 0.0,
        };
    }
    return 0.0;
}

export fn inspector_get_field_i32(inspector: *const Inspector, index: usize) i32 {
    if (inspector.getFieldValue(index)) |value| {
        return switch (value) {
            .I32 => |v| v,
            else => 0,
        };
    }
    return 0;
}

export fn inspector_get_field_bool(inspector: *const Inspector, index: usize) bool {
    if (inspector.getFieldValue(index)) |value| {
        return switch (value) {
            .Bool => |v| v,
            else => false,
        };
    }
    return false;
}

export fn inspector_get_field_by_name_f32(
    inspector: *const Inspector,
    name: [*]const u8,
    name_len: usize,
) f32 {
    const name_slice = name[0..name_len];
    if (inspector.getFieldValueByName(name_slice)) |value| {
        return switch (value) {
            .F32 => |v| v,
            else => 0.0,
        };
    }
    return 0.0;
}

export fn inspector_get_field_by_name_i32(
    inspector: *const Inspector,
    name: [*]const u8,
    name_len: usize,
) i32 {
    const name_slice = name[0..name_len];
    if (inspector.getFieldValueByName(name_slice)) |value| {
        return switch (value) {
            .I32 => |v| v,
            else => 0,
        };
    }
    return 0;
}

// Tests
test "inspector_basic" {
    const test_allocator = std.testing.allocator;

    // Create a test struct descriptor
    const fields = [_]schema.FieldDesc{
        schema.FieldDesc.init("x", "f32", 0, .F32),
        schema.FieldDesc.init("y", "f32", 4, .F32),
    };

    const desc = schema.StructDesc.init("Point", &fields, 8, 4);

    // Create test data
    const Point = extern struct {
        x: f32,
        y: f32,
    };

    const point = Point{ .x = 10.5, .y = 20.3 };
    const data_ptr = @as([*]const u8, @ptrCast(&point));

    const inspector = Inspector.init(data_ptr, &desc);

    const x_value = inspector.getFieldValue(0);
    try std.testing.expect(x_value != null);
    try std.testing.expectEqual(@as(f32, 10.5), x_value.?.F32);

    const y_value = inspector.getFieldValue(1);
    try std.testing.expect(y_value != null);
    try std.testing.expectEqual(@as(f32, 20.3), y_value.?.F32);

    _ = test_allocator;
}

test "inspector_by_name" {
    const test_allocator = std.testing.allocator;

    const fields = [_]schema.FieldDesc{
        schema.FieldDesc.init("x", "f32", 0, .F32),
        schema.FieldDesc.init("y", "f32", 4, .F32),
    };

    const desc = schema.StructDesc.init("Point", &fields, 8, 4);

    const Point = extern struct {
        x: f32,
        y: f32,
    };

    const point = Point{ .x = 100.0, .y = 200.0 };
    const data_ptr = @as([*]const u8, @ptrCast(&point));

    const inspector = Inspector.init(data_ptr, &desc);

    const x_value = inspector.getFieldValueByName("x");
    try std.testing.expect(x_value != null);
    try std.testing.expectEqual(@as(f32, 100.0), x_value.?.F32);

    const y_value = inspector.getFieldValueByName("y");
    try std.testing.expect(y_value != null);
    try std.testing.expectEqual(@as(f32, 200.0), y_value.?.F32);

    const z_value = inspector.getFieldValueByName("z");
    try std.testing.expect(z_value == null);

    _ = test_allocator;
}

test "inspector_different_types" {
    const test_allocator = std.testing.allocator;

    const fields = [_]schema.FieldDesc{
        schema.FieldDesc.init("int_field", "i32", 0, .I32),
        schema.FieldDesc.init("float_field", "f32", 4, .F32),
        schema.FieldDesc.init("bool_field", "bool", 8, .Bool),
    };

    const desc = schema.StructDesc.init("TestStruct", &fields, 12, 4);

    const TestStruct = extern struct {
        int_field: i32,
        float_field: f32,
        bool_field: bool,
    };

    const test_data = TestStruct{
        .int_field = 42,
        .float_field = 3.14,
        .bool_field = true,
    };

    const data_ptr = @as([*]const u8, @ptrCast(&test_data));
    const inspector = Inspector.init(data_ptr, &desc);

    const int_val = inspector.getFieldValue(0);
    try std.testing.expect(int_val != null);
    try std.testing.expectEqual(@as(i32, 42), int_val.?.I32);

    const float_val = inspector.getFieldValue(1);
    try std.testing.expect(float_val != null);
    try std.testing.expectApproxEqAbs(@as(f32, 3.14), float_val.?.F32, 0.001);

    const bool_val = inspector.getFieldValue(2);
    try std.testing.expect(bool_val != null);
    try std.testing.expectEqual(true, bool_val.?.Bool);

    _ = test_allocator;
}
