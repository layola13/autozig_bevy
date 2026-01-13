//! Event system - 基于 Zig + Rust 的高性能事件系统

use autozig_macro::include_zig;
use crate::world::World;
use crate::resource::Resource;
use std::marker::PhantomData;

/// Event trait - 标记为事件的 trait
pub trait Event: Resource {}
impl<T: Resource> Event for T {}

#[repr(C)]
pub struct EventQueueOpaque {
    _private: u8,
}

include_zig!("src/zig/event.zig", {
    fn event_queue_create(capacity: usize) -> *mut EventQueueOpaque;
    fn event_queue_destroy(queue: *mut EventQueueOpaque);
    fn event_queue_push(queue: *mut EventQueueOpaque, event: *const std::ffi::c_void, size: usize);
    fn event_queue_clear(queue: *mut EventQueueOpaque);
});

/// Bevy-compatible Events<E> resource
pub struct Events<E: Event> {
    pub(crate) queue: *mut EventQueueOpaque,
    _marker: PhantomData<E>,
}

impl<E: Event> Events<E> {
    pub fn new() -> Self {
        Self {
            queue: event_queue_create(128),
            _marker: PhantomData,
        }
    }

    pub fn send(&mut self, event: E) {
        let ptr = &event as *const E as *const std::ffi::c_void;
        event_queue_push(self.queue, ptr, std::mem::size_of::<E>());
    }

    pub fn clear(&mut self) {
        event_queue_clear(self.queue);
    }
    
    pub fn get_writer(&mut self) -> EventWriter<'_, E> {
        EventWriter::new(self.queue)
    }

    pub fn get_reader(&self) -> EventReader<'_, E> {
        EventReader::new(self.queue)
    }
}

impl<E: Event> Drop for Events<E> {
    fn drop(&mut self) {
        event_queue_destroy(self.queue);
    }
}

impl<E: Event> Default for Events<E> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<E: Event> Send for Events<E> {}
unsafe impl<E: Event> Sync for Events<E> {}

/// Bevy-compatible EventWriter
pub struct EventWriter<'w, E: Event> {
    queue: *mut EventQueueOpaque,
    _marker: PhantomData<&'w mut E>,
}

impl<'w, E: Event> EventWriter<'w, E> {
    pub(crate) fn new(queue: *mut EventQueueOpaque) -> Self {
        Self {
            queue,
            _marker: PhantomData,
        }
    }

    pub fn send(&mut self, event: E) {
        let ptr = &event as *const E as *const std::ffi::c_void;
        event_queue_push(self.queue, ptr, std::mem::size_of::<E>());
    }
}

/// Bevy-compatible EventReader
pub struct EventReader<'w, E: Event> {
    queue: *mut EventQueueOpaque,
    _marker: PhantomData<&'w E>,
}

impl<'w, E: Event> EventReader<'w, E> {
    pub(crate) fn new(queue: *mut EventQueueOpaque) -> Self {
        Self {
            queue,
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> EventIter<'_, E> {
        EventIter {
            _marker: PhantomData,
        }
    }
}

pub struct EventIter<'a, E: Event> {
    _marker: PhantomData<&'a E>,
}

impl<'a, E: Event> Iterator for EventIter<'a, E> {
    type Item = &'a E;
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Implement actual iteration from Zig queue
        None
    }
}

pub struct EventId<E: Event>(u64, PhantomData<E>);
pub struct EventCursor<E: Event>(u64, PhantomData<E>);
pub struct EventRegistry;
pub struct EventParIter;

/// Event sent when the application should exit
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppExit {
    #[default]
    Success,
    Error(u8),
}


