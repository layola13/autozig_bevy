use autozig_macro::include_zig;
use std::marker::PhantomData;
use crate::world::World;

/// Resource trait - 标记为资源的trait
pub trait Resource: Send + Sync + 'static {}

/// ResourceId - 资源唯一标识
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceId(u64);

/// FromWorld trait - 从World构造资源的trait
pub trait FromWorld {
    fn from_world(world: &mut World) -> Self;
}

/// 为实现Default的类型自动实现FromWorld
impl<T: Default> FromWorld for T {
    fn from_world(_world: &mut World) -> Self {
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
    fn resource_registry_contains(registry: *const ResourceRegistryOpaque, type_id: u64) -> bool;
    fn resource_registry_remove(registry: *mut ResourceRegistryOpaque, type_id: u64) -> *mut std::ffi::c_void;
});

/// 计算TypeId的Hash作为跨语言ID
fn get_type_hash<T: 'static>() -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    std::any::TypeId::of::<T>().hash(&mut hasher);
    hasher.finish()
}

/// ResourceRegistry - 资源注册表（FFI包装）
pub struct ResourceRegistry {
    inner: *mut ResourceRegistryOpaque,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Self {
            inner: resource_registry_create(),
        }
    }

    pub fn insert<R: Resource>(&mut self, resource: R) {
        let type_id = get_type_hash::<R>();
        let ptr = Box::into_raw(Box::new(resource)) as *mut std::ffi::c_void;
        resource_registry_insert(self.inner, type_id, ptr);
    }
    
    pub fn get<R: 'static>(&self) -> Option<Res<'_, R>> {
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

    pub fn get_mut<R: 'static>(&mut self) -> Option<ResMut<'_, R>> {
        let type_id = get_type_hash::<R>();
        let ptr = resource_registry_get(self.inner, type_id);
        if ptr.is_null() {
            None
        } else {
            Some(ResMut {
                ptr: unsafe { &mut *(ptr as *mut R) },
                _marker: PhantomData,
            })
        }
    }

    pub fn remove<R: 'static>(&mut self) -> Option<R> {
        let type_id = get_type_hash::<R>();
        let ptr = resource_registry_remove(self.inner, type_id);
        if ptr.is_null() {
            None
        } else {
            // Reconstruct Box from raw pointer to take ownership
            unsafe {
                let boxed = Box::from_raw(ptr as *mut R);
                Some(*boxed)
            }
        }
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

unsafe impl Send for ResourceRegistry {}
unsafe impl Sync for ResourceRegistry {}

/// Bevy-compatible resource reference
pub struct Res<'w, T> {
    pub(crate) ptr: &'w T,
    pub(crate) _marker: PhantomData<&'w T>,
}

impl<'w, T> std::ops::Deref for Res<'w, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.ptr
    }
}

/// Bevy-compatible mutable resource reference
pub struct ResMut<'w, T> {
    pub(crate) ptr: &'w mut T,
    pub(crate) _marker: PhantomData<&'w mut ()>,
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

impl<'w, T: std::fmt::Debug> std::fmt::Debug for Res<'w, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ptr.fmt(f)
    }
}

impl<'w, T: std::fmt::Debug> std::fmt::Debug for ResMut<'w, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ptr.fmt(f)
    }
}

/// Marker for NonSend resources (TODO)
pub struct NonSend<T>(T);
pub struct NonSendMut<'w, T>(&'w mut T);
