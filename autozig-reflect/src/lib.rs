//! AutoZig Shadow Reflect - 轻量级反射系统
//!
//! 核心理念：
//! - Rust只负责"报户口"（生成静态Schema描述符）
//! - Zig负责"查户口"（通过指针偏移访问内存）
//! - 90% Zig核心 + 10% Rust包装层
//! - 所有反射类型必须 #[repr(C)]

#![no_std]

extern crate alloc;

use alloc::string::String;
use core::any::TypeId;
use core::ptr::NonNull;

// 使用autozig的include_zig!宏导入Zig FFI函数
use autozig::include_zig;

include_zig!("src/zig/type_info.zig", {
    fn type_info_create(type_name: *const u8, type_name_len: usize, type_id: u64, kind: TypeInfoKind) -> *mut TypeInfo;
    fn type_info_destroy(info: *mut TypeInfo);
    fn type_info_get_name(info: *const TypeInfo) -> *const u8;
    fn type_info_get_name_len(info: *const TypeInfo) -> usize;
    fn type_info_get_type_id(info: *const TypeInfo) -> u64;
    fn type_info_get_kind(info: *const TypeInfo) -> TypeInfoKind;
    fn type_info_add_field(info: *mut TypeInfo, field_name: *const u8, field_name_len: usize, field_type_name: *const u8, field_type_name_len: usize, field_offset: usize) -> bool;
    fn type_info_get_field_count(info: *const TypeInfo) -> usize;
    fn type_info_get_field_name(info: *const TypeInfo, index: usize) -> *const u8;
    fn type_info_get_field_name_len(info: *const TypeInfo, index: usize) -> usize;
});

include_zig!("src/zig/type_registry.zig", {
    fn type_registry_create() -> *mut TypeRegistry;
    fn type_registry_destroy(registry: *mut TypeRegistry);
    fn type_registry_register(registry: *mut TypeRegistry, type_id: u64, type_name: *const u8, type_name_len: usize) -> bool;
    fn type_registry_get_type_name(registry: *const TypeRegistry, type_id: u64) -> *const u8;
    fn type_registry_get_type_name_len(registry: *const TypeRegistry, type_id: u64) -> usize;
    fn type_registry_contains(registry: *const TypeRegistry, type_id: u64) -> bool;
    fn type_registry_len(registry: *const TypeRegistry) -> usize;
});

include_zig!("src/zig/struct_trait.zig", {
    fn struct_data_create(field_count: usize) -> *mut StructData;
    fn struct_data_destroy(data: *mut StructData);
    fn struct_data_field_count(data: *const StructData) -> usize;
    fn struct_data_get_field_name(data: *const StructData, index: usize) -> *const u8;
    fn struct_data_get_field_name_len(data: *const StructData, index: usize) -> usize;
});

/// Type info kind - 与Zig的TypeInfoKind保持一致
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TypeInfoKind {
    Struct = 0,
    TupleStruct = 1,
    Tuple = 2,
    List = 3,
    Array = 4,
    Map = 5,
    Enum = 6,
    Value = 7,
}

/// Zig侧的TypeInfo不透明指针
#[repr(C)]
pub struct TypeInfo {
    _opaque: [u8; 0],
}

/// Zig侧的TypeRegistry不透明指针
#[repr(C)]
pub struct TypeRegistry {
    _opaque: [u8; 0],
}

/// Zig侧的StructData不透明指针
#[repr(C)]
pub struct StructData {
    _opaque: [u8; 0],
}

/// Rust侧的TypeInfo包装器
pub struct TypeInfoHandle {
    ptr: NonNull<TypeInfo>,
}

impl TypeInfoHandle {
    /// 创建新的TypeInfo
    pub fn new(type_name: &str, type_id: u64, kind: TypeInfoKind) -> Option<Self> {
        let ptr = core::ptr::NonNull::new(
            type_info_create(
                type_name.as_ptr(),
                type_name.len(),
                type_id,
                kind,
            )
        )?;
        
        Some(Self { ptr })
    }

    /// 添加字段
    pub fn add_field(&mut self, field_name: &str, field_type_name: &str, field_offset: usize) -> bool {
        type_info_add_field(
            self.ptr.as_ptr(),
            field_name.as_ptr(),
            field_name.len(),
            field_type_name.as_ptr(),
            field_type_name.len(),
            field_offset,
        )
    }

