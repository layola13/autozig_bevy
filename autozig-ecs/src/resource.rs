//! Resource system - Bevy-compatible global resources

use autozig_macro::include_zig;
use std::any::TypeId;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

// ============================================================================
// Resource Trait and Types - Resource trait和相关类型
// ============================================================================

/// Resource trait - 标记类型可以作为全局资源
pub trait Resource: Send + Sync + 'static {}

/// ResourceId - 资源的唯一标识符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ResourceId(pub u64);

impl ResourceId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
    
    pub fn of<R: Resource>() -> Self {
        let mut hasher = DefaultHasher::new();
        TypeId::of::<R>().hash(&mut hasher);
        Self(hasher.finish())
    }
}

/// FromWorld trait - 从World构造资源的trait
pub trait FromWorld {
    fn from_world(world: &mut crate::world::World) -> Self;
}

/// 为实现Default的类型自动实现FromWorld
impl<T: Default> FromWorld for T {
    fn from_world(_world: &mut crate::world::World) -> Self {
        T::default()
    }
}

#[repr(C)]
pub struct ResourceRegistryOpaque {
    _private: u8,
}

include_zig!("src/zig/resource.zig", {
    fn resource_registry_create() -> *mut ResourceRegistryOpaque;
    fn resource_registry_destroy(registry: *mut ResourceRegistryOpaque);
    fn resource_registry_insert(registry: *mut ResourceRegistryOpaque, type_id: u64, ptr: *mut std::ffi::c_void) -> bool;
    fn resource_registry_get(registry: *const ResourceRegistryOpaque, type_id: u64) -> *mut std::ffi::c_void;
    fn resource_registry_remove(registry: *mut ResourceRegistryOpaque, type_id: u64) -> bool;
    fn resource_registry_contains(registry: *const ResourceRegistryOpaque, type_id: u64) -> bool;
});

/// 计算TypeId的Hash作为跨语言ID
fn get_type_hash<T: 'static>() -> u64 {
    let mut hasher = DefaultHasher::new();
    TypeId::of::<T>().hash(&mut hasher);
    hasher.finish()
}

pub struct ResourceRegistry {
    inner: *mut ResourceRegistryOpaque,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        let inner = resource_registry_create();
        Self { inner }
    }
    
    pub fn insert<R: 'static>(&mut self, resource: R) {
        let type_id = get_type_hash::<R>();
        let ptr = Box::into_raw(Box::new(resource)) as *mut std::ffi::c_void;
        resource_registry_insert(self.inner, type_id, ptr);
    }
    
    pub fn get<R: 'static>(&self) -> Option<Res<R>> {
        let type_id = get_type_hash::<R>();
        let ptr = resource_registry_get(self.inner, type_id);
        if ptr.is_null() {
            None
        } else {
            Some(Res {
                ptr: unsafe { &*(ptr as *const R) },
                _marker: PhantomData,
            })
        }
    }
    
    pub fn remove<R: 'static>(&mut self) -> bool {
        let type_id = get_type_hash::<R>();
        resource_registry_remove(self.inner, type_id)
    }
    
    pub fn contains<R: 'static>(&self) -> bool {
        let type_id = get_type_hash::<R>();
        resource_registry_contains(self.inner, type_id)
    }
}

impl Drop for ResourceRegistry {
    fn drop(&mut self) {
        resource_registry_destroy(self.inner);
    }
}

impl Default for ResourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Bevy-compatible immutable resource reference
pub struct Res<'w, T> {
    ptr: &'w T,
    _marker: PhantomData<&'w ()>,
}

impl<'w, T> std::ops::Deref for Res<'w, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.ptr
    }
}

/// Bevy-compatible mutable resource reference
pub struct ResMut<'w, T> {
    ptr: &'w mut T,
    _marker: PhantomData<&'w mut ()>,
}

impl<'w, T> std::ops::Deref for ResMut<'w, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.ptr
    }
}

impl<'w, T> std::ops::DerefMut for ResMut<'w, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ptr
    }
}

// ============================================================================
// NonSend Resources - 非Send资源（线程本地资源）
// ============================================================================

/// NonSend<T> - 非Send资源的不可变引用
/// 用于无法跨线程传递的资源
pub struct NonSend<'w, T: 'static> {
    value: &'w T,
    _marker: PhantomData<&'w ()>,
}

impl<'w, T: 'static> NonSend<'w, T> {
    pub fn new(value: &'w T) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

impl<'w, T> std::ops::Deref for NonSend<'w, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

/// NonSendMut<T> - 非Send资源的可变引用
pub struct NonSendMut<'w, T: 'static> {
    value: &'w mut T,
    _marker: PhantomData<&'w mut ()>,
}

impl<'w, T: 'static> NonSendMut<'w, T> {
    pub fn new(value: &'w mut T) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

impl<'w, T> std::ops::Deref for NonSendMut<'w, T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.value
    }
}

impl<'w, T> std::ops::DerefMut for NonSendMut<'w, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.value
    }
}
