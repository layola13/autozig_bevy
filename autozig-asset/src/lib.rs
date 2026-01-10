//! # AutoZig Asset - Bevy Asset System implemented in Zig
//!
//! 90% Zig 实现，10% Rust 包装
//!
//! 提供资产加载、存储和管理的核心功能。

use autozig::include_zig;
use std::marker::PhantomData;
use std::path::Path;

// Include allocator helper for cross-platform memory management
include_zig!("src/zig/allocator.zig", {
    fn _allocator_init();
});


// ============================================================================
// 核心类型定义
// ============================================================================

/// Asset ID - 资产唯一标识符
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetId {
    pub uuid: u128,
    pub type_id: u64,
}

/// Handle ID - 带代数的句柄
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HandleId {
    pub id: AssetId,
    pub generation: u32,
}

/// 加载状态
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadState {
    NotLoaded = 0,
    Loading = 1,
    Loaded = 2,
    Failed = 3,
}

/// Asset Path - 资产路径
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AssetPath {
    path_ptr: *const u8,
    path_len: usize,
    label_ptr: *const u8,
    label_len: usize,
    has_label: bool,
}

/// Asset Event Type
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetEventType {
    Created = 0,
    Modified = 1,
    Removed = 2,
    LoadingStarted = 3,
    LoadingFinished = 4,
    LoadingFailed = 5,
}

/// Asset Event
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AssetEvent {
    pub handle_id: HandleId,
    pub event_type: AssetEventType,
    pub timestamp: i64,
}

// ============================================================================
// Zig FFI 绑定
// ============================================================================

include_zig!("src/zig/asset_all.zig", {
    // Handle functions
    fn asset_id_init(uuid: u128, type_id: u64) -> AssetId;
    fn asset_id_eql(a: AssetId, b: AssetId) -> bool;
    fn asset_id_hash(id: AssetId) -> u64;
    fn handle_id_init(id: AssetId, generation: u32) -> HandleId;
    fn handle_id_eql(a: HandleId, b: HandleId) -> bool;
    fn handle_id_hash(handle: HandleId) -> u64;
    fn generate_uuid() -> u128;
    fn load_state_is_loaded(state: LoadState) -> bool;
    fn load_state_is_loading(state: LoadState) -> bool;
    fn load_state_is_failed(state: LoadState) -> bool;
    
    // Path functions
    fn asset_path_init(path_ptr: *const u8, path_len: usize) -> AssetPath;
    fn asset_path_init_with_label(
        path_ptr: *const u8,
        path_len: usize,
        label_ptr: *const u8,
        label_len: usize,
    ) -> AssetPath;
    fn asset_path_has_label(path: AssetPath) -> bool;
    fn asset_path_get_label_ptr(path: AssetPath) -> *const u8;
    fn asset_path_get_label_len(path: AssetPath) -> usize;
    fn asset_path_eql(a: AssetPath, b: AssetPath) -> bool;
    
    // Storage functions
    fn asset_storage_create(type_id: u64) -> *mut std::ffi::c_void;
    fn asset_storage_destroy(storage: *mut std::ffi::c_void);
    fn asset_storage_add(storage: *mut std::ffi::c_void, data: *mut std::ffi::c_void, uuid: u128) -> HandleId;
    fn asset_storage_get(storage: *const std::ffi::c_void, handle_id: HandleId) -> *mut std::ffi::c_void;
    fn asset_storage_contains(storage: *const std::ffi::c_void, handle_id: HandleId) -> bool;
    fn asset_storage_remove(storage: *mut std::ffi::c_void, handle_id: HandleId) -> *mut std::ffi::c_void;
    fn asset_storage_count(storage: *const std::ffi::c_void) -> usize;
    fn asset_storage_clear(storage: *mut std::ffi::c_void);
    fn asset_storage_get_load_state(storage: *const std::ffi::c_void, handle_id: HandleId) -> LoadState;
    fn asset_storage_set_load_state(storage: *mut std::ffi::c_void, handle_id: HandleId, state: LoadState);
    
    // AssetServer functions
    fn asset_server_create(root_ptr: *const u8, root_len: usize) -> *mut std::ffi::c_void;
    fn asset_server_destroy(server: *mut std::ffi::c_void);
    fn asset_server_load(server: *mut std::ffi::c_void, path_ptr: *const u8, path_len: usize, type_id: u64) -> HandleId;
    fn asset_server_get(server: *const std::ffi::c_void, handle_id: HandleId) -> *mut std::ffi::c_void;
    fn asset_server_get_load_state(server: *const std::ffi::c_void, handle_id: HandleId) -> LoadState;
    fn asset_server_get_path_ptr(server: *const std::ffi::c_void, handle_id: HandleId) -> *const u8;
    fn asset_server_get_path_len(server: *const std::ffi::c_void, handle_id: HandleId) -> usize;
    fn asset_server_unload(server: *mut std::ffi::c_void, handle_id: HandleId);
    fn asset_server_process_events(server: *mut std::ffi::c_void);
    
    // Event functions
    fn asset_event_created(handle_id: HandleId) -> AssetEvent;
    fn asset_event_modified(handle_id: HandleId) -> AssetEvent;
    fn asset_event_removed(handle_id: HandleId) -> AssetEvent;
    fn event_queue_create() -> *mut std::ffi::c_void;
    fn event_queue_destroy(queue: *mut std::ffi::c_void);
    fn event_queue_push(queue: *mut std::ffi::c_void, event: AssetEvent) -> bool;
    fn event_queue_len(queue: *mut std::ffi::c_void) -> usize;
    fn event_queue_clear(queue: *mut std::ffi::c_void);
});

