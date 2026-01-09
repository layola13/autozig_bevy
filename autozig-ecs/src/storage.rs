//! Storage - 存储层Rust API
//! 90% Zig实现，10% Rust包装，无unsafe代码

#![forbid(unsafe_code)]

use autozig::include_zig;
use crate::entity::Entity;

/// 组件存储类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StorageType {
    /// Table存储 - 密集数组，Cache友好，适合频繁迭代
    Table,
    /// SparseSet存储 - 稀疏集合，O(1)操作，适合稀疏组件
    SparseSet,
}

// Opaque指针类型（零大小类型用于类型安全）
#[repr(C)]
pub struct ArchetypeOpaque {
    _private: u8,
}

#[repr(C)]
pub struct TableOpaque {
    _private: u8,
}

#[repr(C)]
pub struct SparseSetOpaque {
    _private: u8,
}

// 引入Zig实现的FFI函数
include_zig!("src/zig/archetype.zig", {
    fn archetype_create(id: u32) -> *mut ArchetypeOpaque;
    fn archetype_destroy(arch_ptr: *mut ArchetypeOpaque);
    fn archetype_add_table_component(arch_ptr: *mut ArchetypeOpaque, component_id: u32) -> bool;
    fn archetype_add_sparse_set_component(arch_ptr: *mut ArchetypeOpaque, component_id: u32) -> bool;
    fn archetype_add_entity(arch_ptr: *mut ArchetypeOpaque, entity: u32) -> i64;
    fn archetype_remove_entity(arch_ptr: *mut ArchetypeOpaque, entity: u32) -> bool;
    fn archetype_get_entity_row(arch_ptr: *const ArchetypeOpaque, entity: u32) -> i64;
    fn archetype_contains_entity(arch_ptr: *const ArchetypeOpaque, entity: u32) -> bool;
    fn archetype_entity_count(arch_ptr: *const ArchetypeOpaque) -> usize;
    fn archetype_has_component(arch_ptr: *const ArchetypeOpaque, component_id: u32) -> bool;
    fn archetype_has_table_component(arch_ptr: *const ArchetypeOpaque, component_id: u32) -> bool;
    fn archetype_has_sparse_set_component(arch_ptr: *const ArchetypeOpaque, component_id: u32) -> bool;
    fn archetype_get_id(arch_ptr: *const ArchetypeOpaque) -> u32;
    fn archetype_table_component_count(arch_ptr: *const ArchetypeOpaque) -> usize;
    fn archetype_sparse_set_component_count(arch_ptr: *const ArchetypeOpaque) -> usize;
    fn archetype_get_entities(arch_ptr: *const ArchetypeOpaque, out_buffer: *mut u32, buffer_len: usize) -> usize;
    fn archetype_clear(arch_ptr: *mut ArchetypeOpaque);
});

include_zig!("src/zig/table.zig", {
    fn table_create() -> *mut TableOpaque;
    fn table_destroy(table_ptr: *mut TableOpaque);
    fn table_add_column(table_ptr: *mut TableOpaque, component_id: u32, item_size: usize) -> bool;
    fn table_push_row(table_ptr: *mut TableOpaque, entity: u32) -> usize;
    fn table_swap_remove(table_ptr: *mut TableOpaque, row: usize) -> bool;
    fn table_entity_count(table_ptr: *const TableOpaque) -> usize;
    fn table_get_entity_row(table_ptr: *const TableOpaque, entity: u32) -> i64;
    fn table_clear(table_ptr: *mut TableOpaque);
    fn table_get_column_ptr(table_ptr: *mut TableOpaque, component_id: u32, row: usize) -> *mut u8;
});

include_zig!("src/zig/entity_sparse_set.zig", {
    fn entity_sparse_set_create() -> *mut SparseSetOpaque;
    fn entity_sparse_set_destroy(set_ptr: *mut SparseSetOpaque);
    fn entity_sparse_set_insert(set_ptr: *mut SparseSetOpaque, entity: u32) -> bool;
    fn entity_sparse_set_remove(set_ptr: *mut SparseSetOpaque, entity: u32) -> bool;
    fn entity_sparse_set_contains(set_ptr: *const SparseSetOpaque, entity: u32) -> bool;
    fn entity_sparse_set_len(set_ptr: *const SparseSetOpaque) -> usize;
    fn entity_sparse_set_clear(set_ptr: *mut SparseSetOpaque);
});

/// Archetype - 原型，管理具有相同组件集合的实体
pub struct Archetype {
    inner: *mut ArchetypeOpaque,
}

impl Archetype {
    /// 创建新的Archetype
    pub fn new(id: u32) -> Option<Self> {
        let inner = archetype_create(id);
        if inner.is_null() {
            None
        } else {
            Some(Self { inner })
        }
    }

    /// 获取Archetype ID
    pub fn id(&self) -> u32 {
        archetype_get_id(self.inner)
    }

    /// 添加Table存储的组件
    pub fn add_table_component(&mut self, component_id: u32) -> bool {
        archetype_add_table_component(self.inner, component_id)
    }

    /// 添加SparseSet存储的组件
    pub fn add_sparse_set_component(&mut self, component_id: u32) -> bool {
        archetype_add_sparse_set_component(self.inner, component_id)
    }

