// Component trait - Rust side definition (10%)

use autozig::include_zig;
use std::marker::PhantomData;

pub trait Component: Send + Sync + 'static {}

// Component storage implementation - 90% Zig
#[repr(C)]
pub struct SparseSetOpaque {
    _private: u8,
}

include_zig!("src/zig/component.zig", {
    fn sparse_set_create(component_size: usize) -> *mut SparseSetOpaque;
    fn sparse_set_destroy(set: *mut SparseSetOpaque);
    fn sparse_set_insert(set: *mut SparseSetOpaque, entity_index: u32, data_ptr: *const u8, data_len: usize) -> bool;
    fn sparse_set_remove(set: *mut SparseSetOpaque, entity_index: u32) -> bool;
    fn sparse_set_contains(set: *const SparseSetOpaque, entity_index: u32) -> bool;
    fn sparse_set_len(set: *const SparseSetOpaque) -> usize;
    fn sparse_set_get_entity(set: *const SparseSetOpaque, dense_index: usize) -> u32;
});

pub struct SparseSet<T: Component> {
    inner: *mut SparseSetOpaque,
    _phantom: PhantomData<T>,
}

impl<T: Component> SparseSet<T> {
    pub fn new() -> Self {
        let inner = sparse_set_create(std::mem::size_of::<T>());
        Self {
            inner,
            _phantom: PhantomData,
        }
    }
    
    pub fn insert(&mut self, entity_index: u32, component: T) -> bool {
        let data = &component as *const T as *const u8;
        let size = std::mem::size_of::<T>();
        let result = sparse_set_insert(self.inner, entity_index, data, size);
        std::mem::forget(component); // Don't drop, Zig owns it now
        result
    }
    
    pub fn remove(&mut self, entity_index: u32) -> bool {
        sparse_set_remove(self.inner, entity_index)
    }
    
    pub fn contains(&self, entity_index: u32) -> bool {
        sparse_set_contains(self.inner, entity_index)
    }
    
    pub fn len(&self) -> usize {
        sparse_set_len(self.inner)
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    pub fn iter_entities(&self) -> SparseSetIter {
        SparseSetIter {
            set: self.inner,
            index: 0,
            len: self.len(),
        }
    }
}

impl<T: Component> Drop for SparseSet<T> {
    fn drop(&mut self) {
        sparse_set_destroy(self.inner);
    }
}

pub struct SparseSetIter {
    set: *const SparseSetOpaque,
    index: usize,
    len: usize,
}

impl Iterator for SparseSetIter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        let entity = sparse_set_get_entity(self.set, self.index);
        self.index += 1;
        Some(entity)
    }
}

