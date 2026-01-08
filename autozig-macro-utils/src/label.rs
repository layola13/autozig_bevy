//! Label trait derivation utilities and identifier conflict detection.
//!
//! This module provides utilities for deriving Label-like traits, which are
//! commonly used in Bevy for scheduling and identification purposes.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Result};

use crate::bevy_manifest::BevyManifest;

/// Defines a label derivation macro implementation.
///
/// This generates the boilerplate code for deriving a Label trait.
///
/// # Arguments
///
/// * `input` - The derive input
/// * `trait_path` - The path to the Label trait to implement
/// * `crate_name` - The name of the crate containing the Label trait
///
/// # Examples
///
/// ```ignore
/// define_label(input, "ComponentLabel", "ecs")
/// ```
pub fn define_label(
    input: &DeriveInput,
    trait_path: &str,
    crate_name: &str,
) -> Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Get the path to the trait from the manifest
    let manifest = BevyManifest::default();
    let bevy_crate_path = manifest.get_path(crate_name);
    let _trait_ident = syn::Ident::new(trait_path, proc_macro2::Span::call_site());

    // Check for identifier conflicts
    validate_label_name(ident)?;

    let trait_path_tokens: TokenStream = format!("{}::{}", bevy_crate_path, trait_path)
        .parse()
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), format!("Failed to parse trait path: {}", e)))?;

    Ok(quote! {
        impl #impl_generics #trait_path_tokens for #ident #ty_generics #where_clause {
            fn dyn_clone(&self) -> ::std::boxed::Box<dyn #trait_path_tokens> {
                ::std::boxed::Box::new(::core::clone::Clone::clone(self))
            }
        }
    })
}

/// Validates a label name to detect common issues.
///
/// This checks for:
/// - Reserved keywords
/// - Common naming conflicts
/// - Invalid identifiers
fn validate_label_name(ident: &Ident) -> Result<()> {
    let name = ident.to_string();

    // Check for reserved keywords
    if is_reserved_keyword(&name) {
        return Err(syn::Error::new_spanned(
            ident,
            format!("'{}' is a reserved keyword and cannot be used as a label", name),
        ));
    }

    // Check for common conflicts
    if is_common_conflict(&name) {
        return Err(syn::Error::new_spanned(
            ident,
            format!(
                "'{}' may conflict with commonly used types. Consider using a more specific name.",
                name
            ),
        ));
    }

    Ok(())
}

/// Checks if a name is a reserved Rust keyword.
fn is_reserved_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
    )
}

/// Checks if a name commonly conflicts with standard types.
fn is_common_conflict(name: &str) -> bool {
    matches!(
        name,
        "String" | "Vec" | "Option" | "Result" | "Box" | "HashMap" | "HashSet" | "Error"
    )
}

/// Generates a Label implementation with additional metadata.
///
/// This is an extended version that includes additional trait bounds and implementations.
pub fn define_label_with_metadata(
    input: &DeriveInput,
    trait_path: &str,
    crate_name: &str,
    additional_bounds: &[TokenStream],
) -> Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let manifest = BevyManifest::default();
    let bevy_crate_path = manifest.get_path(crate_name);

    validate_label_name(ident)?;

    let trait_path_tokens: TokenStream = format!("{}::{}", bevy_crate_path, trait_path)
        .parse()
        .map_err(|e| syn::Error::new(proc_macro2::Span::call_site(), format!("Failed to parse trait path: {}", e)))?;

    let bounds = if !additional_bounds.is_empty() {
        quote! { + #(#additional_bounds)+* }
    } else {
        quote! {}
    };

    Ok(quote! {
        impl #impl_generics #trait_path_tokens for #ident #ty_generics #where_clause {
            fn dyn_clone(&self) -> ::std::boxed::Box<dyn #trait_path_tokens #bounds> {
                ::std::boxed::Box::new(::core::clone::Clone::clone(self))
            }
        }
    })
}

/// Helper to generate a Label trait implementation with common traits.
pub fn derive_simple_label(input: &DeriveInput, trait_path: &str, crate_name: &str) -> Result<TokenStream> {
    let base_impl = define_label(input, trait_path, crate_name)?;
    
    // Add automatic derives for common traits if they're not already present
    let _ident = &input.ident;
    let _has_clone = has_derive_attribute(input, "Clone");
    let _has_copy = has_derive_attribute(input, "Copy");
    let _has_debug = has_derive_attribute(input, "Debug");

    let additional_impls = quote! {
        // Note: Clone must be derived by the user or implemented manually
    };

    Ok(quote! {
        #base_impl
        #additional_impls
    })
}

/// Checks if a derive input has a specific derive attribute.
fn has_derive_attribute(input: &DeriveInput, name: &str) -> bool {
    input.attrs.iter().any(|attr| {
        if attr.path().is_ident("derive") {
            if let syn::Meta::List(meta_list) = &attr.meta {
                return meta_list.tokens.to_string().contains(name);
            }
        }
        false
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_is_reserved_keyword() {
        assert!(is_reserved_keyword("if"));
        assert!(is_reserved_keyword("for"));
        assert!(is_reserved_keyword("while"));
        assert!(!is_reserved_keyword("MyLabel"));
    }

    #[test]
    fn test_is_common_conflict() {
        assert!(is_common_conflict("String"));
        assert!(is_common_conflict("Vec"));
        assert!(!is_common_conflict("MyLabel"));
    }

    #[test]
    fn test_validate_label_name_valid() {
        let ident: Ident = parse_quote!(MyLabel);
        assert!(validate_label_name(&ident).is_ok());
    }

    #[test]
    fn test_validate_label_name_keyword() {
        let ident: Ident = syn::parse_str("r#for").unwrap();
        // Raw identifiers bypass the keyword check
        assert!(validate_label_name(&ident).is_ok());
    }

    #[test]
    fn test_has_derive_attribute() {
        let input: DeriveInput = parse_quote! {
            #[derive(Clone, Debug)]
            struct MyLabel;
        };
        assert!(has_derive_attribute(&input, "Clone"));
        assert!(has_derive_attribute(&input, "Debug"));
        assert!(!has_derive_attribute(&input, "Copy"));
    }
}