use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

pub fn derive_query_data(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let vis = &ast.vis;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => return syn::Error::new_spanned(ast, "QueryData only supports named fields").to_compile_error().into(),
        },
        _ => return syn::Error::new_spanned(ast, "QueryData only supports structs").to_compile_error().into(),
    };
    
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_vis: Vec<_> = fields.iter().map(|f| &f.vis).collect();
    
    let tuple_type = quote! { ( #(#field_types),* ) };
    let fetch_name = quote::format_ident!("{}Fetch", name);
    let item_name = quote::format_ident!("{}Item", name);
    
    quote! {
        const _: () = {
            use autozig_ecs::query::{QueryData, WorldQuery, Fetch, ReadOnlyQueryData, ReadOnlyWorldQuery, FilteredAccess};
            use autozig_ecs::world::World;
            use autozig_ecs::world::unsafe_world_cell::UnsafeWorldCell;
            use autozig_ecs::archetype::Archetype;
            use autozig_ecs::storage::Table;
            use autozig_ecs::component::ComponentId;
            use autozig_ecs::change_detection::Tick;
            use autozig_ecs::table::TableRow;
            use autozig_ecs::entity::Entity;

            #[allow(dead_code)]
            #vis struct #item_name<'w> #where_clause {
                #( #field_vis #field_names: <#field_types as QueryData>::Item<'w> ),*
            }

            impl #impl_generics QueryData for #name #ty_generics #where_clause {
                type Item<'w> = #item_name<'w>; 
                type State = <#tuple_type as QueryData>::State;
                type Fetch<'w> = #fetch_name<'w> #ty_generics;
                type ReadOnly = <#tuple_type as QueryData>::ReadOnly;
                const IS_READ_ONLY: bool = <#tuple_type as QueryData>::IS_READ_ONLY;
                
                fn init_state(world: &mut World) -> Self::State {
                    <#tuple_type as QueryData>::init_state(world)
                }
                
                unsafe fn init_fetch<'w>(
                    world: UnsafeWorldCell<'w>,
                    state: &Self::State,
                    last_run: Tick,
                    this_run: Tick,
                ) -> Self::Fetch<'w> {
                    #fetch_name {
                        inner: <#tuple_type as QueryData>::init_fetch(world, state, last_run, this_run),
                        _marker: std::marker::PhantomData,
                    }
                }
                
                unsafe fn set_archetype<'w>(
                    fetch: &mut Self::Fetch<'w>,
                    state: &Self::State,
                    archetype: &Archetype,
                    table: &Table,
                ) {
                    <#tuple_type as QueryData>::set_archetype(&mut fetch.inner, state, archetype, table)
                }
                
                unsafe fn set_table<'w>(
                    fetch: &mut Self::Fetch<'w>,
                    state: &Self::State,
                    table: &Table,
                ) {
                    <#tuple_type as QueryData>::set_table(&mut fetch.inner, state, table)
                }
                
                fn get_access(state: &Self::State) -> autozig_ecs::query::Access {
                    <#tuple_type as QueryData>::get_access(state)
                }
                
                fn update_component_access(state: &Self::State, access: &mut FilteredAccess) {
                    <#tuple_type as QueryData>::update_component_access(state, access)
                }
                
                fn matches_component_set(state: &Self::State, set: &[ComponentId]) -> bool {
                    <#tuple_type as QueryData>::matches_component_set(state, set)
                }
            }

            pub struct #fetch_name<'w> #ty_generics #where_clause {
                inner: <#tuple_type as QueryData>::Fetch<'w>,
                _marker: std::marker::PhantomData<&'w ()>,
            }

            impl<'w> #impl_generics Fetch<'w> for #fetch_name<'w> #ty_generics #where_clause {
                type Item = #item_name<'w>;
                type State = <#tuple_type as QueryData>::State;

                fn init(state: &Self::State, world: UnsafeWorldCell<'w>, last_run: Tick, this_run: Tick) -> Self {
                    unsafe {
                        Self { 
                            inner: <#tuple_type as QueryData>::init_fetch(world, state, last_run, this_run),
                            _marker: std::marker::PhantomData,
                        }
                    }
                }

                unsafe fn set_table(&mut self, state: &Self::State, table: &Table) {
                    <#tuple_type as QueryData>::set_table(&mut self.inner, state, table);
                }

                unsafe fn set_archetype(&mut self, state: &Self::State, archetype: &Archetype, table: &Table) {
                    <#tuple_type as QueryData>::set_archetype(&mut self.inner, state, archetype, table);
                }

                fn fetch(&mut self, entity: Entity, index: usize) -> Self::Item {
                    use autozig_ecs::query::Fetch;
                    let tuple = self.inner.fetch(entity, index);
                    #[allow(unused_parens)]
                    let (#(#field_names),*) = tuple;
                    #item_name {
                        #( #field_names ),*
                    }
                }

                fn matches_archetype(state: &Self::State, archetype: &Archetype) -> bool {
                    <#tuple_type as QueryData>::matches_component_set(state, &archetype.components())
                }
            }
        };
    }.into()
}
