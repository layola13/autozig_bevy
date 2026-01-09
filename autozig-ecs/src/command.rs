//! Command system - Bevy-compatible deferred command execution

use autozig::include_zig;
use std::marker::PhantomData;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::any::TypeId;
use crate::bundle::Bundle;
use crate::entity::Entity;

#[repr(C)]
pub struct CommandBufferOpaque {
    _private: u8,
}

include_zig!("src/zig/command.zig", {
    fn command_buffer_create() -> *mut CommandBufferOpaque;
    fn command_buffer_destroy(buffer: *mut CommandBufferOpaque);
    fn command_buffer_write_spawn(buffer: *mut CommandBufferOpaque) -> bool;
    fn command_buffer_write_despawn(buffer: *mut CommandBufferOpaque, entity_idx: u32) -> bool;
    fn command_buffer_write_insert(
        buffer: *mut CommandBufferOpaque,
        entity_idx: u32,
        component_id: u32,
        data_ptr: *const u8,
        data_len: usize,
    ) -> bool;
    fn command_buffer_write_remove(
        buffer: *mut CommandBufferOpaque,
        entity_idx: u32,
        component_id: u32,
    ) -> bool;
    fn command_buffer_clear(buffer: *mut CommandBufferOpaque);
    fn command_buffer_get_stream(
        buffer: *const CommandBufferOpaque,
        out_ptr: *mut *const u8,
        out_len: *mut usize,
    );
    fn command_buffer_is_empty(buffer: *const CommandBufferOpaque) -> bool;
    fn command_buffer_apply_simple(buffer: *mut CommandBufferOpaque) -> u32;
});

/// 计算组件类型的Hash作为ID
fn get_component_id<C: 'static>() -> u32 {
    let mut hasher = DefaultHasher::new();
    TypeId::of::<C>().hash(&mut hasher);
    (hasher.finish() & 0xFFFFFFFF) as u32
}

pub struct CommandBuffer {
    inner: *mut CommandBufferOpaque,
}

impl CommandBuffer {
    pub fn new() -> Self {
        let inner = command_buffer_create();
        Self { inner }
    }
    
    /// Get Commands handle for writing
    pub fn commands(&mut self) -> Commands {
        Commands {
            buffer: self.inner,
            _marker: PhantomData,
        }
    }
    
    /// Apply all commands (简化版，实际需要World)
    pub fn apply(&mut self) -> u32 {
        command_buffer_apply_simple(self.inner)
    }
    
    pub fn clear(&mut self) {
        command_buffer_clear(self.inner);
    }
    
    pub fn is_empty(&self) -> bool {
        command_buffer_is_empty(self.inner)
    }
}

impl Drop for CommandBuffer {
    fn drop(&mut self) {
        command_buffer_destroy(self.inner);
    }
}

impl Default for CommandBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Bevy-compatible Commands API
pub struct Commands<'w> {
    buffer: *mut CommandBufferOpaque,
    _marker: PhantomData<&'w mut ()>,
}

impl<'w> Commands<'w> {
    /// Spawn a new entity (returns placeholder)
    pub fn spawn_empty(&mut self) -> EntityCommands {
        command_buffer_write_spawn(self.buffer);
        EntityCommands {
            buffer: self.buffer,
            _marker: PhantomData,
        }
    }
    
    /// Spawn an entity with a bundle of components
    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> EntityCommands {
        // 先写入 spawn 命令
        command_buffer_write_spawn(self.buffer);
        
        // 获取 bundle 的所有组件
        let components = bundle.get_components();
        
        // 为每个组件写入 insert 命令
        for (component_id, data_ptr, data_size) in components.iter() {
            command_buffer_write_insert(
                self.buffer,
                0, // entity_idx 会在 apply 时解析
                *component_id,
                *data_ptr,
                *data_size,
            );
        }
        
        // 防止 bundle 被 drop
        std::mem::forget(bundle);
        
        EntityCommands {
            buffer: self.buffer,
            _marker: PhantomData,
        }
    }
    
    /// Despawn an entity
    pub fn entity(&mut self, entity_idx: u32) -> EntityCommands {
        EntityCommands {
            buffer: self.buffer,
            _marker: PhantomData,
        }
    }
}

/// Commands for a specific entity
pub struct EntityCommands<'w> {
    buffer: *mut CommandBufferOpaque,
    _marker: PhantomData<&'w mut ()>,
}

impl<'w> EntityCommands<'w> {
    /// Insert a component
    pub fn insert<C: 'static>(&mut self, component: C) -> &mut Self {
        let component_id = get_component_id::<C>();
        let data = &component as *const C as *const u8;
        let size = std::mem::size_of::<C>();
        
        // 假设entity_idx为0（实际应该跟踪）
        command_buffer_write_insert(self.buffer, 0, component_id, data, size);
        std::mem::forget(component);
        
        self
    }
    
    /// Insert a bundle of components
    pub fn insert_bundle<B: Bundle>(&mut self, bundle: B) -> &mut Self {
        let components = bundle.get_components();
        
        for (component_id, data_ptr, data_size) in components.iter() {
            command_buffer_write_insert(
                self.buffer,
                0, // entity_idx 会在 apply 时解析
                *component_id,
                *data_ptr,
                *data_size,
            );
        }
        
        std::mem::forget(bundle);
        self
    }
    
    /// Remove a component
    pub fn remove<C: 'static>(&mut self) -> &mut Self {
        let component_id = get_component_id::<C>();
        command_buffer_write_remove(self.buffer, 0, component_id);
        self
    }
    
    /// Remove a bundle of components
    pub fn remove_bundle<B: Bundle>(&mut self) -> &mut Self {
        let ids = B::component_ids();
        
        for component_id in ids.iter() {
            command_buffer_write_remove(self.buffer, 0, *component_id);
        }
        
        self
    }
    
    /// Despawn this entity
    pub fn despawn(&mut self) {
        command_buffer_write_despawn(self.buffer, 0);
    }
}
