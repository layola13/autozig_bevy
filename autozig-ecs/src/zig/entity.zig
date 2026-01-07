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

// 全局allocator定义（所有其他文件通过extern引用）
pub var gpa = std.heap.GeneralPurposeAllocator(.{}){};
pub const g_allocator = gpa.allocator();
