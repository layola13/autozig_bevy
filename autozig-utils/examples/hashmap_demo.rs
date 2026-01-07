//! HashMap和HashSet使用示例

use autozig_utils::prelude::*;

fn main() {
    println!("=== HashMap Demo ===\n");
    hashmap_demo();
    
    println!("\n=== HashSet Demo ===\n");
    hashset_demo();
}

fn hashmap_demo() {
    let mut map = HashMap::new();
    
    // 插入实体ID和组件ID的映射
    println!("插入键值对...");
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    map.insert(4, 400);
    
    println!("HashMap大小: {}", map.len());
    println!("HashMap容量: {}", map.capacity());
    
    // 查询
    println!("\n查询操作:");
    if let Some(value) = map.get(2) {
        println!("  键 2 的值: {}", value);
    }
    
    println!("  包含键 3? {}", map.contains_key(3));
    println!("  包含键 999? {}", map.contains_key(999));
    
    // 更新
    println!("\n更新键 2 的值为 250");
    map.insert(2, 250);
    if let Some(value) = map.get(2) {
        println!("  新值: {}", value);
    }
    
    // 删除
    println!("\n删除键 3");
    if map.remove(3) {
        println!("  删除成功");
    }
    println!("  当前大小: {}", map.len());
    
    // 遍历（通过键）
    println!("\n当前所有键:");
    for key in 1..=4 {
        if let Some(value) = map.get(key) {
            println!("  {} => {}", key, value);
        }
    }
    
    // 清空
    println!("\n清空HashMap");
    map.clear();
    println!("  清空后大小: {}", map.len());
    println!("  是否为空? {}", map.is_empty());
}

fn hashset_demo() {
    let mut set = HashSet::new();
    
    // 插入实体ID
    println!("插入元素...");
    set.insert(10);
    set.insert(20);
    set.insert(30);
    set.insert(40);
    
    // 尝试插入重复元素
    println!("尝试插入重复元素 20: {}", set.insert(20));
    
    println!("\nHashSet大小: {}", set.len());
    println!("HashSet容量: {}", set.capacity());
    
    // 检查包含
    println!("\n包含性检查:");
    println!("  包含 20? {}", set.contains(20));
    println!("  包含 25? {}", set.contains(25));
    println!("  包含 30? {}", set.contains(30));
    
    // 删除
    println!("\n删除元素 30");
    if set.remove(30) {
        println!("  删除成功");
    }
    println!("  当前大小: {}", set.len());
    
    // 集合操作模拟
    println!("\n集合操作示例:");
    let mut set2 = HashSet::new();
    set2.insert(20);
    set2.insert(40);
    set2.insert(60);
    
    println!("  set1 包含: 10, 20, 40");
    println!("  set2 包含: 20, 40, 60");
    
    // 交集检查（手动）
    print!("  交集: ");
    for &item in &[20, 40, 60] {
        if set.contains(item) && set2.contains(item) {
            print!("{} ", item);
        }
    }
    println!();
    
    // 清空
    println!("\n清空HashSet");
    set.clear();
    println!("  清空后大小: {}", set.len());
    println!("  是否为空? {}", set.is_empty());
}