const std = @import("std");

// 导入 Diagnostic 定义（需要完整定义才能销毁）
const diagnostic_mod = @import("diagnostic.zig");
pub const Diagnostic = diagnostic_mod.Diagnostic;

/// 诊断条目 - 用于线性存储
const DiagnosticEntry = struct {
    hash: u64,
    diagnostic: *Diagnostic,
};

/// 诊断存储 - 管理所有诊断数据
/// 使用 ArrayList 替代 HashMap，避免跨FFI边界的哈希查询问题
pub const DiagnosticsStore = struct {
    diagnostics: std.ArrayList(DiagnosticEntry), // 线性存储
    allocator: std.mem.Allocator,

    pub fn create(allocator: std.mem.Allocator) !*DiagnosticsStore {
        const store = try allocator.create(DiagnosticsStore);
        store.allocator = allocator;

        // 使用 allocator 分配一个大小为 0 的切片，这样 deinit 可以安全地释放它
        const empty_slice = try allocator.alloc(DiagnosticEntry, 0);

        store.diagnostics = .{
            .items = empty_slice,
            .capacity = 0,
        };

        return store;
    }

    pub fn destroy(self: *DiagnosticsStore) void {
        // 销毁所有注册的诊断
        for (self.diagnostics.items) |entry| {
            entry.diagnostic.destroy();
        }
        // 清理ArrayList - 现在 items 总是从 allocator 分配的，可以安全 deinit
        self.diagnostics.deinit(self.allocator);
        self.allocator.destroy(self);
    }

    /// 注册诊断
    pub fn register(self: *DiagnosticsStore, hash: u64, diagnostic: *Diagnostic) !void {
        // 检查是否已存在
        for (self.diagnostics.items, 0..) |entry, i| {
            if (entry.hash == hash) {
                // 已存在，替换
                entry.diagnostic.destroy();
                self.diagnostics.items[i].diagnostic = diagnostic;
                return;
            }
        }
        // 不存在，追加
        try self.diagnostics.append(self.allocator, DiagnosticEntry{
            .hash = hash,
            .diagnostic = diagnostic,
        });
    }

    /// 通过哈希获取诊断
    pub fn getByHash(self: *DiagnosticsStore, hash: u64) ?*Diagnostic {
        std.debug.print("DEBUG Zig: getByHash called with hash={}, items.len={}\n", .{ hash, self.diagnostics.items.len });
        for (self.diagnostics.items, 0..) |entry, i| {
            std.debug.print("DEBUG Zig: checking entry[{}]: hash={}\n", .{ i, entry.hash });
            if (entry.hash == hash) {
                std.debug.print("DEBUG Zig: found match at index {}\n", .{i});
                return entry.diagnostic;
            }
        }
        std.debug.print("DEBUG Zig: no match found\n", .{});
        return null;
    }

    /// 检查诊断是否存在
    pub fn contains(self: *DiagnosticsStore, hash: u64) bool {
        for (self.diagnostics.items) |entry| {
            if (entry.hash == hash) {
                return true;
            }
        }
        return false;
    }

    /// 获取诊断数量
    pub fn count(self: *DiagnosticsStore) usize {
        return self.diagnostics.items.len;
    }

    /// 清空所有诊断（销毁所有诊断对象）
    pub fn clear(self: *DiagnosticsStore) void {
        // 销毁所有诊断
        for (self.diagnostics.items) |entry| {
            entry.diagnostic.destroy();
        }
        self.diagnostics.clearRetainingCapacity();
    }

    /// 迭代器上下文
    pub const IteratorContext = struct {
        store: *DiagnosticsStore,
        index: usize,
    };

    /// 创建迭代器
    pub fn iterator(self: *DiagnosticsStore) IteratorContext {
        return IteratorContext{
            .store = self,
            .index = 0,
        };
    }
};

// FFI exports
export fn store_create() ?*DiagnosticsStore {
    const allocator = std.heap.page_allocator;
    return DiagnosticsStore.create(allocator) catch null;
}

export fn store_destroy(store: *DiagnosticsStore) void {
    store.destroy();
}

export fn store_register(store: *DiagnosticsStore, hash: u64, diagnostic: *Diagnostic) void {
    store.register(hash, diagnostic) catch {};
}

export fn store_get_by_hash(store: *DiagnosticsStore, hash: u64) ?*Diagnostic {
    return store.getByHash(hash);
}

export fn store_contains(store: *DiagnosticsStore, hash: u64) bool {
    return store.contains(hash);
}

export fn store_count(store: *DiagnosticsStore) usize {
    return store.count();
}

export fn store_clear(store: *DiagnosticsStore) void {
    store.clear();
}

/// 迭代器FFI
export fn store_iterator_create(store: *DiagnosticsStore) ?*DiagnosticsStore.IteratorContext {
    const allocator = std.heap.page_allocator;
    const ctx = allocator.create(DiagnosticsStore.IteratorContext) catch return null;
    ctx.* = store.iterator();
    return ctx;
}

export fn store_iterator_destroy(ctx: *DiagnosticsStore.IteratorContext) void {
    const allocator = std.heap.page_allocator;
    allocator.destroy(ctx);
}

export fn store_iterator_next(ctx: *DiagnosticsStore.IteratorContext) ?*Diagnostic {
    if (ctx.index >= ctx.store.diagnostics.items.len) {
        return null;
    }
    const diag = ctx.store.diagnostics.items[ctx.index].diagnostic;
    ctx.index += 1;
    return diag;
}
