//! Event system - Bevy-compatible double-buffered events

use autozig_macro::include_zig;
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

// Safety: EventQueueOpaque is thread-safe internally (managed by Zig)
unsafe impl<E> Send for Events<E> {}
unsafe impl<E> Sync for Events<E> {}

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

impl<'w, E> EventReader<'w, E> {
    pub fn len(&self) -> usize {
        event_queue_get_event_count(self.queue)
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
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

// ============================================================================
// Event Advanced Types - Event高级类型
// ============================================================================

/// EventId - 事件唯一标识符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId<E> {
    id: u64,
    _phantom: PhantomData<E>,
}

impl<E> EventId<E> {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
    
    pub fn id(&self) -> u64 {
        self.id
    }
}

/// EventCursor - 事件游标，用于跟踪读取位置
#[derive(Debug, Clone, Copy)]
pub struct EventCursor {
    position: usize,
}

impl EventCursor {
    pub fn new() -> Self {
        Self { position: 0 }
    }
    
    pub fn position(&self) -> usize {
        self.position
    }
    
    pub fn advance(&mut self, count: usize) {
        self.position += count;
    }
    
    pub fn reset(&mut self) {
        self.position = 0;
    }
}

impl Default for EventCursor {
    fn default() -> Self {
        Self::new()
    }
}

/// EventRegistry - 事件注册表
pub struct EventRegistry {
    events: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self {
            events: std::collections::HashMap::new(),
        }
    }
    
    pub fn register<E: Send + Sync + 'static>(&mut self, events: Events<E>) {
        let type_id = std::any::TypeId::of::<E>();
        self.events.insert(type_id, Box::new(events));
    }
    
    pub fn get<E: 'static>(&self) -> Option<&Events<E>> {
        let type_id = std::any::TypeId::of::<E>();
        self.events.get(&type_id)
            .and_then(|any| any.downcast_ref::<Events<E>>())
    }
    
    pub fn get_mut<E: 'static>(&mut self) -> Option<&mut Events<E>> {
        let type_id = std::any::TypeId::of::<E>();
        self.events.get_mut(&type_id)
            .and_then(|any| any.downcast_mut::<Events<E>>())
    }
}

impl Default for EventRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// EventUpdateSignal - 事件更新信号
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventUpdateSignal {
    frame: u64,
}

impl EventUpdateSignal {
    pub fn new(frame: u64) -> Self {
        Self { frame }
    }
    
    pub fn frame(&self) -> u64 {
        self.frame
    }
    
    pub fn increment(&mut self) {
        self.frame += 1;
    }
}

/// EventSequence - 事件序列号
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventSequence(pub u64);

impl EventSequence {
    pub fn new(seq: u64) -> Self {
        Self(seq)
    }
    
    pub fn next(&mut self) -> Self {
        let current = *self;
        self.0 += 1;
        current
    }
}

/// EventInstance - 事件实例（带ID）
pub struct EventInstance<E> {
    pub id: EventId<E>,
    pub event: E,
}

impl<E> EventInstance<E> {
    pub fn new(id: EventId<E>, event: E) -> Self {
        Self { id, event }
    }
}

/// ManualEventReader - 手动事件读取器
pub struct ManualEventReader<E> {
    cursor: EventCursor,
    _phantom: PhantomData<E>,
}

impl<E> ManualEventReader<E> {
    pub fn new() -> Self {
        Self {
            cursor: EventCursor::new(),
            _phantom: PhantomData,
        }
    }
    
    pub fn read<'w>(&mut self, events: &'w Events<E>) -> ManualEventIterator<'w, E>
    where
        E: Clone,
    {
        ManualEventIterator {
            reader: events.get_reader(),
            cursor: self.cursor,
        }
    }
    
    pub fn len(&self, events: &Events<E>) -> usize {
        events.get_reader().len()
    }
    
    pub fn is_empty(&self, events: &Events<E>) -> bool {
        self.len(events) == 0
    }
    
    pub fn clear(&mut self) {
        self.cursor.reset();
    }
}

impl<E> Default for ManualEventReader<E> {
    fn default() -> Self {
        Self::new()
    }
}

/// ManualEventIterator - 手动事件迭代器
pub struct ManualEventIterator<'a, E: Clone> {
    reader: EventReader<'a, E>,
    cursor: EventCursor,
}

impl<'a, E: Clone> Iterator for ManualEventIterator<'a, E> {
    type Item = E;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.reader.read().next()
    }
}

/// SendBatchIds - 批量发送的事件ID集合
pub struct SendBatchIds<E> {
    ids: Vec<EventId<E>>,
}

impl<E> SendBatchIds<E> {
    pub fn new() -> Self {
        Self { ids: Vec::new() }
    }
    
    pub fn push(&mut self, id: EventId<E>) {
        self.ids.push(id);
    }
    
    pub fn ids(&self) -> &[EventId<E>] {
        &self.ids
    }
    
    pub fn len(&self) -> usize {
        self.ids.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}

impl<E> Default for SendBatchIds<E> {
    fn default() -> Self {
        Self::new()
    }
}

/// EventParIter - 并行事件迭代器
pub struct EventParIter<'w, E> {
    events: &'w Events<E>,
    batch_size: usize,
}

impl<'w, E: Clone> EventParIter<'w, E> {
    pub fn new(events: &'w Events<E>, batch_size: usize) -> Self {
        Self { events, batch_size }
    }
    
    pub fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(E) + Send,
    {
        let mut reader = self.events.get_reader();
        for event in reader.read() {
            f(event);
        }
    }
}

// ============================================================================
// Event Traits - 事件trait
// ============================================================================

/// Event - 事件标记trait
/// 所有可以通过Events<T>发送和接收的类型都必须实现此trait
pub trait Event: Send + Sync + 'static {}

/// SetEntityEventTarget - 设置实体事件目标trait
pub trait SetEntityEventTarget {
    fn set_target(&mut self, entity: crate::entity::Entity);
}