    /// 获取类型名
    pub fn type_name(&self) -> String {
        let ptr = type_info_get_name(self.ptr.as_ptr());
        let len = type_info_get_name_len(self.ptr.as_ptr());
        
        if len == 0 {
            return String::new();
        }
        
        let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
        String::from_utf8_lossy(slice).into_owned()
    }

    /// 获取类型ID
    pub fn type_id(&self) -> u64 {
        type_info_get_type_id(self.ptr.as_ptr())
    }

    /// 获取类型kind
    pub fn kind(&self) -> TypeInfoKind {
        type_info_get_kind(self.ptr.as_ptr())
    }

    /// 获取字段数量
    pub fn field_count(&self) -> usize {
        type_info_get_field_count(self.ptr.as_ptr())
    }

    /// 获取字段名
    pub fn field_name(&self, index: usize) -> String {
        let ptr = type_info_get_field_name(self.ptr.as_ptr(), index);
        let len = type_info_get_field_name_len(self.ptr.as_ptr(), index);
        
        if len == 0 {
            return String::new();
        }
        
        let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
        String::from_utf8_lossy(slice).into_owned()
    }
}

impl Drop for TypeInfoHandle {
    fn drop(&mut self) {
        type_info_destroy(self.ptr.as_ptr());
    }
}

/// Rust侧的TypeRegistry包装器
pub struct TypeRegistryHandle {
    ptr: NonNull<TypeRegistry>,
}

impl TypeRegistryHandle {
    /// 创建新的TypeRegistry
    pub fn new() -> Option<Self> {
        let ptr = core::ptr::NonNull::new(type_registry_create())?;
        Some(Self { ptr })
    }

    /// 注册类型
    pub fn register(&mut self, type_id: u64, type_name: &str) -> bool {
        type_registry_register(
            self.ptr.as_ptr(),
            type_id,
            type_name.as_ptr(),
            type_name.len(),
        )
    }

    /// 获取类型名
    pub fn get_type_name(&self, type_id: u64) -> Option<String> {
        if !self.contains(type_id) {
            return None;
        }
        
        let ptr = type_registry_get_type_name(self.ptr.as_ptr(), type_id);
        let len = type_registry_get_type_name_len(self.ptr.as_ptr(), type_id);
        
        if len == 0 {
            return None;
        }
        
        let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
        Some(String::from_utf8_lossy(slice).into_owned())
    }

    /// 检查类型是否已注册
    pub fn contains(&self, type_id: u64) -> bool {
        type_registry_contains(self.ptr.as_ptr(), type_id)
    }

