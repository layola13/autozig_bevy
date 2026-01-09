//! Change Detection Tests - 变更检测系统测试
//! 
//! 测试Tick, ComponentTicks, Changed<T>, Added<T>, RemovedComponents<T>

use autozig_ecs::prelude::*;

// 测试组件
#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Component for Velocity {}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Health(u32);

impl Component for Health {}

// ============================================================================
// Tick Tests - Tick测试
// ============================================================================

#[test]
fn test_tick_increment() {
    let mut tick = Tick::new(0);
    assert_eq!(tick.0, 0);
    
    tick.increment();
    assert_eq!(tick.0, 1);
    
    tick.increment();
    assert_eq!(tick.0, 2);
}

#[test]
fn test_tick_wrapping() {
    let mut tick = Tick::new(u32::MAX);
    tick.increment();
    assert_eq!(tick.0, 0, "Tick should wrap around from u32::MAX to 0");
}

#[test]
fn test_tick_comparison() {
    let tick1 = Tick::new(5);
    let tick2 = Tick::new(3);
    let tick3 = Tick::new(5);
    
    assert!(tick1 > tick2);
    assert!(tick2 < tick1);
    assert_eq!(tick1, tick3);
}

#[test]
fn test_tick_is_newer_than() {
    let newer = Tick::new(10);
    let older = Tick::new(5);
    
    assert!(newer.is_newer_than(older, 100));
    assert!(!older.is_newer_than(newer, 100));
}

// ============================================================================
// ComponentTicks Tests - 组件Tick测试
// ============================================================================

#[test]
fn test_component_ticks_creation() {
    let tick = Tick::new(10);
    let ticks = ComponentTicks::new(tick);
    
    // 初始时，added和changed应该相同
    assert!(ticks.is_added(Tick::new(0), Tick::new(20)));
    assert!(ticks.is_changed(Tick::new(0), Tick::new(20)));
}

#[test]
fn test_component_is_added() {
    let added_tick = Tick::new(10);
    let ticks = ComponentTicks::new(added_tick);
    
    // 在added_tick之前运行的系统应该看到这是新增的
    let last_run = Tick::new(5);
    let this_run = Tick::new(15);
    
    assert!(ticks.is_added(last_run, this_run));
}

#[test]
fn test_component_is_not_added_after_tick() {
    let added_tick = Tick::new(10);
    let ticks = ComponentTicks::new(added_tick);
    
    // 在added_tick之后运行的系统不应该看到这是新增的
    let last_run = Tick::new(15);
    let this_run = Tick::new(20);
    
    assert!(!ticks.is_added(last_run, this_run));
}

#[test]
fn test_component_is_changed() {
    let added_tick = Tick::new(5);
    let mut ticks = ComponentTicks::new(added_tick);
    
    // 修改组件
    ticks.set_changed(Tick::new(10));
    
    // 在change之前运行的系统应该看到变更
    let last_run = Tick::new(7);
    let this_run = Tick::new(15);
    
    assert!(ticks.is_changed(last_run, this_run));
}

#[test]
fn test_component_is_not_changed_after_tick() {
    let added_tick = Tick::new(5);
    let mut ticks = ComponentTicks::new(added_tick);
    ticks.set_changed(Tick::new(10));
    
    // 在change之后运行的系统不应该看到变更
    let last_run = Tick::new(15);
    let this_run = Tick::new(20);
    
    assert!(!ticks.is_changed(last_run, this_run));
}

// ============================================================================
// RemovedComponents Tests - 已移除组件测试
// ============================================================================

#[test]
fn test_removed_components_tracking() {
    let mut removed: RemovedComponents<Position> = RemovedComponents::new(1);
    
    assert_eq!(removed.len(), 0);
    assert!(removed.is_empty());
    
    let e1 = Entity { index: 100, generation: 0 };
    let e2 = Entity { index: 200, generation: 0 };
    let e3 = Entity { index: 300, generation: 0 };
    
    assert!(removed.record(e1));
    assert!(removed.record(e2));
    assert!(removed.record(e3));
    
    assert_eq!(removed.len(), 3);
    assert!(!removed.is_empty());
    
    let entities: Vec<Entity> = removed.iter().collect();
    assert_eq!(entities.len(), 3);
    assert!(entities.contains(&e1));
    assert!(entities.contains(&e2));
    assert!(entities.contains(&e3));
}

#[test]
fn test_removed_components_clear() {
    let mut removed: RemovedComponents<Position> = RemovedComponents::new(1);
    
    let e1 = Entity { index: 100, generation: 0 };
    let e2 = Entity { index: 200, generation: 0 };
    
    removed.record(e1);
    removed.record(e2);
    assert_eq!(removed.len(), 2);
    
    removed.clear();
    assert_eq!(removed.len(), 0);
    assert!(removed.is_empty());
}

