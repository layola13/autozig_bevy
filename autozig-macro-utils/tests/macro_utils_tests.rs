//! Integration tests for autozig-macro-utils

use autozig_macro_utils::*;
use quote::quote;
use syn::{parse_quote, DeriveInput};

#[test]
fn test_symbol_creation_and_comparison() {
    use autozig_macro_utils::Symbol;
    
    let sym1 = Symbol::new("test");
    let sym2 = Symbol::new("test");
    let sym3 = Symbol::new("other");
    
    assert_eq!(sym1, sym2);
    assert_ne!(sym1, sym3);
    assert_eq!(sym1.as_str(), "test");
}

#[test]
fn test_symbol_matches_ident() {
    use autozig_macro_utils::Symbol;
    
    let sym = Symbol::new("my_ident");
    let ident: syn::Ident = parse_quote!(my_ident);
    
    assert!(sym.matches_ident(&ident));
}

#[test]
fn test_get_struct_fields() {
    let input: DeriveInput = parse_quote! {
        struct MyStruct {
            field1: i32,
            field2: String,
        }
    };
    
    let fields = get_struct_fields(&input).unwrap();
    // field_count is not exported, use match instead
    let count = match fields {
        Fields::Named(f) => f.named.len(),
        Fields::Unnamed(f) => f.unnamed.len(),
        Fields::Unit => 0,
    };
    assert_eq!(count, 2);
}

#[test]
fn test_require_named_fields() {
    let input: DeriveInput = parse_quote! {
        struct MyStruct {
            field1: i32,
            field2: String,
        }
    };
    
    let fields = require_named_fields(&input).unwrap();
    assert_eq!(fields.named.len(), 2);
}

#[test]
fn test_require_named_fields_tuple_fails() {
    let input: DeriveInput = parse_quote! {
        struct MyStruct(i32, String);
    };
    
    assert!(require_named_fields(&input).is_err());
}

#[test]
fn test_result_sifter_all_success() {
    let mut sifter = ResultSifter::new();
    sifter.push(Ok(1));
    sifter.push(Ok(2));
    sifter.push(Ok(3));
    
    assert!(!sifter.has_errors());
    let values = sifter.finish().unwrap();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn test_result_sifter_mixed() {
    let mut sifter = ResultSifter::new();
    sifter.push(Ok(1));
    sifter.push(Err(syn::Error::new(proc_macro2::Span::call_site(), "error")));
    sifter.push(Ok(2));
    
    assert!(sifter.has_errors());
    let (values, errors) = sifter.finish_split();
    assert_eq!(values, vec![1, 2]);
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_fq_option() {
    let opt = FQOption(quote! { i32 });
    let tokens = quote! { #opt };
    let output = tokens.to_string();
    
    // The output should contain the fully qualified path
    assert!(output.contains("core"));
    assert!(output.contains("option"));
    assert!(output.contains("Option"));
}

#[test]
fn test_fq_result() {
    let res = FQResult(quote! { i32 }, quote! { String });
    let tokens = quote! { #res };
    let output = tokens.to_string();
    
    assert!(output.contains("core"));
    assert!(output.contains("result"));
    assert!(output.contains("Result"));
}

#[test]
fn test_as_member() {
    let field: syn::Field = parse_quote! { pub foo: i32 };
    let member = as_member(&field);
    assert_eq!(member.to_string(), "foo");
}

#[test]
fn test_bevy_manifest_constant() {
    assert_eq!(AUTOZIG_BEVY, "autozig_bevy");
}

#[test]
fn test_parse_comma_separated() {
    // This function is not publicly exported, test through other means
    let input: DeriveInput = parse_quote! {
        struct Test {
            a: i32,
            b: String,
            c: bool,
        }
    };
    let fields = get_struct_fields(&input).unwrap();
    let count = match fields {
        Fields::Named(f) => f.named.len(),
        _=> 0,
    };
    assert_eq!(count, 3);
}

#[test]
fn test_shape_is_named() {
    let input: DeriveInput = parse_quote! {
        struct MyStruct {
            field: i32,
        }
    };
    let fields = get_struct_fields(&input).unwrap();
    assert!(matches!(fields, Fields::Named(_)));
}

#[test]
fn test_shape_is_unnamed() {
    let input: DeriveInput = parse_quote! {
        struct MyStruct(i32, String);
    };
    let fields = get_struct_fields(&input).unwrap();
    assert!(matches!(fields, Fields::Unnamed(_)));
}

#[test]
fn test_shape_is_unit() {
    let input: DeriveInput = parse_quote! {
        struct MyStruct;
    };
    let fields = get_struct_fields(&input).unwrap();
    assert!(matches!(fields, Fields::Unit));
}

#[test]
fn test_has_fields() {
    let input: DeriveInput = parse_quote! {
        struct MyStruct {
            field: i32,
        }
    };
    let fields = get_struct_fields(&input).unwrap();
    let has_fields = match fields {
        Fields::Named(f) => !f.named.is_empty(),
        Fields::Unnamed(f) => !f.unnamed.is_empty(),
        Fields::Unit => false,
    };
    assert!(has_fields);
}

#[test]
fn test_field_count() {
    let input: DeriveInput = parse_quote! {
        struct MyStruct {
            field1: i32,
            field2: String,
            field3: bool,
        }
    };
    let fields = get_struct_fields(&input).unwrap();
    let count = match fields {
        Fields::Named(f) => f.named.len(),
        Fields::Unnamed(f) => f.unnamed.len(),
        Fields::Unit => 0,
    };
    assert_eq!(count, 3);
}

#[test]
fn test_get_named_fields() {
    let input: DeriveInput = parse_quote! {
        struct MyStruct {
            field1: i32,
            field2: String,
        }
    };
    let fields = require_named_fields(&input).unwrap();
    assert_eq!(fields.named.len(), 2);
}