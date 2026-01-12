//! Thin array pointer for type-erased storage

use std::alloc::Layout;
use std::ptr::NonNull;

/// ThinArrayPtr - 薄数组指针，用于类型擦除的数组存储
/// Note: Cannot derive Copy because it implements Drop
#[derive(Clone)]
pub struct ThinArrayPtr<T> {
    ptr: NonNull<u8>,
    capacity: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> ThinArrayPtr<T> {
    /// 创建新的ThinArrayPtr
    pub fn new(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe {
            let raw = std::alloc::alloc(layout);
            NonNull::new(raw).expect("Failed to allocate memory")
        };
        
        Self {
            ptr,
            capacity,
            _marker: std::marker::PhantomData,
        }
    }
    
    /// 获取指定索引的指针（不检查边界）
    pub unsafe fn get_unchecked(&self, index: usize) -> *mut T {
        self.ptr.as_ptr().add(index * std::mem::size_of::<T>()) as *mut T
    }
    
    /// 获取指定索引的引用（不检查边界）
    pub unsafe fn get_unchecked_ref(&self, index: usize) -> &T {
        &*self.get_unchecked(index)
    }
    
    /// 获取指定索引的可变引用（不检查边界）
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        &mut *self.get_unchecked(index)
    }
    
    /// 获取容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// 获取原始指针
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr() as *const T
    }
    
    /// 获取原始可变指针
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr.as_ptr() as *mut T
    }
    
    // TODO: 实现剩余的thin_array_ptr API
}

impl<T> Drop for ThinArrayPtr<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {
                std::alloc::dealloc(self.ptr.as_ptr(), layout);
            }
        }
    }
}

// Safety: ThinArrayPtr可以在线程间安全传递
unsafe impl<T: Send> Send for ThinArrayPtr<T> {}
unsafe impl<T: Sync> Sync for ThinArrayPtr<T> {}