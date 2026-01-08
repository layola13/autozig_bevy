//! Deref and DerefMut derive implementations

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Error, Fields, Index, Meta,
};

/// Derive the `Deref` trait for a struct
pub fn derive_deref(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let result = derive_deref_inner(&ast);
    
    match result {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Derive the `DerefMut` trait for a struct
pub fn derive_deref_mut(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let result = derive_deref_mut_inner(&ast);
    
    match result {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_deref_inner(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    
    let (field_type, field_access) = get_deref_field(ast)?;
    
    Ok(quote! {
        impl #impl_generics ::core::ops::Deref for #ident #ty_generics #where_clause {
            type Target = #field_type;
            
            fn deref(&self) -> &Self::Target {
                #field_access
            }
        }
    })
}

fn derive_deref_mut_inner(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    
    let (_field_type, field_access) = get_deref_field(ast)?;
    
    // Convert the immutable field access to mutable
    let access_str = field_access.to_string();
    let field_access_mut: proc_macro2::TokenStream = access_str
        .replace("& self", "&mut self")
        .parse()
        .unwrap();
    
    Ok(quote! {
        impl #impl_generics ::core::ops::DerefMut for #ident #ty_generics #where_clause {
            fn deref_mut(&mut self) -> &mut Self::Target {
                #field_access_mut
            }
        }
    })
}

/// Find the field to deref to
fn get_deref_field(ast: &DeriveInput) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
    let data = match &ast.data {
        Data::Struct(data) => data,
        _ => {
            return Err(Error::new_spanned(
                ast,
                "Deref can only be derived for structs",
            ))
        }
    };
    
    match &data.fields {
        Fields::Named(fields) => {
            // For named fields, look for #[deref] attribute or use single field
            let mut deref_field = None;
            
            for field in &fields.named {
                let has_deref_attr = field.attrs.iter().any(|attr| {
                    if let Meta::Path(path) = &attr.meta {
                        path.is_ident("deref")
                    } else {
                        false
                    }
                });
                
                if has_deref_attr {
                    if deref_field.is_some() {
                        return Err(Error::new_spanned(
                            field,
                            "Multiple fields marked with #[deref]",
                        ));
                    }
                    deref_field = Some(field);
                }
            }
            
            // If no #[deref] attribute, use single field if there's only one
            if deref_field.is_none() {
                if fields.named.len() == 1 {
                    deref_field = fields.named.first();
                } else {
                    return Err(Error::new_spanned(
                        &fields.named,
                        "Multiple fields require one to be marked with #[deref]",
                    ));
                }
            }
            
            let field = deref_field.unwrap();
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            
            Ok((quote! { #field_type }, quote! { &self.#field_name }))
        }
        Fields::Unnamed(fields) => {
            // For tuple structs
            let mut deref_field = None;
            let mut deref_index = 0;
            
            for (i, field) in fields.unnamed.iter().enumerate() {
                let has_deref_attr = field.attrs.iter().any(|attr| {
                    if let Meta::Path(path) = &attr.meta {
                        path.is_ident("deref")
                    } else {
                        false
                    }
                });
                
                if has_deref_attr {
                    if deref_field.is_some() {
                        return Err(Error::new_spanned(
                            field,
                            "Multiple fields marked with #[deref]",
                        ));
                    }
                    deref_field = Some(field);
                    deref_index = i;
                }
            }
            
            // If no #[deref] attribute, use single field if there's only one
            if deref_field.is_none() {
                if fields.unnamed.len() == 1 {
                    deref_field = fields.unnamed.first();
                    deref_index = 0;
                } else {
                    return Err(Error::new_spanned(
                        &fields.unnamed,
                        "Multiple fields require one to be marked with #[deref]",
                    ));
                }
            }
            
            let field = deref_field.unwrap();
            let field_type = &field.ty;
            let index = Index::from(deref_index);
            
            Ok((quote! { #field_type }, quote! { &self.#index }))
        }
        Fields::Unit => {
            Err(Error::new_spanned(
                ast,
                "Cannot derive Deref for unit structs",
            ))
        }
    }
}