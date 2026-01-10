const std = @import("std");
const builtin = @import("builtin");
const alloc = @import("allocator.zig");

// ============================================================================
// Handle 模块 - 资产句柄和ID管理
// ============================================================================

/// Asset ID - 资产唯一标识符
pub const AssetId = extern struct {
    uuid: u128,
    type_id: u64,

    pub fn init(uuid: u128, type_id: u64) AssetId {
        return AssetId{ .uuid = uuid, .type_id = type_id };
    }

    pub fn eql(self: AssetId, other: AssetId) bool {
        return self.uuid == other.uuid and self.type_id == other.type_id;
    }

    pub fn hash(self: AssetId) u64 {
        const h1 = @as(u64, @truncate(self.uuid));
        const h2 = @as(u64, @truncate(self.uuid >> 64));
        return h1 ^ h2 ^ self.type_id;
    }
};

/// Handle ID - 带代数的句柄
pub const HandleId = extern struct {
    id: AssetId,
    generation: u32,

    pub fn init(id: AssetId, generation: u32) HandleId {
        return HandleId{ .id = id, .generation = generation };
    }

    pub fn eql(self: HandleId, other: HandleId) bool {
        return self.id.eql(other.id) and self.generation == other.generation;
    }

    pub fn hash(self: HandleId) u64 {
        return self.id.hash() ^ @as(u64, self.generation);
    }
};

/// 加载状态
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

/// 资产元数据
pub const AssetMeta = struct {
    path: []const u8,
    load_state: LoadState,
    ref_count: u32,

    pub fn init(path: []const u8) AssetMeta {
        return AssetMeta{
            .path = path,
            .load_state = .NotLoaded,
            .ref_count = 1,
        };
    }
};

var rng_state: u64 = 0x123456789abcdef0;

pub fn generateUuid() u128 {
    rng_state = rng_state *% 6364136223846793005 +% 1442695040888963407;
    const low = rng_state;
    rng_state = rng_state *% 6364136223846793005 +% 1442695040888963407;
    const high = rng_state;
    return (@as(u128, high) << 64) | @as(u128, low);
}

// ============================================================================
// Path 模块 - 资产路径解析
// ============================================================================

pub const AssetPath = extern struct {
    path_ptr: [*]const u8,
    path_len: usize,
    label_ptr: [*]const u8,
    label_len: usize,
    has_label: bool,

    pub fn init(path: []const u8) AssetPath {
        return AssetPath{
            .path_ptr = path.ptr,
            .path_len = path.len,
            .label_ptr = undefined,
            .label_len = 0,
            .has_label = false,
        };
    }

    pub fn initWithLabel(path: []const u8, label: []const u8) AssetPath {
        return AssetPath{
            .path_ptr = path.ptr,
            .path_len = path.len,
            .label_ptr = label.ptr,
            .label_len = label.len,
            .has_label = true,
        };
    }

    pub fn getPath(self: AssetPath) []const u8 {
        return self.path_ptr[0..self.path_len];
    }

    pub fn getLabel(self: AssetPath) ?[]const u8 {
        if (!self.has_label) return null;
        return self.label_ptr[0..self.label_len];
    }

    pub fn hasLabel(self: AssetPath) bool {
        return self.has_label;
    }

    pub fn eql(self: AssetPath, other: AssetPath) bool {
        if (!std.mem.eql(u8, self.getPath(), other.getPath())) return false;
        const self_label = self.getLabel();
        const other_label = other.getLabel();
        if (self_label == null and other_label == null) return true;
        if (self_label == null or other_label == null) return false;
        return std.mem.eql(u8, self_label.?, other_label.?);
    }

    pub fn parse(allocator: std.mem.Allocator, path_str: []const u8) !AssetPath {
        if (std.mem.indexOf(u8, path_str, "#")) |idx| {
            const path = try allocator.dupe(u8, path_str[0..idx]);
            const label = try allocator.dupe(u8, path_str[idx + 1 ..]);
            return initWithLabel(path, label);
        } else {
            const path = try allocator.dupe(u8, path_str);
            return init(path);
        }
    }

    pub fn deinit(self: *const AssetPath, allocator: *const std.mem.Allocator) void {
        allocator.free(self.getPath());
        if (self.getLabel()) |label| {
            allocator.free(label);
        }
    }
};

// ============================================================================
// Storage 模块 - 资产存储
// ============================================================================

