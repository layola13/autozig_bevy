// Asset Core - Core asset types and ID management
// Implements core functionality for asset identification and indexing

const std = @import("std");

// Asset ID structure (matches Rust #[repr(C)])
pub const AssetId = extern struct {
    uuid: u128,
    type_id: u64,
};

// Generate a new UUID v4
pub export fn generate_uuid() u128 {
    // Simple UUID generation using timestamp-based pseudo-random
    const timestamp: u128 = @intCast(std.time.milliTimestamp());
    const high = timestamp << 64;
    const low = timestamp;
    return high | low;
}

// Asset ID initialization
pub export fn asset_id_init(uuid: u128, type_id: u64) AssetId {
    return AssetId{
        .uuid = uuid,
        .type_id = type_id,
    };
}

// Asset ID equality check
pub export fn asset_id_eql(a: AssetId, b: AssetId) bool {
    return a.uuid == b.uuid and a.type_id == b.type_id;
}

// Asset ID hash function
pub export fn asset_id_hash(id: AssetId) u64 {
    // Simple hash combining uuid and type_id
    const uuid_hash = @as(u64, @truncate(id.uuid ^ (id.uuid >> 64)));
    return uuid_hash ^ id.type_id;
}

// Handle ID structure
pub const HandleId = extern struct {
    id: AssetId,
    generation: u32,
};

// Handle ID initialization
pub export fn handle_id_init(id: AssetId, generation: u32) HandleId {
    return HandleId{
        .id = id,
        .generation = generation,
    };
}

// Handle ID equality
pub export fn handle_id_eql(a: HandleId, b: HandleId) bool {
    return asset_id_eql(a.id, b.id) and a.generation == b.generation;
}

// Handle ID hash
pub export fn handle_id_hash(handle: HandleId) u64 {
    const id_hash = asset_id_hash(handle.id);
    return id_hash ^ @as(u64, handle.generation);
}

// Load state enum
pub const LoadState = enum(u32) {
    NotLoaded = 0,
    Loading = 1,
    Loaded = 2,
    Failed = 3,
};

// Load state checks
pub export fn load_state_is_loaded(state: LoadState) bool {
    return state == .Loaded;
}

pub export fn load_state_is_loading(state: LoadState) bool {
    return state == .Loading;
}

pub export fn load_state_is_failed(state: LoadState) bool {
    return state == .Failed;
}
