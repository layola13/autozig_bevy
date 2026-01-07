//! HashMap和HashSet的Rust包装层 - 90% Zig实现

use autozig::include_zig;
use std::marker::PhantomData;

// 引入Zig实现的HashMap和HashSet
include_zig!("src/zig/hashmap.zig", {
    // HashMap FFI
    fn hashmap_create() -> *mut ZigHashMap;
    fn hashmap_destroy(map: *mut ZigHashMap);
    fn hashmap_insert(map: *mut ZigHashMap, key: u64, value: u64) -> bool;
    fn hashmap_get(map: *mut ZigHashMap, key: u64, out_value: *mut u64) -> bool;
    fn hashmap_remove(map: *mut ZigHashMap, key: u64) -> bool;
    fn hashmap_contains(map: *mut ZigHashMap, key: u64) -> bool;
    fn hashmap_len(map: *mut ZigHashMap) -> usize;
    fn hashmap_clear(map: *mut ZigHashMap);
    fn hashmap_capacity(map: *mut ZigHashMap) -> usize;
    
    // HashSet FFI
    fn hashset_create() -> *mut ZigHashSet;
    fn hashset_destroy(set: *mut ZigHashSet);
    fn hashset_insert(set: *mut ZigHashSet, key: u64) -> bool;
    fn hashset_remove(set: *mut ZigHashSet, key: u64) -> bool;
    fn hashset_contains(set: *mut ZigHashSet, key: u64) -> bool;
    fn hashset_len(set: *mut ZigHashSet) -> usize;
    fn hashset_clear(set: *mut ZigHashSet);
    fn hashset_capacity(set: *mut ZigHashSet) -> usize;
});

// Opaque types for Zig structures
#[repr(C)]
struct ZigHashMap {
    _private: [u8; 0],
}

#[repr(C)]
struct ZigHashSet {
    _private: [u8; 0],
}

/// HashMap - 基于Zig实现的高性能哈希表
/// 
/// 专为u64键值对优化，适用于实体ID、句柄等场景
pub struct HashMap {
    ptr: *mut ZigHashMap,
    _marker: PhantomData<ZigHashMap>,
}

impl HashMap {
    /// 创建新的HashMap
    pub fn new() -> Self {
        Self {
            ptr: hashmap_create(),
            _marker: PhantomData,
        }
    }
    
    /// 插入键值对
    pub fn insert(&mut self, key: u64, value: u64) -> bool {
        hashmap_insert(self.ptr, key, value)
    }
    
    /// 获取指定键的值
    pub fn get(&self, key: u64) -> Option<u64> {
        let mut value: u64 = 0;
        if hashmap_get(self.ptr, key, &mut value) {
            Some(value)
        } else {
            None
        }
    }
    
    /// 删除指定键
    pub fn remove(&mut self, key: u64) -> bool {
        hashmap_remove(self.ptr, key)
    }
    
    /// 检查是否包含指定键
    pub fn contains_key(&self, key: u64) -> bool {
        hashmap_contains(self.ptr, key)
    }
    
    /// 获取元素数量
    pub fn len(&self) -> usize {
        hashmap_len(self.ptr)
    }
    
    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// 清空所有元素
    pub fn clear(&mut self) {
        hashmap_clear(self.ptr);
    }
    
    /// 获取容量
    pub fn capacity(&self) -> usize {
        hashmap_capacity(self.ptr)
    }
}

impl Default for HashMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HashMap {
    fn drop(&mut self) {
        hashmap_destroy(self.ptr);
    }
}

// HashMap是线程安全的（通过Zig的原子操作）
// 但在WASM单线程环境下这是无操作的
impl Send for HashMap {}
impl Sync for HashMap {}

/// HashSet - 基于Zig实现的高性能哈希集合
pub struct HashSet {
    ptr: *mut ZigHashSet,
    _marker: PhantomData<ZigHashSet>,
}

impl HashSet {
    /// 创建新的HashSet
    pub fn new() -> Self {
        Self {
            ptr: hashset_create(),
            _marker: PhantomData,
        }
    }
    
    /// 插入元素
    pub fn insert(&mut self, key: u64) -> bool {
        hashset_insert(self.ptr, key)
    }
    
    /// 删除元素
    pub fn remove(&mut self, key: u64) -> bool {
        hashset_remove(self.ptr, key)
    }
    
    /// 检查是否包含元素
    pub fn contains(&self, key: u64) -> bool {
        hashset_contains(self.ptr, key)
    }
    
    /// 获取元素数量
    pub fn len(&self) -> usize {
        hashset_len(self.ptr)
    }
    
    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// 清空所有元素
    pub fn clear(&mut self) {
        hashset_clear(self.ptr);
    }
    
    /// 获取容量
    pub fn capacity(&self) -> usize {
        hashset_capacity(self.ptr)
    }
}

impl Default for HashSet {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for HashSet {
    fn drop(&mut self) {
        hashset_destroy(self.ptr);
    }
}

impl Send for HashSet {}
impl Sync for HashSet {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hashmap_basic() {
        let mut map = HashMap::new();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
        
        assert!(map.insert(1, 100));
        assert!(map.insert(2, 200));
        assert_eq!(map.len(), 2);
        
        assert_eq!(map.get(1), Some(100));
        assert_eq!(map.get(2), Some(200));
        assert_eq!(map.get(999), None);
        
        assert!(map.contains_key(1));
        assert!(!map.contains_key(999));
        
        assert!(map.remove(1));
        assert_eq!(map.len(), 1);
        assert!(!map.contains_key(1));
        
        map.clear();
        assert_eq!(map.len(), 0);
        assert!(map.is_empty());
    }
    
    #[test]
    fn test_hashset_basic() {
        let mut set = HashSet::new();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
        
        assert!(set.insert(1));
        assert!(set.insert(2));
        assert!(set.insert(3));
        assert_eq!(set.len(), 3);
        
        assert!(set.contains(1));
        assert!(set.contains(2));
        assert!(!set.contains(999));
        
        assert!(set.remove(2));
        assert_eq!(set.len(), 2);
        assert!(!set.contains(2));
        
        set.clear();
        assert_eq!(set.len(), 0);
        assert!(set.is_empty());
    }
}