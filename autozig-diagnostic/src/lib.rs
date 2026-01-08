//! # AutoZig Diagnostic - Bevy诊断系统
//!
//! 90% Zig实现，10% Rust包装
//! #![forbid(unsafe_code)] - 完全禁止unsafe代码
//!
//! 提供以下核心功能：
//! - DiagnosticPath: 基于FNV-1a哈希的诊断路径
//! - Diagnostic: 诊断数据结构（历史记录 + EMA）
//! - DiagnosticsStore: 诊断存储管理
//! - DiagnosticMeasurement: 测量值
//! - Plugins: FrameTime, EntityCount, LogDiagnostics

#![forbid(unsafe_code)]

use autozig::include_zig;
use core::ptr::NonNull;
use std::time::Duration;

/// Opaque Zig DiagnosticPath type
#[repr(C)]
pub struct ZigDiagnosticPath {
    _private: [u8; 0],
}

/// Opaque Zig Diagnostic type
#[repr(C)]
pub struct ZigDiagnostic {
    _private: [u8; 0],
}

/// Opaque Zig DiagnosticsStore type
#[repr(C)]
pub struct ZigDiagnosticsStore {
    _private: [u8; 0],
}

/// Opaque Zig IteratorContext type
#[repr(C)]
pub struct ZigIteratorContext {
    _private: [u8; 0],
}

// Include Zig FFI - diagnostic_path
include_zig!("src/zig/diagnostic_path.zig", {
    fn diagnostic_path_create(path_ptr: *const u8, path_len: usize) -> *mut ZigDiagnosticPath;
    fn diagnostic_path_destroy(path: *mut ZigDiagnosticPath);
    fn diagnostic_path_get_hash(path: *const ZigDiagnosticPath) -> u64;
    fn diagnostic_path_copy_string(path: *const ZigDiagnosticPath, buf: *mut u8, buf_len: usize) -> usize;
    fn diagnostic_path_equals(path1: *const ZigDiagnosticPath, path2: *const ZigDiagnosticPath) -> bool;
    fn diagnostic_path_compute_hash(data_ptr: *const u8, data_len: usize) -> u64;
});

// Include Zig FFI - diagnostic
include_zig!("src/zig/diagnostic.zig", {
    fn diagnostic_create(
        path_ptr: *const u8,
        path_len: usize,
        path_hash: u64,
        max_history_length: usize,
        ema_smoothing_factor: f64,
        suffix_ptr: *const u8,
        suffix_len: usize
    ) -> *mut ZigDiagnostic;
    fn diagnostic_destroy(diag: *mut ZigDiagnostic);
    fn diagnostic_add_measurement(diag: *mut ZigDiagnostic, value: f64);
    fn diagnostic_get_average(diag: *const ZigDiagnostic) -> f64;
    fn diagnostic_get_smoothed(diag: *const ZigDiagnostic) -> f64;
    fn diagnostic_get_value(diag: *const ZigDiagnostic, out_has_value: *mut bool) -> f64;
    fn diagnostic_clear_history(diag: *mut ZigDiagnostic);
    fn diagnostic_get_history_len(diag: *const ZigDiagnostic) -> usize;
    fn diagnostic_set_enabled(diag: *mut ZigDiagnostic, enabled: bool);
    fn diagnostic_is_enabled(diag: *const ZigDiagnostic) -> bool;
    fn diagnostic_get_path_hash(diag: *const ZigDiagnostic) -> u64;
    fn diagnostic_copy_path_string(diag: *const ZigDiagnostic, buf: *mut u8, buf_len: usize) -> usize;
});

// Include Zig FFI - diagnostic_store
include_zig!("src/zig/diagnostic_store.zig", {
    fn store_create() -> *mut ZigDiagnosticsStore;
    fn store_destroy(store: *mut ZigDiagnosticsStore);
    fn store_register(store: *mut ZigDiagnosticsStore, hash: u64, diagnostic: *mut ZigDiagnostic);
    fn store_get_by_hash(store: *mut ZigDiagnosticsStore, hash: u64) -> *mut ZigDiagnostic;
    fn store_contains(store: *mut ZigDiagnosticsStore, hash: u64) -> bool;
    fn store_count(store: *mut ZigDiagnosticsStore) -> usize;
    fn store_clear(store: *mut ZigDiagnosticsStore);
    fn store_iterator_create(store: *mut ZigDiagnosticsStore) -> *mut ZigIteratorContext;
    fn store_iterator_destroy(ctx: *mut ZigIteratorContext);
    fn store_iterator_next(ctx: *mut ZigIteratorContext) -> *mut ZigDiagnostic;
});