// ============================================================================
// Rust 高级接口
// ============================================================================

impl AssetId {
    pub fn new(uuid: u128, type_id: u64) -> Self {
        asset_id_init(uuid, type_id)
    }

    pub fn eq(&self, other: &Self) -> bool {
        asset_id_eql(*self, *other)
    }

    pub fn hash_value(&self) -> u64 {
        asset_id_hash(*self)
    }
}

impl HandleId {
    pub fn new(id: AssetId, generation: u32) -> Self {
        handle_id_init(id, generation)
    }

    pub fn eq(&self, other: &Self) -> bool {
        handle_id_eql(*self, *other)
    }

    pub fn hash_value(&self) -> u64 {
        handle_id_hash(*self)
    }
}

impl LoadState {
    pub fn is_loaded(&self) -> bool {
        load_state_is_loaded(*self)
    }

    pub fn is_loading(&self) -> bool {
        load_state_is_loading(*self)
    }

    pub fn is_failed(&self) -> bool {
        load_state_is_failed(*self)
    }
}

impl AssetPath {
    pub fn new(path: &str) -> Self {
        asset_path_init(path.as_ptr(), path.len())
    }

    pub fn with_label(path: &str, label: &str) -> Self {
        asset_path_init_with_label(path.as_ptr(), path.len(), label.as_ptr(), label.len())
    }

    pub fn path(&self) -> &str {
        let slice = unsafe { std::slice::from_raw_parts(self.path_ptr, self.path_len) };
        std::str::from_utf8(slice).unwrap_or("")
    }

    pub fn label(&self) -> Option<&str> {
        if asset_path_has_label(*self) {
            let slice = unsafe {
                std::slice::from_raw_parts(
                    asset_path_get_label_ptr(*self),
                    asset_path_get_label_len(*self),
                )
            };
            Some(std::str::from_utf8(slice).unwrap_or(""))
        } else {
            None
        }
    }

    pub fn eq(&self, other: &Self) -> bool {
        asset_path_eql(*self, *other)
    }
}

/// 生成 UUID
pub fn new_uuid() -> u128 {
    generate_uuid()
}

/// Handle<T> - 类型安全的资产句柄
#[derive(Debug, Clone, Copy)]
pub struct Handle<T> {
    pub id: HandleId,
    _marker: PhantomData<T>,
}

impl<T> Handle<T> {
    pub fn new(id: HandleId) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }

    pub fn id(&self) -> HandleId {
        self.id
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<T> Eq for Handle<T> {}

impl<T> std::hash::Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash_value().hash(state);
    }
}

/// Assets<T> - 类型安全的资产存储
pub struct Assets<T> {
    storage: *mut std::ffi::c_void,
    type_id: u64,
    _marker: PhantomData<T>,
}

