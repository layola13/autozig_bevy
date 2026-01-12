//! WorldId Zig implementation
//! Provides core operations for World identification

const std = @import("std");

/// WorldId structure matching Rust repr(C)
pub const WorldId = extern struct {
    index: u32,
};

/// Creates a new WorldId with the given index
export fn world_id_create(index: u32) WorldId {
    return WorldId{ .index = index };
}

/// Returns the index of a WorldId
export fn world_id_index(id: WorldId) u32 {
    return id.index;
}

/// Checks if two WorldIds are equal
export fn world_id_equals(a: WorldId, b: WorldId) bool {
    return a.index == b.index;
}
