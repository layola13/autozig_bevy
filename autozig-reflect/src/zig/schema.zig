// Schema definitions for reflection system - Zig implementation (90%)

const std = @import("std");

/// Field type enumeration
pub const FieldType = enum(u8) {
    F32 = 0,
    F64 = 1,
    I8 = 2,
    I16 = 3,
    I32 = 4,
    I64 = 5,
    U8 = 6,
    U16 = 7,
    U32 = 8,
    U64 = 9,
    Bool = 10,
    String = 11,
    Struct = 12,
    Array = 13,
    List = 14,
    Unknown = 255,
};

/// Field descriptor - describes a single field in a struct
pub const FieldDesc = extern struct {
    name: [*]const u8,
    name_len: usize,
    type_name: [*]const u8,
    type_name_len: usize,
    offset: usize,
    field_type: FieldType,

    pub fn init(name: []const u8, type_name: []const u8, offset: usize, field_type: FieldType) FieldDesc {
        return FieldDesc{
            .name = name.ptr,
            .name_len = name.len,
            .type_name = type_name.ptr,
            .type_name_len = type_name.len,
            .offset = offset,
            .field_type = field_type,
        };
    }

    pub fn getName(self: *const FieldDesc) []const u8 {
        return self.name[0..self.name_len];
    }

    pub fn getTypeName(self: *const FieldDesc) []const u8 {
        return self.type_name[0..self.type_name_len];
    }
};

/// Struct descriptor - describes a complete struct
pub const StructDesc = extern struct {
    name: [*]const u8,
    name_len: usize,
    fields: [*]const FieldDesc,
    field_count: usize,
    size: usize,
    alignment: usize,

    pub fn init(name: []const u8, fields: []const FieldDesc, size: usize, alignment: usize) StructDesc {
        return StructDesc{
            .name = name.ptr,
            .name_len = name.len,
            .fields = fields.ptr,
            .field_count = fields.len,
            .size = size,
            .alignment = alignment,
        };
    }

    pub fn getName(self: *const StructDesc) []const u8 {
        return self.name[0..self.name_len];
    }

    pub fn getFields(self: *const StructDesc) []const FieldDesc {
        return self.fields[0..self.field_count];
    }

    pub fn getField(self: *const StructDesc, index: usize) ?*const FieldDesc {
        if (index >= self.field_count) return null;
        return &self.fields[index];
    }

    pub fn findField(self: *const StructDesc, name: []const u8) ?*const FieldDesc {
        const fields = self.getFields();
        for (fields) |*field| {
            const field_name = field.getName();
            if (std.mem.eql(u8, field_name, name)) {
                return field;
            }
        }
        return null;
    }
};

/// Schema registry for storing struct descriptors
pub const SchemaRegistry = struct {
    descriptors: std.ArrayList(StructDesc),
    allocator: std.mem.Allocator,

    pub fn init(allocator: std.mem.Allocator) !*SchemaRegistry {
        const registry = try allocator.create(SchemaRegistry);
        registry.* = SchemaRegistry{
            .descriptors = std.ArrayList(StructDesc).init(allocator),
            .allocator = allocator,
        };
        return registry;
    }

    pub fn deinit(self: *SchemaRegistry) void {
        self.descriptors.deinit();
        self.allocator.destroy(self);
    }

    pub fn registerStruct(self: *SchemaRegistry, desc: StructDesc) !void {
        try self.descriptors.append(desc);
    }

    pub fn findStruct(self: *const SchemaRegistry, name: []const u8) ?*const StructDesc {
        for (self.descriptors.items) |*desc| {
            const desc_name = desc.getName();
            if (std.mem.eql(u8, desc_name, name)) {
                return desc;
            }
        }
        return null;
    }

    pub fn len(self: *const SchemaRegistry) usize {
        return self.descriptors.items.len;
    }
};

// Global allocator for schema module
var schema_gpa = std.heap.GeneralPurposeAllocator(.{}){};
const schema_allocator = schema_gpa.allocator();

// FFI exports
export fn schema_registry_create() ?*SchemaRegistry {
    return SchemaRegistry.init(schema_allocator) catch null;
}

export fn schema_registry_destroy(registry: *SchemaRegistry) void {
    registry.deinit();
}