pub const AssetStorage = struct {
    allocator: std.mem.Allocator,
    type_id: u64,
    assets: std.AutoHashMap(HandleId, *anyopaque),
    meta_data: std.AutoHashMap(HandleId, AssetMeta),
    load_states: std.AutoHashMap(HandleId, LoadState),
    next_generation: u32,

    pub fn init(allocator: std.mem.Allocator, type_id: u64) AssetStorage {
        return AssetStorage{
            .allocator = allocator,
            .type_id = type_id,
            .assets = std.AutoHashMap(HandleId, *anyopaque).init(allocator),
            .meta_data = std.AutoHashMap(HandleId, AssetMeta).init(allocator),
            .load_states = std.AutoHashMap(HandleId, LoadState).init(allocator),
            .next_generation = 1,
        };
    }

    pub fn deinit(self: *AssetStorage) void {
        self.assets.deinit();
        self.meta_data.deinit();
        self.load_states.deinit();
    }

    pub fn add(self: *AssetStorage, asset: *anyopaque, uuid: u128) !HandleId {
        const asset_id = AssetId.init(uuid, self.type_id);
        const handle_id = HandleId.init(asset_id, self.next_generation);
        self.next_generation += 1;

        try self.assets.put(handle_id, asset);
        try self.load_states.put(handle_id, .Loaded);

        return handle_id;
    }

    pub fn get(self: *const AssetStorage, handle_id: HandleId) ?*anyopaque {
        return self.assets.get(handle_id);
    }

    pub fn contains(self: *const AssetStorage, handle_id: HandleId) bool {
        return self.assets.contains(handle_id);
    }

    pub fn remove(self: *AssetStorage, handle_id: HandleId) ?*anyopaque {
        _ = self.load_states.remove(handle_id);
        _ = self.meta_data.remove(handle_id);
        if (self.assets.fetchRemove(handle_id)) |kv| {
            return kv.value;
        }
        return null;
    }

    pub fn count(self: *const AssetStorage) usize {
        return self.assets.count();
    }

    pub fn clear(self: *AssetStorage) void {
        self.assets.clearRetainingCapacity();
        self.meta_data.clearRetainingCapacity();
        self.load_states.clearRetainingCapacity();
    }

    pub fn getLoadState(self: *const AssetStorage, handle_id: HandleId) LoadState {
        return self.load_states.get(handle_id) orelse .NotLoaded;
    }

    pub fn setLoadState(self: *AssetStorage, handle_id: HandleId, state: LoadState) void {
        self.load_states.put(handle_id, state) catch {};
    }
};

// ============================================================================
// Events 模块 - 资产事件系统
// ============================================================================

pub const AssetEventType = enum(u32) {
    Created = 0,
    Modified = 1,
    Removed = 2,
    LoadingStarted = 3,
    LoadingFinished = 4,
    LoadingFailed = 5,
};

pub const AssetEvent = extern struct {
    handle_id: HandleId,
    event_type: AssetEventType,
    timestamp: i64,

    /// 获取时间戳（WASM 兼容）
    fn getTimestamp() i64 {
        if (builtin.cpu.arch.isWasm()) {
            // WASM 环境：返回0（无时钟支持）
            return 0;
        } else {
            // 原生环境：使用系统时钟
            return std.time.milliTimestamp();
        }
    }

    pub fn created(handle_id: HandleId) AssetEvent {
        return AssetEvent{
            .handle_id = handle_id,
            .event_type = .Created,
            .timestamp = getTimestamp(),
        };
    }

    pub fn modified(handle_id: HandleId) AssetEvent {
        return AssetEvent{
            .handle_id = handle_id,
            .event_type = .Modified,
            .timestamp = getTimestamp(),
        };
    }

    pub fn removed(handle_id: HandleId) AssetEvent {
        return AssetEvent{
            .handle_id = handle_id,
            .event_type = .Removed,
            .timestamp = getTimestamp(),
        };
    }
};

pub const EventQueue = struct {
    allocator: std.mem.Allocator,
    events: std.ArrayList(AssetEvent),

    pub fn init(allocator: std.mem.Allocator) EventQueue {
        return EventQueue{
            .allocator = allocator,
            .events = std.ArrayList(AssetEvent){},
        };
    }

    pub fn deinit(self: *EventQueue) void {
        self.events.deinit(self.allocator);
    }

    pub fn push(self: *EventQueue, event: AssetEvent) !void {
        try self.events.append(self.allocator, event);
    }

    pub fn len(self: *const EventQueue) usize {
        return self.events.items.len;
    }

    pub fn clear(self: *EventQueue) void {
        self.events.clearRetainingCapacity();
    }
};