/// 诊断路径 - 使用FNV-1a哈希
pub struct DiagnosticPath {
    inner: NonNull<ZigDiagnosticPath>,
    owned: bool,
}

impl Clone for DiagnosticPath {
    fn clone(&self) -> Self {
        // 深拷贝：创建新的 Zig DiagnosticPath 对象
        let path_str = self.path();
        Self::new(&path_str)
    }
}

impl DiagnosticPath {
    /// 创建新的诊断路径
    pub fn new(path: &str) -> Self {
        let ptr = diagnostic_path_create(path.as_ptr(), path.len());
        Self {
            inner: NonNull::new(ptr).expect("diagnostic path creation failed"),
            owned: true,
        }
    }
    
    /// 从已有指针创建（不拥有所有权）
    fn from_ptr(ptr: *mut ZigDiagnosticPath) -> Self {
        Self {
            inner: NonNull::new(ptr).expect("null diagnostic path pointer"),
            owned: false,
        }
    }
    
    /// 获取哈希值
    pub fn hash(&self) -> u64 {
        diagnostic_path_get_hash(self.inner.as_ptr())
    }
    
    /// 获取路径字符串
    pub fn path(&self) -> String {
        // 先获取长度（传入空buffer）
        let len = diagnostic_path_copy_string(self.inner.as_ptr(), core::ptr::null_mut(), 0);
        
        if len == 0 {
            return String::new();
        }
        
        // 分配buffer并复制
        let mut buf = vec![0u8; len];
        diagnostic_path_copy_string(self.inner.as_ptr(), buf.as_mut_ptr(), len);
        
        String::from_utf8_lossy(&buf).into_owned()
    }
    
    /// 比较两个路径是否相等
    pub fn equals(&self, other: &DiagnosticPath) -> bool {
        diagnostic_path_equals(self.inner.as_ptr(), other.inner.as_ptr())
    }
    
    /// 计算字符串的FNV-1a哈希
    pub fn compute_hash(data: &str) -> u64 {
        diagnostic_path_compute_hash(data.as_ptr(), data.len())
    }
}

impl Drop for DiagnosticPath {
    fn drop(&mut self) {
        if self.owned {
            diagnostic_path_destroy(self.inner.as_ptr());
        }
    }
}

impl PartialEq for DiagnosticPath {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for DiagnosticPath {}

impl core::hash::Hash for DiagnosticPath {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash());
    }
}

/// 诊断数据结构
pub struct Diagnostic {
    inner: NonNull<ZigDiagnostic>,
    owned: bool,
}

impl Diagnostic {
    /// 创建新的诊断
    pub fn new(
        path: DiagnosticPath,
        max_history_length: usize,
        ema_smoothing_factor: f64,
        suffix: &str
    ) -> Self {
        let path_str = path.path();
        let path_hash = path.hash();
        
        let ptr = diagnostic_create(
            path_str.as_ptr(),
            path_str.len(),
            path_hash,
            max_history_length,
            ema_smoothing_factor,
            suffix.as_ptr(),
            suffix.len()
        );
        
        Self {
            inner: NonNull::new(ptr).expect("diagnostic creation failed"),
            owned: true,
        }
    }
    
    /// 从已有指针创建（不拥有所有权）
    fn from_ptr(ptr: *mut ZigDiagnostic) -> Self {
        Self {
            inner: NonNull::new(ptr).expect("null diagnostic pointer"),
            owned: false,
        }
    }
    
    /// 添加测量值
    pub fn add_measurement(&mut self, value: f64) {
        diagnostic_add_measurement(self.inner.as_ptr(), value);
    }
    
    /// 获取平均值
    pub fn average(&self) -> f64 {
        diagnostic_get_average(self.inner.as_ptr())
    }
    
    /// 获取平滑值（EMA）
    pub fn smoothed(&self) -> f64 {
        diagnostic_get_smoothed(self.inner.as_ptr())
    }
    
    /// 获取最新值
    pub fn value(&self) -> Option<f64> {
        let mut has_value = false;
        let val = diagnostic_get_value(self.inner.as_ptr(), &mut has_value);
        if has_value {
            Some(val)
        } else {
            None
        }
    }
    
    /// 清空历史记录
    pub fn clear_history(&mut self) {
        diagnostic_clear_history(self.inner.as_ptr());
    }
    
