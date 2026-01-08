//! Attribute parsing utilities for procedural macros.

use syn::spanned::Spanned;
use syn::{Attribute, Lit, LitBool, LitStr, Meta, Result};

/// Extracts a string literal from an attribute.
///
/// # Examples
///
/// ```ignore
/// // For an attribute like #[my_attr(name = "value")]
/// let name = get_lit_str("name", &attr)?;
/// ```
pub fn get_lit_str(attr_name: &str, attr: &Attribute) -> Result<Option<LitStr>> {
    if !attr.path().is_ident(attr_name) {
        return Ok(None);
    }

    match &attr.meta {
        Meta::NameValue(meta) => {
            if let Expr::Lit(expr_lit) = &meta.value {
                if let Lit::Str(lit) = &expr_lit.lit {
                    return Ok(Some(lit.clone()));
                }
            }
            Err(syn::Error::new_spanned(
                attr,
                format!("expected string literal for `{}`", attr_name),
            ))
        }
        Meta::List(meta) => {
            meta.parse_args::<LitStr>()
                .map(Some)
                .or_else(|_| {
                    Err(syn::Error::new_spanned(
                        attr,
                        format!("expected string literal for `{}`", attr_name),
                    ))
                })
        }
        _ => Err(syn::Error::new_spanned(
            attr,
            format!("expected name-value or list attribute for `{}`", attr_name),
        )),
    }
}

/// Extracts a boolean literal from an attribute.
///
/// # Examples
///
/// ```ignore
/// // For an attribute like #[my_attr(flag = true)]
/// let flag = get_lit_bool("flag", &attr)?;
/// ```
pub fn get_lit_bool(attr_name: &str, attr: &Attribute) -> Result<Option<LitBool>> {
    if !attr.path().is_ident(attr_name) {
        return Ok(None);
    }

    match &attr.meta {
        Meta::NameValue(meta) => {
            if let Expr::Lit(expr_lit) = &meta.value {
                if let Lit::Bool(lit) = &expr_lit.lit {
                    return Ok(Some(lit.clone()));
                }
            }
            Err(syn::Error::new_spanned(
                attr,
                format!("expected boolean literal for `{}`", attr_name),
            ))
        }
        Meta::List(meta) => {
            meta.parse_args::<LitBool>()
                .map(Some)
                .or_else(|_| {
                    Err(syn::Error::new_spanned(
                        attr,
                        format!("expected boolean literal for `{}`", attr_name),
                    ))
                })
        }
        Meta::Path(_) => {
            // For attributes like #[flag], treat as true
            Ok(Some(LitBool::new(true, attr.span())))
        }
    }
}

// Need to import Expr for compilation
use syn::Expr;

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_get_lit_str() {
        let attr: Attribute = parse_quote!(#[name = "test_value"]);
        let result = get_lit_str("name", &attr).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().value(), "test_value");
    }

    #[test]
    fn test_get_lit_bool() {
        let attr: Attribute = parse_quote!(#[flag = true]);
        let result = get_lit_bool("flag", &attr).unwrap();
        assert!(result.is_some());
        assert!(result.unwrap().value);
    }

    #[test]
    fn test_get_lit_bool_path() {
        let attr: Attribute = parse_quote!(#[flag]);
        let result = get_lit_bool("flag", &attr).unwrap();
        assert!(result.is_some());
        assert!(result.unwrap().value);
    }
}