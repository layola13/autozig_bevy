const std = @import("std");

// ============================================================================
// Asset Loader - 资产加载器
// ============================================================================
pub const AssetLoader = struct {
    extensions_ptr: [*]const [*]const u8,
    extensions_len_ptr: [*]const usize,
    extensions_count: usize,
    load_fn: *const fn ([*]const u8, usize) callconv(.C) ?*anyopaque,
    type_id: u64,

    pub fn init(
        extensions: []const []const u8,
        load_fn: *const fn ([*]const u8, usize) callconv(.C) ?*anyopaque,
        type_id: u64,
        allocator: std.mem.Allocator,
    ) !AssetLoader {
        // 分配扩展名数组
        const ext_ptrs = try allocator.alloc([*]const u8, extensions.len);
        const ext_lens = try allocator.alloc(usize, extensions.len);

        for (extensions, 0..) |ext, i| {
            ext_ptrs[i] = ext.ptr;
            ext_lens[i] = ext.len;
        }

        return .{
            .extensions_ptr = ext_ptrs.ptr,
            .extensions_len_ptr = ext_lens.ptr,
            .extensions_count = extensions.len,
            .load_fn = load_fn,
            .type_id = type_id,
        };
    }

    pub fn getExtensions(self: AssetLoader, allocator: std.mem.Allocator) ![][]const u8 {
        const result = try allocator.alloc([]const u8, self.extensions_count);
        for (0..self.extensions_count) |i| {
            const ptr = self.extensions_ptr[i];
            const len = self.extensions_len_ptr[i];
            result[i] = ptr[0..len];
        }
        return result;
    }

    pub fn supportsExtension(self: AssetLoader, ext: []const u8) bool {
        for (0..self.extensions_count) |i| {
            const ptr = self.extensions_ptr[i];
            const len = self.extensions_len_ptr[i];
            const loader_ext = ptr[0..len];
            if (std.mem.eql(u8, loader_ext, ext)) {
                return true;
            }
        }
        return false;
    }

    pub fn load(self: AssetLoader, data: []const u8) ?*anyopaque {
        return self.load_fn(data.ptr, data.len);
    }

    pub fn deinit(self: *AssetLoader, allocator: std.mem.Allocator) void {
        const ext_ptrs = self.extensions_ptr[0..self.extensions_count];
        allocator.free(ext_ptrs);
        const ext_lens = self.extensions_len_ptr[0..self.extensions_count];
        allocator.free(ext_lens);
    }
};

// ============================================================================
// Loader Registry - 加载器注册表
// ============================================================================
pub const LoaderRegistry = struct {
    allocator: std.mem.Allocator,
    loaders: std.ArrayList(AssetLoader),
    extension_map: std.StringHashMap(usize), // extension -> loader index

    pub fn init(allocator: std.mem.Allocator) LoaderRegistry {
        return .{
            .allocator = allocator,
            .loaders = std.ArrayList(AssetLoader).init(allocator),
            .extension_map = std.StringHashMap(usize).init(allocator),
        };
    }

    pub fn deinit(self: *LoaderRegistry) void {
        for (self.loaders.items) |*loader| {
            loader.deinit(self.allocator);
        }
        self.loaders.deinit();

        // 清理 extension_map 的键
        var iter = self.extension_map.keyIterator();
        while (iter.next()) |key| {
            self.allocator.free(key.*);
        }
        self.extension_map.deinit();
    }

    pub fn registerLoader(self: *LoaderRegistry, loader: AssetLoader) !void {
        const loader_index = self.loaders.items.len;
        try self.loaders.append(loader);

        // 注册所有扩展名
        for (0..loader.extensions_count) |i| {
            const ptr = loader.extensions_ptr[i];
            const len = loader.extensions_len_ptr[i];
            const ext = ptr[0..len];

            const ext_copy = try self.allocator.dupe(u8, ext);
            try self.extension_map.put(ext_copy, loader_index);
        }
    }

    pub fn getLoaderForExtension(self: *const LoaderRegistry, ext: []const u8) ?*const AssetLoader {
        if (self.extension_map.get(ext)) |index| {
            return &self.loaders.items[index];
        }
        return null;
    }

    pub fn getLoaderForPath(self: *const LoaderRegistry, path: []const u8) ?*const AssetLoader {
        if (std.mem.lastIndexOfScalar(u8, path, '.')) |dot_pos| {
            if (dot_pos < path.len - 1) {
                const ext = path[dot_pos + 1 ..];
                return self.getLoaderForExtension(ext);
            }
        }
        return null;
    }

    pub fn hasLoaderFor(self: *const LoaderRegistry, ext: []const u8) bool {
        return self.extension_map.contains(ext);
    }
};

// ============================================================================
// 简单的文本加载器示例
// ============================================================================
pub fn createTextLoader(allocator: std.mem.Allocator, type_id: u64) !AssetLoader {
    const extensions = [_][]const u8{ "txt", "text", "md" };
    return try AssetLoader.init(&extensions, textLoadFn, type_id, allocator);
}

fn textLoadFn(data_ptr: [*]const u8, data_len: usize) callconv(.C) ?*anyopaque {
    const allocator = std.heap.c_allocator;
    const data = data_ptr[0..data_len];

    // 复制文本数据
    const text_copy = allocator.dupe(u8, data) catch return null;
    return @ptrCast(text_copy.ptr);
}

// ============================================================================
// FFI exports
// ============================================================================

export fn loader_registry_create() ?*LoaderRegistry {
    const allocator = std.heap.c_allocator;
    const registry = allocator.create(LoaderRegistry) catch return null;
    registry.* = LoaderRegistry.init(allocator);
    return registry;
}

export fn loader_registry_destroy(registry: *LoaderRegistry) void {
    const allocator = registry.allocator;
    registry.deinit();
    allocator.destroy(registry);
}

export fn loader_registry_register(
    registry: *LoaderRegistry,
    extensions_ptr: [*]const [*]const u8,
    extensions_len_ptr: [*]const usize,
    extensions_count: usize,
    load_fn: *const fn ([*]const u8, usize) callconv(.C) ?*anyopaque,
    type_id: u64,
) bool {
    const loader = AssetLoader{
        .extensions_ptr = extensions_ptr,
        .extensions_len_ptr = extensions_len_ptr,
        .extensions_count = extensions_count,
        .load_fn = load_fn,
        .type_id = type_id,
    };

    registry.registerLoader(loader) catch return false;
    return true;
}

export fn loader_registry_has_loader_for_ext(
    registry: *const LoaderRegistry,
    ext_ptr: [*]const u8,
    ext_len: usize,
) bool {
    const ext = ext_ptr[0..ext_len];
    return registry.hasLoaderFor(ext);
}

export fn loader_registry_load_from_path(
    registry: *const LoaderRegistry,
    path_ptr: [*]const u8,
    path_len: usize,
    data_ptr: [*]const u8,
    data_len: usize,
) ?*anyopaque {
    const path = path_ptr[0..path_len];
    const data = data_ptr[0..data_len];

    if (registry.getLoaderForPath(path)) |loader| {
        return loader.load(data);
    }
    return null;
}

export fn create_text_loader(type_id: u64) ?*AssetLoader {
    const allocator = std.heap.c_allocator;
    const loader = allocator.create(AssetLoader) catch return null;
    loader.* = createTextLoader(allocator, type_id) catch {
        allocator.destroy(loader);
        return null;
    };
    return loader;
}

export fn asset_loader_destroy(loader: *AssetLoader) void {
    const allocator = std.heap.c_allocator;
    loader.deinit(allocator);
    allocator.destroy(loader);
}