pub const EventSystem = struct {
    allocator: std.mem.Allocator,
    queue: EventQueue,

    pub fn init(allocator: std.mem.Allocator) EventSystem {
        return EventSystem{
            .allocator = allocator,
            .queue = EventQueue.init(allocator),
        };
    }

    pub fn deinit(self: *EventSystem) void {
        self.queue.deinit();
    }

    pub fn send(self: *EventSystem, event: AssetEvent) !void {
        try self.queue.push(event);
    }

    pub fn processEvents(self: *EventSystem) void {
        self.queue.clear();
    }
};

// ============================================================================
// Loader 模块 - 资产加载器
// ============================================================================

pub const LoadFn = *const fn ([]const u8) anyerror!*anyopaque;

pub const AssetLoader = struct {
    type_id: u64,
    extensions: []const []const u8,
    load_fn: LoadFn,

    pub fn init(type_id: u64, extensions: []const []const u8, load_fn: LoadFn) AssetLoader {
        return AssetLoader{
            .type_id = type_id,
            .extensions = extensions,
            .load_fn = load_fn,
        };
    }

    pub fn supportsExtension(self: *const AssetLoader, ext: []const u8) bool {
        for (self.extensions) |supported_ext| {
            if (std.mem.eql(u8, ext, supported_ext)) {
                return true;
            }
        }
        return false;
    }

    pub fn load(self: *const AssetLoader, data: []const u8) !*anyopaque {
        return self.load_fn(data);
    }
};

pub const LoaderRegistry = struct {
    allocator: std.mem.Allocator,
    loaders: std.ArrayList(AssetLoader),

    pub fn init(allocator: std.mem.Allocator) LoaderRegistry {
        return LoaderRegistry{
            .allocator = allocator,
            .loaders = std.ArrayList(AssetLoader){},
        };
    }

    pub fn deinit(self: *LoaderRegistry) void {
        self.loaders.deinit(self.allocator);
    }

    pub fn registerLoader(self: *LoaderRegistry, loader: AssetLoader) !void {
        try self.loaders.append(self.allocator, loader);
    }

    pub fn getLoaderForExtension(self: *const LoaderRegistry, ext: []const u8) ?*const AssetLoader {
        for (self.loaders.items) |*loader| {
            if (loader.supportsExtension(ext)) {
                return loader;
            }
        }
        return null;
    }

    pub fn getLoaderForPath(self: *const LoaderRegistry, path: []const u8) ?*const AssetLoader {
        if (std.mem.lastIndexOf(u8, path, ".")) |idx| {
            const ext = path[idx + 1 ..];
            return self.getLoaderForExtension(ext);
        }
        return null;
    }

    pub fn hasLoaderFor(self: *const LoaderRegistry, ext: []const u8) bool {
        return self.getLoaderForExtension(ext) != null;
    }
};

fn textLoaderFn(data: []const u8) !*anyopaque {
    _ = data;
    return error.NotImplemented;
}

pub fn createTextLoader(allocator: std.mem.Allocator, type_id: u64) !AssetLoader {
    const extensions = try allocator.alloc([]const u8, 2);
    extensions[0] = "txt";
    extensions[1] = "md";
    return AssetLoader.init(type_id, extensions, textLoaderFn);
}

// ============================================================================
// AssetServer 模块 - 资产服务器
// ============================================================================

