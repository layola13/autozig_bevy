const std = @import("std");
const handle = @import("handle.zig");
const path_mod = @import("path.zig");
const storage_mod = @import("asset_storage.zig");
const loader_mod = @import("loader.zig");
const events_mod = @import("events.zig");

const AssetId = handle.AssetId;
const HandleId = handle.HandleId;
const LoadState = handle.LoadState;
const AssetMeta = handle.AssetMeta;
const AssetPath = path_mod.AssetPath;
const AssetStorage = storage_mod.AssetStorage;
const LoaderRegistry = loader_mod.LoaderRegistry;
const EventSystem = events_mod.EventSystem;
const AssetEvent = events_mod.AssetEvent;

// ============================================================================
// Asset Server - 资产服务器
// ============================================================================
pub const AssetServer = struct {
    allocator: std.mem.Allocator,
    storages: std.AutoHashMap(u64, *AssetStorage),
    metadata: std.AutoHashMap(u64, AssetMeta),
    path_to_handle: std.StringHashMap(HandleId),
    loader_registry: LoaderRegistry,
    event_system: EventSystem,
    asset_root: []const u8,

    pub fn init(allocator: std.mem.Allocator, asset_root: []const u8) !AssetServer {
        const root_copy = try allocator.dupe(u8, asset_root);
        return .{
            .allocator = allocator,
            .storages = std.AutoHashMap(u64, *AssetStorage).init(allocator),
            .metadata = std.AutoHashMap(u64, AssetMeta).init(allocator),
            .path_to_handle = std.StringHashMap(HandleId).init(allocator),
            .loader_registry = LoaderRegistry.init(allocator),
            .event_system = EventSystem.init(allocator),
            .asset_root = root_copy,
        };
    }

    pub fn deinit(self: *AssetServer) void {
        // 清理所有存储
        var storage_iter = self.storages.valueIterator();
        while (storage_iter.next()) |storage_ptr| {
            storage_ptr.*.deinit();
            self.allocator.destroy(storage_ptr.*);
        }
        self.storages.deinit();

        // 清理元数据
        var meta_iter = self.metadata.valueIterator();
        while (meta_iter.next()) |meta| {
            meta.deinit(self.allocator);
        }
        self.metadata.deinit();

        // 清理路径映射
        var path_iter = self.path_to_handle.keyIterator();
        while (path_iter.next()) |key| {
            self.allocator.free(key.*);
        }
        self.path_to_handle.deinit();

        self.loader_registry.deinit();
        self.event_system.deinit();
        self.allocator.free(self.asset_root);
    }

    /// 获取或创建指定类型的存储
    fn getOrCreateStorage(self: *AssetServer, type_id: u64) !*AssetStorage {
        if (self.storages.get(type_id)) |storage| {
            return storage;
        }

        const storage = try self.allocator.create(AssetStorage);
        storage.* = AssetStorage.init(self.allocator, type_id);
        try self.storages.put(type_id, storage);
        return storage;
    }

    /// 加载资产
    pub fn load(self: *AssetServer, path: []const u8, type_id: u64) !HandleId {
        // 检查是否已加载
        if (self.path_to_handle.get(path)) |existing_handle| {
            return existing_handle;
        }

        // 生成新的 UUID 和句柄
        const uuid = handle.generateUuid();
        const asset_id = AssetId.init(uuid, type_id);

        const storage = try self.getOrCreateStorage(type_id);
        const handle_id = try storage.add(null, uuid);

        // 设置为加载中状态
        try storage.setLoadState(handle_id, .Loading);

        // 保存路径映射
        const path_copy = try self.allocator.dupe(u8, path);
        try self.path_to_handle.put(path_copy, handle_id);

        // 创建元数据
        var meta = try AssetMeta.init(self.allocator, path);
        meta.load_state = .Loading;
        const meta_key = asset_id.hash();
        try self.metadata.put(meta_key, meta);

        // 发送加载开始事件
        try self.event_system.send(AssetEvent.loadingStarted(handle_id));

        // 异步加载（这里简化为同步）
        self.loadAssetSync(path, handle_id, type_id) catch |err| {
            try storage.setLoadState(handle_id, .Failed);
            if (self.metadata.getPtr(meta_key)) |meta_ptr| {
                meta_ptr.load_state = .Failed;
            }
            try self.event_system.send(AssetEvent.loadingFailed(handle_id));
            return err;
        };

        return handle_id;
    }

    /// 同步加载资产数据
    fn loadAssetSync(self: *AssetServer, path: []const u8, handle_id: HandleId, type_id: u64) !void {
        // 构建完整路径
        const full_path = try std.fs.path.join(self.allocator, &[_][]const u8{ self.asset_root, path });
        defer self.allocator.free(full_path);

        // 读取文件
        const file = try std.fs.cwd().openFile(full_path, .{});
        defer file.close();

        const file_size = try file.getEndPos();
        const data = try self.allocator.alloc(u8, file_size);
        defer self.allocator.free(data);

        _ = try file.readAll(data);

        // 使用加载器加载
        if (self.loader_registry.getLoaderForPath(path)) |loader| {
            if (loader.load(data)) |asset_data| {
                const storage = self.storages.get(type_id).?;

                // 更新存储中的数据
                const key = handle_id.id.hash();
                if (storage.entries.getPtr(key)) |entry| {
                    entry.data_ptr = asset_data;
                    entry.load_state = .Loaded;
                }

                try storage.setLoadState(handle_id, .Loaded);

                // 更新元数据
                const meta_key = handle_id.id.hash();
                if (self.metadata.getPtr(meta_key)) |meta| {
                    meta.load_state = .Loaded;
                }

                // 发送加载完成事件
                try self.event_system.send(AssetEvent.loadingFinished(handle_id));
                return;
            }
        }

        return error.NoLoaderFound;
    }

    /// 获取资产
    pub fn get(self: *const AssetServer, handle_id: HandleId) ?*anyopaque {
        if (self.storages.get(handle_id.id.type_id)) |storage| {
            return storage.get(handle_id);
        }
        return null;
    }

    /// 获取加载状态
    pub fn getLoadState(self: *const AssetServer, handle_id: HandleId) LoadState {
        if (self.storages.get(handle_id.id.type_id)) |storage| {
            return storage.getLoadState(handle_id);
        }
        return .NotLoaded;
    }

    /// 获取资产路径
    pub fn getPath(self: *const AssetServer, handle_id: HandleId) ?[]const u8 {
        const meta_key = handle_id.id.hash();
        if (self.metadata.get(meta_key)) |meta| {
            return meta.getPath();
        }
        return null;
    }

    /// 卸载资产
    pub fn unload(self: *AssetServer, handle_id: HandleId) !void {
        if (self.storages.get(handle_id.id.type_id)) |storage| {
            if (storage.remove(handle_id)) |data_ptr| {
                // 释放数据（这里需要根据实际类型处理）
                _ = data_ptr;

                // 发送移除事件
                try self.event_system.send(AssetEvent.removed(handle_id));

                // 清理元数据
                const meta_key = handle_id.id.hash();
                if (self.metadata.fetchRemove(meta_key)) |kv| {
                    var meta = kv.value;
                    const path = meta.getPath();

                    // 清理路径映射
                    if (self.path_to_handle.fetchRemove(path)) |path_kv| {
                        self.allocator.free(path_kv.key);
                    }

                    meta.deinit(self.allocator);
                }
            }
        }
    }

    /// 处理事件
    pub fn processEvents(self: *AssetServer) void {
        self.event_system.processEvents();
    }

    /// 注册加载器
    pub fn registerLoader(self: *AssetServer, loader: loader_mod.AssetLoader) !void {
        try self.loader_registry.registerLoader(loader);
    }
};

