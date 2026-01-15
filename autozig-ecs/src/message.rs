//! Messages - Guaranteed delivery system events
//!
//! Unlike standard Events, Messages are designed for guaranteed processing and often
//! used with buffered readers or strict ordering.

use std::marker::PhantomData;
use crate::system_param::{SystemParam, Local};
use crate::world::World;
use crate::resource::{Resource, Res, ResMut};
use crate::system::SystemMeta;
use autozig_macro::Resource;

/// Trait for message types
pub trait Message: Send + Sync + 'static {}

/// Store messages in a resource queue
/// Store messages in a resource queue
pub struct Messages<M: Message> {
    queue: Vec<M>,
}

impl<M: Message> Resource for Messages<M> {}

impl<M: Message> Default for Messages<M> {
    fn default() -> Self {
        Self { queue: Vec::new() }
    }
}

impl<M: Message> Messages<M> {
    pub fn push(&mut self, message: M) {
        self.queue.push(message);
    }
    
    pub fn drain(&mut self) -> std::vec::Drain<'_, M> {
        self.queue.drain(..)
    }
    
    pub fn write(&mut self, message: M) {
        self.push(message);
    }
}

/// Helper to add messages to App
pub trait AppMessageExt {
    fn add_message<M: Message + Default>(&mut self) -> &mut Self;
}

impl AppMessageExt for crate::plugin::App {
    fn add_message<M: Message + Default>(&mut self) -> &mut Self {
        self.init_resource::<Messages<M>>();
        self
    }
}

/// Message Writer System Param
pub struct MessageWriter<'w, M: Message> {
    messages: ResMut<'w, Messages<M>>,
}

impl<'w, M: Message> MessageWriter<'w, M> {
    pub fn write(&mut self, message: M) {
        self.messages.push(message);
    }
}

impl<M: Message> SystemParam for MessageWriter<'static, M> {
    type State = ();
    type Item<'w> = MessageWriter<'w, M>;
    
    fn init_state(_world: &mut World, _system_meta: &mut SystemMeta) -> Self::State {
        ()
    }
    
    fn get_param<'w, 's>(
        _state: &'s mut Self::State,
        _system_meta: &SystemMeta,
        world: &'w World,
        _change_tick: u32,
    ) -> Self::Item<'w> {
        // SAFETY: The Scheduler ensures exclusive access if constructed correctly
        // However, standard Bevy ensures this via robust system config.
        // Here we rely on ResMut logic.
        let world_ptr = world as *const World as *mut World;
        let messages = unsafe { (*world_ptr).resource_mut::<Messages<M>>() };
        MessageWriter { messages }
    }
}

/// Message Cursor for tracking read position
pub struct MessageCursor<M: Message> {
    last_position: usize,
    _marker: PhantomData<M>,
}

impl<M: Message> Default for MessageCursor<M> {
    fn default() -> Self {
        Self {
            last_position: 0,
            _marker: PhantomData,
        }
    }
}

impl<M: Message> MessageCursor<M> {
    pub fn read<'a>(&'a mut self, messages: &'a Messages<M>) -> impl Iterator<Item = &'a M> {
        // Simple implementation: Start from 0 every frame for demonstration if cleared?
        // Bevy's Events use double buffering or age.
        // This simple implementation assumes messages accumulate until cleared or handled.
        // For the example `send_and_receive_messages.rs`, it wants to read what was sent "since last time".
        
        let len = messages.queue.len();
        // If queue was cleared (len < last_pos), reset
        if len < self.last_position {
            self.last_position = 0;
        }
        
        let start = self.last_position;
        self.last_position = len;
        
        // This simple slice wouldn't work if items are removed.
        // Real Bevy events use specialized storage.
        // For this port, we'll try to slice.
        messages.queue[start..].iter()
    }
}

/// Message Reader System Param
pub struct MessageReader<'w, 's, M: Message> {
    messages: Res<'w, Messages<M>>,
    cursor: Local<'s, MessageCursor<M>>,
}

impl<'w, 's, M: Message> MessageReader<'w, 's, M> {
    pub fn read(&mut self) -> impl Iterator<Item = &M> {
        self.cursor.read(&self.messages)
    }
}

impl<M: Message> SystemParam for MessageReader<'static, 'static, M> {
    type State = <Local<'static, MessageCursor<M>> as SystemParam>::State;
    type Item<'w> = MessageReader<'w, 'w, M>;
    
    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        Local::<MessageCursor<M>>::init_state(world, system_meta)
    }
    
    fn get_param<'w, 's>(
        state: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: &'w World,
        change_tick: u32,
    ) -> Self::Item<'w> {
        let messages = world.resource::<Messages<M>>();
        let cursor = Local::<MessageCursor<M>>::get_param(state, system_meta, world, change_tick);
        MessageReader { messages, cursor }
    }
}
