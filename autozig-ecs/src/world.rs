//! World - ECS核心容器, 90% Zig implementation

use autozig::include_zig;
use crate::entity::Entity;
use crate::bundle::Bundle;
use crate::change_detection::{Tick, RemovedComponents};
use crate::component::Component;
use std::collections::HashMap;
use std::any::TypeId;

// Dummy component for type erasure
#[derive(Debug, Clone, Copy)]
struct DummyComponent;
impl Component for DummyComponent {}

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

// Bundle operations - 从 bundle.rs 导入
use crate::bundle::{bundle_spawn, bundle_insert, bundle_remove};


pub struct World {
    inner: *mut WorldOpaque,
    change_tick: Tick,
    last_change_tick: Tick,
    removed_components: HashMap<TypeId, Box<dyn std::any::Any>>,
}

impl World {
    pub fn new() -> Self {
        let inner = world_create();
        Self {
            inner,
            change_tick: Tick::new(0),
            last_change_tick: Tick::new(0),
            removed_components: HashMap::new(),
        }
    }
    
    /// 推进世界tick，用于变更检测
    pub fn tick(&mut self) {
        self.last_change_tick = self.change_tick;
        self.change_tick.increment();
        
        // 清理所有已移除组件记录
        // Note: We can't directly clear type-erased RemovedComponents
        // Each system that uses RemovedComponents<T> should clear it
        self.removed_components.clear();
    }
    
    /// 获取当前tick
    pub fn current_tick(&self) -> Tick {
        self.change_tick
    }
    
    /// 获取上次变更tick
    pub fn last_change_tick(&self) -> Tick {
        self.last_change_tick
    }
    
    /// 记录组件移除（用于RemovedComponents追踪）
    pub fn record_component_removed<T: Component>(&mut self, entity: Entity) {
        let type_id = TypeId::of::<T>();
        let removed = self.removed_components
            .entry(type_id)
            .or_insert_with(|| Box::new(RemovedComponents::<T>::new(0)));
        
        if let Some(removed) = removed.downcast_mut::<RemovedComponents<T>>() {
            removed.record(entity);
        }
    }
    
    /// 获取已移除的组件追踪器
    pub fn get_removed_components<T: Component>(&self) -> Option<&RemovedComponents<T>> {
        let type_id = TypeId::of::<T>();
        self.removed_components
            .get(&type_id)
            .and_then(|any| any.downcast_ref::<RemovedComponents<T>>())
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
    
    /// Spawn an entity with a bundle of components
    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> Entity {
        let components = bundle.get_components();
        
        if components.is_empty() {
            return self.spawn_empty();
        }
        
        let mut ids = Vec::with_capacity(components.len());
        let mut data_ptrs = Vec::with_capacity(components.len());
        let mut sizes = Vec::with_capacity(components.len());
        
        for (id, ptr, size) in components.iter() {
            ids.push(*id);
            data_ptrs.push(*ptr);
            sizes.push(*size);
        }
        
        let entity = bundle_spawn(
            self.inner,
            ids.as_ptr(),
            ids.len(),
            data_ptrs.as_ptr(),
            sizes.as_ptr(),
        );
        
        // 防止 bundle 被 drop（数据已转移到 Zig）
        std::mem::forget(bundle);
        
        entity
    }
    
    /// Insert a bundle of components to an existing entity
    pub fn insert_bundle<B: Bundle>(&mut self, entity: Entity, bundle: B) -> bool {
        let components = bundle.get_components();
        
        if components.is_empty() {
            return true;
        }
        
        let mut ids = Vec::with_capacity(components.len());
        let mut data_ptrs = Vec::with_capacity(components.len());
        let mut sizes = Vec::with_capacity(components.len());
        
        for (id, ptr, size) in components.iter() {
            ids.push(*id);
            data_ptrs.push(*ptr);
            sizes.push(*size);
        }
        
        let result = bundle_insert(
            self.inner,
            entity,
            ids.as_ptr(),
            ids.len(),
            data_ptrs.as_ptr(),
            sizes.as_ptr(),
        );
        
        // 防止 bundle 被 drop（数据已转移到 Zig）
        std::mem::forget(bundle);
        
        result
    }
    
    /// Remove a bundle of components from an entity
    pub fn remove_bundle<B: Bundle>(&mut self, entity: Entity) -> bool {
        let ids = B::component_ids();
        
        if ids.is_empty() {
            return true;
        }
        
        bundle_remove(
            self.inner,
            entity,
            ids.as_ptr(),
            ids.len(),
        )
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

