//! Utilities for validating and working with struct shapes in procedural macros.

use syn::{Data, DeriveInput, Field, Fields as SynFields, Result};

/// Re-export of syn::Fields for convenience.
pub use syn::Fields;

/// Named fields (struct with named fields).
pub type FieldsNamed = syn::FieldsNamed;

/// Unnamed fields (tuple struct).
pub type FieldsUnnamed = syn::FieldsUnnamed;

/// Gets the fields from a struct's data.
///
/// # Errors
///
/// Returns an error if the input is not a struct.
///
/// # Examples
///
/// ```ignore
/// let fields = get_struct_fields(&derive_input)?;
/// ```
pub fn get_struct_fields(input: &DeriveInput) -> Result<&SynFields> {
    match &input.data {
        Data::Struct(data) => Ok(&data.fields),
        Data::Enum(_) => Err(syn::Error::new_spanned(
            input,
            "expected struct, found enum",
        )),
        Data::Union(_) => Err(syn::Error::new_spanned(
            input,
            "expected struct, found union",
        )),
    }
}

/// Requires that a struct has named fields and returns them.
///
/// # Errors
///
/// Returns an error if the input is not a struct with named fields.
///
/// # Examples
///
/// ```ignore
/// let fields = require_named_fields(&derive_input)?;
/// for field in &fields.named {
///     // Process each field
/// }
/// ```
pub fn require_named_fields(input: &DeriveInput) -> Result<&FieldsNamed> {
    match get_struct_fields(input)? {
        SynFields::Named(fields) => Ok(fields),
        SynFields::Unnamed(_) => Err(syn::Error::new_spanned(
            input,
            "expected struct with named fields, found tuple struct",
        )),
        SynFields::Unit => Err(syn::Error::new_spanned(
            input,
            "expected struct with named fields, found unit struct",
        )),
    }
}

/// Requires that a struct has unnamed fields and returns them.
///
/// # Errors
///
/// Returns an error if the input is not a tuple struct.
pub fn require_unnamed_fields(input: &DeriveInput) -> Result<&FieldsUnnamed> {
    match get_struct_fields(input)? {
        SynFields::Unnamed(fields) => Ok(fields),
        SynFields::Named(_) => Err(syn::Error::new_spanned(
            input,
            "expected tuple struct, found struct with named fields",
        )),
        SynFields::Unit => Err(syn::Error::new_spanned(
            input,
            "expected tuple struct, found unit struct",
        )),
    }
}

/// Gets all fields from a struct as a slice.
pub fn get_fields(fields: &SynFields) -> Vec<&Field> {
    match fields {
        SynFields::Named(fields) => fields.named.iter().collect(),
        SynFields::Unnamed(fields) => fields.unnamed.iter().collect(),
        SynFields::Unit => Vec::new(),
    }
}

/// Checks if fields are named (struct with named fields).
pub fn is_named(fields: &SynFields) -> bool {
    matches!(fields, SynFields::Named(_))
}

/// Checks if fields are unnamed (tuple struct).
pub fn is_unnamed(fields: &SynFields) -> bool {
    matches!(fields, SynFields::Unnamed(_))
}

/// Checks if fields are unit (no fields).
pub fn is_unit(fields: &SynFields) -> bool {
    matches!(fields, SynFields::Unit)
}

/// Gets the number of fields in a struct.
pub fn field_count(fields: &SynFields) -> usize {
    match fields {
        SynFields::Named(fields) => fields.named.len(),
        SynFields::Unnamed(fields) => fields.unnamed.len(),
        SynFields::Unit => 0,
    }
}

/// Checks if a struct has at least one field.
pub fn has_fields(fields: &SynFields) -> bool {
    field_count(fields) > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_get_struct_fields_named() {
        let input: DeriveInput = parse_quote! {
            struct MyStruct {
                field1: i32,
                field2: String,
            }
        };
        let fields = get_struct_fields(&input).unwrap();
        assert!(is_named(fields));
        assert_eq!(field_count(fields), 2);
    }

    #[test]
    fn test_get_struct_fields_unnamed() {
        let input: DeriveInput = parse_quote! {
            struct MyStruct(i32, String);
        };
        let fields = get_struct_fields(&input).unwrap();
        assert!(is_unnamed(fields));
        assert_eq!(field_count(fields), 2);
    }

    #[test]
    fn test_get_struct_fields_unit() {
        let input: DeriveInput = parse_quote! {
            struct MyStruct;
        };
        let fields = get_struct_fields(&input).unwrap();
        assert!(is_unit(fields));
        assert_eq!(field_count(fields), 0);
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
    fn test_require_unnamed_fields() {
        let input: DeriveInput = parse_quote! {
            struct MyStruct(i32, String);
        };
        let fields = require_unnamed_fields(&input).unwrap();
        assert_eq!(fields.unnamed.len(), 2);
    }

    #[test]
    fn test_get_fields() {
        let input: DeriveInput = parse_quote! {
            struct MyStruct {
                field1: i32,
                field2: String,
            }
        };
        let syn_fields = get_struct_fields(&input).unwrap();
        let fields = get_fields(syn_fields);
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_has_fields() {
        let input: DeriveInput = parse_quote! {
            struct MyStruct {
                field1: i32,
            }
        };
        let fields = get_struct_fields(&input).unwrap();
        assert!(has_fields(fields));
    }
}