pub const AssetServer = struct {
    allocator: std.mem.Allocator,
    asset_root: []const u8,
    storages: std.AutoHashMap(u64, *AssetStorage),
    event_system: EventSystem,
    loader_registry: LoaderRegistry,
    path_to_handle: std.StringHashMap(HandleId),

    pub fn init(allocator: std.mem.Allocator, asset_root: []const u8) !AssetServer {
        const root_copy = try allocator.dupe(u8, asset_root);
        return AssetServer{
            .allocator = allocator,
            .asset_root = root_copy,
            .storages = std.AutoHashMap(u64, *AssetStorage).init(allocator),
            .event_system = EventSystem.init(allocator),
            .loader_registry = LoaderRegistry.init(allocator),
            .path_to_handle = std.StringHashMap(HandleId).init(allocator),
        };
    }

    pub fn deinit(self: *AssetServer) void {
        var it = self.storages.valueIterator();
        while (it.next()) |storage_ptr| {
            storage_ptr.*.deinit();
            self.allocator.destroy(storage_ptr.*);
        }
        self.storages.deinit();
        self.event_system.deinit();
        self.loader_registry.deinit();
        self.path_to_handle.deinit();
        self.allocator.free(self.asset_root);
    }

    pub fn getOrCreateStorage(self: *AssetServer, type_id: u64) !*AssetStorage {
        if (self.storages.get(type_id)) |storage| {
            return storage;
        }

        const storage = try self.allocator.create(AssetStorage);
        storage.* = AssetStorage.init(self.allocator, type_id);
        try self.storages.put(type_id, storage);
        return storage;
    }

    pub fn load(self: *AssetServer, path: []const u8, type_id: u64) !HandleId {
        if (self.path_to_handle.get(path)) |existing_handle| {
            return existing_handle;
        }

        const storage = try self.getOrCreateStorage(type_id);
        const uuid = generateUuid();
        const dummy_data = try self.allocator.create(u8);
        dummy_data.* = 0;

        const handle_id = try storage.add(dummy_data, uuid);
        try self.path_to_handle.put(try self.allocator.dupe(u8, path), handle_id);

        const event = AssetEvent.created(handle_id);
        try self.event_system.send(event);

        return handle_id;
    }

    pub fn get(self: *const AssetServer, handle_id: HandleId) ?*anyopaque {
        if (self.storages.get(handle_id.id.type_id)) |storage| {
            return storage.get(handle_id);
        }
        return null;
    }

    pub fn getLoadState(self: *const AssetServer, handle_id: HandleId) LoadState {
        if (self.storages.get(handle_id.id.type_id)) |storage| {
            return storage.getLoadState(handle_id);
        }
        return .NotLoaded;
    }

    pub fn getPath(self: *const AssetServer, handle_id: HandleId) ?[]const u8 {
        var it = self.path_to_handle.iterator();
        while (it.next()) |entry| {
            if (entry.value_ptr.eql(handle_id)) {
                return entry.key_ptr.*;
            }
        }
        return null;
    }

    pub fn unload(self: *AssetServer, handle_id: HandleId) void {
        if (self.storages.get(handle_id.id.type_id)) |storage| {
            _ = storage.remove(handle_id);
        }
    }

    pub fn processEvents(self: *AssetServer) void {
        self.event_system.processEvents();
    }
};

// ============================================================================
// C FFI 导出
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

export fn asset_path_init(path_ptr: [*]const u8, path_len: usize) AssetPath {
    return AssetPath.init(path_ptr[0..path_len]);
}

export fn asset_path_init_with_label(path_ptr: [*]const u8, path_len: usize, label_ptr: [*]const u8, label_len: usize) AssetPath {
    return AssetPath.initWithLabel(path_ptr[0..path_len], label_ptr[0..label_len]);
}

export fn asset_path_has_label(path: AssetPath) bool {
    return path.hasLabel();
}

export fn asset_path_get_label_ptr(path: AssetPath) [*]const u8 {
    if (path.getLabel()) |label| {
        return label.ptr;
    }
    return undefined;
}

export fn asset_path_get_label_len(path: AssetPath) usize {
    if (path.getLabel()) |label| {
        return label.len;
    }
    return 0;
}

export fn asset_path_eql(a: AssetPath, b: AssetPath) bool {
    return a.eql(b);
}

// Storage FFI
export fn asset_storage_create(type_id: u64) ?*anyopaque {
    const allocator = alloc.g_allocator;
    const storage = allocator.create(AssetStorage) catch return null;
    storage.* = AssetStorage.init(allocator, type_id);
    return @ptrCast(storage);
}

export fn asset_storage_destroy(storage: ?*anyopaque) void {
    if (storage) |s| {
        const stor: *AssetStorage = @ptrCast(@alignCast(s));
        stor.deinit();
        alloc.g_allocator.destroy(stor);
    }
}

export fn asset_storage_add(storage: ?*anyopaque, data: ?*anyopaque, uuid: u128) HandleId {
    const stor: *AssetStorage = @ptrCast(@alignCast(storage.?));
    return stor.add(data.?, uuid) catch HandleId.init(AssetId.init(0, 0), 0);
}

export fn asset_storage_get(storage: ?*const anyopaque, handle_id: HandleId) ?*anyopaque {
    const stor: *const AssetStorage = @ptrCast(@alignCast(storage.?));
    return stor.get(handle_id);
}

export fn asset_storage_contains(storage: ?*const anyopaque, handle_id: HandleId) bool {
    const stor: *const AssetStorage = @ptrCast(@alignCast(storage.?));
    return stor.contains(handle_id);
}

export fn asset_storage_remove(storage: ?*anyopaque, handle_id: HandleId) ?*anyopaque {
    const stor: *AssetStorage = @ptrCast(@alignCast(storage.?));
    return stor.remove(handle_id);
}

