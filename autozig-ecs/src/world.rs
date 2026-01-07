//! World - ECS核心容器, 90% Zig implementation

use autozig::include_zig;
use crate::entity::Entity;

// Opaque pointer to Zig World structure (零大小类型用于类型安全)
#[repr(C)]
pub struct WorldOpaque {
    _private: u8,
}

// World的Zig实现 - 引用外部zig文件 (路径相对于Cargo.toml)
include_zig!("src/zig/world.zig", {
    fn world_create() -> *mut WorldOpaque;
    fn world_destroy(world_ptr: *mut WorldOpaque);
    fn world_spawn_empty(world_ptr: *mut WorldOpaque) -> Entity;
    fn world_despawn(world_ptr: *mut WorldOpaque, entity: Entity) -> bool;
    fn world_entity_count(world_ptr: *const WorldOpaque) -> u32;
    fn world_contains_entity(world_ptr: *const WorldOpaque, entity: Entity) -> bool;
});


pub struct World {
    inner: *mut WorldOpaque,
}

impl World {
    pub fn new() -> Self {
        let inner = world_create();
        Self { inner }
    }
    
    pub fn spawn_empty(&mut self) -> Entity {
        world_spawn_empty(self.inner)
    }
    
    pub fn despawn(&mut self, entity: Entity) -> bool {
        world_despawn(self.inner, entity)
    }
    
    pub fn entity_count(&self) -> u32 {
        world_entity_count(self.inner)
    }
    
    pub fn contains(&self, entity: Entity) -> bool {
        world_contains_entity(self.inner, entity)
    }
}

impl Drop for World {
    fn drop(&mut self) {
        world_destroy(self.inner);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

