//! Archetype存储系统集成测试
//! 测试Table + SparseSet双模式存储

use autozig_ecs::prelude::*;
use autozig_ecs::storage::{Archetype, Table};

#[test]
fn test_archetype_creation() {
    let archetype = Archetype::new(0);
    assert!(archetype.is_some(), "Archetype应该成功创建");
    
    let arch = archetype.unwrap();
    assert_eq!(arch.id(), 0, "Archetype ID应该为0");
    assert_eq!(arch.entity_count(), 0, "新Archetype应该没有entity");
}

#[test]
fn test_archetype_add_entity() {
    let mut archetype = Archetype::new(1).unwrap();
    let entity = Entity::new(100, 0);
    
    let row = archetype.add_entity(entity);
    assert!(row.is_some(), "添加entity应该成功");
    assert_eq!(row.unwrap(), 0, "第一个entity应该在行0");
    assert_eq!(archetype.entity_count(), 1, "应该有1个entity");
    assert!(archetype.contains_entity(entity), "应该包含该entity");
}

#[test]
fn test_archetype_remove_entity() {
    let mut archetype = Archetype::new(2).unwrap();
    let entity1 = Entity::new(100, 0);
    let entity2 = Entity::new(101, 0);
    
    archetype.add_entity(entity1);
    archetype.add_entity(entity2);
    assert_eq!(archetype.entity_count(), 2, "应该有2个entity");
    
    let removed = archetype.remove_entity(entity1);
    assert!(removed, "移除entity应该成功");
    assert_eq!(archetype.entity_count(), 1, "应该剩余1个entity");
    assert!(!archetype.contains_entity(entity1), "不应该包含已移除的entity");
    assert!(archetype.contains_entity(entity2), "应该仍包含entity2");
}

#[test]
fn test_table_column_storage() {
    let mut table = Table::new().unwrap();
    
    // 添加两列：Position(component_id=1, 8字节) 和 Velocity(component_id=2, 8字节)
    assert!(table.add_column(1, 8), "添加Position列应该成功");
    assert!(table.add_column(2, 8), "添加Velocity列应该成功");
    
    let entity = Entity::new(200, 0);
    let row = table.push_row(entity);
    assert!(row.is_some(), "添加行应该成功");
    assert_eq!(table.entity_count(), 1, "应该有1行数据");
}

#[test]
fn test_table_swap_remove() {
    let mut table = Table::new().unwrap();
    table.add_column(1, 4);
    
    let entity1 = Entity::new(100, 0);
    let entity2 = Entity::new(101, 0);
    let entity3 = Entity::new(102, 0);
    
    table.push_row(entity1);
    table.push_row(entity2);
    table.push_row(entity3);
    assert_eq!(table.entity_count(), 3, "应该有3行");
    
    let removed = table.swap_remove(1);
    assert!(removed, "swap-remove应该成功");
    assert_eq!(table.entity_count(), 2, "应该剩余2行");
}

#[test]
fn test_sparse_set_insert() {
    let mut sparse_set = StorageSparseSet::new().unwrap();
    let entity1 = Entity::new(10, 0);
    let entity2 = Entity::new(20, 0);
    
    assert!(sparse_set.insert(entity1), "插入entity1应该成功");
    assert!(sparse_set.insert(entity2), "插入entity2应该成功");
    assert_eq!(sparse_set.len(), 2, "应该有2个entity");
}

#[test]
fn test_sparse_set_remove() {
    let mut sparse_set = StorageSparseSet::new().unwrap();
    let entity = Entity::new(50, 0);
    
    sparse_set.insert(entity);
    assert!(sparse_set.contains(entity), "应该包含entity");
    
    let removed = sparse_set.remove(entity);
    assert!(removed, "移除应该成功");
    assert!(!sparse_set.contains(entity), "不应该再包含entity");
    assert_eq!(sparse_set.len(), 0, "应该为空");
}

#[test]
fn test_sparse_set_contains() {
    let mut sparse_set = StorageSparseSet::new().unwrap();
    let entity1 = Entity::new(100, 0);
    let entity2 = Entity::new(200, 0);
    
    sparse_set.insert(entity1);
    
    assert!(sparse_set.contains(entity1), "应该包含entity1");
    assert!(!sparse_set.contains(entity2), "不应该包含entity2");
}