    /// 添加entity到此Archetype
    pub fn add_entity(&mut self, entity: Entity) -> Option<usize> {
        let row = archetype_add_entity(self.inner, entity.index());
        if row >= 0 {
            Some(row as usize)
        } else {
            None
        }
    }

    /// 从Archetype移除entity
    pub fn remove_entity(&mut self, entity: Entity) -> bool {
        archetype_remove_entity(self.inner, entity.index())
    }

    /// 获取entity的行号
    pub fn get_entity_row(&self, entity: Entity) -> Option<usize> {
        let row = archetype_get_entity_row(self.inner, entity.index());
        if row >= 0 {
            Some(row as usize)
        } else {
            None
        }
    }

    /// 检查是否包含entity
    pub fn contains_entity(&self, entity: Entity) -> bool {
        archetype_contains_entity(self.inner, entity.index())
    }

    /// 获取entity数量
    pub fn entity_count(&self) -> usize {
        archetype_entity_count(self.inner)
    }

    /// 检查是否包含指定组件
    pub fn has_component(&self, component_id: u32) -> bool {
        archetype_has_component(self.inner, component_id)
    }

    /// 检查是否包含Table组件
    pub fn has_table_component(&self, component_id: u32) -> bool {
        archetype_has_table_component(self.inner, component_id)
    }

    /// 检查是否包含SparseSet组件
    pub fn has_sparse_set_component(&self, component_id: u32) -> bool {
        archetype_has_sparse_set_component(self.inner, component_id)
    }

    /// 获取Table组件数量
    pub fn table_component_count(&self) -> usize {
        archetype_table_component_count(self.inner)
    }

    /// 获取SparseSet组件数量
    pub fn sparse_set_component_count(&self) -> usize {
        archetype_sparse_set_component_count(self.inner)
    }

    /// 获取所有entities
    pub fn entities(&self) -> Vec<Entity> {
        let count = self.entity_count();
        let mut buffer = vec![0u32; count];
        let actual = archetype_get_entities(self.inner, buffer.as_mut_ptr(), count);
        buffer.truncate(actual);
        buffer.into_iter().map(|idx| Entity::from_raw(idx)).collect()
    }

    /// 清空所有entities
    pub fn clear(&mut self) {
        archetype_clear(self.inner);
    }
}

impl Drop for Archetype {
    fn drop(&mut self) {
        archetype_destroy(self.inner);
    }
}

/// Table - 列式存储，适合频繁迭代的组件
pub struct Table {
    inner: *mut TableOpaque,
}

impl Table {
    /// 创建新的Table
    pub fn new() -> Option<Self> {
        let inner = table_create();
        if inner.is_null() {
            None
        } else {
            Some(Self { inner })
        }
    }

    /// 添加列（组件类型）
    pub fn add_column(&mut self, component_id: u32, item_size: usize) -> bool {
        table_add_column(self.inner, component_id, item_size)
    }

    /// 添加行（entity）
    pub fn push_row(&mut self, entity: Entity) -> Option<usize> {
        let row = table_push_row(self.inner, entity.index());
        if row == usize::MAX {
            None
        } else {
            Some(row)
        }
    }

    /// swap-remove删除行
    pub fn swap_remove(&mut self, row: usize) -> bool {
        table_swap_remove(self.inner, row)
    }

    /// 获取entity数量
    pub fn entity_count(&self) -> usize {
        table_entity_count(self.inner)
    }

    /// 获取entity的行号
    pub fn get_entity_row(&self, entity: Entity) -> Option<usize> {
        let row = table_get_entity_row(self.inner, entity.index());
        if row >= 0 {
            Some(row as usize)
        } else {
            None
        }
    }

    /// 清空所有数据
    pub fn clear(&mut self) {
        table_clear(self.inner);
    }

    /// 获取列数据指针（用于高级操作）
    pub fn get_column_ptr(&mut self, component_id: u32, row: usize) -> Option<*mut u8> {
        let ptr = table_get_column_ptr(self.inner, component_id, row);
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        table_destroy(self.inner);
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new().expect("Failed to create Table")
    }
}

/// SparseSet - 稀疏集合，适合稀疏组件
pub struct SparseSet {
    inner: *mut SparseSetOpaque,
}

impl SparseSet {
    /// 创建新的SparseSet
    pub fn new() -> Option<Self> {
        let inner = entity_sparse_set_create();
        if inner.is_null() {
            None
        } else {
            Some(Self { inner })
        }
    }

    /// 插入entity
    pub fn insert(&mut self, entity: Entity) -> bool {
        entity_sparse_set_insert(self.inner, entity.index())
    }

    /// 移除entity
    pub fn remove(&mut self, entity: Entity) -> bool {
        entity_sparse_set_remove(self.inner, entity.index())
    }

    /// 检查是否包含entity
    pub fn contains(&self, entity: Entity) -> bool {
        entity_sparse_set_contains(self.inner, entity.index())
    }

    /// 获取长度
    pub fn len(&self) -> usize {
        entity_sparse_set_len(self.inner)
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 清空集合
    pub fn clear(&mut self) {
        entity_sparse_set_clear(self.inner);
    }
}

impl Drop for SparseSet {
    fn drop(&mut self) {
        entity_sparse_set_destroy(self.inner);
    }
}

impl Default for SparseSet {
    fn default() -> Self {
        Self::new().expect("Failed to create SparseSet")
    }
}