//! Event system - Bevy-compatible double-buffered events

use autozig::include_zig;
use std::marker::PhantomData;

#[repr(C)]
pub struct EventQueueOpaque {
    _private: u8,
}

include_zig!("src/zig/event.zig", {
    fn event_queue_create(event_size: usize) -> *mut EventQueueOpaque;
    fn event_queue_destroy(queue: *mut EventQueueOpaque);
    fn event_queue_push(queue: *mut EventQueueOpaque, data_ptr: *const u8) -> bool;
    fn event_queue_swap(queue: *mut EventQueueOpaque);
    fn event_queue_get_reader(queue: *const EventQueueOpaque, out_ptr: *mut *const u8, out_len: *mut usize);
    fn event_queue_get_event_count(queue: *const EventQueueOpaque) -> usize;
    fn event_queue_clear(queue: *mut EventQueueOpaque);
});

pub struct Events<E> {
    queue: *mut EventQueueOpaque,
    _marker: PhantomData<E>,
}

impl<E> Events<E> {
    pub fn new() -> Self {
        let queue = event_queue_create(std::mem::size_of::<E>());
        Self {
            queue,
            _marker: PhantomData,
        }
    }
    
    /// Swap buffers (call this every frame)
    pub fn update(&mut self) {
        event_queue_swap(self.queue);
    }
    
    /// Get a writer for sending events
    pub fn get_writer(&mut self) -> EventWriter<E> {
        EventWriter {
            queue: self.queue,
            _marker: PhantomData,
        }
    }
    
    /// Get a reader for reading events
    pub fn get_reader(&self) -> EventReader<E> {
        EventReader {
            queue: self.queue,
            cursor: 0,
            _marker: PhantomData,
        }
    }
    
    /// Clear all events
    pub fn clear(&mut self) {
        event_queue_clear(self.queue);
    }
}

impl<E> Drop for Events<E> {
    fn drop(&mut self) {
        event_queue_destroy(self.queue);
    }
}

impl<E> Default for Events<E> {
    fn default() -> Self {
        Self::new()
    }
}

/// Bevy-compatible event writer
pub struct EventWriter<'w, E> {
    queue: *mut EventQueueOpaque,
    _marker: PhantomData<(&'w (), E)>,
}

impl<'w, E> EventWriter<'w, E> {
    pub fn send(&mut self, event: E) {
        let data = &event as *const E as *const u8;
        event_queue_push(self.queue, data);
        std::mem::forget(event); // 不要drop，Zig管理内存
    }
}

/// Bevy-compatible event reader
pub struct EventReader<'w, E> {
    queue: *const EventQueueOpaque,
    cursor: usize,
    _marker: PhantomData<(&'w (), E)>,
}

impl<'w, E: Clone> EventReader<'w, E> {
    pub fn read(&mut self) -> EventIterator<'w, E> {
        let mut ptr: *const u8 = std::ptr::null();
        let mut len: usize = 0;
        
        event_queue_get_reader(self.queue, &mut ptr, &mut len);
        
        EventIterator {
            data: unsafe { std::slice::from_raw_parts(ptr, len) },
            cursor: 0,
            event_size: std::mem::size_of::<E>(),
            _marker: PhantomData,
        }
    }
    
    pub fn len(&self) -> usize {
        event_queue_get_event_count(self.queue)
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub struct EventIterator<'a, E> {
    data: &'a [u8],
    cursor: usize,
    event_size: usize,
    _marker: PhantomData<E>,
}

impl<'a, E: Clone> Iterator for EventIterator<'a, E> {
    type Item = E;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.data.len() {
            return None;
        }
        
        let event_bytes = &self.data[self.cursor..self.cursor + self.event_size];
        self.cursor += self.event_size;
        
        // 从字节重建事件
        unsafe {
            let event_ptr = event_bytes.as_ptr() as *const E;
            Some(event_ptr.read())
        }
    }
}
