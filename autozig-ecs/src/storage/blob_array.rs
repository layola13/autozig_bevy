//! BlobArray - 类型擦除的数组存储

use std::ptr::NonNull;

/// BlobArray - 存储类型擦除的数据数组
pub struct BlobArray {
    data: Option<NonNull<u8>>,
    len: usize,
    capacity: usize,
    item_layout: std::alloc::Layout,
}

impl BlobArray {
    pub fn new(item_layout: std::alloc::Layout, capacity: usize) -> Self {
        Self {
            data: None,
            len: 0,
            capacity,
            item_layout,
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    // TODO: 实现剩余的~12个API
    pub fn drop_last_element(&mut self) {
        if self.len > 0 {
            self.len -= 1;
        }
    }
    
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl Drop for BlobArray {
    fn drop(&mut self) {
        // TODO: 正确清理内存
    }
}