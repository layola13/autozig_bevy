//! Uniform buffer layout calculation for WebGPU
//! 85% Zig implementation - WGSL alignment and layout rules

const std = @import("std");

// ============================================================================
// Types
// ============================================================================

/// Uniform field information
pub const UniformField = extern struct {
    offset: u32,
    size: u32,
    alignment: u32,
    _padding: u32,
};

/// Bind group layout entry
pub const BindGroupLayoutEntry = extern struct {
    binding: u32,
    visibility: u32,
    buffer_type: u32,
    min_binding_size: u64,
};

// ============================================================================
// WGSL Alignment Rules (std140 and std430)
// ============================================================================

/// Get std140 alignment for a given size
/// std140 rules:
/// - scalars: 4 bytes (f32, i32, u32)
/// - vec2: 8 bytes
/// - vec3: 16 bytes (aligned as vec4)
/// - vec4: 16 bytes
/// - mat4x4: 16 bytes per column (64 bytes total)
export fn uniform_layout_get_std140_alignment(size: u32) u32 {
    if (size <= 4) {
        return 4; // scalar (f32, i32, u32, bool)
    } else if (size <= 8) {
        return 8; // vec2
    } else if (size <= 12) {
        return 16; // vec3 (treated as vec4)
    } else if (size <= 16) {
        return 16; // vec4, mat2x2
    } else if (size <= 32) {
        return 16; // mat3x2, mat2x3, mat4x2, mat2x4
    } else if (size <= 48) {
        return 16; // mat3x3 (3 columns, 16-byte aligned each)
    } else if (size <= 64) {
        return 16; // mat4x3, mat3x4
    } else {
        return 16; // mat4x4 and larger structures
    }
}

/// Get std430 alignment for a given size (more compact)
/// std430 rules:
/// - scalars: 4 bytes
/// - vec2: 8 bytes
/// - vec3: 16 bytes
/// - vec4: 16 bytes
/// - arrays: element alignment
export fn uniform_layout_get_std430_alignment(size: u32) u32 {
    if (size <= 4) {
        return 4; // scalar
    } else if (size <= 8) {
        return 8; // vec2
    } else if (size <= 12) {
        return 16; // vec3
    } else {
        return 16; // vec4 and larger
    }
}

// ============================================================================
// Offset Alignment
// ============================================================================

/// Align an offset to the required alignment
/// Returns the next aligned offset >= the input offset
export fn uniform_layout_align_offset(offset: u64, alignment: u32) u64 {
    if (alignment == 0) return offset;
    const align64: u64 = alignment;
    const mask: u64 = align64 - 1;
    return (offset + mask) & ~mask;
}

// ============================================================================
// Layout Calculation
// ============================================================================

/// Calculate total size of a uniform buffer given field layout
export fn uniform_layout_calculate_size(fields: [*]const UniformField, field_count: usize) u64 {
    if (field_count == 0) return 0;

    var max_end: u64 = 0;
    var i: usize = 0;
    while (i < field_count) : (i += 1) {
        const field = fields[i];
        const field_end = @as(u64, field.offset) + @as(u64, field.size);
        if (field_end > max_end) {
            max_end = field_end;
        }
    }

    // Align to 16 bytes (WGSL uniform buffer alignment requirement)
    return uniform_layout_align_offset(max_end, 16);
}

/// Calculate the minimum required alignment for the uniform buffer
export fn uniform_layout_calculate_alignment(fields: [*]const UniformField, field_count: usize) u32 {
    if (field_count == 0) return 16; // Minimum alignment for uniform buffers

    var max_alignment: u32 = 16;
    var i: usize = 0;
    while (i < field_count) : (i += 1) {
        const field = fields[i];
        if (field.alignment > max_alignment) {
            max_alignment = field.alignment;
        }
    }

    return max_alignment;
}

// ============================================================================
// Bind Group Layout Generation
// ============================================================================

/// Create bind group layout entries for common uniform buffer setup
/// Returns the number of entries created
export fn uniform_layout_create_bind_group_layout(
    entries: [*]BindGroupLayoutEntry,
    max_entries: usize,
    visibility: u32,
) usize {
    if (max_entries == 0) return 0;

    // Entry 0: Uniform buffer (common for transform matrices)
    entries[0] = BindGroupLayoutEntry{
        .binding = 0,
        .visibility = visibility,
        .buffer_type = 0, // Uniform
        .min_binding_size = 256, // Minimum size for transform data
    };

    if (max_entries < 2) return 1;

    // Entry 1: Material uniform buffer
    entries[1] = BindGroupLayoutEntry{
        .binding = 1,
        .visibility = visibility,
        .buffer_type = 0, // Uniform
        .min_binding_size = 64, // Minimum size for material data
    };

    if (max_entries < 3) return 2;

    // Entry 2: Light uniform buffer
    entries[2] = BindGroupLayoutEntry{
        .binding = 2,
        .visibility = visibility,
        .buffer_type = 0, // Uniform
        .min_binding_size = 1024, // Size for light data array
    };

    return 3;
}

// ============================================================================
// Common Type Sizes and Alignments
// ============================================================================

/// Get size and alignment for common WGSL types
pub const WgslType = enum {
    f32,
    i32,
    u32,
    vec2f,
    vec3f,
    vec4f,
    mat3x3f,
    mat4x4f,
};

pub fn getTypeSize(wgsl_type: WgslType) u32 {
    return switch (wgsl_type) {
        .f32, .i32, .u32 => 4,
        .vec2f => 8,
        .vec3f => 12,
        .vec4f => 16,
        .mat3x3f => 48, // 3 columns * 16 bytes (vec4 alignment)
        .mat4x4f => 64, // 4 columns * 16 bytes
    };
}