#[test]
fn test_storage_type_selection() {
    let mut archetype = Archetype::new(3).unwrap();
    
    // 添加Table组件
    assert!(archetype.add_table_component(1), "添加Table组件应该成功");
    assert!(archetype.add_table_component(2), "添加Table组件应该成功");
    
    // 添加SparseSet组件
    assert!(archetype.add_sparse_set_component(10), "添加SparseSet组件应该成功");
    
    assert!(archetype.has_table_component(1), "应该有Table组件1");
    assert!(archetype.has_table_component(2), "应该有Table组件2");
    assert!(archetype.has_sparse_set_component(10), "应该有SparseSet组件10");
    assert!(!archetype.has_table_component(10), "组件10不应该在Table中");
}

#[test]
fn test_mixed_storage_archetype() {
    let mut archetype = Archetype::new(4).unwrap();
    
    // 混合存储：Position和Velocity用Table，Tag用SparseSet
    archetype.add_table_component(1); // Position
    archetype.add_table_component(2); // Velocity
    archetype.add_sparse_set_component(100); // PlayerTag
    
    assert_eq!(archetype.table_component_count(), 2, "应该有2个Table组件");
    assert_eq!(archetype.sparse_set_component_count(), 1, "应该有1个SparseSet组件");
    
    let entity = Entity::new(999, 0);
    archetype.add_entity(entity);
    assert_eq!(archetype.entity_count(), 1, "应该有1个entity");
}

#[test]
fn test_archetype_entity_iteration() {
    let mut archetype = Archetype::new(5).unwrap();
    
    let entities = vec![
        Entity::new(1, 0),
        Entity::new(2, 0),
        Entity::new(3, 0),
        Entity::new(4, 0),
        Entity::new(5, 0),
    ];
    
    for entity in &entities {
        archetype.add_entity(*entity);
    }
    
    let stored_entities = archetype.entities();
    assert_eq!(stored_entities.len(), 5, "应该有5个entity");
    
    for entity in &entities {
        assert!(archetype.contains_entity(*entity), "应该包含所有添加的entity");
    }
}

#[test]
fn test_archetype_component_access() {
    let mut archetype = Archetype::new(6).unwrap();
    
    archetype.add_table_component(1);
    archetype.add_table_component(2);
    archetype.add_sparse_set_component(10);
    archetype.add_sparse_set_component(11);
    
    assert!(archetype.has_component(1), "应该有组件1");
    assert!(archetype.has_component(2), "应该有组件2");
    assert!(archetype.has_component(10), "应该有组件10");
    assert!(archetype.has_component(11), "应该有组件11");
    assert!(!archetype.has_component(999), "不应该有组件999");
}

#[test]
fn test_table_entity_lookup() {
    let mut table = Table::new().unwrap();
    table.add_column(1, 4);
    
    let entity1 = Entity::new(100, 0);
    let entity2 = Entity::new(200, 0);
    
    table.push_row(entity1);
    table.push_row(entity2);
    
    let row1 = table.get_entity_row(entity1);
    assert_eq!(row1, Some(0), "entity1应该在行0");
    
    let row2 = table.get_entity_row(entity2);
    assert_eq!(row2, Some(1), "entity2应该在行1");
    
    let row_none = table.get_entity_row(Entity::new(999, 0));
    assert_eq!(row_none, None, "不存在的entity应该返回None");
}

#[test]
fn test_sparse_set_is_empty() {
    let mut sparse_set = StorageSparseSet::new().unwrap();
    assert!(sparse_set.is_empty(), "新建的SparseSet应该为空");
    
    sparse_set.insert(Entity::new(1, 0));
    assert!(!sparse_set.is_empty(), "插入后不应该为空");
    
    sparse_set.clear();
    assert!(sparse_set.is_empty(), "清空后应该为空");
}

#[test]
fn test_archetype_clear() {
    let mut archetype = Archetype::new(7).unwrap();
    
    for i in 0..10 {
        archetype.add_entity(Entity::new(i, 0));
    }
    assert_eq!(archetype.entity_count(), 10, "应该有10个entity");
    
    archetype.clear();
    assert_eq!(archetype.entity_count(), 0, "清空后应该没有entity");
}

#[test]
fn test_table_clear() {
    let mut table = Table::new().unwrap();
    table.add_column(1, 4);
    
    for i in 0..5 {
        table.push_row(Entity::new(i, 0));
    }
    assert_eq!(table.entity_count(), 5, "应该有5行");
    
    table.clear();
    assert_eq!(table.entity_count(), 0, "清空后应该没有行");
}