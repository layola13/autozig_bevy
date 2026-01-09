const std = @import("std");
const common = @import("common.zig");
const Entity = common.Entity;
const g_allocator = common.g_allocator;

// Rust 闭包的表示 (fat pointer)
pub const RustClosure = struct {
    data_ptr: *anyopaque, // 闭包捕获的数据
    vtable_ptr: *anyopaque, // 虚表指针
};

// Trampoline 函数类型 - 从 Zig 调用 Rust 闭包
pub const TrampolineFn = *const fn (
    closure: *RustClosure,
    world: *anyopaque,
) callconv(.c) void;

// World 访问标记
pub const WorldAccessFlags = packed struct(u8) {
    reads_resources: bool = false,
    writes_resources: bool = false,
    reads_components: bool = false,
    writes_components: bool = false,
    _padding: u4 = 0,
};

// 闭包系统元数据
pub const ClosureSystem = struct {
    name: []const u8,
    closure: RustClosure,
    trampoline: TrampolineFn,
    world_access: WorldAccessFlags,
};

// 闭包系统管理器
pub const ClosureSystemRegistry = struct {
    allocator: std.mem.Allocator,
    systems: std.ArrayList(ClosureSystem),

    pub fn init(allocator: std.mem.Allocator) !*ClosureSystemRegistry {
        const registry = try allocator.create(ClosureSystemRegistry);
        registry.* = .{
            .allocator = allocator,
            .systems = std.ArrayList(ClosureSystem){},
        };
        return registry;
    }

    pub fn deinit(self: *ClosureSystemRegistry) void {
        self.systems.deinit(self.allocator);
        self.allocator.destroy(self);
    }

    pub fn registerClosure(
        self: *ClosureSystemRegistry,
        name: []const u8,
        closure: RustClosure,
        trampoline: TrampolineFn,
        access: WorldAccessFlags,
    ) !void {
        try self.systems.append(self.allocator, .{
            .name = name,
            .closure = closure,
            .trampoline = trampoline,
            .world_access = access,
        });
    }

    pub fn runAll(self: *ClosureSystemRegistry, world: *anyopaque) void {
        for (self.systems.items) |*system| {
            system.trampoline(&system.closure, world);
        }
    }

    pub fn systemCount(self: *const ClosureSystemRegistry) usize {
        return self.systems.items.len;
    }
};

// 全局allocator在entity.zig中定义，合并后直接可用

// C API 导出
export fn closure_registry_create() ?*ClosureSystemRegistry {
    return ClosureSystemRegistry.init(g_allocator) catch null;
}

export fn closure_registry_destroy(registry: *ClosureSystemRegistry) void {
    registry.deinit();
}

export fn closure_registry_register(
    registry: *ClosureSystemRegistry,
    name_ptr: [*]const u8,
    name_len: usize,
    data_ptr: *anyopaque,
    vtable_ptr: *anyopaque,
    trampoline: TrampolineFn,
    access_flags: u8,
) bool {
    const name = name_ptr[0..name_len];
    const closure = RustClosure{
        .data_ptr = data_ptr,
        .vtable_ptr = vtable_ptr,
    };
    const access: WorldAccessFlags = @bitCast(access_flags);
    registry.registerClosure(name, closure, trampoline, access) catch return false;
    return true;
}

export fn closure_registry_run_all(
    registry: *ClosureSystemRegistry,
    world: *anyopaque,
) void {
    registry.runAll(world);
}

export fn closure_registry_system_count(registry: *const ClosureSystemRegistry) usize {
    return registry.systemCount();
}