// ============================================================================
// World Tick Tests - World tick测试
// ============================================================================

#[test]
fn test_world_tick_advancement() {
    let mut world = World::new();
    
    assert_eq!(world.current_tick().0, 0);
    assert_eq!(world.last_change_tick().0, 0);
    
    world.tick();
    assert_eq!(world.current_tick().0, 1);
    assert_eq!(world.last_change_tick().0, 0);
    
    world.tick();
    assert_eq!(world.current_tick().0, 2);
    assert_eq!(world.last_change_tick().0, 1);
}

#[test]
fn test_world_removed_components_tracking() {
    let mut world = World::new();
    
    let entity1 = world.spawn_empty();
    let entity2 = world.spawn_empty();
    
    // 记录移除的组件
    world.record_component_removed::<Position>(entity1);
    world.record_component_removed::<Position>(entity2);
    
    // 获取已移除的组件
    let removed = world.get_removed_components::<Position>();
    assert!(removed.is_some());
    
    let removed = removed.unwrap();
    assert_eq!(removed.len(), 2);
    
    let entities: Vec<Entity> = removed.iter().collect();
    assert!(entities.contains(&entity1));
    assert!(entities.contains(&entity2));
}

#[test]
fn test_world_tick_clears_removed_components() {
    let mut world = World::new();
    
    let entity = world.spawn_empty();
    world.record_component_removed::<Position>(entity);
    
    // Tick应该清理已移除组件记录
    world.tick();
    
    let removed = world.get_removed_components::<Position>();
    // Note: tick() clears the entire HashMap, so removed will be None
    assert!(removed.is_none() || removed.unwrap().is_empty());
}

// ============================================================================
// Query Filter Tests - 查询过滤器测试
// ============================================================================

#[test]
fn test_changed_filter_creation() {
    let _filter: Changed<Position> = Changed::default();
}

#[test]
fn test_added_filter_creation() {
    let _filter: Added<Position> = Added::default();
}

#[test]
fn test_changed_and_added_filters_are_query_filters() {
    // 编译时检查：确保Changed和Added实现了QueryFilter
    fn assert_is_filter<T: QueryFilter>() {}
    
    assert_is_filter::<Changed<Position>>();
    assert_is_filter::<Added<Position>>();
    assert_is_filter::<(Changed<Position>, Added<Velocity>)>();
}

// ============================================================================
// Integration Tests - 集成测试
// ============================================================================

#[test]
fn test_change_detection_workflow() {
    let mut world = World::new();
    
    // 测试当前功能：创建实体和推进tick
    let entity = world.spawn_empty();
    assert!(world.contains(entity));
    
    let tick_before = world.current_tick();
    world.tick();
    let tick_after = world.current_tick();
    
    assert_eq!(tick_after.0, tick_before.0 + 1);
}

#[test]
fn test_multiple_tick_advancement() {
    let mut world = World::new();
    
    for i in 0..10 {
        assert_eq!(world.current_tick().0, i as u32);
        world.tick();
    }
    
    assert_eq!(world.current_tick().0, 10);
}

#[test]
fn test_component_ticks_with_world() {
    let mut world = World::new();
    
    let current = world.current_tick();
    let ticks = ComponentTicks::new(current);
    
    // 推进world
    world.tick();
    
    // ticks应该反映在上一个tick创建
    assert!(ticks.is_added(world.last_change_tick(), world.current_tick()));
}

#[test]
fn test_removed_components_different_types() {
    let mut world = World::new();
    
    let entity1 = world.spawn_empty();
    let entity2 = world.spawn_empty();
    
    // 记录不同类型的移除
    world.record_component_removed::<Position>(entity1);
    world.record_component_removed::<Velocity>(entity2);
    
    // 应该可以分别获取
    let pos_removed = world.get_removed_components::<Position>();
    let vel_removed = world.get_removed_components::<Velocity>();
    
    assert!(pos_removed.is_some());
    assert!(vel_removed.is_some());
    
    assert_eq!(pos_removed.unwrap().len(), 1);
    assert_eq!(vel_removed.unwrap().len(), 1);
}

#[test]
fn test_tick_ordering() {
    let mut world = World::new();
    
    let mut ticks = Vec::new();
    for _ in 0..5 {
        ticks.push(world.current_tick());
        world.tick();
    }
    
    // 确保ticks是递增的
    for i in 1..ticks.len() {
        assert!(ticks[i] > ticks[i - 1]);
    }
}