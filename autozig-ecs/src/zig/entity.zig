const std = @import("std");

// Entity structure matching Rust repr(C)
pub const Entity = extern struct {
    index: u32,
    generation: u32,
};

export fn entity_create(index: u32, generation: u32) Entity {
    return Entity{
        .index = index,
        .generation = generation,
    };
}

export fn entity_index(entity: Entity) u32 {
    return entity.index;
}

export fn entity_generation(entity: Entity) u32 {
    return entity.generation;
}

export fn entity_to_bits(entity: Entity) u64 {
    const index_bits: u64 = entity.index;
    const gen_bits: u64 = entity.generation;
    return index_bits | (gen_bits << 32);
}

export fn entity_from_bits(bits: u64) Entity {
    const index: u32 = @truncate(bits);
    const generation: u32 = @truncate(bits >> 32);
    return Entity{
        .index = index,
        .generation = generation,
    };
}

// 全局allocator定义 - 在 FFI 环境下使用 c_allocator (映射到 malloc)
// 这是 Rust + Zig 混合编译的黄金标准
pub const g_allocator = std.heap.c_allocator;

// 显式初始化函数 - Rust 必须在调用任何其他函数之前先调用此函数
var is_initialized: bool = false;

export fn autozig_init() void {
    is_initialized = true;
}

export fn autozig_is_initialized() bool {
    return is_initialized;
}
