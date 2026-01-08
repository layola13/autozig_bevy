const std = @import("std");

/// FNV-1a hash constants
const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;

/// 诊断路径 - 使用FNV-1a哈希
pub const DiagnosticPath = struct {
    path: []const u8,
    hash: u64,
    allocator: std.mem.Allocator,

    pub fn create(allocator: std.mem.Allocator, path: []const u8) !*DiagnosticPath {
        const path_copy = try allocator.dupe(u8, path);
        const hash = computeFnv1aHash(path);

        const diag_path = try allocator.create(DiagnosticPath);
        diag_path.* = DiagnosticPath{
            .path = path_copy,
            .hash = hash,
            .allocator = allocator,
        };

        return diag_path;
    }

    pub fn destroy(self: *DiagnosticPath) void {
        self.allocator.free(self.path);
        self.allocator.destroy(self);
    }

    pub fn getHash(self: *const DiagnosticPath) u64 {
        return self.hash;
    }

    pub fn getPath(self: *const DiagnosticPath) []const u8 {
        return self.path;
    }

    pub fn equals(self: *const DiagnosticPath, other: *const DiagnosticPath) bool {
        return self.hash == other.hash and std.mem.eql(u8, self.path, other.path);
    }
};

/// 计算FNV-1a哈希
pub fn computeFnv1aHash(data: []const u8) u64 {
    var hash: u64 = FNV_OFFSET_BASIS;

    for (data) |byte| {
        hash ^= @as(u64, byte);
        hash = hash *% FNV_PRIME; // wrapping multiplication
    }

    return hash;
}

// FFI exports
export fn diagnostic_path_create(path_ptr: [*]const u8, path_len: usize) ?*DiagnosticPath {
    const allocator = std.heap.page_allocator;
    const path = path_ptr[0..path_len];
    return DiagnosticPath.create(allocator, path) catch null;
}

export fn diagnostic_path_destroy(path: *DiagnosticPath) void {
    path.destroy();
}

export fn diagnostic_path_get_hash(path: *const DiagnosticPath) u64 {
    return path.getHash();
}

export fn diagnostic_path_copy_string(path: *const DiagnosticPath, buf: [*]u8, buf_len: usize) usize {
    const p = path.getPath();
    if (buf_len == 0) {
        // 只返回长度
        return p.len;
    }

    // 复制字符串到buffer
    const copy_len = @min(p.len, buf_len);
    @memcpy(buf[0..copy_len], p[0..copy_len]);
    return copy_len;
}

export fn diagnostic_path_equals(path1: *const DiagnosticPath, path2: *const DiagnosticPath) bool {
    return path1.equals(path2);
}

export fn diagnostic_path_compute_hash(data_ptr: [*]const u8, data_len: usize) u64 {
    const data = data_ptr[0..data_len];
    return computeFnv1aHash(data);
}
