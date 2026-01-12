//! Utility Types and Traits - 工具类型和trait

use crate::entity::Entity;
use crate::component::Component;
use std::marker::PhantomData;

// ============================================================================
// Error Handling - 错误处理
// ============================================================================

/// BevyError - Bevy通用错误类型
#[derive(Debug, Clone)]
pub struct BevyError {
    pub message: String,
    pub source: Option<Box<BevyError>>,
}

impl BevyError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }
    
    pub fn with_source(mut self, source: BevyError) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

impl std::fmt::Display for BevyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(source) = &self.source {
            write!(f, "\nCaused by: {}", source)?;
        }
        Ok(())
    }
}

impl std::error::Error for BevyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

/// DefaultErrorHandler - 默认错误处理器
pub struct DefaultErrorHandler;

impl DefaultErrorHandler {
    pub fn handle_error(error: &dyn std::error::Error) {
        eprintln!("Error: {}", error);
        if let Some(source) = error.source() {
            eprintln!("Caused by: {}", source);
        }
    }
}

// ============================================================================
// Messaging System - 消息系统
// ============================================================================

/// Message - 消息trait
pub trait Message: Send + Sync + 'static {}

/// MessageBus - 消息总线
pub struct MessageBus<M: Message> {
    messages: Vec<M>,
    _phantom: PhantomData<M>,
}

impl<M: Message> MessageBus<M> {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            _phantom: PhantomData,
        }
    }
    
    pub fn send(&mut self, message: M) {
        self.messages.push(message);
    }
    
    pub fn receive(&mut self) -> Option<M> {
        if self.messages.is_empty() {
            None
        } else {
            Some(self.messages.remove(0))
        }
    }
    
    pub fn clear(&mut self) {
        self.messages.clear();
    }
    
    pub fn len(&self) -> usize {
        self.messages.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }
}

impl<M: Message> Default for MessageBus<M> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Marker Types - 标记类型
// ============================================================================

/// Marker<T> - 通用标记类型
pub struct Marker<T> {
    _phantom: PhantomData<T>,
}

impl<T> Marker<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for Marker<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for Marker<T> {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl<T> Copy for Marker<T> {}

impl<T: Send + Sync + 'static> Component for Marker<T> {}

// ============================================================================
// Adapter and Combinator Traits - 适配器和组合器trait
// ============================================================================

/// Adapt - 适配器trait
pub trait Adapt<Input, Output> {
    fn adapt(&mut self, input: Input) -> Output;
}

/// Combine - 组合器trait
pub trait Combine<A, B> {
    type Output;
    fn combine(&self, a: A, b: B) -> Self::Output;
}

// 为函数实现Adapt
impl<F, I, O> Adapt<I, O> for F
where
    F: FnMut(I) -> O,
{
    fn adapt(&mut self, input: I) -> O {
        self(input)
    }
}

// ============================================================================
// Entity Mapping and Cloning - 实体映射和克隆
// ============================================================================

/// MapEntities - 实体映射trait
pub trait MapEntities {
    fn map_entities(&mut self, entity_map: &EntityMap);
}

/// EntityMap - 实体映射表
pub struct EntityMap {
    map: std::collections::HashMap<Entity, Entity>,
}

impl EntityMap {
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, from: Entity, to: Entity) {
        self.map.insert(from, to);
    }
    
    pub fn get(&self, entity: Entity) -> Option<Entity> {
        self.map.get(&entity).copied()
    }
    
    pub fn map(&self, entity: Entity) -> Entity {
        self.get(entity).unwrap_or(entity)
    }
    
    pub fn len(&self) -> usize {
        self.map.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl Default for EntityMap {
    fn default() -> Self {
        Self::new()
    }
}

/// CloneByFilter - 通过过滤器克隆trait
pub trait CloneByFilter {
    fn clone_filtered<F>(&self, filter: F) -> Self
    where
        F: Fn(&Entity) -> bool;
}

// ============================================================================
// Traversal - 遍历
// ============================================================================

/// Traversal - 遍历trait
pub trait Traversal {
    type Item;
    
    fn traverse<F>(&self, visitor: F)
    where
        F: FnMut(&Self::Item);
}

// ============================================================================
// Seal Pattern - 密封trait模式
// ============================================================================

/// Seal - 密封trait（防止外部实现）
pub trait Seal: private::Sealed {}

mod private {
    pub trait Sealed {}
}

// ============================================================================
// Custom Parameters - 自定义参数
// ============================================================================

/// CustomParam - 自定义系统参数
pub struct CustomParam<T> {
    value: T,
}

impl<T> CustomParam<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
    
    pub fn get(&self) -> &T {
        &self.value
    }
    
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    
    pub fn into_inner(self) -> T {
        self.value
    }
}

/// ConstGenericParam<const N: usize> - 常量泛型参数
pub struct ConstGenericParam<const N: usize> {
    _phantom: PhantomData<[(); N]>,
}

impl<const N: usize> ConstGenericParam<N> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
    
    pub const fn size() -> usize {
        N
    }
}

impl<const N: usize> Default for ConstGenericParam<N> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Type Utilities - 类型工具
// ============================================================================

/// TypeIdMap - TypeId映射表
pub struct TypeIdMap<V> {
    map: std::collections::HashMap<std::any::TypeId, V>,
}

impl<V> TypeIdMap<V> {
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
        }
    }
    
    pub fn insert<T: 'static>(&mut self, value: V) -> Option<V> {
        self.map.insert(std::any::TypeId::of::<T>(), value)
    }
    
    pub fn get<T: 'static>(&self) -> Option<&V> {
        self.map.get(&std::any::TypeId::of::<T>())
    }
    
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut V> {
        self.map.get_mut(&std::any::TypeId::of::<T>())
    }
    
    pub fn remove<T: 'static>(&mut self) -> Option<V> {
        self.map.remove(&std::any::TypeId::of::<T>())
    }
    
    pub fn contains<T: 'static>(&self) -> bool {
        self.map.contains_key(&std::any::TypeId::of::<T>())
    }
    
    pub fn len(&self) -> usize {
        self.map.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<V> Default for TypeIdMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Debug and Diagnostics - 调试和诊断
// ============================================================================

/// DebugName - 调试名称
#[derive(Clone, Debug)]
pub struct DebugName(pub String);

impl DebugName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for DebugName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for DebugName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Component for DebugName {}

/// Diagnostics - 诊断信息收集器
pub struct Diagnostics {
    entries: Vec<DiagnosticEntry>,
}

impl Diagnostics {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    
    pub fn add_entry(&mut self, name: String, value: f64) {
        self.entries.push(DiagnosticEntry { name, value });
    }
    
    pub fn get_entry(&self, name: &str) -> Option<f64> {
        self.entries.iter()
            .find(|e| e.name == name)
            .map(|e| e.value)
    }
    
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl Default for Diagnostics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct DiagnosticEntry {
    name: String,
    value: f64,
}