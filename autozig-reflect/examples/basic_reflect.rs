//! 基础反射示例
//! 
//! 展示如何使用autozig-reflect进行类型反射

use autozig_reflect::*;

fn main() {
    println!("=== AutoZig Reflect 基础示例 ===\n");
    
    // 示例1: 创建TypeInfo
    println!("1. 创建TypeInfo:");
    let mut info = TypeInfoHandle::new("Position", 12345, TypeInfoKind::Struct)
        .expect("Failed to create TypeInfo");
    
    info.add_field("x", "f32", 0);
    info.add_field("y", "f32", 4);
    
    println!("  类型名: {}", info.type_name());
    println!("  类型ID: {}", info.type_id());
    println!("  字段数: {}", info.field_count());
    for i in 0..info.field_count() {
        println!("    字段{}: {}", i, info.field_name(i));
    }
    println!();
    
    // 示例2: 使用TypeRegistry
    println!("2. 使用TypeRegistry:");
    let mut registry = TypeRegistryHandle::new()
        .expect("Failed to create TypeRegistry");
    
    registry.register(1, "i32");
    registry.register(2, "f64");
    registry.register(3, "String");
    registry.register(4, "Vec<T>");
    
    println!("  注册的类型数量: {}", registry.len());
    for type_id in [1, 2, 3, 4] {
        if let Some(name) = registry.get_type_name(type_id) {
            println!("    类型ID {}: {}", type_id, name);
        }
    }
    println!();
    
    // 示例3: 定义反射结构体
    println!("3. Position结构体反射:");
    
    #[repr(C)]
    struct Position {
        x: f32,
        y: f32,
    }
    
    impl Reflect for Position {
        fn type_name(&self) -> &'static str {
            "Position"
        }
    }
    
    impl Struct for Position {
        fn field_count(&self) -> usize {
            2
        }
        
        fn field_name(&self, index: usize) -> Option<&'static str> {
            match index {
                0 => Some("x"),
                1 => Some("y"),
                _ => None,
            }
        }
        
        fn field_descriptors(&self) -> &'static [FieldDescriptor] {
            static DESCRIPTORS: &[FieldDescriptor] = &[
                FieldDescriptor {
                    name: "x",
                    type_name: "f32",
                    offset: 0,
                },
                FieldDescriptor {
                    name: "y",
                    type_name: "f32",
                    offset: 4,
                },
            ];
            DESCRIPTORS
        }
    }
    
    let pos = Position { x: 100.5, y: 200.3 };
    
    println!("  类型名: {}", pos.type_name());
    println!("  字段数: {}", pos.field_count());
    
    let descriptors = pos.field_descriptors();
    for (i, desc) in descriptors.iter().enumerate() {
        println!("    字段{}: {} (类型: {}, 偏移: {} bytes)", 
                 i, desc.name, desc.type_name, desc.offset);
    }
    println!();
    
    // 示例4: 偏移量计算
    println!("4. 字段偏移量计算:");
    
    #[repr(C)]
    struct Transform {
        position_x: f32,
        position_y: f32,
        rotation: f32,
        scale: f32,
    }
    
    let x_offset = offset_of!(Transform, position_x);
    let y_offset = offset_of!(Transform, position_y);
    let rotation_offset = offset_of!(Transform, rotation);
    let scale_offset = offset_of!(Transform, scale);
    
    println!("  Transform结构体字段偏移:");
    println!("    position_x: {} bytes", x_offset);
    println!("    position_y: {} bytes", y_offset);
    println!("    rotation: {} bytes", rotation_offset);
    println!("    scale: {} bytes", scale_offset);
    println!("    总大小: {} bytes", std::mem::size_of::<Transform>());
    println!();
    
    // 示例5: 多种TypeInfoKind
    println!("5. 不同的TypeInfoKind:");
    let kinds = [
        ("MyStruct", TypeInfoKind::Struct),
        ("MyTuple", TypeInfoKind::Tuple),
        ("MyEnum", TypeInfoKind::Enum),
        ("MyArray", TypeInfoKind::Array),
        ("MyList", TypeInfoKind::List),
    ];
    
    for (i, (name, kind)) in kinds.iter().enumerate() {
        if let Some(info) = TypeInfoHandle::new(name, 1000 + i as u64, *kind) {
            println!("  {}: {:?}", info.type_name(), info.kind());
        }
    }
    
    println!("\n=== 示例完成 ===");
}