const std = @import("std");

// ============================================================================
// Asset ID - 资产的唯一标识符
// ============================================================================
pub const AssetId = extern struct {
    uuid: u128,
    type_id: u64,

    pub fn init(uuid: u128, type_id: u64) AssetId {
        return .{ .uuid = uuid, .type_id = type_id };
    }

    pub fn eql(self: AssetId, other: AssetId) bool {
        return self.uuid == other.uuid and self.type_id == other.type_id;
    }

    pub fn hash(self: AssetId) u64 {
        // 简单的哈希组合
        const uuid_hash = @as(u64, @truncate(self.uuid ^ (self.uuid >> 64)));
        return uuid_hash ^ self.type_id;
    }
};

// ============================================================================
// Handle ID - 带代数的句柄标识符
// ============================================================================
pub const HandleId = extern struct {
    id: AssetId,
    generation: u32,

    pub fn init(id: AssetId, generation: u32) HandleId {
        return .{ .id = id, .generation = generation };
    }

    pub fn eql(self: HandleId, other: HandleId) bool {
        return self.id.eql(other.id) and self.generation == other.generation;
    }

    pub fn hash(self: HandleId) u64 {
        const id_hash = self.id.hash();
        return id_hash ^ @as(u64, self.generation);
    }
};

// ============================================================================
// Load State - 资产加载状态
// ============================================================================
pub const LoadState = enum(u32) {
    NotLoaded = 0,
    Loading = 1,
    Loaded = 2,
    Failed = 3,

    pub fn isLoaded(self: LoadState) bool {
        return self == .Loaded;
    }

    pub fn isLoading(self: LoadState) bool {
        return self == .Loading;
    }

    pub fn isFailed(self: LoadState) bool {
        return self == .Failed;
    }
};

// ============================================================================
// Asset Meta - 资产元数据
// ============================================================================
pub const AssetMeta = extern struct {
    path_ptr: [*]const u8,
    path_len: usize,
    dependencies_ptr: [*]const AssetId,
    dependencies_len: usize,
    load_state: LoadState,
    ref_count: u32,

    pub fn init(allocator: std.mem.Allocator, path: []const u8) !AssetMeta {
        const path_copy = try allocator.dupe(u8, path);
        return AssetMeta{
            .path_ptr = path_copy.ptr,
            .path_len = path_copy.len,
            .dependencies_ptr = undefined,
            .dependencies_len = 0,
            .load_state = .NotLoaded,
            .ref_count = 0,
        };
    }

    pub fn getPath(self: AssetMeta) []const u8 {
        return self.path_ptr[0..self.path_len];
    }

    pub fn getDependencies(self: AssetMeta) []const AssetId {
        if (self.dependencies_len == 0) return &[_]AssetId{};
        return self.dependencies_ptr[0..self.dependencies_len];
    }

    pub fn setDependencies(self: *AssetMeta, deps: []const AssetId) void {
        self.dependencies_ptr = deps.ptr;
        self.dependencies_len = deps.len;
    }

    pub fn incRef(self: *AssetMeta) void {
        self.ref_count += 1;
    }

    pub fn decRef(self: *AssetMeta) bool {
        if (self.ref_count > 0) {
            self.ref_count -= 1;
        }
        return self.ref_count == 0;
    }

    pub fn deinit(self: *AssetMeta, allocator: std.mem.Allocator) void {
        allocator.free(self.getPath());
    }
};

// ============================================================================
// UUID 生成器 (简化版)
// ============================================================================
var uuid_counter: std.atomic.Value(u64) = std.atomic.Value(u64).init(1);

pub fn generateUuid() u128 {
    const timestamp = @as(u64, @intCast(std.time.milliTimestamp()));
    const counter = uuid_counter.fetchAdd(1, .monotonic);
    return (@as(u128, timestamp) << 64) | counter;
}

// ============================================================================
// FFI exports
// ============================================================================

export fn asset_id_init(uuid: u128, type_id: u64) AssetId {
    return AssetId.init(uuid, type_id);
}

export fn asset_id_eql(a: AssetId, b: AssetId) bool {
    return a.eql(b);
}

export fn asset_id_hash(id: AssetId) u64 {
    return id.hash();
}

export fn handle_id_init(id: AssetId, generation: u32) HandleId {
    return HandleId.init(id, generation);
}

export fn handle_id_eql(a: HandleId, b: HandleId) bool {
    return a.eql(b);
}

export fn handle_id_hash(handle: HandleId) u64 {
    return handle.hash();
}

export fn generate_uuid() u128 {
    return generateUuid();
}

export fn load_state_is_loaded(state: LoadState) bool {
    return state.isLoaded();
}

export fn load_state_is_loading(state: LoadState) bool {
    return state.isLoading();
}

export fn load_state_is_failed(state: LoadState) bool {
    return state.isFailed();
}