    /// 获取注册的类型数量
    pub fn len(&self) -> usize {
        type_registry_len(self.ptr.as_ptr())
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Drop for TypeRegistryHandle {
    fn drop(&mut self) {
        type_registry_destroy(self.ptr.as_ptr());
    }
}

impl Default for TypeRegistryHandle {
    fn default() -> Self {
        Self::new().expect("Failed to create TypeRegistry")
    }
}

/// Rust侧的StructData包装器
pub struct StructDataHandle {
    ptr: NonNull<StructData>,
}

impl StructDataHandle {
    /// 创建新的StructData
    pub fn new(field_count: usize) -> Option<Self> {
        let ptr = core::ptr::NonNull::new(struct_data_create(field_count))?;
        Some(Self { ptr })
    }

    /// 获取字段数量
    pub fn field_count(&self) -> usize {
        struct_data_field_count(self.ptr.as_ptr())
    }

    /// 获取字段名
    pub fn get_field_name(&self, index: usize) -> String {
        let ptr = struct_data_get_field_name(self.ptr.as_ptr(), index);
        let len = struct_data_get_field_name_len(self.ptr.as_ptr(), index);
        
        if len == 0 {
            return String::new();
        }
        
        let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
        String::from_utf8_lossy(slice).into_owned()
    }
}

impl Drop for StructDataHandle {
    fn drop(&mut self) {
        struct_data_destroy(self.ptr.as_ptr());
    }
}

/// Reflect trait - 核心反射接口
pub trait Reflect where Self: 'static {
    /// 获取类型名
    fn type_name(&self) -> &'static str;
    
    /// 获取类型ID
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

/// 字段描述符
#[repr(C)]
#[derive(Debug, Clone)]
pub struct FieldDescriptor {
    pub name: &'static str,
    pub type_name: &'static str,
    pub offset: usize,
}

/// Struct trait - 结构体反射接口
pub trait Struct: Reflect {
    /// 获取字段数量
    fn field_count(&self) -> usize;
    
    /// 获取字段名
    fn field_name(&self, index: usize) -> Option<&'static str>;
    
    /// 获取所有字段描述符
    fn field_descriptors(&self) -> &'static [FieldDescriptor];
}

/// 计算字段偏移量的宏
#[macro_export]
macro_rules! offset_of {
    ($type:ty, $field:ident) => {{
        let dummy = core::mem::MaybeUninit::<$type>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let field_ptr = unsafe { core::ptr::addr_of!((*dummy_ptr).$field) };
        (field_ptr as usize) - (dummy_ptr as usize)
    }};
}

/// 实现Reflect的派生宏辅助函数
#[doc(hidden)]
pub fn type_id_to_u64(type_id: TypeId) -> u64 {
    // 使用hash将TypeId转换为u64
    use core::hash::{Hash, Hasher};
    
    // 简单的FNV-1a hasher
    struct SimpleHasher(u64);
    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 { self.0 }
        fn write(&mut self, bytes: &[u8]) {
            for &byte in bytes {
                self.0 ^= byte as u64;
                self.0 = self.0.wrapping_mul(0x100000001b3);
            }
        }
    }
    
    let mut hasher = SimpleHasher(0xcbf29ce484222325);
    type_id.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_info_creation() {
        let info = TypeInfoHandle::new("TestStruct", 12345, TypeInfoKind::Struct);
        assert!(info.is_some());
        
        let info = info.unwrap();
        assert_eq!(info.type_name(), "TestStruct");
        assert_eq!(info.type_id(), 12345);
        assert_eq!(info.kind(), TypeInfoKind::Struct);
    }

    #[test]
    fn test_type_info_fields() {
        let mut info = TypeInfoHandle::new("Point", 54321, TypeInfoKind::Struct).unwrap();
        
        assert!(info.add_field("x", "f32", 0));
        assert!(info.add_field("y", "f32", 4));
        
        assert_eq!(info.field_count(), 2);
        assert_eq!(info.field_name(0), "x");
        assert_eq!(info.field_name(1), "y");
    }

    #[test]
    fn test_type_registry_creation() {
        let registry = TypeRegistryHandle::new();
        assert!(registry.is_some());
        
        let registry = registry.unwrap();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn test_type_registry_register() {
        let mut registry = TypeRegistryHandle::new().unwrap();
        
        assert!(registry.register(1, "i32"));
        assert!(registry.register(2, "f64"));
        
        assert_eq!(registry.len(), 2);
        assert!(registry.contains(1));
        assert!(registry.contains(2));
        assert!(!registry.contains(3));
    }

    #[test]
    fn test_type_registry_get_name() {
        let mut registry = TypeRegistryHandle::new().unwrap();
        
        registry.register(42, "TestType");
        
        let name = registry.get_type_name(42);
        assert!(name.is_some());
        assert_eq!(name.unwrap(), "TestType");
        
        let none = registry.get_type_name(999);
        assert!(none.is_none());
    }

    #[test]
    fn test_offset_macro() {
        #[repr(C)]
        struct Position {
            x: f32,
            y: f32,
        }
        
        let x_offset = offset_of!(Position, x);
        let y_offset = offset_of!(Position, y);
        
        assert_eq!(x_offset, 0);
        assert_eq!(y_offset, 4);
    }

    #[test]
    fn test_position_reflect() {
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
        
        let pos = Position { x: 1.0, y: 2.0 };
        assert_eq!(pos.type_name(), "Position");
        assert_eq!(pos.field_count(), 2);
        assert_eq!(pos.field_name(0), Some("x"));
        assert_eq!(pos.field_name(1), Some("y"));
        
        let descriptors = pos.field_descriptors();
        assert_eq!(descriptors.len(), 2);
        assert_eq!(descriptors[0].name, "x");
        assert_eq!(descriptors[0].type_name, "f32");
        assert_eq!(descriptors[0].offset, 0);
    }
}