export fn schema_registry_register(
    registry: *SchemaRegistry,
    name: [*]const u8,
    name_len: usize,
    fields: [*]const FieldDesc,
    field_count: usize,
    size: usize,
    alignment: usize,
) bool {
    const name_slice = name[0..name_len];
    const fields_slice = fields[0..field_count];
    const desc = StructDesc.init(name_slice, fields_slice, size, alignment);
    registry.registerStruct(desc) catch return false;
    return true;
}

export fn schema_registry_find(
    registry: *const SchemaRegistry,
    name: [*]const u8,
    name_len: usize,
) ?*const StructDesc {
    const name_slice = name[0..name_len];
    return registry.findStruct(name_slice);
}

export fn schema_registry_len(registry: *const SchemaRegistry) usize {
    return registry.len();
}

export fn field_desc_get_offset(desc: *const FieldDesc) usize {
    return desc.offset;
}

export fn field_desc_get_type(desc: *const FieldDesc) FieldType {
    return desc.field_type;
}

export fn struct_desc_get_size(desc: *const StructDesc) usize {
    return desc.size;
}

export fn struct_desc_get_field_count(desc: *const StructDesc) usize {
    return desc.field_count;
}

export fn struct_desc_get_field(desc: *const StructDesc, index: usize) ?*const FieldDesc {
    return desc.getField(index);
}

// Tests
test "field_desc_basic" {
    const test_allocator = std.testing.allocator;

    const field = FieldDesc.init("x", "f32", 0, .F32);

    try std.testing.expectEqualStrings("x", field.getName());
    try std.testing.expectEqualStrings("f32", field.getTypeName());
    try std.testing.expectEqual(@as(usize, 0), field.offset);
    try std.testing.expectEqual(FieldType.F32, field.field_type);

    _ = test_allocator;
}

test "struct_desc_basic" {
    const test_allocator = std.testing.allocator;

    const fields = [_]FieldDesc{
        FieldDesc.init("x", "f32", 0, .F32),
        FieldDesc.init("y", "f32", 4, .F32),
    };

    const desc = StructDesc.init("Point", &fields, 8, 4);

    try std.testing.expectEqualStrings("Point", desc.getName());
    try std.testing.expectEqual(@as(usize, 2), desc.field_count);
    try std.testing.expectEqual(@as(usize, 8), desc.size);
    try std.testing.expectEqual(@as(usize, 4), desc.alignment);

    _ = test_allocator;
}

test "struct_desc_find_field" {
    const test_allocator = std.testing.allocator;

    const fields = [_]FieldDesc{
        FieldDesc.init("x", "f32", 0, .F32),
        FieldDesc.init("y", "f32", 4, .F32),
    };

    const desc = StructDesc.init("Point", &fields, 8, 4);

    const x_field = desc.findField("x");
    try std.testing.expect(x_field != null);
    try std.testing.expectEqualStrings("x", x_field.?.getName());

    const y_field = desc.findField("y");
    try std.testing.expect(y_field != null);
    try std.testing.expectEqualStrings("y", y_field.?.getName());

    const z_field = desc.findField("z");
    try std.testing.expect(z_field == null);

    _ = test_allocator;
}

test "schema_registry_basic" {
    const test_allocator = std.testing.allocator;

    var registry = try SchemaRegistry.init(test_allocator);
    defer registry.deinit();

    try std.testing.expectEqual(@as(usize, 0), registry.len());
}

test "schema_registry_register" {
    const test_allocator = std.testing.allocator;

    var registry = try SchemaRegistry.init(test_allocator);
    defer registry.deinit();

    const fields = [_]FieldDesc{
        FieldDesc.init("x", "f32", 0, .F32),
        FieldDesc.init("y", "f32", 4, .F32),
    };

    const desc = StructDesc.init("Point", &fields, 8, 4);
    try registry.registerStruct(desc);

    try std.testing.expectEqual(@as(usize, 1), registry.len());
}

test "schema_registry_find" {
    const test_allocator = std.testing.allocator;

    var registry = try SchemaRegistry.init(test_allocator);
    defer registry.deinit();

    const fields = [_]FieldDesc{
        FieldDesc.init("x", "f32", 0, .F32),
        FieldDesc.init("y", "f32", 4, .F32),
    };

    const desc = StructDesc.init("Point", &fields, 8, 4);
    try registry.registerStruct(desc);

    const found = registry.findStruct("Point");
    try std.testing.expect(found != null);
    try std.testing.expectEqualStrings("Point", found.?.getName());

    const not_found = registry.findStruct("Vector");
    try std.testing.expect(not_found == null);
}
