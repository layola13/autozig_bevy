//! Column storage for table components

use crate::change_detection::{Tick, ComponentTicks};
use std::alloc::Layout;
use std::ptr::NonNull;

/// Column - 表中的一列，存储特定组件类型的数据
#[derive(Debug)]
pub struct Column {
    data: NonNull<u8>,
    ticks: Vec<ComponentTicks>,
    item_layout: Layout,
    len: usize,
    capacity: usize,
}

impl Column {
    pub fn new(item_layout: Layout, capacity: usize) -> Self {
        let data = if capacity > 0 {
            let layout = Layout::from_size_align(
                item_layout.size() * capacity,
                item_layout.align(),
            ).unwrap();
            
            unsafe {
                let ptr = std::alloc::alloc(layout);
                NonNull::new(ptr).expect("Failed to allocate column memory")
            }
        } else {
            NonNull::dangling()
        };
        
        Self {
            data,
            ticks: Vec::with_capacity(capacity),
            item_layout,
            len: 0,
            capacity,
        }
    }
    
    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    pub fn item_layout(&self) -> Layout {
        self.item_layout
    }
    
    /// 获取指定索引的原始指针（不检查边界）
    pub unsafe fn get_ptr_unchecked(&self, index: usize) -> *mut u8 {
        self.data.as_ptr().add(index * self.item_layout.size())
    }
    
    /// 获取指定索引的ticks（不检查边界）
    pub unsafe fn get_ticks_unchecked(&self, index: usize) -> ComponentTicks {
        *self.ticks.get_unchecked(index)
    }
    
    /// 获取指定索引的可变ticks引用（不检查边界）
    pub unsafe fn get_ticks_unchecked_mut(&mut self, index: usize) -> &mut ComponentTicks {
        self.ticks.get_unchecked_mut(index)
    }
    
    /// 获取数据切片
    pub fn get_data_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.data.as_ptr(),
                self.len * self.item_layout.size(),
            )
        }
    }
    
    /// 获取可变数据切片
    pub fn get_data_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.data.as_ptr(),
                self.len * self.item_layout.size(),
            )
        }
    }
    
    /// 获取ticks切片
    pub fn get_ticks_slice(&self) -> &[ComponentTicks] {
        &self.ticks[..self.len]
    }
    
    /// 获取可变ticks切片
    pub fn get_ticks_slice_mut(&mut self) -> &mut [ComponentTicks] {
        &mut self.ticks[..self.len]
    }
    
    /// Reserves capacity for at least `additional` more elements
    pub fn reserve(&mut self, additional: usize) {
        let new_capacity = self.len + additional;
        if new_capacity <= self.capacity {
            return;
        }
        
        // 重新分配内存
        let new_layout = Layout::from_size_align(
            self.item_layout.size() * new_capacity,
            self.item_layout.align(),
        ).unwrap();
        
        unsafe {
            let new_ptr = std::alloc::alloc(new_layout);
            let new_data = NonNull::new(new_ptr).expect("Failed to allocate column memory");
            
            // 复制旧数据
            if self.len > 0 {
                std::ptr::copy_nonoverlapping(
                    self.data.as_ptr(),
                    new_data.as_ptr(),
                    self.len * self.item_layout.size(),
                );
            }
            
            // 释放旧内存
            if self.capacity > 0 {
                let old_layout = Layout::from_size_align(
                    self.item_layout.size() * self.capacity,
                    self.item_layout.align(),
                ).unwrap();
                std::alloc::dealloc(self.data.as_ptr(), old_layout);
            }
            
            self.data = new_data;
            self.capacity = new_capacity;
        }
        
        // 为ticks预留空间
        self.ticks.reserve(additional);
    }
    
    /// 清空列
    pub fn clear(&mut self) {
        self.len = 0;
        self.ticks.clear();
    }
    
    // TODO: 实现剩余的column API (~10个)
}

impl Drop for Column {
    fn drop(&mut self) {
        if self.capacity > 0 {
            let layout = Layout::from_size_align(
                self.item_layout.size() * self.capacity,
                self.item_layout.align(),
            ).unwrap();
            
            unsafe {
                std::alloc::dealloc(self.data.as_ptr(), layout);
            }
        }
    }
}

// Safety: Column可以在线程间安全传递
unsafe impl Send for Column {}
unsafe impl Sync for Column {}