//! Entity types and utilities - 90% Zig implementation

use autozig::include_zig;

// Entity structure matching Zig repr(C)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    pub index: u32,
    pub generation: u32,
}

// Entity的Zig实现 - 引用外部zig文件 (路径相对于Cargo.toml)
include_zig!("src/zig/entity.zig", {
    fn entity_create(index: u32, generation: u32) -> Entity;
    fn entity_index(entity: Entity) -> u32;
    fn entity_generation(entity: Entity) -> u32;
    fn entity_to_bits(entity: Entity) -> u64;
    fn entity_from_bits(bits: u64) -> Entity;
    fn autozig_init();
    fn autozig_is_initialized() -> bool;
});

/// 初始化 AutoZig 运行时 - 必须在使用任何其他功能之前调用
pub fn init() {
    autozig_init();
}

/// 检查 AutoZig 是否已初始化
pub fn is_initialized() -> bool {
    autozig_is_initialized()
}

impl Entity {
    pub const PLACEHOLDER: Self = Self { index: u32::MAX, generation: 0 };
    
    pub fn new(index: u32, generation: u32) -> Self {
        entity_create(index, generation)
    }
    
    pub fn index(self) -> u32 {
        entity_index(self)
    }
    
    pub fn generation(self) -> u32 {
        entity_generation(self)
    }
    
    pub fn to_bits(self) -> u64 {
        entity_to_bits(self)
    }
    
    pub fn from_bits(bits: u64) -> Self {
        entity_from_bits(bits)
    }
    
    pub fn from_raw(index: u32) -> Self {
        Self { index, generation: 0 }
    }
}