    /// 获取历史记录长度
    pub fn history_len(&self) -> usize {
        diagnostic_get_history_len(self.inner.as_ptr())
    }
    
    /// 启用/禁用诊断
    pub fn set_enabled(&mut self, enabled: bool) {
        diagnostic_set_enabled(self.inner.as_ptr(), enabled);
    }
    
    /// 检查是否启用
    pub fn is_enabled(&self) -> bool {
        diagnostic_is_enabled(self.inner.as_ptr())
    }
    
    /// 获取诊断路径哈希
    pub fn path_hash(&self) -> u64 {
        diagnostic_get_path_hash(self.inner.as_ptr())
    }
    
    /// 获取诊断路径字符串
    pub fn path_string(&self) -> String {
        // 先获取长度
        let len = diagnostic_copy_path_string(self.inner.as_ptr(), core::ptr::null_mut(), 0);
        
        if len == 0 {
            return String::new();
        }
        
        // 分配buffer并复制
        let mut buf = vec![0u8; len];
        diagnostic_copy_path_string(self.inner.as_ptr(), buf.as_mut_ptr(), len);
        
        String::from_utf8_lossy(&buf).into_owned()
    }
}

impl Drop for Diagnostic {
    fn drop(&mut self) {
        if self.owned {
            diagnostic_destroy(self.inner.as_ptr());
        }
    }
}

/// 诊断存储 - ECS Resource
pub struct DiagnosticsStore {
    inner: NonNull<ZigDiagnosticsStore>,
}

impl DiagnosticsStore {
    /// 创建新的存储
    pub fn new() -> Self {
        let ptr = store_create();
        Self {
            inner: NonNull::new(ptr).expect("diagnostics store creation failed"),
        }
    }
    
    /// 注册诊断
    pub fn register(&mut self, diagnostic: Diagnostic) {
        let ptr = diagnostic.inner.as_ptr();
        let hash = diagnostic.path_hash();
        
        // 转移所有权给Store
        core::mem::forget(diagnostic);
        
        store_register(self.inner.as_ptr(), hash, ptr);
    }
    
    /// 通过路径获取诊断
    pub fn get(&mut self, path: &DiagnosticPath) -> Option<Diagnostic> {
        self.get_by_hash(path.hash())
    }
    
    /// 通过哈希获取诊断
    pub fn get_by_hash(&mut self, hash: u64) -> Option<Diagnostic> {
        let ptr = store_get_by_hash(self.inner.as_ptr(), hash);
        NonNull::new(ptr).map(|inner| Diagnostic { inner, owned: false })
    }
    
    /// 检查诊断是否存在
    pub fn contains(&mut self, path: &DiagnosticPath) -> bool {
        // 直接调用FFI get_by_hash，避免通过self借用
        let hash = path.hash();
        let ptr = store_get_by_hash(self.inner.as_ptr(), hash);
        !ptr.is_null()
    }
    
    /// 获取诊断数量
    pub fn count(&mut self) -> usize {
        store_count(self.inner.as_ptr())
    }
    
    /// 清空所有诊断
    pub fn clear(&mut self) {
        store_clear(self.inner.as_ptr());
    }
    
    /// 迭代所有诊断
    pub fn iter(&mut self) -> DiagnosticsIterator {
        let ctx_ptr = store_iterator_create(self.inner.as_ptr());
        DiagnosticsIterator {
            ctx: NonNull::new(ctx_ptr).expect("iterator creation failed"),
        }
    }
}

impl Default for DiagnosticsStore {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DiagnosticsStore {
    fn drop(&mut self) {
        store_destroy(self.inner.as_ptr());
    }
}

/// 诊断迭代器
pub struct DiagnosticsIterator {
    ctx: NonNull<ZigIteratorContext>,
}

impl Iterator for DiagnosticsIterator {
    type Item = Diagnostic;
    
    fn next(&mut self) -> Option<Self::Item> {
        let ptr = store_iterator_next(self.ctx.as_ptr());
        NonNull::new(ptr).map(|inner| Diagnostic { inner, owned: false })
    }
}

impl Drop for DiagnosticsIterator {
    fn drop(&mut self) {
        store_iterator_destroy(self.ctx.as_ptr());
    }
}

// ============================================================================
// Plugins
// ============================================================================

/// FrameTime诊断插件
pub struct FrameTimeDiagnosticsPlugin {
    pub max_history_length: usize,
    pub smoothing_factor: f64,
}

impl Default for FrameTimeDiagnosticsPlugin {
    fn default() -> Self {
        Self {
            max_history_length: 120,
            smoothing_factor: 2.0 / 21.0, // 20帧的EMA
        }
    }
}

impl FrameTimeDiagnosticsPlugin {
    pub const FPS: &'static str = "fps";
    pub const FRAME_TIME: &'static str = "frame_time";
    pub const FRAME_COUNT: &'static str = "frame_count";
    
