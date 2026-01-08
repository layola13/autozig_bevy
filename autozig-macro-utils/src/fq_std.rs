//! Fully qualified standard library types for use in procedural macros.
//!
//! When generating code in procedural macros, it's important to use fully qualified
//! paths to avoid ambiguity and ensure the generated code works regardless of what
//! the user has imported.

use proc_macro2::TokenStream;
use quote::quote;

/// Generates a fully qualified `Option` type.
///
/// # Examples
///
/// ```ignore
/// let option_type = FQOption(quote! { i32 });
/// // Generates: ::core::option::Option<i32>
/// ```
pub struct FQOption(pub TokenStream);

impl quote::ToTokens for FQOption {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner = &self.0;
        tokens.extend(quote! {
            ::core::option::Option<#inner>
        });
    }
}

/// Generates a fully qualified `Result` type.
///
/// # Examples
///
/// ```ignore
/// let result_type = FQResult(quote! { i32 }, quote! { String });
/// // Generates: ::core::result::Result<i32, String>
/// ```
pub struct FQResult(pub TokenStream, pub TokenStream);

impl quote::ToTokens for FQResult {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ok = &self.0;
        let err = &self.1;
        tokens.extend(quote! {
            ::core::result::Result<#ok, #err>
        });
    }
}

/// Generates a fully qualified `Box` type.
pub struct FQBox(pub TokenStream);

impl quote::ToTokens for FQBox {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let inner = &self.0;
        tokens.extend(quote! {
            ::std::boxed::Box<#inner>
        });
    }
}

/// Generates a fully qualified `Default` trait bound.
pub struct FQDefault;

impl quote::ToTokens for FQDefault {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            ::core::default::Default
        });
    }
}

/// Generates a fully qualified `Clone` trait bound.
pub struct FQClone;

impl quote::ToTokens for FQClone {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            ::core::clone::Clone
        });
    }
}

/// Generates a fully qualified `Send` trait bound.
pub struct FQSend;

impl quote::ToTokens for FQSend {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            ::core::marker::Send
        });
    }
}

/// Generates a fully qualified `Sync` trait bound.
pub struct FQSync;

impl quote::ToTokens for FQSync {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            ::core::marker::Sync
        });
    }
}

/// Generates a fully qualified `Any` trait bound.
pub struct FQAny;

impl quote::ToTokens for FQAny {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            ::core::any::Any
        });
    }
}

/// Helper macro to create fully qualified paths.
///
/// # Examples
///
/// ```ignore
/// let path = fq_path!(std::vec::Vec);
/// // Generates: ::std::vec::Vec
/// ```
#[macro_export]
macro_rules! fq_path {
    ($($path:tt)*) => {
        quote::quote! { ::$($path)* }
    };
}

/// Common fully qualified standard types as constants.
pub mod std_types {
    use super::*;

    /// Fully qualified `String` type.
    pub fn string() -> TokenStream {
        quote! { ::std::string::String }
    }

    /// Fully qualified `Vec` type.
    pub fn vec(inner: TokenStream) -> TokenStream {
        quote! { ::std::vec::Vec<#inner> }
    }

    /// Fully qualified `HashMap` type.
    pub fn hash_map(key: TokenStream, value: TokenStream) -> TokenStream {
        quote! { ::std::collections::HashMap<#key, #value> }
    }

    /// Fully qualified `HashSet` type.
    pub fn hash_set(inner: TokenStream) -> TokenStream {
        quote! { ::std::collections::HashSet<#inner> }
    }

    /// Fully qualified `Cow` type.
    pub fn cow(inner: TokenStream) -> TokenStream {
        quote! { ::std::borrow::Cow<#inner> }
    }

    /// Fully qualified `Arc` type.
    pub fn arc(inner: TokenStream) -> TokenStream {
        quote! { ::std::sync::Arc<#inner> }
    }

    /// Fully qualified `Rc` type.
    pub fn rc(inner: TokenStream) -> TokenStream {
        quote! { ::std::rc::Rc<#inner> }
    }

    /// Fully qualified `Cell` type.
    pub fn cell(inner: TokenStream) -> TokenStream {
        quote! { ::std::cell::Cell<#inner> }
    }

    /// Fully qualified `RefCell` type.
    pub fn ref_cell(inner: TokenStream) -> TokenStream {
        quote! { ::std::cell::RefCell<#inner> }
    }

    /// Fully qualified `Mutex` type.
    pub fn mutex(inner: TokenStream) -> TokenStream {
        quote! { ::std::sync::Mutex<#inner> }
    }

    /// Fully qualified `RwLock` type.
    pub fn rw_lock(inner: TokenStream) -> TokenStream {
        quote! { ::std::sync::RwLock<#inner> }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fq_option() {
        let opt = FQOption(quote! { i32 });
        let tokens = quote! { #opt };
        assert_eq!(tokens.to_string(), ":: core :: option :: Option < i32 >");
    }

    #[test]
    fn test_fq_result() {
        let res = FQResult(quote! { i32 }, quote! { String });
        let tokens = quote! { #res };
        assert_eq!(tokens.to_string(), ":: core :: result :: Result < i32 , String >");
    }

    #[test]
    fn test_fq_box() {
        let b = FQBox(quote! { i32 });
        let tokens = quote! { #b };
        assert_eq!(tokens.to_string(), ":: std :: boxed :: Box < i32 >");
    }

    #[test]
    fn test_fq_default() {
        let d = FQDefault;
        let tokens = quote! { #d };
        assert_eq!(tokens.to_string(), ":: core :: default :: Default");
    }

    #[test]
    fn test_fq_clone() {
        let c = FQClone;
        let tokens = quote! { #c };
        assert_eq!(tokens.to_string(), ":: core :: clone :: Clone");
    }

    #[test]
    fn test_std_types_string() {
        let s = std_types::string();
        assert_eq!(s.to_string(), ":: std :: string :: String");
    }

    #[test]
    fn test_std_types_vec() {
        let v = std_types::vec(quote! { i32 });
        assert_eq!(v.to_string(), ":: std :: vec :: Vec < i32 >");
    }
}