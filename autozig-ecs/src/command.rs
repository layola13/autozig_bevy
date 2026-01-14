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

use crate::component::info::hash_type_id;

/// 计算组件类型的Hash作为ID
fn get_component_id<C: 'static>() -> u32 {
    hash_type_id(TypeId::of::<C>())
}

pub struct CommandBuffer {
    inner: *mut CommandBufferOpaque,
}

// SAFETY: CommandBuffer is designed to be used across threads
unsafe impl Send for CommandBuffer {}
unsafe impl Sync for CommandBuffer {}

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
    
    /// Apply all commands
    pub fn apply(&mut self) -> u32 {
        command_buffer_apply_simple(self.inner)
    }

    pub fn apply_with_world(&mut self, world: &mut crate::world::World) {
        let mut cmds = self.commands();
        cmds.apply(world);
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

unsafe impl<'w> Send for Commands<'w> {}
unsafe impl<'w> Sync for Commands<'w> {}

impl<'w> Commands<'w> {

    /// Create new Commands from internals (used by World)
    pub fn new_from_entities(_allocator: &crate::entity::EntityAllocator, _entities: &crate::entity::Entities) -> Self {
        // In this implementation, Commands creates its own internal buffer
        // In real Bevy, it borrows from World's queue or similar.
        // Here we create a temporary buffer for immediate flush? 
        // No, Commands usually writes to a CommandQueue.
        // Our Commands wraps a CommandBufferOpaque created via new().
        // But World::commands() implies we are creating one attached to World?
        // Or creating a standalone one that will be applied to World later?
        // World::commands() usually returns a Commands that queues to a system's queue.
        // But here we are creating one on the fly.
        // Let's create a fresh buffer.
        let buffer = command_buffer_create();
        Self {
            buffer,
            resource_queue: Vec::new(),
            _marker: PhantomData,
        }
    }


    /// Spawn a new entity (returns placeholder)
    pub fn spawn_empty(&mut self) -> EntityCommands<'_> {
        command_buffer_write_spawn(self.buffer);
        EntityCommands {
            buffer: self.buffer,
            entity: None,
            queue: &mut self.resource_queue,
            _marker: PhantomData,
        }
    }
    
    /// Spawn an entity with a bundle of components (alias for spawn_bundle)
    pub fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityCommands<'_> {
        self.spawn_bundle(bundle)
    }

    /// Spawn an entity with a bundle of components
    pub fn spawn_bundle<B: Bundle>(&mut self, bundle: B) -> EntityCommands<'_> {
        command_buffer_write_spawn(self.buffer);
        let components = bundle.get_components();
        for (type_id, data_ptr, data_size) in components.iter() {
            command_buffer_write_insert(
                self.buffer,
                u32::MAX, // Sentinel for "Latest Spawned"
                hash_type_id(*type_id),
                *data_ptr,
                *data_size,
            );
        }
        std::mem::forget(bundle);
        EntityCommands {
            buffer: self.buffer,
            entity: None, 
            queue: &mut self.resource_queue,
            _marker: PhantomData,
        }
    }

    /// Insert a resource
    pub fn insert_resource<R: crate::resource::Resource>(&mut self, resource: R) {
        self.resource_queue.push(Box::new(move |world| {
            world.insert_resource(resource);
        }));
    }

    /// Despawn an entity
    pub fn entity(&mut self, entity: Entity) -> EntityCommands<'_> {
        EntityCommands {
            buffer: self.buffer,
            entity: Some(entity),
            queue: &mut self.resource_queue,
            _marker: PhantomData,
        }
    }
    
    /// Add a custom command
    pub fn add<C: Command>(&mut self, command: C) {
        self.resource_queue.push(Box::new(move |world| {
            command.apply(world);
        }));
    }

    /// Apply all commands
    pub fn apply(&mut self, world: &mut crate::world::World) {
        unsafe {
            let mut ptr: *const u8 = std::ptr::null();
            let mut len: usize = 0;
            command_buffer_get_stream(self.buffer, &mut ptr, &mut len);
            
            if len > 0 {
                let bytes = std::slice::from_raw_parts(ptr, len);
                let mut cursor = 0;
                let mut last_entity = Entity::from_raw(0); 
                
                while cursor < len {
                    let op = bytes[cursor];
                    cursor += 1;
                    match op {
                        1 => { last_entity = world.spawn_empty().id(); },
                        2 => { 
                             let entity_idx = std::ptr::read_unaligned(bytes[cursor..].as_ptr() as *const u32);
                             cursor += 4;
                             let entity = Entity::from_raw(entity_idx); 
                             world.despawn(entity); 
                        },
                        3 => { 
                            let entity_idx = std::ptr::read_unaligned(bytes[cursor..].as_ptr() as *const u32);
                            cursor += 4;
                            let component_id = std::ptr::read_unaligned(bytes[cursor..].as_ptr() as *const u32);
                            cursor += 4;
                            let data_len = std::ptr::read_unaligned(bytes[cursor..].as_ptr() as *const u32) as usize;
                            cursor += 4;
                            let target_entity = if entity_idx == u32::MAX { last_entity } else { Entity::from_raw(entity_idx) };
                            let data_ptr = bytes[cursor..].as_ptr();
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
                        4 => { cursor += 8; },
                        _ => { break; }
                    }
                }
            }
        }
        command_buffer_clear(self.buffer);
        for cmd in self.resource_queue.drain(..) {
            cmd(world);
        }
    }
}

