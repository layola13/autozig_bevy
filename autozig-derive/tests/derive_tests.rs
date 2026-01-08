//! Integration tests for autozig-derive macros

use autozig_derive::{Deref, DerefMut, EnumVariantMeta};

// ============================================================================
// Deref Tests
// ============================================================================

#[test]
fn test_deref_single_field_tuple() {
    #[derive(Deref)]
    struct Wrapper(String);
    
    let wrapper = Wrapper(String::from("hello"));
    let s: &String = &*wrapper;
    assert_eq!(s, "hello");
}

#[test]
fn test_deref_single_field_named() {
    #[derive(Deref)]
    struct Container {
        value: String,
    }
    
    let container = Container {
        value: String::from("world"),
    };
    let s: &String = &*container;
    assert_eq!(s, "world");
}

#[test]
fn test_deref_multi_field_with_attr() {
    #[derive(Deref)]
    struct MyStruct {
        #[deref]
        value: String,
        other: i32,
    }
    
    let my_struct = MyStruct {
        value: String::from("test"),
        other: 42,
    };
    let s: &String = &*my_struct;
    assert_eq!(s, "test");
    assert_eq!(my_struct.other, 42);
}

#[test]
fn test_deref_with_generics() {
    #[derive(Deref)]
    struct GenericWrapper<T>(T);
    
    let wrapper = GenericWrapper(vec![1, 2, 3]);
    let v: &Vec<i32> = &*wrapper;
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], 1);
}

#[test]
fn test_deref_multi_field_tuple_with_attr() {
    #[derive(Deref)]
    struct TupleWrapper(i32, #[deref] String, f64);
    
    let wrapper = TupleWrapper(10, String::from("middle"), 3.14);
    let s: &String = &*wrapper;
    assert_eq!(s, "middle");
}

// ============================================================================
// DerefMut Tests
// ============================================================================

#[test]
fn test_deref_mut_single_field() {
    #[derive(Deref, DerefMut)]
    struct MutWrapper(String);
    
    let mut wrapper = MutWrapper(String::from("hello"));
    wrapper.push_str(" world");
    assert_eq!(&*wrapper, "hello world");
}

#[test]
fn test_deref_mut_multi_field() {
    #[derive(Deref, DerefMut)]
    struct MutStruct {
        #[deref]
        value: Vec<i32>,
        count: usize,
    }
    
    let mut my_struct = MutStruct {
        value: vec![1, 2, 3],
        count: 0,
    };
    my_struct.push(4);
    assert_eq!(my_struct.value.len(), 4);
    assert_eq!(my_struct.value[3], 4);
}

#[test]
fn test_deref_mut_with_generics() {
    #[derive(Deref, DerefMut)]
    struct GenericMutWrapper<T>(T);
    
    let mut wrapper = GenericMutWrapper(vec![1, 2, 3]);
    wrapper.push(4);
    assert_eq!(wrapper.0.len(), 4);
}

#[test]
fn test_deref_mut_named_field() {
    #[derive(Deref, DerefMut)]
    struct NamedMut {
        data: String,
    }
    
    let mut named = NamedMut {
        data: String::from("test"),
    };
    named.push_str("ing");
    assert_eq!(&*named, "testing");
}

// ============================================================================
// EnumVariantMeta Tests
// ============================================================================

#[test]
fn test_enum_variant_index() {
    #[derive(EnumVariantMeta)]
    enum Color {
        Red,
        Green,
        Blue,
    }
    
    assert_eq!(Color::Red.enum_variant_index(), 0);
    assert_eq!(Color::Green.enum_variant_index(), 1);
    assert_eq!(Color::Blue.enum_variant_index(), 2);
}

#[test]
fn test_enum_variant_name() {
    #[derive(EnumVariantMeta)]
    enum Status {
        Pending,
        Active,
        Completed,
    }
    
    assert_eq!(Status::Pending.enum_variant_name(), "Pending");
    assert_eq!(Status::Active.enum_variant_name(), "Active");
    assert_eq!(Status::Completed.enum_variant_name(), "Completed");
}

#[test]
fn test_enum_with_generics() {
    #[derive(EnumVariantMeta)]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    
    let ok: Result<i32, String> = Result::Ok(42);
    let err: Result<i32, String> = Result::Err(String::from("error"));
    
    assert_eq!(ok.enum_variant_index(), 0);
    assert_eq!(ok.enum_variant_name(), "Ok");
    
    assert_eq!(err.enum_variant_index(), 1);
    assert_eq!(err.enum_variant_name(), "Err");
}

#[test]
fn test_enum_with_tuple_variants() {
    #[derive(EnumVariantMeta)]
    enum Data {
        None,
        Single(i32),
        Pair(i32, i32),
    }
    
    assert_eq!(Data::None.enum_variant_index(), 0);
    assert_eq!(Data::Single(42).enum_variant_index(), 1);
    assert_eq!(Data::Pair(1, 2).enum_variant_index(), 2);
    
    assert_eq!(Data::None.enum_variant_name(), "None");
    assert_eq!(Data::Single(42).enum_variant_name(), "Single");
    assert_eq!(Data::Pair(1, 2).enum_variant_name(), "Pair");
}

#[test]
fn test_enum_with_struct_variants() {
    #[derive(EnumVariantMeta)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write { text: String },
    }
    
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write {
        text: String::from("hello"),
    };
    
    assert_eq!(quit.enum_variant_index(), 0);
    assert_eq!(move_msg.enum_variant_index(), 1);
    assert_eq!(write_msg.enum_variant_index(), 2);
    
    assert_eq!(quit.enum_variant_name(), "Quit");
    assert_eq!(move_msg.enum_variant_name(), "Move");
    assert_eq!(write_msg.enum_variant_name(), "Write");
}

// ============================================================================
// Additional Combined Tests
// ============================================================================

#[test]
fn test_deref_with_option() {
    #[derive(Deref, DerefMut)]
    struct OptionWrapper(Option<String>);
    
    let mut wrapper = OptionWrapper(Some(String::from("test")));
    
    // Test deref
    if let Some(s) = &*wrapper {
        assert_eq!(s, "test");
    }
    
    // Test deref_mut
    if let Some(s) = &mut *wrapper {
        s.push_str("ing");
    }
    
    assert_eq!(wrapper.0.as_ref().unwrap(), "testing");
}

#[test]
fn test_complex_generic_deref() {
    #[derive(Deref, DerefMut)]
    struct VecWrapper<T>(Vec<T>);
    
    let mut wrapper = VecWrapper(vec![1, 2, 3]);
    wrapper.push(4);
    wrapper.push(5);
    
    assert_eq!(wrapper.len(), 5);
    assert_eq!(wrapper[4], 5);
}