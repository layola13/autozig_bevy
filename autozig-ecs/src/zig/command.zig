const std = @import("std");
const common = @import("common.zig");
const Entity = common.Entity;
const g_allocator = common.g_allocator;

// OpCode - 命令操作码
pub const OpCode = enum(u8) {
    Spawn = 1,
    Despawn = 2,
    InsertComponent = 3,
    RemoveComponent = 4,
};

// CommandBuffer - 延迟命令队列
pub const CommandBuffer = struct {
    stream: std.ArrayList(u8),
    allocator: std.mem.Allocator,
    
    pub fn init(allocator: std.mem.Allocator) !*CommandBuffer {
        const buffer = try allocator.create(CommandBuffer);
        buffer.* = CommandBuffer{
            .stream = std.ArrayList(u8){},
            .allocator = allocator,
        };
        return buffer;
    }
    
    pub fn deinit(self: *CommandBuffer) void {
        self.stream.deinit(self.allocator);
        self.allocator.destroy(self);
    }
    
    // 写入Spawn命令
    pub fn writeSpawn(self: *CommandBuffer) !void {
        try self.stream.append(self.allocator, @intFromEnum(OpCode.Spawn));
    }
    
    // 写入Despawn命令
    pub fn writeDespawn(self: *CommandBuffer, entity_idx: u32) !void {
        try self.stream.append(self.allocator, @intFromEnum(OpCode.Despawn));
        try self.stream.appendSlice(self.allocator, std.mem.asBytes(&entity_idx));
    }
    
    // 写入InsertComponent命令
    pub fn writeInsert(
        self: *CommandBuffer,
        entity_idx: u32,
        component_id: u32,
        data: []const u8,
    ) !void {
        try self.stream.append(self.allocator, @intFromEnum(OpCode.InsertComponent));
        try self.stream.appendSlice(self.allocator, std.mem.asBytes(&entity_idx));
        try self.stream.appendSlice(self.allocator, std.mem.asBytes(&component_id));
        
        // 写入数据长度
        const data_len: u32 = @intCast(data.len);
        try self.stream.appendSlice(self.allocator, std.mem.asBytes(&data_len));
        
        // 写入数据
        try self.stream.appendSlice(self.allocator, data);
    }
    
    // 写入RemoveComponent命令
    pub fn writeRemove(self: *CommandBuffer, entity_idx: u32, component_id: u32) !void {
        try self.stream.append(self.allocator, @intFromEnum(OpCode.RemoveComponent));
        try self.stream.appendSlice(self.allocator, std.mem.asBytes(&entity_idx));
        try self.stream.appendSlice(self.allocator, std.mem.asBytes(&component_id));
    }
    
    // 清空命令缓冲
    pub fn clear(self: *CommandBuffer) void {
        self.stream.clearRetainingCapacity();
    }
    
    // 获取原始字节流（用于apply）
    pub fn getStream(self: *const CommandBuffer) []const u8 {
        return self.stream.items;
    }
    
    pub fn isEmpty(self: *const CommandBuffer) bool {
        return self.stream.items.len == 0;
    }
};

// 全局allocator在entity.zig中定义，合并后直接可用

// Exported C API
export fn command_buffer_create() ?*CommandBuffer {
    return CommandBuffer.init(g_allocator) catch null;
}

export fn command_buffer_destroy(buffer: *CommandBuffer) void {
    buffer.deinit();
}

export fn command_buffer_write_spawn(buffer: *CommandBuffer) bool {
    buffer.writeSpawn() catch return false;
    return true;
}

export fn command_buffer_write_despawn(buffer: *CommandBuffer, entity_idx: u32) bool {
    buffer.writeDespawn(entity_idx) catch return false;
    return true;
}

export fn command_buffer_write_insert(
    buffer: *CommandBuffer,
    entity_idx: u32,
    component_id: u32,
    data_ptr: [*]const u8,
    data_len: usize,
) bool {
    const data = data_ptr[0..data_len];
    buffer.writeInsert(entity_idx, component_id, data) catch return false;
    return true;
}

export fn command_buffer_write_remove(
    buffer: *CommandBuffer,
    entity_idx: u32,
    component_id: u32,
) bool {
    buffer.writeRemove(entity_idx, component_id) catch return false;
    return true;
}

export fn command_buffer_clear(buffer: *CommandBuffer) void {
    buffer.clear();
}

export fn command_buffer_get_stream(
    buffer: *const CommandBuffer,
    out_ptr: *[*]const u8,
    out_len: *usize,
) void {
    const stream = buffer.getStream();
    out_ptr.* = stream.ptr;
    out_len.* = stream.len;
}

export fn command_buffer_is_empty(buffer: *const CommandBuffer) bool {
    return buffer.isEmpty();
}

// 简化的apply函数（示例，实际需要与World集成）
export fn command_buffer_apply_simple(buffer: *CommandBuffer) u32 {
    var executed: u32 = 0;
    var cursor: usize = 0;
    const bytes = buffer.stream.items;
    
    while (cursor < bytes.len) {
        const op = @as(OpCode, @enumFromInt(bytes[cursor]));
        cursor += 1;
        
        switch (op) {
            .Spawn => {
                // 实际执行spawn逻辑
                executed += 1;
            },
            .Despawn => {
                // 跳过entity_idx (4 bytes)
                cursor += 4;
                executed += 1;
            },
            .InsertComponent => {
                // 跳过entity_idx (4) + component_id (4)
                cursor += 8;
                // 读取data_len
                const data_len = std.mem.bytesToValue(u32, bytes[cursor..][0..4]);
                cursor += 4;
                // 跳过data
                cursor += data_len;
                executed += 1;
            },
            .RemoveComponent => {
                // 跳过entity_idx (4) + component_id (4)
                cursor += 8;
                executed += 1;
            },
        }
    }
    
    buffer.clear();
    return executed;
}
