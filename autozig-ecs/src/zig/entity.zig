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

const builtin = @import("builtin");

// 全局allocator定义 - 根据目标平台选择合适的 allocator
// Native: c_allocator (映射到 malloc，性能最佳)
// WASM: page_allocator (WASM 专用，无需 libc)
pub const g_allocator = if (builtin.cpu.arch.isWasm())
    std.heap.page_allocator
else
    std.heap.c_allocator;

// 显式初始化函数 - Rust 必须在调用任何其他函数之前先调用此函数
var is_initialized: bool = false;

export fn autozig_init() void {
    is_initialized = true;
}

export fn autozig_is_initialized() bool {
    return is_initialized;
}
