// common.zig - 共享定义，所有其他 Zig 模块引用
const std = @import("std");
const entity_mod = @import("entity.zig");

// Re-export from entity.zig
pub const Entity = entity_mod.Entity;
pub const g_allocator = entity_mod.g_allocator;

// Empty export for include_zig! discovery
export fn common_noop() void {}
