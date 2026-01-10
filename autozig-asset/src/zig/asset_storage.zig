const std = @import("std");
const handle = @import("handle.zig");

const AssetId = handle.AssetId;
const HandleId = handle.HandleId;
const LoadState = handle.LoadState;

// ============================================================================
// Asset Entry - 资产存储条目
// ============================================================================
pub const AssetEntry = struct {
    data_ptr: ?*anyopaque,
    handle_id: HandleId,
    load_state: LoadState,
    type_id: u64,

    pub fn init(data: ?*anyopaque, handle_id: HandleId, type_id: u64) AssetEntry {
        return .{
            .data_ptr = data,
            .handle_id = handle_id,
            .load_state = .Loaded,
            .type_id = type_id,
        };
    }

    pub fn isValid(self: AssetEntry) bool {
        return self.data_ptr != null and self.load_state == .Loaded;
    }
};

// ============================================================================
// Asset Storage - 通用资产存储
// ============================================================================
pub const AssetStorage = struct {
    allocator: std.mem.Allocator,
    entries: std.AutoHashMap(u64, AssetEntry),
    next_generation: u32,
    type_id: u64,

    pub fn init(allocator: std.mem.Allocator, type_id: u64) AssetStorage {
        return .{
            .allocator = allocator,
            .entries = std.AutoHashMap(u64, AssetEntry).init(allocator),
            .next_generation = 1,
            .type_id = type_id,
        };
    }

    pub fn deinit(self: *AssetStorage) void {
        self.entries.deinit();
    }

    /// 添加资产并返回句柄
    pub fn add(self: *AssetStorage, data: *anyopaque, uuid: u128) !HandleId {
        const asset_id = AssetId.init(uuid, self.type_id);
        const handle_id = HandleId.init(asset_id, self.next_generation);
        self.next_generation +%= 1;

        const entry = AssetEntry.init(data, handle_id, self.type_id);
        const key = asset_id.hash();
        try self.entries.put(key, entry);

        return handle_id;
    }

    /// 获取资产数据指针
    pub fn get(self: *const AssetStorage, handle_id: HandleId) ?*anyopaque {
        if (handle_id.id.type_id != self.type_id) return null;

        const key = handle_id.id.hash();
        if (self.entries.get(key)) |entry| {
            if (entry.handle_id.eql(handle_id) and entry.isValid()) {
                return entry.data_ptr;
            }
        }
        return null;
    }

    /// 检查句柄是否有效
    pub fn contains(self: *const AssetStorage, handle_id: HandleId) bool {
        if (handle_id.id.type_id != self.type_id) return false;

        const key = handle_id.id.hash();
        if (self.entries.get(key)) |entry| {
            return entry.handle_id.eql(handle_id) and entry.isValid();
        }
        return false;
    }

    /// 移除资产
    pub fn remove(self: *AssetStorage, handle_id: HandleId) ?*anyopaque {
        if (handle_id.id.type_id != self.type_id) return null;

        const key = handle_id.id.hash();
        if (self.entries.fetchRemove(key)) |kv| {
            const entry = kv.value;
            if (entry.handle_id.eql(handle_id)) {
                return entry.data_ptr;
            }
        }
        return null;
    }

    /// 获取资产数量
    pub fn count(self: *const AssetStorage) usize {
        return self.entries.count();
    }

    /// 清空所有资产
    pub fn clear(self: *AssetStorage) void {
        self.entries.clearRetainingCapacity();
    }

    /// 获取加载状态
    pub fn getLoadState(self: *const AssetStorage, handle_id: HandleId) LoadState {
        if (handle_id.id.type_id != self.type_id) return .NotLoaded;

        const key = handle_id.id.hash();
        if (self.entries.get(key)) |entry| {
            if (entry.handle_id.eql(handle_id)) {
                return entry.load_state;
            }
        }
        return .NotLoaded;
    }

    /// 设置加载状态
    pub fn setLoadState(self: *AssetStorage, handle_id: HandleId, state: LoadState) !void {
        if (handle_id.id.type_id != self.type_id) return;

        const key = handle_id.id.hash();
        if (self.entries.getPtr(key)) |entry| {
            if (entry.handle_id.eql(handle_id)) {
                entry.load_state = state;
            }
        }
    }

    /// 迭代所有资产
    pub const Iterator = struct {
        inner: std.AutoHashMap(u64, AssetEntry).Iterator,

        pub fn next(self: *Iterator) ?struct { HandleId, *anyopaque } {
            while (self.inner.next()) |kv| {
                const entry = kv.value_ptr.*;
                if (entry.isValid() and entry.data_ptr != null) {
                    return .{ entry.handle_id, entry.data_ptr.? };
                }
            }
            return null;
        }
    };

    pub fn iterator(self: *AssetStorage) Iterator {
        return .{ .inner = self.entries.iterator() };
    }
};

// ============================================================================
// FFI exports
// ============================================================================

export fn asset_storage_create(type_id: u64) ?*AssetStorage {
    const alloc = @import("allocator.zig"); const allocator = alloc.g_allocator;
    const storage = allocator.create(AssetStorage) catch return null;
    storage.* = AssetStorage.init(allocator, type_id);
    return storage;
}

export fn asset_storage_destroy(storage: *AssetStorage) void {
    const allocator = storage.allocator;
    storage.deinit();
    allocator.destroy(storage);
}

export fn asset_storage_add(storage: *AssetStorage, data: *anyopaque, uuid: u128) HandleId {
    return storage.add(data, uuid) catch {
        // 出错时返回无效句柄
        const invalid_id = AssetId.init(0, 0);
        return HandleId.init(invalid_id, 0);
    };
}

export fn asset_storage_get(storage: *const AssetStorage, handle_id: HandleId) ?*anyopaque {
    return storage.get(handle_id);
}

export fn asset_storage_contains(storage: *const AssetStorage, handle_id: HandleId) bool {
    return storage.contains(handle_id);
}

export fn asset_storage_remove(storage: *AssetStorage, handle_id: HandleId) ?*anyopaque {
    return storage.remove(handle_id);
}

export fn asset_storage_count(storage: *const AssetStorage) usize {
    return storage.count();
}

export fn asset_storage_clear(storage: *AssetStorage) void {
    storage.clear();
}

export fn asset_storage_get_load_state(storage: *const AssetStorage, handle_id: HandleId) LoadState {
    return storage.getLoadState(handle_id);
}

export fn asset_storage_set_load_state(storage: *AssetStorage, handle_id: HandleId, state: LoadState) void {
    storage.setLoadState(handle_id, state) catch {};
}