export fn asset_storage_count(storage: ?*const anyopaque) usize {
    const stor: *const AssetStorage = @ptrCast(@alignCast(storage.?));
    return stor.count();
}

export fn asset_storage_clear(storage: ?*anyopaque) void {
    const stor: *AssetStorage = @ptrCast(@alignCast(storage.?));
    stor.clear();
}

export fn asset_storage_get_load_state(storage: ?*const anyopaque, handle_id: HandleId) LoadState {
    const stor: *const AssetStorage = @ptrCast(@alignCast(storage.?));
    return stor.getLoadState(handle_id);
}

export fn asset_storage_set_load_state(storage: ?*anyopaque, handle_id: HandleId, state: LoadState) void {
    const stor: *AssetStorage = @ptrCast(@alignCast(storage.?));
    stor.setLoadState(handle_id, state);
}

// AssetServer FFI
export fn asset_server_create(root_ptr: [*]const u8, root_len: usize) ?*anyopaque {
    const allocator = alloc.g_allocator;
    const server = allocator.create(AssetServer) catch return null;
    server.* = AssetServer.init(allocator, root_ptr[0..root_len]) catch {
        allocator.destroy(server);
        return null;
    };
    return @ptrCast(server);
}

export fn asset_server_destroy(server: ?*anyopaque) void {
    if (server) |s| {
        const srv: *AssetServer = @ptrCast(@alignCast(s));
        srv.deinit();
        alloc.g_allocator.destroy(srv);
    }
}

export fn asset_server_load(server: ?*anyopaque, path_ptr: [*]const u8, path_len: usize, type_id: u64) HandleId {
    const srv: *AssetServer = @ptrCast(@alignCast(server.?));
    return srv.load(path_ptr[0..path_len], type_id) catch HandleId.init(AssetId.init(0, 0), 0);
}

export fn asset_server_get(server: ?*const anyopaque, handle_id: HandleId) ?*anyopaque {
    const srv: *const AssetServer = @ptrCast(@alignCast(server.?));
    return srv.get(handle_id);
}

export fn asset_server_get_load_state(server: ?*const anyopaque, handle_id: HandleId) LoadState {
    const srv: *const AssetServer = @ptrCast(@alignCast(server.?));
    return srv.getLoadState(handle_id);
}

export fn asset_server_get_path_ptr(server: ?*const anyopaque, handle_id: HandleId) [*]const u8 {
    const srv: *const AssetServer = @ptrCast(@alignCast(server.?));
    if (srv.getPath(handle_id)) |path| {
        return path.ptr;
    }
    return undefined;
}

export fn asset_server_get_path_len(server: ?*const anyopaque, handle_id: HandleId) usize {
    const srv: *const AssetServer = @ptrCast(@alignCast(server.?));
    if (srv.getPath(handle_id)) |path| {
        return path.len;
    }
    return 0;
}

export fn asset_server_unload(server: ?*anyopaque, handle_id: HandleId) void {
    const srv: *AssetServer = @ptrCast(@alignCast(server.?));
    srv.unload(handle_id);
}

export fn asset_server_process_events(server: ?*anyopaque) void {
    const srv: *AssetServer = @ptrCast(@alignCast(server.?));
    srv.processEvents();
}

// Event FFI
export fn asset_event_created(handle_id: HandleId) AssetEvent {
    return AssetEvent.created(handle_id);
}

export fn asset_event_modified(handle_id: HandleId) AssetEvent {
    return AssetEvent.modified(handle_id);
}

export fn asset_event_removed(handle_id: HandleId) AssetEvent {
    return AssetEvent.removed(handle_id);
}

export fn event_queue_create() ?*anyopaque {
    const allocator = alloc.g_allocator;
    const queue = allocator.create(EventQueue) catch return null;
    queue.* = EventQueue.init(allocator);
    return @ptrCast(queue);
}

export fn event_queue_destroy(queue: ?*anyopaque) void {
    if (queue) |q| {
        const qu: *EventQueue = @ptrCast(@alignCast(q));
        qu.deinit();
        alloc.g_allocator.destroy(qu);
    }
}

export fn event_queue_push(queue: ?*anyopaque, event: AssetEvent) bool {
    const qu: *EventQueue = @ptrCast(@alignCast(queue.?));
    qu.push(event) catch return false;
    return true;
}

export fn event_queue_len(queue: ?*anyopaque) usize {
    const qu: *EventQueue = @ptrCast(@alignCast(queue.?));
    return qu.len();
}

export fn event_queue_clear(queue: ?*anyopaque) void {
    const qu: *EventQueue = @ptrCast(@alignCast(queue.?));
    qu.clear();
}
