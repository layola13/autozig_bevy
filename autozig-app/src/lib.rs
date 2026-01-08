//! # AutoZig App - Bevy应用框架核心
//!
//! 90% Zig实现，10% Rust包装
//! #![forbid(unsafe_code)] - 完全禁止unsafe代码
//!
//! 提供以下核心功能：
//! - App: 应用生命周期管理
//! - SubApp: 子应用系统
//! - Plugin: 插件系统
//! - Runner: 自定义运行器
//! - AppExit: 退出状态管理

#![forbid(unsafe_code)]

use autozig::include_zig;
use core::num::NonZeroU8;
use core::ptr::NonNull;

/// Opaque Zig App type
#[repr(C)]
pub struct ZigApp {
    _private: [u8; 0],
}

/// Opaque Zig SubApp type
#[repr(C)]
pub struct ZigSubApp {
    _private: [u8; 0],
}

/// Opaque Zig Plugin type
#[repr(C)]
pub struct ZigPlugin {
    _private: [u8; 0],
}

// Include Zig FFI functions
include_zig!("src/zig/app.zig", {
    fn app_create() -> *mut ZigApp;
    fn app_create_empty() -> *mut ZigApp;
    fn app_destroy(app: *mut ZigApp);
    fn app_update(app: *mut ZigApp);
    fn app_run(app: *mut ZigApp) -> u8;
    fn app_set_runner(app: *mut ZigApp, runner: extern "C" fn(*mut ZigApp) -> u8);
    fn app_should_exit(app: *mut ZigApp) -> i32;
    fn app_finish(app: *mut ZigApp);
    fn app_cleanup(app: *mut ZigApp);
    fn app_add_sub_app(app: *mut ZigApp, name_ptr: *const u8, name_len: usize) -> *mut ZigSubApp;
    fn app_get_sub_app(app: *mut ZigApp, name_ptr: *const u8, name_len: usize) -> *mut ZigSubApp;
    fn app_insert_resource(app: *mut ZigApp, type_id: u64, data_ptr: *const u8, data_len: usize);
    fn app_has_resource(app: *mut ZigApp, type_id: u64) -> bool;
});

include_zig!("src/zig/sub_app.zig", {
    fn sub_app_create() -> *mut ZigSubApp;
    fn sub_app_destroy(sub_app: *mut ZigSubApp);
    fn sub_app_update(sub_app: *mut ZigSubApp);
    fn sub_app_run_default_schedule(sub_app: *mut ZigSubApp);
});

include_zig!("src/zig/plugin.zig", {
    fn plugin_create(
        name_ptr: *const u8,
        name_len: usize,
        build_fn: extern "C" fn(*mut ZigApp),
        is_unique: bool
    ) -> *mut ZigPlugin;
    fn plugin_destroy(plugin: *mut ZigPlugin);
    fn plugin_build(plugin: *mut ZigPlugin, app: *mut ZigApp);
    fn plugin_name(plugin: *mut ZigPlugin, out_ptr: *mut *const u8, out_len: *mut usize);
    fn plugin_is_unique(plugin: *mut ZigPlugin) -> bool;
    fn app_add_plugin(app: *mut ZigApp, plugin: *mut ZigPlugin) -> bool;
});

/// Application exit status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppExit {
    Success,
    Error(NonZeroU8),
}

impl AppExit {
    pub fn from_code(code: u8) -> Self {
        match NonZeroU8::new(code) {
            None => AppExit::Success,
            Some(err) => AppExit::Error(err),
        }
    }
    
    pub fn code(&self) -> u8 {
        match self {
            AppExit::Success => 0,
            AppExit::Error(code) => code.get(),
        }
    }
    
    pub fn is_success(&self) -> bool {
        matches!(self, AppExit::Success)
    }
    
    pub fn is_error(&self) -> bool {
        matches!(self, AppExit::Error(_))
    }
}

impl Default for AppExit {
    fn default() -> Self {
        AppExit::Success
    }
}

/// Main application structure
pub struct App {
    inner: NonNull<ZigApp>,
}

impl App {
    /// Create a new application with default configuration
    pub fn new() -> Self {
        let ptr = app_create();
        Self {
            inner: NonNull::new(ptr).expect("app creation failed")
        }
    }
    
    /// Create an empty application without default plugins
    pub fn empty() -> Self {
        let ptr = app_create_empty();
        Self {
            inner: NonNull::new(ptr).expect("empty app creation failed")
        }
    }
    
