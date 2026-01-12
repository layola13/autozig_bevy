const std = @import("std");

/// WASM 兼容的全局分配器
fn getGlobalAllocator() std.mem.Allocator {
    const builtin = @import("builtin");
    if (builtin.target.cpu.arch.isWasm()) {
        // WASM 环境：使用固定大小的缓冲区分配器
        const State = struct {
            var buffer: [1024 * 1024 * 10]u8 = undefined; // 10MB 缓冲区
            var fba = std.heap.FixedBufferAllocator.init(&buffer);
            var initialized = false;
        };

        if (!State.initialized) {
            State.fba = std.heap.FixedBufferAllocator.init(&State.buffer);
            State.initialized = true;
        }

        return State.fba.allocator();
    } else {
        // 非 WASM 环境：使用标准 page_allocator
        return std.heap.page_allocator;
    }
}

/// WASM兼容的时间戳获取
fn getTimestamp() i64 {
    // 在 WASM 环境下使用简单的计数器
    // 注意：这不是真实的时间戳，仅用于相对时间测量
    const builtin = @import("builtin");
    if (builtin.target.cpu.arch.isWasm()) {
        // WASM 环境：使用静态计数器
        const State = struct {
            var counter: i64 = 0;
        };
        State.counter += 1;
        return State.counter;
    } else {
        // 非 WASM 环境：使用实际时间戳
        return @intCast(std.time.nanoTimestamp());
    }
}

/// 测量值（内联定义，避免跨文件导入）
pub const DiagnosticMeasurement = struct {
    value: f64,
    timestamp: i64,
};

/// 诊断路径（内联定义）
pub const DiagnosticPath = struct {
    path: []const u8,
    hash: u64,
    allocator: std.mem.Allocator,
};

/// 诊断数据结构
pub const Diagnostic = struct {
    path_str: []const u8,
    path_hash: u64,
    suffix: []const u8,
    history: std.ArrayList(DiagnosticMeasurement),
    sum: f64,
    ema: f64, // Exponential Moving Average
    ema_smoothing_factor: f64,
    max_history_length: usize,
    is_enabled: bool,
    allocator: std.mem.Allocator,

    pub fn create(allocator: std.mem.Allocator, path_str: []const u8, path_hash: u64, max_history_length: usize, ema_smoothing_factor: f64, suffix: []const u8) !*Diagnostic {
        const diag = try allocator.create(Diagnostic);

        const path_copy = try allocator.dupe(u8, path_str);
        const suffix_copy = try allocator.dupe(u8, suffix);

        // 创建空的历史记录数组（手动构造ArrayList）
        const empty_history = try allocator.alloc(DiagnosticMeasurement, 0);

        diag.* = Diagnostic{
            .path_str = path_copy,
            .path_hash = path_hash,
            .suffix = suffix_copy,
            .history = .{ .items = empty_history, .capacity = 0 },
            .sum = 0.0,
            .ema = 0.0,
            .ema_smoothing_factor = ema_smoothing_factor,
            .max_history_length = max_history_length,
            .is_enabled = true,
            .allocator = allocator,
        };

        return diag;
    }

    pub fn destroy(self: *Diagnostic) void {
        self.history.deinit(self.allocator);
        self.allocator.free(self.path_str);
        self.allocator.free(self.suffix);
        self.allocator.destroy(self);
    }

    /// 添加测量值
    pub fn addMeasurement(self: *Diagnostic, value: f64) !void {
        if (!self.is_enabled) return;

        const measurement = DiagnosticMeasurement{
            .value = value,
            .timestamp = @intCast(getTimestamp()),
        };

        // 添加到历史记录
        try self.history.append(self.allocator, measurement);

        // 更新总和
        self.sum += value;

        // 更新EMA
        if (self.history.items.len == 1) {
            // 第一个值直接作为EMA
            self.ema = value;
        } else {
            // EMA = α * new_value + (1 - α) * old_ema
            self.ema = self.ema_smoothing_factor * value +
                (1.0 - self.ema_smoothing_factor) * self.ema;
        }

        // 限制历史记录长度
        while (self.history.items.len > self.max_history_length) {
            const removed = self.history.orderedRemove(0);
            self.sum -= removed.value;
        }
    }

    /// 获取平均值
    pub fn getAverage(self: *const Diagnostic) f64 {
        if (self.history.items.len == 0) return 0.0;
        return self.sum / @as(f64, @floatFromInt(self.history.items.len));
    }

    /// 获取平滑值（EMA）
    pub fn getSmoothed(self: *const Diagnostic) f64 {
        return self.ema;
    }

    /// 获取最新值
    pub fn getValue(self: *const Diagnostic) ?f64 {
        if (self.history.items.len == 0) return null;
        return self.history.items[self.history.items.len - 1].value;
    }

    /// 清空历史记录
    pub fn clearHistory(self: *Diagnostic) void {
        self.history.clearRetainingCapacity();
        self.sum = 0.0;
        self.ema = 0.0;
    }

    /// 获取历史记录数量
    pub fn getHistoryLen(self: *const Diagnostic) usize {
        return self.history.items.len;
    }

    /// 启用/禁用诊断
    pub fn setEnabled(self: *Diagnostic, enabled: bool) void {
        self.is_enabled = enabled;
    }

    pub fn isEnabled(self: *const Diagnostic) bool {
        return self.is_enabled;
    }
};

// FFI exports
export fn diagnostic_create(path_ptr: [*]const u8, path_len: usize, path_hash: u64, max_history_length: usize, ema_smoothing_factor: f64, suffix_ptr: [*]const u8, suffix_len: usize) ?*Diagnostic {
    const allocator = getGlobalAllocator();
    const path = path_ptr[0..path_len];
    const suffix = suffix_ptr[0..suffix_len];
    return Diagnostic.create(allocator, path, path_hash, max_history_length, ema_smoothing_factor, suffix) catch null;
}

export fn diagnostic_destroy(diag: *Diagnostic) void {
    diag.destroy();
}

export fn diagnostic_add_measurement(diag: *Diagnostic, value: f64) void {
    diag.addMeasurement(value) catch {};
}

export fn diagnostic_get_average(diag: *const Diagnostic) f64 {
    return diag.getAverage();
}

export fn diagnostic_get_smoothed(diag: *const Diagnostic) f64 {
    return diag.getSmoothed();
}

export fn diagnostic_get_value(diag: *const Diagnostic, out_has_value: *bool) f64 {
    if (diag.getValue()) |val| {
        out_has_value.* = true;
        return val;
    } else {
        out_has_value.* = false;
        return 0.0;
    }
}

export fn diagnostic_clear_history(diag: *Diagnostic) void {
    diag.clearHistory();
}

export fn diagnostic_get_history_len(diag: *const Diagnostic) usize {
    return diag.getHistoryLen();
}

export fn diagnostic_set_enabled(diag: *Diagnostic, enabled: bool) void {
    diag.setEnabled(enabled);
}

export fn diagnostic_is_enabled(diag: *const Diagnostic) bool {
    return diag.isEnabled();
}

export fn diagnostic_get_path_hash(diag: *const Diagnostic) u64 {
    return diag.path_hash;
}

export fn diagnostic_copy_path_string(diag: *const Diagnostic, buf: [*]u8, buf_len: usize) usize {
    if (buf_len == 0) {
        // 只返回长度
        return diag.path_str.len;
    }

    // 复制字符串到buffer
    const copy_len = @min(diag.path_str.len, buf_len);
    @memcpy(buf[0..copy_len], diag.path_str[0..copy_len]);
    return copy_len;
}
