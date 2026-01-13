//! Command system - Bevy-compatible deferred command execution

use autozig_macro::include_zig;
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
    pub fn commands(&mut self) -> Commands<'_> {
        Commands {
            buffer: self.inner,
            resource_queue: Vec::new(),
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
// ... (lines 80-264 unchanged) ...


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
    resource_queue: Vec<Box<dyn FnOnce(&mut crate::world::World) + Send + Sync>>,
    _marker: PhantomData<&'w mut ()>,
}

impl<'w> Commands<'w> {
    pub fn new(world: &crate::world::World) -> Self {
        unsafe { Self::new_from_entities(&world.allocator, &world.entities) }
    }

    /// Creates a new Commands instance from entities storage
    ///
    /// # Safety
    ///
    /// The caller must ensure that the entities storage remains valid
    pub unsafe fn new_from_entities(
        _allocator: &crate::entity::EntityAllocator,
        _entities: &crate::entity::Entities,
    ) -> Self {
        // 创建一个新的命令缓冲区
        let buffer = command_buffer_create();
        Self {
            buffer,
            resource_queue: Vec::new(),
            _marker: PhantomData,
        }
    }
    
    /// Spawn a new entity (returns placeholder)
    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands<'_> {
        self.spawn_bundle(bundle)
    }

    /// Spawn a new entity (returns placeholder)
    pub fn spawn_empty(&mut self) -> EntityCommands<'_> {
        command_buffer_write_spawn(self.buffer);
        EntityCommands {
            buffer: self.buffer,
            entity: None, // Implies "Latest"
            _marker: PhantomData,
        }
    }
    
    /// Spawn an entity with a bundle of components
    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> EntityCommands<'_> {
        // First writes spawn
        command_buffer_write_spawn(self.buffer);
        
        let components = bundle.get_components();
        
        for (component_id, data_ptr, data_size) in components.iter() {
            command_buffer_write_insert(
                self.buffer,
                u32::MAX, // Sentinel for "Latest Spawned"
                *component_id,
                *data_ptr,
                *data_size,
            );
        }
        
        std::mem::forget(bundle);
        
        EntityCommands {
            buffer: self.buffer,
            entity: None, 
            _marker: PhantomData,
        }
    }

    /// Insert a resource
    pub fn insert_resource<R: crate::resource::Resource>(&mut self, resource: R) {
        self.resource_queue.push(Box::new(move |world| {
            world.insert_resource(resource);
        }));
    }

    /// Spawn a batch of bundles
    pub fn spawn_batch<I>(&mut self, iter: I) 
    where 
        I: IntoIterator,
        I::Item: Bundle,
    {
        for bundle in iter {
             self.spawn_bundle(bundle);
        }
    }
    
    /// Despawn an entity
    pub fn entity(&mut self, entity: Entity) -> EntityCommands<'_> {
        EntityCommands {
            buffer: self.buffer,
            entity: Some(entity),
            _marker: PhantomData,
        }
    }
    
    /// Apply all commands
    pub fn apply(&mut self, world: &mut crate::world::World) {
        // Apply Buffer (Spawn, Insert, Despawn)
        unsafe {
            let mut ptr: *const u8 = std::ptr::null();
            let mut len: usize = 0;
            command_buffer_get_stream(self.buffer, &mut ptr, &mut len);
            
            if len > 0 {
                let bytes = std::slice::from_raw_parts(ptr, len);
                let mut cursor = 0;
                let mut last_entity = Entity::from_raw(0); // Placeholder
                
                while cursor < len {
                    let op = bytes[cursor];
                    cursor += 1;
                    
                    match op {
                        1 => { // Spawn
                            last_entity = world.spawn_empty().id();
                        },
                        2 => { // Despawn
                             let entity_idx = std::ptr::read_unaligned(bytes[cursor..].as_ptr() as *const u32);
                             cursor += 4;
                             let entity = Entity::from_raw(entity_idx); // Simplification: assuming index matches
                             world.despawn(entity); 
                        },
                        3 => { // InsertComponent
                            let entity_idx = std::ptr::read_unaligned(bytes[cursor..].as_ptr() as *const u32);
                            cursor += 4;
                            let component_id = std::ptr::read_unaligned(bytes[cursor..].as_ptr() as *const u32);
                            cursor += 4;
                            let data_len = std::ptr::read_unaligned(bytes[cursor..].as_ptr() as *const u32) as usize;
                            cursor += 4;
                            
                            let target_entity = if entity_idx == u32::MAX { last_entity } else { Entity::from_raw(entity_idx) };
                            
                            let data_ptr = bytes[cursor..].as_ptr();
                            
                            // Use internal bundle insertion
                            world.insert_bundle_components_internal(
                                target_entity,
                                vec![(
                                    crate::component::ComponentId::new(component_id as usize),
                                    data_ptr,
                                    data_len
                                )]
                            );
                            
                            cursor += data_len;
                        },
                        4 => { // RemoveComponent
                             // Skip for now (not used in demo)
                             cursor += 8;
                        },
                        _ => { // Unknown
                             // Just break to avoid infinite loop on bad data
                             break;
                        }
                    }
                }
            }
        }
        
        command_buffer_clear(self.buffer);

        // Apply Resource Queue
        for cmd in self.resource_queue.drain(..) {
            cmd(world);
        }
    }
}

/// Commands for a specific entity
pub struct EntityCommands<'w> {
    buffer: *mut CommandBufferOpaque,
    entity: Option<Entity>,
    _marker: PhantomData<&'w mut ()>,
}

impl<'w> EntityCommands<'w> {
    /// Insert a component
    pub fn insert<C: 'static>(&mut self, component: C) -> &mut Self {
        let component_id = get_component_id::<C>();
        let data = &component as *const C as *const u8;
        let size = std::mem::size_of::<C>();
        
        // Use sentinel for latest, or actual index
        let entity_idx = self.entity.map(|e| e.index()).unwrap_or(u32::MAX);
        
        unsafe { command_buffer_write_insert(self.buffer, entity_idx, component_id, data, size); }
        std::mem::forget(component);
        
        self
    }
    
    // ... insert_bundle similar update ...
    pub fn insert_bundle<B: Bundle>(&mut self, bundle: B) -> &mut Self {
        let components = bundle.get_components();
        let entity_idx = self.entity.map(|e| e.index()).unwrap_or(u32::MAX);

        for (component_id, data_ptr, data_size) in components.iter() {
            unsafe {
                command_buffer_write_insert(
                    self.buffer,
                    entity_idx,
                    *component_id,
                    *data_ptr,
                    *data_size,
                );
            }
        }
        
        std::mem::forget(bundle);
        self
    }
    
    // ... remove ...
    pub fn remove<C: 'static>(&mut self) -> &mut Self {
         let component_id = get_component_id::<C>();
         let entity_idx = self.entity.map(|e| e.index()).unwrap_or(u32::MAX);
         unsafe { command_buffer_write_remove(self.buffer, entity_idx, component_id); }
         self
    }
    
    pub fn despawn(&mut self) {
        let entity_idx = self.entity.map(|e| e.index()).unwrap_or(u32::MAX);
        if entity_idx != u32::MAX { // Can only despawn known entity easily via OpCode? 
             // Logic for despawning sentinel target requires explicit support or just assume "Latest".
             unsafe { command_buffer_write_despawn(self.buffer, entity_idx); }
        }
    }

}

