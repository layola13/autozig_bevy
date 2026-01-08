//! Utilities for generating field member access expressions.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Index, Member};

/// Converts a field to a member access token.
///
/// For named fields, this returns the field name.
/// For unnamed fields (tuple struct), this returns the field index.
///
/// # Examples
///
/// ```ignore
/// // For a named field `foo`:
/// let member = as_member(&field); // Generates: foo
///
/// // For an unnamed field at index 0:
/// let member = as_member(&field); // Generates: 0
/// ```
pub fn as_member(field: &Field) -> TokenStream {
    match &field.ident {
        Some(ident) => quote! { #ident },
        None => {
            // For unnamed fields, we need to find the index
            // This is typically handled by the caller who knows the index
            // For safety, we'll generate a compilation error if used incorrectly
            quote! {
                compile_error!("Cannot determine member for unnamed field without index. Use as_member_with_index instead.")
            }
        }
    }
}

/// Converts a field with a known index to a member access token.
///
/// This is useful for tuple structs where fields don't have names.
///
/// # Examples
///
/// ```ignore
/// // For a tuple struct field at index 0:
/// let member = as_member_with_index(&field, 0); // Generates: 0
/// ```
pub fn as_member_with_index(field: &Field, index: usize) -> TokenStream {
    match &field.ident {
        Some(ident) => quote! { #ident },
        None => {
            let idx = Index::from(index);
            quote! { #idx }
        }
    }
}

/// Generates a member access expression for a field.
///
/// # Examples
///
/// ```ignore
/// // For a named field `foo`:
/// let access = member_access(&field); // Generates: self.foo
///
/// // For an unnamed field at index 0:
/// let access = member_access_with_index(&field, 0); // Generates: self.0
/// ```
pub fn member_access(field: &Field) -> TokenStream {
    let member = as_member(field);
    quote! { self.#member }
}

/// Generates a member access expression for a field with a known index.
pub fn member_access_with_index(field: &Field, index: usize) -> TokenStream {
    let member = as_member_with_index(field, index);
    quote! { self.#member }
}

/// Creates a `Member` from a field.
///
/// This is useful when you need a `syn::Member` type for code generation.
pub fn field_to_member(field: &Field, index: usize) -> Member {
    match &field.ident {
        Some(ident) => Member::Named(ident.clone()),
        None => Member::Unnamed(Index::from(index)),
    }
}

/// Generates a destructuring pattern for struct fields.
///
/// # Examples
///
/// ```ignore
/// // For named fields:
/// let pattern = destructure_fields(&fields); // Generates: { field1, field2, ... }
///
/// // For tuple fields:
/// let pattern = destructure_fields(&fields); // Generates: (field0, field1, ...)
/// ```
pub fn destructure_fields(fields: &syn::Fields) -> TokenStream {
    match fields {
        syn::Fields::Named(fields) => {
            let names = fields.named.iter().map(|f| &f.ident);
            quote! { { #(#names),* } }
        }
        syn::Fields::Unnamed(fields) => {
            let indices = (0..fields.unnamed.len()).map(|i| {
                let ident = quote::format_ident!("field{}", i);
                ident
            });
            quote! { ( #(#indices),* ) }
        }
        syn::Fields::Unit => quote! {},
    }
}

/// Generates field identifiers for unnamed fields.
///
/// This creates temporary identifiers like `field0`, `field1`, etc.
pub fn unnamed_field_idents(count: usize) -> Vec<syn::Ident> {
    (0..count)
        .map(|i| quote::format_ident!("field{}", i))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_as_member_named() {
        let field: Field = parse_quote! { pub foo: i32 };
        let member = as_member(&field);
        assert_eq!(member.to_string(), "foo");
    }

    #[test]
    fn test_as_member_with_index_unnamed() {
        let field: Field = parse_quote! { i32 };
        let member = as_member_with_index(&field, 0);
        assert_eq!(member.to_string(), "0");
    }

    #[test]
    fn test_as_member_with_index_named() {
        let field: Field = parse_quote! { pub foo: i32 };
        let member = as_member_with_index(&field, 0);
        assert_eq!(member.to_string(), "foo");
    }

    #[test]
    fn test_field_to_member_named() {
        let field: Field = parse_quote! { pub foo: i32 };
        let member = field_to_member(&field, 0);
        match member {
            Member::Named(ident) => assert_eq!(ident.to_string(), "foo"),
            _ => panic!("Expected named member"),
        }
    }

    #[test]
    fn test_field_to_member_unnamed() {
        let field: Field = parse_quote! { i32 };
        let member = field_to_member(&field, 2);
        match member {
            Member::Unnamed(index) => assert_eq!(index.index, 2),
            _ => panic!("Expected unnamed member"),
        }
    }

    #[test]
    fn test_unnamed_field_idents() {
        let idents = unnamed_field_idents(3);
        assert_eq!(idents.len(), 3);
        assert_eq!(idents[0].to_string(), "field0");
        assert_eq!(idents[1].to_string(), "field1");
        assert_eq!(idents[2].to_string(), "field2");
    }
}