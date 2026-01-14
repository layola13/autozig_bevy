//! Derive macros for autozig_bevy
//! 
//! This crate provides procedural derive macros for common patterns:
//! - `Deref` and `DerefMut` for newtype wrappers
//! - `EnumVariantMeta` for enum introspection

extern crate proc_macro;

mod derefs;
mod enum_variant_meta;

use proc_macro::TokenStream;

/// Derives the `Deref` trait for a struct.
/// 
/// For single-field structs, automatically uses that field.
/// For multi-field structs, you must mark one field with `#[deref]`.
/// 
/// # Examples
/// 
/// ```ignore
/// #[derive(Deref)]
/// struct Wrapper(String);
/// 
/// #[derive(Deref)]
/// struct MyStruct {
///     #[deref]
///     value: String,
///     other: i32,
/// }
/// ```
#[proc_macro_derive(Deref, attributes(deref))]
pub fn derive_deref(input: TokenStream) -> TokenStream {
    derefs::derive_deref(input)
}

/// Derives the `DerefMut` trait for a struct.
/// 
/// Has the same requirements as `Deref`.
/// 
/// # Examples
/// 
/// ```ignore
/// #[derive(Deref, DerefMut)]
/// struct Wrapper(String);
/// ```
#[proc_macro_derive(DerefMut, attributes(deref))]
pub fn derive_deref_mut(input: TokenStream) -> TokenStream {
    derefs::derive_deref_mut(input)
}

/// Derives enum variant metadata methods.
/// 
/// Generates two methods:
/// - `enum_variant_index(&self) -> usize` - Returns the variant index
/// - `enum_variant_name(&self) -> &'static str` - Returns the variant name
/// 
/// # Examples
/// 
/// ```ignore
/// #[derive(EnumVariantMeta)]
/// enum MyEnum {
///     A,
///     B,
///     C,
/// }
/// 
/// let e = MyEnum::B;
/// assert_eq!(e.enum_variant_index(), 1);
/// assert_eq!(e.enum_variant_name(), "B");
/// ```
#[proc_macro_derive(EnumVariantMeta)]
pub fn derive_enum_variant_meta(input: TokenStream) -> TokenStream {
    enum_variant_meta::derive_enum_variant_meta(input)
}

mod query_data;

/// Derives `QueryData` for a struct.
///
/// This allows the struct to be used as a query parameter in systems.
/// The struct fields must implement `QueryData`.
///
/// # Attributes
///
/// - `#[query_data(mutable)]`: (Optional) Marks the query data as mutable (currently ignored by simple implementation).
///
/// # Examples
///
/// ```ignore
/// #[derive(QueryData)]
/// struct MyQuery {
///     a: &'static ComponentA,
///     b: &'static mut ComponentB,
/// }
/// ```
#[proc_macro_derive(QueryData, attributes(query_data))]
pub fn derive_query_data(input: TokenStream) -> TokenStream {
    query_data::derive_query_data(input)
}