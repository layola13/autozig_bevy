//! Bundle system - Batch component operations (90% Zig + 10% Rust)
//! 
//! Implements Bevy-compatible Bundle trait for adding/removing multiple components at once

use autozig::include_zig;
use crate::component::Component;
use crate::entity::Entity;
use std::marker::PhantomData;
use std::any::TypeId;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Bundle trait - 标记类型可以作为组件集合
/// 
/// 任何实现了Component的类型都自动实现Bundle
/// 元组类型也实现Bundle（支持空元组到12元组）
pub trait Bundle: Send + Sync + 'static {
    /// 获取Bundle中所有组件的ID列表
    fn component_ids() -> Vec<u32>;
    
    /// 获取Bundle中组件的数据指针和大小
    /// 返回 (component_id, data_ptr, data_size) 的列表
    fn get_components(&self) -> Vec<(u32, *const u8, usize)>;
}

/// 计算组件类型的Hash作为ID
fn get_component_id<C: 'static>() -> u32 {
    let mut hasher = DefaultHasher::new();
    TypeId::of::<C>().hash(&mut hasher);
    (hasher.finish() & 0xFFFFFFFF) as u32
}

// 单个组件自动实现Bundle
impl<T: Component> Bundle for T {
    fn component_ids() -> Vec<u32> {
        vec![get_component_id::<T>()]
    }
    
    fn get_components(&self) -> Vec<(u32, *const u8, usize)> {
        let id = get_component_id::<T>();
        let ptr = self as *const T as *const u8;
        let size = std::mem::size_of::<T>();
        vec![(id, ptr, size)]
    }
}

// Zig 底层实现
#[repr(C)]
pub struct BundleInfoOpaque {
    _private: u8,
}

include_zig!("src/zig/bundle.zig", {
    fn bundle_info_create(
        component_ids_ptr: *const u32,
        component_ids_len: usize,
    ) -> *mut BundleInfoOpaque;
    fn bundle_info_destroy(info: *mut BundleInfoOpaque);
    pub fn bundle_spawn(
        world_ptr: *mut crate::world::WorldOpaque,
        component_ids_ptr: *const u32,
        component_ids_len: usize,
        component_data_ptr: *const *const u8,
        component_sizes_ptr: *const usize,
    ) -> Entity;
    pub fn bundle_insert(
        world_ptr: *mut crate::world::WorldOpaque,
        entity: Entity,
        component_ids_ptr: *const u32,
        component_ids_len: usize,
        component_data_ptr: *const *const u8,
        component_sizes_ptr: *const usize,
    ) -> bool;
    pub fn bundle_remove(
        world_ptr: *mut crate::world::WorldOpaque,
        entity: Entity,
        component_ids_ptr: *const u32,
        component_ids_len: usize,
    ) -> bool;
});

pub struct BundleInfo {
    inner: *mut BundleInfoOpaque,
}

impl BundleInfo {
    pub fn new<B: Bundle>() -> Self {
        let ids = B::component_ids();
        let inner = bundle_info_create(ids.as_ptr(), ids.len());
        Self { inner }
    }
}

impl Drop for BundleInfo {
    fn drop(&mut self) {
        bundle_info_destroy(self.inner);
    }
}

// ============================================================================
// 元组实现 Bundle - 支持空元组到12元组
// ============================================================================

// 空元组实现
impl Bundle for () {
    fn component_ids() -> Vec<u32> {
        Vec::new()
    }
    
    fn get_components(&self) -> Vec<(u32, *const u8, usize)> {
        Vec::new()
    }
}

// 宏：为元组实现Bundle
macro_rules! impl_bundle_for_tuple {
    ($($T:ident),*) => {
        #[allow(non_snake_case)]
        impl<$($T: Bundle),*> Bundle for ($($T,)*) {
            fn component_ids() -> Vec<u32> {
                let mut ids = Vec::new();
                $(
                    ids.extend($T::component_ids());
                )*
                ids
            }
            
            fn get_components(&self) -> Vec<(u32, *const u8, usize)> {
                let mut components = Vec::new();
                let ($($T,)*) = self;
                $(
                    components.extend($T.get_components());
                )*
                components
            }
        }
    };
}

// 生成2到12元组的Bundle实现
impl_bundle_for_tuple!(A);
impl_bundle_for_tuple!(A, B);
impl_bundle_for_tuple!(A, B, C);
impl_bundle_for_tuple!(A, B, C, D);
impl_bundle_for_tuple!(A, B, C, D, E);
impl_bundle_for_tuple!(A, B, C, D, E, F);
impl_bundle_for_tuple!(A, B, C, D, E, F, G);
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H);
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_bundle_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Position { x: f32, y: f32 }
    impl Component for Position {}

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Velocity { x: f32, y: f32 }
    impl Component for Velocity {}

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Health(u32);
    impl Component for Health {}

    #[test]
    fn test_single_component_bundle() {
        let pos = Position { x: 1.0, y: 2.0 };
        let ids = <Position as Bundle>::component_ids();
        assert_eq!(ids.len(), 1);
        
        let components = pos.get_components();
        assert_eq!(components.len(), 1);
    }

    #[test]
    fn test_empty_tuple_bundle() {
        let bundle = ();
        let ids = <() as Bundle>::component_ids();
        assert_eq!(ids.len(), 0);
        
        let components = bundle.get_components();
        assert_eq!(components.len(), 0);
    }

    #[test]
    fn test_tuple_bundle_2() {
        let bundle = (
            Position { x: 10.0, y: 20.0 },
            Velocity { x: 1.0, y: 2.0 },
        );
        
        let ids = <(Position, Velocity) as Bundle>::component_ids();
        assert_eq!(ids.len(), 2);
        
        let components = bundle.get_components();
        assert_eq!(components.len(), 2);
    }

    #[test]
    fn test_tuple_bundle_3() {
        let bundle = (
            Position { x: 10.0, y: 20.0 },
            Velocity { x: 1.0, y: 2.0 },
            Health(100),
        );
        
        let ids = <(Position, Velocity, Health) as Bundle>::component_ids();
        assert_eq!(ids.len(), 3);
        
        let components = bundle.get_components();
        assert_eq!(components.len(), 3);
    }

    #[test]
    fn test_nested_bundle() {
        let bundle = (
            (Position { x: 10.0, y: 20.0 }, Velocity { x: 1.0, y: 2.0 }),
            Health(100),
        );
        
        let components = bundle.get_components();
        assert_eq!(components.len(), 3);
    }
}