/// Helper trait for commands
pub trait Command: Send + Sync + 'static {
    fn apply(self, world: &mut crate::world::World);
}

impl<F> Command for F where F: FnOnce(&mut crate::world::World) + Send + Sync + 'static {
    fn apply(self, world: &mut crate::world::World) {
        self(world);
    }
}

/// Commands for a specific entity
pub struct EntityCommands<'w> {
    buffer: *mut CommandBufferOpaque,
    entity: Option<Entity>,
    queue: &'w mut Vec<Box<dyn FnOnce(&mut crate::world::World) + Send + Sync>>,
    _marker: PhantomData<&'w mut ()>,
}

impl<'w> EntityCommands<'w> {
    /// Get the entity ID
    pub fn id(&self) -> Entity {
        self.entity.expect("Entity ID not available for buffered spawn")
    }

    /// Add a custom command
    pub fn add<C: Command>(&mut self, command: C) -> &mut Self {
        self.queue.push(Box::new(move |world| {
            command.apply(world);
        }));
        self
    }

    /// Insert a component
    pub fn insert<C: 'static>(&mut self, component: C) -> &mut Self {
        let component_id = get_component_id::<C>();
        let data = &component as *const C as *const u8;
        let size = std::mem::size_of::<C>();
        
        // Use sentinel for latest, or actual index
        let entity_idx = self.entity.map(|e| e.index()).unwrap_or(u32::MAX);
        
        command_buffer_write_insert(self.buffer, entity_idx, component_id, data, size);
        std::mem::forget(component);
        
        self
    }
    
    // ... insert_bundle similar update ...
    pub fn insert_bundle<B: Bundle>(&mut self, bundle: B) -> &mut Self {
        let components = bundle.get_components();
        let entity_idx = self.entity.map(|e| e.index()).unwrap_or(u32::MAX);

        for (type_id, data_ptr, data_size) in components.iter() {
            command_buffer_write_insert(
                self.buffer,
                entity_idx,
                hash_type_id(*type_id),
                *data_ptr,
                *data_size,
            );
        }
        
        std::mem::forget(bundle);
        self
    }
    
    // ... remove ...
    pub fn remove<C: 'static>(&mut self) -> &mut Self {
         let component_id = get_component_id::<C>();
         let entity_idx = self.entity.map(|e| e.index()).unwrap_or(u32::MAX);
         command_buffer_write_remove(self.buffer, entity_idx, component_id);
         self
    }
    
    pub fn despawn(&mut self) {
        let entity_idx = self.entity.map(|e| e.index()).unwrap_or(u32::MAX);
        if entity_idx != u32::MAX { // Can only despawn known entity easily via OpCode? 
             // Logic for despawning sentinel target requires explicit support or just assume "Latest".
             command_buffer_write_despawn(self.buffer, entity_idx);
        }
    }

}

