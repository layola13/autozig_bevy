const std = @import("std");

// ============================================================================
// Asset Path - 资产路径（支持子资产标签）
// ============================================================================
pub const AssetPath = extern struct {
    path_ptr: [*]const u8,
    path_len: usize,
    label_ptr: [*]const u8,
    label_len: usize,
    has_label: bool,

    pub fn init(path: []const u8) AssetPath {
        return .{
            .path_ptr = path.ptr,
            .path_len = path.len,
            .label_ptr = undefined,
            .label_len = 0,
            .has_label = false,
        };
    }

    pub fn initWithLabel(path: []const u8, label: []const u8) AssetPath {
        return .{
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

    /// 解析路径字符串，格式: "path/to/asset.ext" 或 "path/to/asset.ext#label"
    pub fn parse(allocator: std.mem.Allocator, path_str: []const u8) !AssetPath {
        // 查找 '#' 分隔符
        if (std.mem.indexOfScalar(u8, path_str, '#')) |hash_pos| {
            // 有标签
            const path_part = path_str[0..hash_pos];
            const label_part = path_str[hash_pos + 1 ..];

            const path_copy = try allocator.dupe(u8, path_part);
            const label_copy = try allocator.dupe(u8, label_part);

            return AssetPath{
                .path_ptr = path_copy.ptr,
                .path_len = path_copy.len,
                .label_ptr = label_copy.ptr,
                .label_len = label_copy.len,
                .has_label = true,
            };
        } else {
            // 无标签
            const path_copy = try allocator.dupe(u8, path_str);
            return AssetPath{
                .path_ptr = path_copy.ptr,
                .path_len = path_copy.len,
                .label_ptr = undefined,
                .label_len = 0,
                .has_label = false,
            };
        }
    }

    /// 转换为字符串，格式: "path" 或 "path#label"
    pub fn toString(self: AssetPath, allocator: std.mem.Allocator) ![]u8 {
        const path = self.getPath();

        if (self.getLabel()) |label| {
            // 需要格式化为 "path#label"
            const result = try allocator.alloc(u8, path.len + 1 + label.len);
            @memcpy(result[0..path.len], path);
            result[path.len] = '#';
            @memcpy(result[path.len + 1 ..], label);
            return result;
        } else {
            // 只返回路径
            return try allocator.dupe(u8, path);
        }
    }

    pub fn deinit(self: *AssetPath, allocator: std.mem.Allocator) void {
        allocator.free(self.getPath());
        if (self.has_label) {
            allocator.free(self.getLabel().?);
        }
    }

    pub fn eql(self: AssetPath, other: AssetPath) bool {
        if (!std.mem.eql(u8, self.getPath(), other.getPath())) {
            return false;
        }

        const self_label = self.getLabel();
        const other_label = other.getLabel();

        if (self_label == null and other_label == null) {
            return true;
        }

        if (self_label != null and other_label != null) {
            return std.mem.eql(u8, self_label.?, other_label.?);
        }

        return false;
    }
};

// ============================================================================
// 路径工具函数
// ============================================================================

/// 获取文件扩展名
pub fn getExtension(path: []const u8) ?[]const u8 {
    if (std.mem.lastIndexOfScalar(u8, path, '.')) |dot_pos| {
        if (dot_pos < path.len - 1) {
            return path[dot_pos + 1 ..];
        }
    }
    return null;
}

/// 获取文件名（不含路径）
pub fn getFileName(path: []const u8) []const u8 {
    if (std.mem.lastIndexOfScalar(u8, path, '/')) |slash_pos| {
        return path[slash_pos + 1 ..];
    }
    if (std.mem.lastIndexOfScalar(u8, path, '\\')) |slash_pos| {
        return path[slash_pos + 1 ..];
    }
    return path;
}

/// 获取父目录
pub fn getParentDir(path: []const u8) ?[]const u8 {
    if (std.mem.lastIndexOfScalar(u8, path, '/')) |slash_pos| {
        if (slash_pos > 0) {
            return path[0..slash_pos];
        }
    }
    if (std.mem.lastIndexOfScalar(u8, path, '\\')) |slash_pos| {
        if (slash_pos > 0) {
            return path[0..slash_pos];
        }
    }
    return null;
}

/// 规范化路径（移除 ./ 和 ../）
pub fn normalizePath(allocator: std.mem.Allocator, path: []const u8) ![]u8 {
    var components = std.ArrayList([]const u8).init(allocator);
    defer components.deinit();

    var iter = std.mem.splitScalar(u8, path, '/');
    while (iter.next()) |component| {
        if (component.len == 0 or std.mem.eql(u8, component, ".")) {
            continue;
        } else if (std.mem.eql(u8, component, "..")) {
            if (components.items.len > 0) {
                _ = components.pop();
            }
        } else {
            try components.append(component);
        }
    }

    // 重新组合路径
    var total_len: usize = 0;
    for (components.items) |comp| {
        total_len += comp.len + 1; // +1 for '/'
    }

    if (total_len == 0) {
        return try allocator.dupe(u8, ".");
    }

    const result = try allocator.alloc(u8, total_len - 1); // -1 because no trailing '/'
    var pos: usize = 0;
    for (components.items, 0..) |comp, i| {
        if (i > 0) {
            result[pos] = '/';
            pos += 1;
        }
        @memcpy(result[pos .. pos + comp.len], comp);
        pos += comp.len;
    }

    return result;
}

// ============================================================================
// FFI exports
// ============================================================================

export fn asset_path_init(path_ptr: [*]const u8, path_len: usize) AssetPath {
    const path = path_ptr[0..path_len];
    return AssetPath.init(path);
}

export fn asset_path_init_with_label(
    path_ptr: [*]const u8,
    path_len: usize,
    label_ptr: [*]const u8,
    label_len: usize,
) AssetPath {
    const path = path_ptr[0..path_len];
    const label = label_ptr[0..label_len];
    return AssetPath.initWithLabel(path, label);
}

export fn asset_path_has_label(path: AssetPath) bool {
    return path.has_label;
}

export fn asset_path_get_label_ptr(path: AssetPath) [*]const u8 {
    if (path.has_label) {
        return path.label_ptr;
    }
    return undefined;
}

export fn asset_path_get_label_len(path: AssetPath) usize {
    if (path.has_label) {
        return path.label_len;
    }
    return 0;
}

export fn asset_path_eql(a: AssetPath, b: AssetPath) bool {
    return a.eql(b);
}
