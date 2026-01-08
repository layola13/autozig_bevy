const std = @import("std");

// UUID v4 结构 (128 bits)
pub const Uuid = extern struct {
    bytes: [16]u8,

    // 创建新的UUID v4 (random)
    pub fn new() Uuid {
        var uuid = Uuid{ .bytes = undefined };

        // 生成随机字节
        var prng = std.Random.DefaultPrng.init(@as(u64, @intCast(std.time.milliTimestamp())));
        const random = prng.random();
        random.bytes(&uuid.bytes);

        // 设置版本 (v4) 和变体 (RFC4122)
        uuid.bytes[6] = (uuid.bytes[6] & 0x0f) | 0x40; // version 4
        uuid.bytes[8] = (uuid.bytes[8] & 0x3f) | 0x80; // variant RFC4122

        return uuid;
    }

    // 从字节数组创建
    pub fn fromBytes(bytes: [16]u8) Uuid {
        return Uuid{ .bytes = bytes };
    }

    // 转换为字符串 (标准格式: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx)
    pub fn toString(self: Uuid, buffer: *[36]u8) void {
        const hex = "0123456789abcdef";
        var pos: usize = 0;

        for (self.bytes, 0..) |byte, i| {
            // 添加连字符
            if (i == 4 or i == 6 or i == 8 or i == 10) {
                buffer[pos] = '-';
                pos += 1;
            }

            buffer[pos] = hex[byte >> 4];
            buffer[pos + 1] = hex[byte & 0x0f];
            pos += 2;
        }
    }

    // 从字符串解析 (简化版本，仅支持标准格式)
    pub fn fromString(str: []const u8) ?Uuid {
        if (str.len != 36) return null;

        var uuid = Uuid{ .bytes = undefined };
        var byte_idx: usize = 0;
        var i: usize = 0;

        while (i < str.len) : (i += 1) {
            if (str[i] == '-') continue;

            if (byte_idx >= 16) return null;

            const high = hexCharToValue(str[i]) orelse return null;
            i += 1;
            if (i >= str.len) return null;
            const low = hexCharToValue(str[i]) orelse return null;

            uuid.bytes[byte_idx] = (high << 4) | low;
            byte_idx += 1;
        }

        if (byte_idx != 16) return null;
        return uuid;
    }

    fn hexCharToValue(c: u8) ?u8 {
        return switch (c) {
            '0'...'9' => c - '0',
            'a'...'f' => c - 'a' + 10,
            'A'...'F' => c - 'A' + 10,
            else => null,
        };
    }

    // 比较两个UUID是否相等
    pub fn eql(self: Uuid, other: Uuid) bool {
        return std.mem.eql(u8, &self.bytes, &other.bytes);
    }

    // 转换为u128
    pub fn toU128(self: Uuid) u128 {
        var result: u128 = 0;
        for (self.bytes, 0..) |byte, i| {
            result |= @as(u128, byte) << @intCast(i * 8);
        }
        return result;
    }

    // 从u128创建
    pub fn fromU128(value: u128) Uuid {
        var uuid = Uuid{ .bytes = undefined };
        var v = value;
        for (0..16) |i| {
            uuid.bytes[i] = @truncate(v);
            v >>= 8;
        }
        return uuid;
    }
};

// FFI导出函数
export fn uuid_new() Uuid {
    return Uuid.new();
}

export fn uuid_from_bytes(bytes: *const [16]u8) Uuid {
    return Uuid.fromBytes(bytes.*);
}

export fn uuid_to_string(uuid: Uuid, buffer: *[36]u8) void {
    uuid.toString(buffer);
}

export fn uuid_from_string(str_ptr: [*]const u8, str_len: usize) Uuid {
    const str = str_ptr[0..str_len];
    return Uuid.fromString(str) orelse Uuid{ .bytes = [_]u8{0} ** 16 };
}

export fn uuid_equal(a: Uuid, b: Uuid) bool {
    return a.eql(b);
}

export fn uuid_to_u128(uuid: Uuid) u128 {
    return uuid.toU128();
}

export fn uuid_from_u128(value: u128) Uuid {
    return Uuid.fromU128(value);
}

export fn uuid_get_bytes(uuid: Uuid, out_bytes: *[16]u8) void {
    out_bytes.* = uuid.bytes;
}

// 单元测试
test "UUID creation and format" {
    const uuid = Uuid.new();

    // 检查版本位
    try std.testing.expectEqual(@as(u8, 0x40), uuid.bytes[6] & 0xf0);

    // 检查变体位
    try std.testing.expectEqual(@as(u8, 0x80), uuid.bytes[8] & 0xc0);

    // 测试字符串转换
    var buffer: [36]u8 = undefined;
    uuid.toString(&buffer);

    // 验证格式
    try std.testing.expectEqual(@as(u8, '-'), buffer[8]);
    try std.testing.expectEqual(@as(u8, '-'), buffer[13]);
    try std.testing.expectEqual(@as(u8, '-'), buffer[18]);
    try std.testing.expectEqual(@as(u8, '-'), buffer[23]);
}

test "UUID string parsing" {
    const uuid1 = Uuid.new();
    var buffer: [36]u8 = undefined;
    uuid1.toString(&buffer);

    const uuid2 = Uuid.fromString(&buffer);
    try std.testing.expect(uuid2 != null);
    try std.testing.expect(uuid1.eql(uuid2.?));
}

test "UUID equality" {
    const uuid1 = Uuid.new();
    const uuid2 = uuid1;
    const uuid3 = Uuid.new();

    try std.testing.expect(uuid1.eql(uuid2));
    try std.testing.expect(!uuid1.eql(uuid3));
}

test "UUID u128 conversion" {
    const uuid1 = Uuid.new();
    const value = uuid1.toU128();
    const uuid2 = Uuid.fromU128(value);

    try std.testing.expect(uuid1.eql(uuid2));
}

test "UUID bytes access" {
    var bytes = [_]u8{0} ** 16;
    bytes[0] = 0x12;
    bytes[15] = 0x34;

    const uuid = Uuid.fromBytes(bytes);
    try std.testing.expectEqual(@as(u8, 0x12), uuid.bytes[0]);
    try std.testing.expectEqual(@as(u8, 0x34), uuid.bytes[15]);
}
