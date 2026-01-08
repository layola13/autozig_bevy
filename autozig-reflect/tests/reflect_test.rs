//! 集成测试 - 测试完整的反射功能

use autozig_reflect::*;

#[test]
fn test_type_info_basic() {
    let info = TypeInfoHandle::new("TestStruct", 12345, TypeInfoKind::Struct);
    assert!(info.is_some(), "Failed to create TypeInfo");
    
    let info = info.unwrap();
    assert_eq!(info.type_name(), "TestStruct");
    assert_eq!(info.type_id(), 12345);
    assert_eq!(info.kind(), TypeInfoKind::Struct);
}

#[test]
fn test_type_info_with_fields() {
    let mut info = TypeInfoHandle::new("Point", 54321, TypeInfoKind::Struct)
        .expect("Failed to create TypeInfo");
    
    assert!(info.add_field("x", "f32", 0), "Failed to add field x");
    assert!(info.add_field("y", "f32", 4), "Failed to add field y");
    
    assert_eq!(info.field_count(), 2);
    assert_eq!(info.field_name(0), "x");
    assert_eq!(info.field_name(1), "y");
}

#[test]
fn test_type_registry_operations() {
    let mut registry = TypeRegistryHandle::new()
        .expect("Failed to create TypeRegistry");
    
    // 初始状态应为空
    assert_eq!(registry.len(), 0);
    assert!(registry.is_empty());
    
    // 注册类型
    assert!(registry.register(1, "i32"), "Failed to register i32");
    assert!(registry.register(2, "f64"), "Failed to register f64");
    assert!(registry.register(3, "String"), "Failed to register String");
    
    // 验证注册
    assert_eq!(registry.len(), 3);
    assert!(!registry.is_empty());
    assert!(registry.contains(1));
    assert!(registry.contains(2));
    assert!(registry.contains(3));
    assert!(!registry.contains(999));
    
    // 获取类型名
    assert_eq!(registry.get_type_name(1), Some("i32".to_string()));
    assert_eq!(registry.get_type_name(2), Some("f64".to_string()));
    assert_eq!(registry.get_type_name(3), Some("String".to_string()));
    assert_eq!(registry.get_type_name(999), None);
}

#[test]
fn test_position_struct_reflection() {
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
    
    let pos = Position { x: 10.5, y: 20.3 };
    
    // 测试Reflect trait
    assert_eq!(pos.type_name(), "Position");
    
    // 测试Struct trait
    assert_eq!(pos.field_count(), 2);
    assert_eq!(pos.field_name(0), Some("x"));
    assert_eq!(pos.field_name(1), Some("y"));
    assert_eq!(pos.field_name(2), None);
    
    // 测试field descriptors
    let descriptors = pos.field_descriptors();
    assert_eq!(descriptors.len(), 2);
    
    assert_eq!(descriptors[0].name, "x");
    assert_eq!(descriptors[0].type_name, "f32");
    assert_eq!(descriptors[0].offset, 0);
    
    assert_eq!(descriptors[1].name, "y");
    assert_eq!(descriptors[1].type_name, "f32");
    assert_eq!(descriptors[1].offset, 4);
}

#[test]
fn test_offset_calculation() {
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
    
    assert_eq!(x_offset, 0);
    assert_eq!(y_offset, 4);
    assert_eq!(rotation_offset, 8);
    assert_eq!(scale_offset, 12);
}

#[test]
fn test_complex_struct_reflection() {
    #[repr(C)]
    struct Velocity {
        dx: f32,
        dy: f32,
    }
    
    impl Reflect for Velocity {
        fn type_name(&self) -> &'static str {
            "Velocity"
        }
    }
    
    impl Struct for Velocity {
        fn field_count(&self) -> usize {
            2
        }
        
        fn field_name(&self, index: usize) -> Option<&'static str> {
            match index {
                0 => Some("dx"),
                1 => Some("dy"),
                _ => None,
            }
        }
        
        fn field_descriptors(&self) -> &'static [FieldDescriptor] {
            static DESCRIPTORS: &[FieldDescriptor] = &[
                FieldDescriptor {
                    name: "dx",
                    type_name: "f32",
                    offset: 0,
                },
                FieldDescriptor {
                    name: "dy",
                    type_name: "f32",
                    offset: 4,
                },
            ];
            DESCRIPTORS
        }
    }
    
    let vel = Velocity { dx: 5.0, dy: -3.5 };
    
    assert_eq!(vel.type_name(), "Velocity");
    assert_eq!(vel.field_count(), 2);
    
    let descriptors = vel.field_descriptors();
    assert_eq!(descriptors.len(), 2);
    assert_eq!(descriptors[0].name, "dx");
    assert_eq!(descriptors[1].name, "dy");
}

#[test]
fn test_multiple_type_infos() {
    let mut info1 = TypeInfoHandle::new("Type1", 100, TypeInfoKind::Struct)
        .expect("Failed to create TypeInfo1");
    let mut info2 = TypeInfoHandle::new("Type2", 200, TypeInfoKind::Struct)
        .expect("Failed to create TypeInfo2");
    
    info1.add_field("field1", "i32", 0);
    info2.add_field("field2", "f64", 0);
    
    assert_eq!(info1.type_name(), "Type1");
    assert_eq!(info2.type_name(), "Type2");
    assert_eq!(info1.field_count(), 1);
    assert_eq!(info2.field_count(), 1);
}

#[test]
fn test_type_info_kinds() {
    let struct_info = TypeInfoHandle::new("MyStruct", 1, TypeInfoKind::Struct)
        .expect("Failed to create struct info");
    let tuple_info = TypeInfoHandle::new("MyTuple", 2, TypeInfoKind::Tuple)
        .expect("Failed to create tuple info");
    let enum_info = TypeInfoHandle::new("MyEnum", 3, TypeInfoKind::Enum)
        .expect("Failed to create enum info");
    
    assert_eq!(struct_info.kind(), TypeInfoKind::Struct);
    assert_eq!(tuple_info.kind(), TypeInfoKind::Tuple);
    assert_eq!(enum_info.kind(), TypeInfoKind::Enum);
}

#[test]
fn test_registry_duplicate_registration() {
    let mut registry = TypeRegistryHandle::new()
        .expect("Failed to create TypeRegistry");
    
    // 首次注册
    assert!(registry.register(42, "TestType"));
    assert_eq!(registry.len(), 1);
    
    // 重复注册同一个type_id（应该不增加计数）
    assert!(registry.register(42, "TestType"));
    assert_eq!(registry.len(), 1);
}