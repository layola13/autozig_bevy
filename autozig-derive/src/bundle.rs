use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

pub fn derive_bundle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = match input.data {
        Data::Struct(ref s) => &s.fields,
        _ => panic!("Bundle can only be derived for structs"),
    };

    let mut component_ids_body = Vec::new();
    let mut get_components_body = Vec::new();

    match fields {
        Fields::Named(ref fields) => {
            for field in &fields.named {
                let ty = &field.ty;
                let ident = &field.ident;
                component_ids_body.push(quote! {
                    <#ty as autozig_ecs::bundle::Bundle>::component_ids()
                });
                get_components_body.push(quote! {
                    autozig_ecs::bundle::Bundle::get_components(&self.#ident)
                });
            }
        }
        Fields::Unnamed(ref fields) => {
            for (i, field) in fields.unnamed.iter().enumerate() {
                let ty = &field.ty;
                let index = syn::Index::from(i);
                component_ids_body.push(quote! {
                    <#ty as autozig_ecs::bundle::Bundle>::component_ids()
                });
                get_components_body.push(quote! {
                    autozig_ecs::bundle::Bundle::get_components(&self.#index)
                });
            }
        }
        Fields::Unit => {}
    }

    let expanded = quote! {
        impl #impl_generics autozig_ecs::bundle::Bundle for #name #ty_generics #where_clause {
            fn component_ids() -> Vec<std::any::TypeId> {
                let mut ids = Vec::new();
                #(
                    ids.extend(#component_ids_body);
                )*
                ids
            }

            fn get_components(&self) -> Vec<(std::any::TypeId, *const u8, usize)> {
                let mut components = Vec::new();
                #(
                    components.extend(#get_components_body);
                )*
                components
            }
        }
    };

    TokenStream::from(expanded)
}