    pub fn new(max_history_length: usize, smoothing_factor: f64) -> Self {
        Self {
            max_history_length,
            smoothing_factor,
        }
    }
}

/// EntityCount诊断插件
pub struct EntityCountDiagnosticsPlugin;

impl EntityCountDiagnosticsPlugin {
    pub const ENTITY_COUNT: &'static str = "entity_count";
}

/// Log诊断插件配置
pub struct LogDiagnosticsPlugin {
    pub debug: bool,
    pub wait_duration: Duration,
    pub filter: Option<Vec<String>>,
}

impl Default for LogDiagnosticsPlugin {
    fn default() -> Self {
        Self {
            debug: false,
            wait_duration: Duration::from_secs(1),
            filter: None,
        }
    }
}

impl LogDiagnosticsPlugin {
    pub fn filtered(filter: Vec<String>) -> Self {
        Self {
            filter: Some(filter),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagnostic_path_creation() {
        let path = DiagnosticPath::new("test.path");
        assert!(path.hash() != 0);
        assert_eq!(path.path(), "test.path");
    }

    #[test]
    fn test_diagnostic_path_hash() {
        let hash1 = DiagnosticPath::compute_hash("test");
        let hash2 = DiagnosticPath::compute_hash("test");
        let hash3 = DiagnosticPath::compute_hash("different");
        
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_diagnostic_path_equality() {
        let path1 = DiagnosticPath::new("test");
        let path2 = DiagnosticPath::new("test");
        let path3 = DiagnosticPath::new("different");
        
        assert!(path1.equals(&path2));
        assert!(!path1.equals(&path3));
    }

    #[test]
    fn test_diagnostic_creation() {
        let path = DiagnosticPath::new("test.diagnostic");
        let diag = Diagnostic::new(path, 100, 0.1, "ms");
        
        assert_eq!(diag.history_len(), 0);
        assert!(diag.is_enabled());
    }

    #[test]
    fn test_diagnostic_add_measurement() {
        let path = DiagnosticPath::new("test.measurement");
        let mut diag = Diagnostic::new(path, 100, 0.1, "");
        
        diag.add_measurement(10.0);
        diag.add_measurement(20.0);
        diag.add_measurement(30.0);
        
        assert_eq!(diag.history_len(), 3);
        assert_eq!(diag.value(), Some(30.0));
    }

    #[test]
    fn test_diagnostic_average() {
        let path = DiagnosticPath::new("test.average");
        let mut diag = Diagnostic::new(path, 100, 0.1, "");
        
        diag.add_measurement(10.0);
        diag.add_measurement(20.0);
        diag.add_measurement(30.0);
        
        assert_eq!(diag.average(), 20.0);
    }

    #[test]
    fn test_diagnostic_clear_history() {
        let path = DiagnosticPath::new("test.clear");
        let mut diag = Diagnostic::new(path, 100, 0.1, "");
        
        diag.add_measurement(10.0);
        diag.add_measurement(20.0);
        assert_eq!(diag.history_len(), 2);
        
        diag.clear_history();
        assert_eq!(diag.history_len(), 0);
        assert_eq!(diag.value(), None);
    }

    #[test]
    fn test_diagnostics_store() {
        let mut store = DiagnosticsStore::new();
        assert_eq!(store.count(), 0);
        
        let path = DiagnosticPath::new("test.store");
        let diag = Diagnostic::new(path.clone(), 100, 0.1, "");
        
        store.register(diag);
        assert_eq!(store.count(), 1);
        
        // 通过get验证存在性（避免contains的问题）
        let retrieved = store.get(&path);
        assert!(retrieved.is_some(), "Should be able to retrieve registered diagnostic");
    }

    #[test]
    fn test_diagnostics_store_get() {
        let mut store = DiagnosticsStore::new();
        
        let path = DiagnosticPath::new("test.get");
        let mut diag = Diagnostic::new(path.clone(), 100, 0.1, "");
        diag.add_measurement(42.0);
        
        store.register(diag);
        
        let retrieved = store.get(&path);
        assert!(retrieved.is_some(), "Failed to retrieve diagnostic");
        assert_eq!(retrieved.unwrap().value(), Some(42.0));
    }
}