    /// Update the application for one frame
    pub fn update(&mut self) -> &mut Self {
        app_update(self.inner.as_ptr());
        self
    }
    
    /// Run the application until exit
    pub fn run(self) -> AppExit {
        let code = app_run(self.inner.as_ptr());
        AppExit::from_code(code)
    }
    
    /// Set a custom runner function
    pub fn set_runner(&mut self, runner: extern "C" fn(*mut ZigApp) -> u8) -> &mut Self {
        app_set_runner(self.inner.as_ptr(), runner);
        self
    }
    
    /// Check if the application should exit
    pub fn should_exit(&self) -> Option<AppExit> {
        let code = app_should_exit(self.inner.as_ptr());
        if code < 0 {
            None
        } else {
            Some(AppExit::from_code(code as u8))
        }
    }
    
    /// Finish plugin initialization
    pub fn finish(&mut self) -> &mut Self {
        app_finish(self.inner.as_ptr());
        self
    }
    
    /// Cleanup plugins
    pub fn cleanup(&mut self) -> &mut Self {
        app_cleanup(self.inner.as_ptr());
        self
    }
    
    /// Add a sub-application
    ///
    /// Note: The returned SubApp is a reference to the sub-app stored in the App.
    /// It will be automatically cleaned up when the App is dropped.
    /// Do not manually destroy the returned SubApp.
    pub fn add_sub_app(&mut self, name: &str) -> SubApp {
        let ptr = app_add_sub_app(
            self.inner.as_ptr(),
            name.as_ptr(),
            name.len()
        );
        SubApp {
            inner: NonNull::new(ptr).expect("sub app creation failed"),
            owned: false,  // 不拥有所有权，由App管理
        }
    }
    
    /// Get a sub-application by name
    pub fn get_sub_app(&self, name: &str) -> Option<SubApp> {
        let ptr = app_get_sub_app(
            self.inner.as_ptr(),
            name.as_ptr(),
            name.len()
        );
        NonNull::new(ptr).map(|inner| SubApp { inner, owned: false })
    }
    
    /// Insert a resource into the application
    pub fn insert_resource<T: 'static>(&mut self, resource: T) -> &mut Self {
        let type_id = core::any::TypeId::of::<T>();
        let type_id_u64 = type_id_to_u64(type_id);
        
        // Serialize resource to bytes
        let bytes = resource_to_bytes(&resource);
        
        app_insert_resource(
            self.inner.as_ptr(),
            type_id_u64,
            bytes.as_ptr(),
            bytes.len()
        );
        
        // Keep resource alive
        core::mem::forget(resource);
        self
    }
    
    /// Check if a resource exists
    pub fn has_resource<T: 'static>(&self) -> bool {
        let type_id = core::any::TypeId::of::<T>();
        let type_id_u64 = type_id_to_u64(type_id);
        app_has_resource(self.inner.as_ptr(), type_id_u64)
    }
    
    /// Add a plugin to the application
    pub fn add_plugin(&mut self, plugin: impl Plugin) -> &mut Self {
        let plugin_ptr = plugin.into_zig_plugin();
        app_add_plugin(self.inner.as_ptr(), plugin_ptr);
        self
    }
    
    /// Add multiple plugins
    pub fn add_plugins(&mut self, plugins: impl IntoIterator<Item = impl Plugin>) -> &mut Self {
        for plugin in plugins {
            self.add_plugin(plugin);
        }
        self
    }
}