impl<T: 'static> Assets<T> {
    pub fn new() -> Self {
        let type_id = Self::type_id();
        let storage = asset_storage_create(type_id);
        assert!(!storage.is_null(), "Failed to create asset storage");
        Self {
            storage,
            type_id,
            _marker: PhantomData,
        }
    }

    fn type_id() -> u64 {
        // 使用 TypeId 的哈希值作为唯一标识
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        std::any::TypeId::of::<T>().hash(&mut hasher);
        hasher.finish()
    }

    pub fn add(&mut self, asset: T) -> Handle<T> {
        let boxed = Box::new(asset);
        let ptr = Box::into_raw(boxed) as *mut std::ffi::c_void;
        let uuid = generate_uuid();
        let handle_id = asset_storage_add(self.storage, ptr, uuid);
        Handle::new(handle_id)
    }

    pub fn get(&self, handle: &Handle<T>) -> Option<&T> {
        let ptr = asset_storage_get(self.storage, handle.id);
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { (ptr as *const T).as_ref()? })
        }
    }

    pub fn get_mut(&mut self, handle: &Handle<T>) -> Option<&mut T> {
        let ptr = asset_storage_get(self.storage, handle.id);
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { (ptr as *mut T).as_mut()? })
        }
    }

    pub fn contains(&self, handle: &Handle<T>) -> bool {
        asset_storage_contains(self.storage, handle.id)
    }

    pub fn remove(&mut self, handle: &Handle<T>) -> Option<T> {
        let ptr = asset_storage_remove(self.storage, handle.id);
        if ptr.is_null() {
            None
        } else {
            let boxed = (ptr as *mut T).cast::<T>();
            Some(*unsafe { Box::from_raw(boxed) })
        }
    }

    pub fn len(&self) -> usize {
        asset_storage_count(self.storage)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        asset_storage_clear(self.storage);
    }

    pub fn get_load_state(&self, handle: &Handle<T>) -> LoadState {
        asset_storage_get_load_state(self.storage, handle.id)
    }
}

impl<T> Drop for Assets<T> {
    fn drop(&mut self) {
        if !self.storage.is_null() {
            asset_storage_destroy(self.storage);
        }
    }
}

impl<T: 'static> Default for Assets<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// AssetServer - 资产服务器
pub struct AssetServer {
    server: *mut std::ffi::c_void,
}

impl AssetServer {
    pub fn new<P: AsRef<Path>>(asset_root: P) -> Self {
        let root_str = asset_root.as_ref().to_string_lossy();
        let server = asset_server_create(root_str.as_ptr(), root_str.len());
        assert!(!server.is_null(), "Failed to create asset server");
        Self { server }
    }

    pub fn load<T: 'static>(&self, path: &str) -> Handle<T> {
        // 使用 TypeId 的哈希值作为唯一标识
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        std::any::TypeId::of::<T>().hash(&mut hasher);
        let type_id: u64 = hasher.finish();
        let handle_id = asset_server_load(self.server, path.as_ptr(), path.len(), type_id);
        Handle::new(handle_id)
    }

    pub fn get<T>(&self, handle: &Handle<T>) -> Option<&T> {
        let ptr = asset_server_get(self.server, handle.id);
        if ptr.is_null() {
            None
        } else {
            unsafe { (ptr as *const T).as_ref() }
        }
    }

    pub fn get_load_state<T>(&self, handle: &Handle<T>) -> LoadState {
        asset_server_get_load_state(self.server, handle.id)
    }

    pub fn get_path<T>(&self, handle: &Handle<T>) -> Option<String> {
        let ptr = asset_server_get_path_ptr(self.server, handle.id);
        if ptr.is_null() {
            return None;
        }
        let len = asset_server_get_path_len(self.server, handle.id);
        if len == 0 {
            return None;
        }
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        String::from_utf8(slice.to_vec()).ok()
    }

    pub fn unload<T>(&mut self, handle: &Handle<T>) {
        asset_server_unload(self.server, handle.id);
    }

    pub fn process_events(&mut self) {
        asset_server_process_events(self.server);
    }
}

impl Drop for AssetServer {
    fn drop(&mut self) {
        if !self.server.is_null() {
            asset_server_destroy(self.server);
        }
    }
}

/// EventQueue - 事件队列
pub struct EventQueue {
    queue: *mut std::ffi::c_void,
}

impl EventQueue {
    pub fn new() -> Self {
        let queue = event_queue_create();
        assert!(!queue.is_null(), "Failed to create event queue");
        Self { queue }
    }

    pub fn push(&mut self, event: AssetEvent) -> bool {
        event_queue_push(self.queue, event)
    }

    pub fn len(&self) -> usize {
        event_queue_len(self.queue)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn clear(&mut self) {
        event_queue_clear(self.queue);
    }
}

impl Drop for EventQueue {
    fn drop(&mut self) {
        if !self.queue.is_null() {
            event_queue_destroy(self.queue);
        }
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Asset Trait
// ============================================================================

/// Asset trait - 所有资产必须实现此 trait
pub trait Asset: Send + Sync + 'static {
    fn type_uuid() -> u128 {
        0
    }
}

// 为常见类型实现 Asset
impl Asset for String {}
impl Asset for Vec<u8> {}
impl Asset for () {}

/// TextAsset - 文本资产
#[derive(Debug, Clone)]
pub struct TextAsset {
    pub content: String,
}

impl Asset for TextAsset {}

impl TextAsset {
    pub fn new(content: String) -> Self {
        Self { content }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, std::str::Utf8Error> {
        Ok(Self {
            content: std::str::from_utf8(bytes)?.to_string(),
        })
    }
}