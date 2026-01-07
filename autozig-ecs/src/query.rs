// Query placeholder - to be implemented

use autozig::include_zig;

#[repr(C)]
pub struct QueryStateOpaque {
    _private: u8,
}

include_zig!("src/zig/query.zig", {
    fn query_state_create() -> *mut QueryStateOpaque;
    fn query_state_destroy(state: *mut QueryStateOpaque);
    fn query_state_add_entity(state: *mut QueryStateOpaque, entity_index: u32) -> bool;
    fn query_state_clear(state: *mut QueryStateOpaque);
    fn query_state_count(state: *const QueryStateOpaque) -> usize;
    fn query_state_get_entity(state: *const QueryStateOpaque, index: usize) -> u32;
});

pub struct QueryState {
    inner: *mut QueryStateOpaque,
}

impl QueryState {
    pub fn new() -> Self {
        let inner = query_state_create();
        Self { inner }
    }
    
    pub fn add_entity(&mut self, entity_index: u32) -> bool {
        query_state_add_entity(self.inner, entity_index)
    }
    
    pub fn clear(&mut self) {
        query_state_clear(self.inner);
    }
    
    pub fn count(&self) -> usize {
        query_state_count(self.inner)
    }
    
    pub fn iter(&self) -> QueryIter {
        QueryIter {
            state: self.inner,
            index: 0,
            len: self.count(),
        }
    }
}

impl Drop for QueryState {
    fn drop(&mut self) {
        query_state_destroy(self.inner);
    }
}

impl Default for QueryState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct QueryIter {
    state: *const QueryStateOpaque,
    index: usize,
    len: usize,
}

impl Iterator for QueryIter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        let entity = query_state_get_entity(self.state, self.index);
        self.index += 1;
        if entity == 0xFFFFFFFF {
            None
        } else {
            Some(entity)
        }
    }
}

