//! EnumVariantMeta derive implementation

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields};

/// Derive the `EnumVariantMeta` methods for an enum
pub fn derive_enum_variant_meta(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let result = derive_enum_variant_meta_inner(&ast);
    
    match result {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_enum_variant_meta_inner(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let enum_data = match &ast.data {
        Data::Enum(data) => data,
        _ => {
            return Err(Error::new_spanned(
                ast,
                "EnumVariantMeta can only be derived for enums",
            ))
        }
    };
    
    let ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    
    // Generate match arms for variant_index
    let index_arms = enum_data.variants.iter().enumerate().map(|(index, variant)| {
        let variant_ident = &variant.ident;
        match &variant.fields {
            Fields::Named(_) => quote! { Self::#variant_ident { .. } => #index },
            Fields::Unnamed(_) => quote! { Self::#variant_ident(..) => #index },
            Fields::Unit => quote! { Self::#variant_ident => #index },
        }
    });
    
    // Generate match arms for variant_name
    let name_arms = enum_data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let variant_name = variant_ident.to_string();
        match &variant.fields {
            Fields::Named(_) => quote! { Self::#variant_ident { .. } => #variant_name },
            Fields::Unnamed(_) => quote! { Self::#variant_ident(..) => #variant_name },
            Fields::Unit => quote! { Self::#variant_ident => #variant_name },
        }
    });
    
    Ok(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            /// Returns the index of the current enum variant.
            ///
            /// The index corresponds to the order in which the variant is defined,
            /// starting from 0.
            pub fn enum_variant_index(&self) -> usize {
                match self {
                    #(#index_arms,)*
                }
            }
            
            /// Returns the name of the current enum variant as a static string.
            pub fn enum_variant_name(&self) -> &'static str {
                match self {
                    #(#name_arms,)*
                }
            }
        }
    })
}