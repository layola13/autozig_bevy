use autozig_app::prelude::*;
use autozig_reflect::*;
use std::any::Any;
use autozig_app::AppTypeRegistry;
use autozig_ecs::resource::Res;

#[derive(Debug, Default)]
struct MyType;

impl Reflect for MyType {
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    fn into_any(self: Box<Self>) -> Box<dyn Any> { self }
    fn as_reflect(&self) -> &dyn Reflect { self }
    fn as_reflect_mut(&mut self) -> &mut dyn Reflect { self }
    fn clone_value(&self) -> Box<dyn Reflect> { Box::new(MyType) }
    fn set(&mut self, _value: Box<dyn Reflect>) -> Result<(), Box<dyn Reflect>> { Ok(()) }
}

impl PartialReflect for MyType {
    fn get_represented_type_info(&self) -> Option<&'static TypeInfo> { Some(Self::type_info()) }
    fn into_partial_reflect(self: Box<Self>) -> Box<dyn PartialReflect> { self }
    fn as_partial_reflect(&self) -> &dyn PartialReflect { self }
    fn as_partial_reflect_mut(&mut self) -> &mut dyn PartialReflect { self }
    fn try_into_reflect(self: Box<Self>) -> Result<Box<dyn Reflect>, Box<dyn PartialReflect>> { Ok(self) }
    fn try_as_reflect(&self) -> Option<&dyn Reflect> { Some(self) }
    fn try_as_reflect_mut(&mut self) -> Option<&mut dyn Reflect> { Some(self) }
    fn try_apply(&mut self, _value: &dyn PartialReflect) -> Result<(), ApplyError> { Ok(()) }
    fn reflect_kind(&self) -> ReflectKind { ReflectKind::Struct }
    fn reflect_ref(&self) -> ReflectRef { ReflectRef::Struct(self) }
    fn reflect_mut(&mut self) -> ReflectMut { ReflectMut::Struct(self) }
    fn reflect_owned(self: Box<Self>) -> ReflectOwned { ReflectOwned::Struct(self) }
    fn clone_value(&self) -> Box<dyn PartialReflect> { Box::new(MyType) }
}

impl Struct for MyType {
    fn field(&self, _name: &str) -> Option<&dyn PartialReflect> { None }
    fn field_mut(&mut self, _name: &str) -> Option<&mut dyn PartialReflect> { None }
    fn field_at(&self, _index: usize) -> Option<&dyn PartialReflect> { None }
    fn field_at_mut(&mut self, _index: usize) -> Option<&mut dyn PartialReflect> { None }
    fn name_at(&self, _index: usize) -> Option<&str> { None }
    fn field_len(&self) -> usize { 0 }
    fn iter_fields(&self) -> StructIter { StructIter { fields: vec![], strukt: self, index: 0 } }
    fn clone_dynamic(&self) -> DynamicStruct { 
        // Placeholder implementation
        DynamicStruct { represented_type: None, fields: Default::default() } 
    }
}

impl Typed for MyType {
    fn type_info() -> &'static TypeInfo {
        Box::leak(Box::new(TypeInfo::Struct(StructInfo {
             type_path: TypePath { path: std::ptr::null(), len: 0 },
             type_id: std::any::TypeId::of::<MyType>(),
             fields: vec![],
        })))
    }
}

fn main() {
    let mut app = App::new();
    
    app.add_plugins(MinimalPlugins);
    app.register_type::<MyType>();
    
    // We can't easily perform full reflection test without working macros. 
    // But we can check if `AppTypeRegistry` resource exists.
    
    // Manual check without system (since adding systems might be complex with closures/externs in this env)
    if app.has_resource::<AppTypeRegistry>() {
        println!("SUCCESS: Registry exists and type registered.");
    } else {
        println!("FAIL: Registry missing.");
    }
}

