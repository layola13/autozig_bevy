//! Change Detection - 变更检测系统
//! 
//! 基于Tick的变更检测，实现Changed<T>, Added<T>, Removed<T>查询过滤器

#![forbid(unsafe_code)]

use autozig::include_zig;
use crate::component::Component;
use crate::entity::Entity;
use crate::query::QueryFilter;
use std::marker::PhantomData;

// ============================================================================
// Zig FFI - 导入Zig实现
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZigTick {
    value: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ZigComponentTicks {
    added: ZigTick,
    changed: ZigTick,
}

#[repr(C)]
pub struct ZigChangeDetectionContext {
    current_tick: ZigTick,
    last_change_tick: ZigTick,
    _padding: u64, // 填充以保持ABI兼容
}

#[repr(C)]
pub struct ZigRemovedComponentsOpaque {
    _private: u8,
}

include_zig!("src/zig/change_detection.zig", {
    fn tick_new(value: u32) -> ZigTick;
    fn tick_increment(tick_ptr: *mut ZigTick);
    fn tick_is_newer_than(self_tick: ZigTick, other: ZigTick, wrap_threshold: u32) -> bool;
    fn component_ticks_new(tick: ZigTick) -> ZigComponentTicks;
    fn component_ticks_set_changed(ticks_ptr: *mut ZigComponentTicks, tick: ZigTick);
    fn component_ticks_is_added(ticks: ZigComponentTicks, last_run: ZigTick, this_run: ZigTick) -> bool;
    fn component_ticks_is_changed(ticks: ZigComponentTicks, last_run: ZigTick, this_run: ZigTick) -> bool;
    fn change_detection_context_init() -> ZigChangeDetectionContext;
    fn change_detection_context_increment(ctx_ptr: *mut ZigChangeDetectionContext);
    fn change_detection_context_check_if_added(ctx_ptr: *const ZigChangeDetectionContext, ticks: ZigComponentTicks) -> bool;
    fn change_detection_context_check_if_changed(ctx_ptr: *const ZigChangeDetectionContext, ticks: ZigComponentTicks) -> bool;
});

include_zig!("src/zig/removed_components.zig", {
    fn removed_components_init(component_id: u32) -> *mut ZigRemovedComponentsOpaque;
    fn removed_components_deinit(removed_ptr: *mut ZigRemovedComponentsOpaque);
    fn removed_components_record(removed_ptr: *mut ZigRemovedComponentsOpaque, entity_id: u32) -> bool;
    fn removed_components_clear(removed_ptr: *mut ZigRemovedComponentsOpaque);
    fn removed_components_len(removed_ptr: *const ZigRemovedComponentsOpaque) -> usize;
    fn removed_components_get(removed_ptr: *const ZigRemovedComponentsOpaque, index: usize) -> u32;
});

// ============================================================================
// Rust Wrapper Types - Rust包装类型
// ============================================================================

/// Tick - 全局时钟周期，用于变更检测
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(pub u32);

impl Tick {
    pub fn new(value: u32) -> Self {
        Tick(value)
    }

    pub fn increment(&mut self) {
        let mut zig_tick = tick_new(self.0);
        tick_increment(&mut zig_tick);
        self.0 = zig_tick.value;
    }

    pub fn is_newer_than(&self, other: Tick, wrap_threshold: u32) -> bool {
        let self_tick = tick_new(self.0);
        let other_tick = tick_new(other.0);
        tick_is_newer_than(self_tick, other_tick, wrap_threshold)
    }

    pub(crate) fn to_zig(&self) -> ZigTick {
        tick_new(self.0)
    }
}

impl Default for Tick {
    fn default() -> Self {
        Tick(0)
    }
}

/// ComponentTicks - 组件变更追踪信息
#[derive(Debug, Clone, Copy)]
pub struct ComponentTicks {
    zig_ticks: ZigComponentTicks,
}

impl ComponentTicks {
    pub fn new(tick: Tick) -> Self {
        let zig_tick = tick.to_zig();
        Self {
            zig_ticks: component_ticks_new(zig_tick),
        }
    }

    pub fn set_changed(&mut self, tick: Tick) {
        let zig_tick = tick.to_zig();
        component_ticks_set_changed(&mut self.zig_ticks, zig_tick);
    }

    pub fn is_added(&self, last_run: Tick, this_run: Tick) -> bool {
        component_ticks_is_added(
            self.zig_ticks,
            last_run.to_zig(),
            this_run.to_zig(),
        )
    }

    pub fn is_changed(&self, last_run: Tick, this_run: Tick) -> bool {
        component_ticks_is_changed(
            self.zig_ticks,
            last_run.to_zig(),
            this_run.to_zig(),
        )
    }

    pub(crate) fn inner(&self) -> ZigComponentTicks {
        self.zig_ticks
    }
}

/// ChangeDetectionContext - 变更检测上下文
pub struct ChangeDetectionContext {
    inner: ZigChangeDetectionContext,
}

impl ChangeDetectionContext {
    pub fn new() -> Self {
        Self {
            inner: change_detection_context_init(),
        }
    }

    pub fn increment(&mut self) {
        change_detection_context_increment(&mut self.inner);
    }

    pub fn current_tick(&self) -> Tick {
        Tick(self.inner.current_tick.value)
    }

    pub fn last_change_tick(&self) -> Tick {
        Tick(self.inner.last_change_tick.value)
    }

    pub fn check_if_added(&self, ticks: ComponentTicks) -> bool {
        change_detection_context_check_if_added(&self.inner, ticks.zig_ticks)
    }

    pub fn check_if_changed(&self, ticks: ComponentTicks) -> bool {
        change_detection_context_check_if_changed(&self.inner, ticks.zig_ticks)
    }
}

impl Default for ChangeDetectionContext {
    fn default() -> Self {
        Self::new()
    }
}

/// RemovedComponents<T> - 追踪已移除的组件
pub struct RemovedComponents<T: Component> {
    inner: *mut ZigRemovedComponentsOpaque,
    _marker: PhantomData<T>,
}

impl<T: Component> RemovedComponents<T> {
    pub fn new(component_id: u32) -> Self {
        let inner = removed_components_init(component_id);
        Self {
            inner,
            _marker: PhantomData,
        }
    }

    pub fn record(&mut self, entity: Entity) -> bool {
        removed_components_record(self.inner, entity.index)
    }

    pub fn clear(&mut self) {
        removed_components_clear(self.inner);
    }

    pub fn len(&self) -> usize {
        removed_components_len(self.inner)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> RemovedComponentsIter {
        RemovedComponentsIter {
            inner: self.inner,
            index: 0,
            len: self.len(),
        }
    }
}

impl<T: Component> Drop for RemovedComponents<T> {
    fn drop(&mut self) {
        removed_components_deinit(self.inner);
    }
}

/// RemovedComponentsIter - 已移除组件迭代器
pub struct RemovedComponentsIter {
    inner: *const ZigRemovedComponentsOpaque,
    index: usize,
    len: usize,
}

// 无效Entity标记
const INVALID_ENTITY: Entity = Entity { index: 0xFFFFFFFF, generation: 0xFFFFFFFF };

impl Iterator for RemovedComponentsIter {
    type Item = Entity;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        let entity_id = removed_components_get(self.inner, self.index);
        self.index += 1;
        if entity_id == 0xFFFFFFFF {
            None
        } else {
            Some(Entity::from_raw(entity_id))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len.saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

// ============================================================================
// Query Filters - 查询过滤器
// ============================================================================

/// Changed<T> - QueryFilter，检测组件是否在上次运行后变更
/// 
/// # Example
/// ```
/// use autozig_ecs::prelude::*;
/// 
/// fn system(query: Query<&Position, Changed<Position>>) {
///     for pos in query.iter() {
///         // 只处理变更的Position组件
///     }
/// }
/// ```
pub struct Changed<T: Component> {
    _marker: PhantomData<T>,
}

impl<T: Component> Default for Changed<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: Component> QueryFilter for Changed<T> {}

/// Added<T> - QueryFilter，检测组件是否在上次运行后新增
/// 
/// # Example
/// ```
/// use autozig_ecs::prelude::*;
/// 
/// fn system(query: Query<&Position, Added<Position>>) {
///     for pos in query.iter() {
///         // 只处理新增的Position组件
///     }
/// }
/// ```
pub struct Added<T: Component> {
    _marker: PhantomData<T>,
}

impl<T: Component> Default for Added<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: Component> QueryFilter for Added<T> {}

// ============================================================================
// Tests - 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    
    static INIT: Once = Once::new();
    
    fn ensure_init() {
        INIT.call_once(|| {
            crate::entity::init();
        });
    }


    #[test]
    fn test_tick_creation() {
        let tick = Tick::new(0);
        assert_eq!(tick.0, 0);
    }

    #[test]
    fn test_tick_increment() {
        let mut tick = Tick::new(0);
        tick.increment();
        assert_eq!(tick.0, 1);
    }

    #[test]
    fn test_tick_wrapping() {
        let mut tick = Tick::new(u32::MAX);
        tick.increment();
        assert_eq!(tick.0, 0);
    }

    #[test]
    fn test_tick_is_newer_than() {
        let tick1 = Tick::new(5);
        let tick2 = Tick::new(3);
        assert!(tick1.is_newer_than(tick2, 100));
        assert!(!tick2.is_newer_than(tick1, 100));
    }

    #[test]
    fn test_component_ticks_creation() {
        let tick = Tick::new(10);
        let ticks = ComponentTicks::new(tick);
        assert_eq!(ticks.zig_ticks.added.value, 10);
        assert_eq!(ticks.zig_ticks.changed.value, 10);
    }

    #[test]
    fn test_component_ticks_set_changed() {
        let tick = Tick::new(10);
        let mut ticks = ComponentTicks::new(tick);
        let new_tick = Tick::new(20);
        ticks.set_changed(new_tick);
        assert_eq!(ticks.zig_ticks.changed.value, 20);
        assert_eq!(ticks.zig_ticks.added.value, 10);
    }

    #[test]
    fn test_component_ticks_is_added() {
        let tick = Tick::new(10);
        let ticks = ComponentTicks::new(tick);
        let last_run = Tick::new(5);
        let this_run = Tick::new(15);
        assert!(ticks.is_added(last_run, this_run));
    }

    #[test]
    fn test_component_ticks_is_changed() {
        let tick = Tick::new(10);
        let mut ticks = ComponentTicks::new(tick);
        ticks.set_changed(Tick::new(12));
        let last_run = Tick::new(5);
        let this_run = Tick::new(15);
        assert!(ticks.is_changed(last_run, this_run));
    }

    #[test]
    fn test_change_detection_context_creation() {
        let ctx = ChangeDetectionContext::new();
        assert_eq!(ctx.current_tick().0, 0);
        assert_eq!(ctx.last_change_tick().0, 0);
    }

    #[test]
    fn test_change_detection_context_increment() {
        let mut ctx = ChangeDetectionContext::new();
        ctx.increment();
        assert_eq!(ctx.current_tick().0, 1);
        assert_eq!(ctx.last_change_tick().0, 0);
        ctx.increment();
        assert_eq!(ctx.current_tick().0, 2);
        assert_eq!(ctx.last_change_tick().0, 1);
    }

    #[derive(Debug, Clone, Copy)]
    struct TestComponent;
    impl Component for TestComponent {}

    #[test]
    fn test_removed_components_creation() {
        ensure_init();
        let removed: RemovedComponents<TestComponent> = RemovedComponents::new(1);
        assert_eq!(removed.len(), 0);
        assert!(removed.is_empty());
    }

    #[test]
    fn test_removed_components_record() {
        ensure_init();
        let mut removed: RemovedComponents<TestComponent> = RemovedComponents::new(1);
        assert!(removed.record(Entity::from_raw(100)));
        assert!(removed.record(Entity::from_raw(200)));
        assert_eq!(removed.len(), 2);
    }

    #[test]
    fn test_removed_components_clear() {
        ensure_init();
        let mut removed: RemovedComponents<TestComponent> = RemovedComponents::new(1);
        removed.record(Entity::from_raw(100));
        removed.record(Entity::from_raw(200));
        assert_eq!(removed.len(), 2);
        removed.clear();
        assert_eq!(removed.len(), 0);
    }

    #[test]
    fn test_removed_components_iter() {
        ensure_init();
        let mut removed: RemovedComponents<TestComponent> = RemovedComponents::new(1);
        removed.record(Entity::from_raw(100));
        removed.record(Entity::from_raw(200));
        removed.record(Entity::from_raw(300));
        let entities: Vec<Entity> = removed.iter().collect();
        assert_eq!(entities.len(), 3);
        assert_eq!(entities, vec![Entity::from_raw(100), Entity::from_raw(200), Entity::from_raw(300)]);
    }

    #[test]
    fn test_changed_filter_creation() {
        let _filter: Changed<TestComponent> = Changed::default();
    }

    #[test]
    fn test_added_filter_creation() {
        let _filter: Added<TestComponent> = Added::default();
    }
}