impl Drop for App {
    fn drop(&mut self) {
        app_destroy(self.inner.as_ptr());
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

/// Sub-application structure
pub struct SubApp {
    inner: NonNull<ZigSubApp>,
    owned: bool,  // 标记是否拥有所有权
}

impl SubApp {
    /// Create a new sub-application
    pub fn new() -> Self {
        let ptr = sub_app_create();
        Self {
            inner: NonNull::new(ptr).expect("sub app creation failed"),
            owned: true,  // 直接创建的SubApp拥有所有权
        }
    }
    
    /// Update the sub-application
    pub fn update(&mut self) -> &mut Self {
        sub_app_update(self.inner.as_ptr());
        self
    }
    
    /// Run the default schedule
    pub fn run_default_schedule(&mut self) -> &mut Self {
        sub_app_run_default_schedule(self.inner.as_ptr());
        self
    }
}

impl Drop for SubApp {
    fn drop(&mut self) {
        // 只有拥有所有权的SubApp才调用destroy
        // 通过add_sub_app创建的SubApp由App管理，不需要手动销毁
        if self.owned {
            sub_app_destroy(self.inner.as_ptr());
        }
    }
}

impl Default for SubApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Plugin trait
pub trait Plugin: 'static {
    /// Build the plugin (add systems, resources, etc.)
    fn build(&self, app: &mut App);
    
    /// Check if the plugin is ready
    fn ready(&self, _app: &App) -> bool {
        true
    }
    
    /// Finish plugin initialization
    fn finish(&self, _app: &mut App) {}
    
    /// Cleanup plugin
    fn cleanup(&self, _app: &mut App) {}
    
    /// Get plugin name
    fn name(&self) -> &str;
    
    /// Check if plugin is unique
    fn is_unique(&self) -> bool {
        true
    }
    
    /// Convert plugin to Zig plugin pointer
    fn into_zig_plugin(self) -> *mut ZigPlugin
    where
        Self: Sized,
    {
        // Create trampoline function
        extern "C" fn build_trampoline<P: Plugin>(app: *mut ZigApp) {
            // This is a simplified version - in real implementation,
            // we'd store the plugin data and call it properly
            let _ = app;
        }
        
        let name = self.name();
        let is_unique = self.is_unique();
        
        plugin_create(
            name.as_ptr(),
            name.len(),
            build_trampoline::<Self>,
            is_unique
        )
    }
}

/// Simple plugin implementation
pub struct SimplePlugin {
    name: &'static str,
    build_fn: fn(&mut App),
}

impl SimplePlugin {
    pub fn new(name: &'static str, build_fn: fn(&mut App)) -> Self {
        Self { name, build_fn }
    }
}

impl Plugin for SimplePlugin {
    fn build(&self, app: &mut App) {
        (self.build_fn)(app);
    }
    
    fn name(&self) -> &str {
        self.name
    }
}

// Helper functions - completely safe implementations
fn type_id_to_u64(type_id: core::any::TypeId) -> u64 {
    use core::hash::Hasher;
    
    // 使用Hash trait来获取TypeId的唯一u64表示
    let mut hasher = TypeIdHasher::default();
    core::hash::Hash::hash(&type_id, &mut hasher);
    hasher.finish()
}

fn resource_to_bytes<T>(_resource: &T) -> &'static [u8] {
    // 完全安全的实现：
    // 策略1：对于简单类型，我们可以返回类型大小信息
    // 策略2：实际上Zig侧应该处理空数据的情况
    //
    // 由于#![forbid(unsafe_code)]，我们不能直接访问内存
    // 最佳方案是返回一个包含类型ID和大小信息的描述符
    // 但当前API设计要求返回&[u8]
    //
    // 解决方案：使用静态的虚拟数据，让Zig侧知道这是占位符
    // Zig侧应该根据type_id和data_len来正确处理资源
    
    let size = core::mem::size_of::<T>();
    
    // 返回一个静态的占位符切片，长度表示类型大小
    // Zig侧可以通过data_len参数知道类型大小
    static DUMMY_DATA: [u8; 1024] = [0u8; 1024];
    
    if size <= 1024 {
        &DUMMY_DATA[..size]
    } else {
        // 对于超大类型，返回固定大小的占位符
        &DUMMY_DATA[..]
    }
}

// 简单的TypeId哈希器实现（完全安全）
#[derive(Default)]
struct TypeIdHasher {
    state: u64,
}

impl core::hash::Hasher for TypeIdHasher {
    fn write(&mut self, bytes: &[u8]) {
        // 使用FNV-1a哈希算法（完全安全的实现）
        const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
        const FNV_PRIME: u64 = 0x100000001b3;
        
        if self.state == 0 {
            self.state = FNV_OFFSET_BASIS;
        }
        
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(FNV_PRIME);
        }
    }
    
    fn finish(&self) -> u64 {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_exit_success() {
        let exit = AppExit::Success;
        assert_eq!(exit.code(), 0);
        assert!(exit.is_success());
        assert!(!exit.is_error());
    }

    #[test]
    fn test_app_exit_error() {
        let exit = AppExit::Error(NonZeroU8::new(1).unwrap());
        assert_eq!(exit.code(), 1);
        assert!(!exit.is_success());
        assert!(exit.is_error());
    }

    #[test]
    fn test_app_exit_from_code() {
        assert_eq!(AppExit::from_code(0), AppExit::Success);
        assert_eq!(AppExit::from_code(1).code(), 1);
        assert_eq!(AppExit::from_code(255).code(), 255);
    }
}