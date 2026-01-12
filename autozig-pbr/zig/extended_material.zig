//! Extended Material - 扩展材质系统占位符实现
//! ExtendedMaterial主要在Rust层面使用泛型实现，Zig侧提供基础支持

const std = @import("std");

// ExtendedMaterial的核心逻辑在Rust泛型中实现
// 这里仅提供必要的辅助函数占位符

pub const MaterialExtensionKey = extern struct {
    id: u64,
    flags: u32,
    _padding: u32,
};

export fn material_extension_key_init(id: u64, flags: u32) MaterialExtensionKey {
    return MaterialExtensionKey{
        .id = id,
        .flags = flags,
        ._padding = 0,
    };
}

export fn material_extension_key_matches(key1: *const MaterialExtensionKey, key2: *const MaterialExtensionKey) bool {
    return key1.id == key2.id and key1.flags == key2.flags;
}