pub fn getTypeAlignment(wgsl_type: WgslType) u32 {
    return switch (wgsl_type) {
        .f32, .i32, .u32 => 4,
        .vec2f => 8,
        .vec3f => 16, // Aligned as vec4 in std140
        .vec4f => 16,
        .mat3x3f => 16, // Column alignment
        .mat4x4f => 16, // Column alignment
    };
}

/// Create a uniform field descriptor
export fn uniform_field_create(offset: u32, size: u32, alignment: u32) UniformField {
    return UniformField{
        .offset = offset,
        .size = size,
        .alignment = alignment,
        ._padding = 0,
    };
}

// ============================================================================
// Layout Builder Helper
// ============================================================================

pub const LayoutBuilder = struct {
    current_offset: u32,
    max_alignment: u32,
    fields: [64]UniformField,
    field_count: usize,

    pub fn init() LayoutBuilder {
        return LayoutBuilder{
            .current_offset = 0,
            .max_alignment = 16,
            .fields = undefined,
            .field_count = 0,
        };
    }

    pub fn addField(self: *LayoutBuilder, size: u32, alignment: u32) !void {
        if (self.field_count >= self.fields.len) {
            return error.TooManyFields;
        }

        // Align current offset
        self.current_offset = @intCast(uniform_layout_align_offset(self.current_offset, alignment));

        // Create field
        self.fields[self.field_count] = UniformField{
            .offset = self.current_offset,
            .size = size,
            .alignment = alignment,
            ._padding = 0,
        };

        self.field_count += 1;
        self.current_offset += size;

        if (alignment > self.max_alignment) {
            self.max_alignment = alignment;
        }
    }

    pub fn getTotalSize(self: *const LayoutBuilder) u32 {
        return @intCast(uniform_layout_align_offset(self.current_offset, self.max_alignment));
    }

    pub fn getAlignment(self: *const LayoutBuilder) u32 {
        return self.max_alignment;
    }
};

// ============================================================================
// Exported Helper Functions
// ============================================================================

/// Calculate padding needed to align to next boundary
export fn uniform_layout_calculate_padding(offset: u32, alignment: u32) u32 {
    const aligned = uniform_layout_align_offset(offset, alignment);
    return @intCast(aligned - offset);
}

/// Check if offset is properly aligned
export fn uniform_layout_is_aligned(offset: u32, alignment: u32) bool {
    if (alignment == 0) return true;
    return (offset % alignment) == 0;
}

/// Get the offset of next field after current field
export fn uniform_layout_next_field_offset(current_offset: u32, current_size: u32, next_alignment: u32) u32 {
    const next_offset = current_offset + current_size;
    return @intCast(uniform_layout_align_offset(next_offset, next_alignment));
}

// ============================================================================
// Tests
// ============================================================================

test "uniform_layout_std140_alignment" {
    try std.testing.expectEqual(@as(u32, 4), uniform_layout_get_std140_alignment(4)); // f32
    try std.testing.expectEqual(@as(u32, 8), uniform_layout_get_std140_alignment(8)); // vec2
    try std.testing.expectEqual(@as(u32, 16), uniform_layout_get_std140_alignment(12)); // vec3
    try std.testing.expectEqual(@as(u32, 16), uniform_layout_get_std140_alignment(16)); // vec4
    try std.testing.expectEqual(@as(u32, 16), uniform_layout_get_std140_alignment(64)); // mat4x4
}

test "uniform_layout_align_offset" {
    try std.testing.expectEqual(@as(u64, 0), uniform_layout_align_offset(0, 16));
    try std.testing.expectEqual(@as(u64, 16), uniform_layout_align_offset(1, 16));
    try std.testing.expectEqual(@as(u64, 16), uniform_layout_align_offset(15, 16));
    try std.testing.expectEqual(@as(u64, 16), uniform_layout_align_offset(16, 16));
    try std.testing.expectEqual(@as(u64, 32), uniform_layout_align_offset(17, 16));
}

test "uniform_layout_calculate_size" {
    var fields = [_]UniformField{
        .{ .offset = 0, .size = 16, .alignment = 16, ._padding = 0 }, // mat4x4 column
        .{ .offset = 16, .size = 16, .alignment = 16, ._padding = 0 }, // mat4x4 column
        .{ .offset = 32, .size = 4, .alignment = 4, ._padding = 0 }, // f32
    };

    const size = uniform_layout_calculate_size(&fields, fields.len);
    try std.testing.expect(size >= 36); // At least 32 + 4
    try std.testing.expect(size % 16 == 0); // Aligned to 16
}

test "uniform_layout_is_aligned" {
    try std.testing.expect(uniform_layout_is_aligned(0, 16));
    try std.testing.expect(uniform_layout_is_aligned(16, 16));
    try std.testing.expect(uniform_layout_is_aligned(32, 16));
    try std.testing.expect(!uniform_layout_is_aligned(1, 16));
    try std.testing.expect(!uniform_layout_is_aligned(15, 16));
}

test "uniform_layout_builder" {
    var builder = LayoutBuilder.init();

    // Add mat4x4 (64 bytes, 16-byte aligned)
    try builder.addField(64, 16);
    try std.testing.expectEqual(@as(u32, 0), builder.fields[0].offset);

    // Add vec4 (16 bytes, 16-byte aligned)
    try builder.addField(16, 16);
    try std.testing.expectEqual(@as(u32, 64), builder.fields[1].offset);

    // Add f32 (4 bytes, 4-byte aligned)
    try builder.addField(4, 4);
    try std.testing.expectEqual(@as(u32, 80), builder.fields[2].offset);

    const total_size = builder.getTotalSize();
    try std.testing.expect(total_size >= 84);
    try std.testing.expect(total_size % 16 == 0);
}