// ============================================================================
// FFI exports
// ============================================================================

export fn asset_server_create(root_ptr: [*]const u8, root_len: usize) ?*AssetServer {
    const alloc = @import("allocator.zig"); const allocator = alloc.g_allocator;
    const root = root_ptr[0..root_len];

    const server = allocator.create(AssetServer) catch return null;
    server.* = AssetServer.init(allocator, root) catch {
        allocator.destroy(server);
        return null;
    };
    return server;
}

export fn asset_server_destroy(server: *AssetServer) void {
    const allocator = server.allocator;
    server.deinit();
    allocator.destroy(server);
}

export fn asset_server_load(
    server: *AssetServer,
    path_ptr: [*]const u8,
    path_len: usize,
    type_id: u64,
) HandleId {
    const path = path_ptr[0..path_len];
    return server.load(path, type_id) catch {
        const invalid_id = AssetId.init(0, 0);
        return HandleId.init(invalid_id, 0);
    };
}

export fn asset_server_get(server: *const AssetServer, handle_id: HandleId) ?*anyopaque {
    return server.get(handle_id);
}

export fn asset_server_get_load_state(server: *const AssetServer, handle_id: HandleId) LoadState {
    return server.getLoadState(handle_id);
}

export fn asset_server_get_path_ptr(server: *const AssetServer, handle_id: HandleId) ?[*]const u8 {
    if (server.getPath(handle_id)) |path| {
        return path.ptr;
    }
    return null;
}

export fn asset_server_get_path_len(server: *const AssetServer, handle_id: HandleId) usize {
    if (server.getPath(handle_id)) |path| {
        return path.len;
    }
    return 0;
}

export fn asset_server_unload(server: *AssetServer, handle_id: HandleId) void {
    server.unload(handle_id) catch {};
}

export fn asset_server_process_events(server: *AssetServer) void {
    server.processEvents();
}
