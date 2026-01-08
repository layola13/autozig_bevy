//! Custom parser utilities for procedural macros.

use syn::parse::{Parse, ParseStream, Parser as SynParser};
use syn::punctuated::Punctuated;
use syn::{Result, Token};

/// Creates a parser for a punctuated list terminated by a specific token.
///
/// This is useful for parsing comma-separated lists or other punctuated sequences
/// that end with a specific terminator.
///
/// # Examples
///
/// ```ignore
/// use syn::{Ident, Token};
/// use syn::punctuated::Punctuated;
///
/// // Parse a comma-separated list of identifiers terminated by a semicolon
/// let parser = terminated_parser::<Ident, Token![,], Token![;]>();
/// let result: Punctuated<Ident, Token![,]> = parser.parse_str("foo, bar, baz;")?;
/// ```
pub fn terminated_parser<T, P, E>() -> impl SynParser<Output = Punctuated<T, P>>
where
    T: Parse,
    P: Parse,
    E: Parse,
{
    move |input: ParseStream| {
        let mut items = Punctuated::new();
        
        loop {
            // Check if we've reached the end or terminator
            if input.is_empty() {
                break;
            }
            
            // Try to parse the terminator
            let fork = input.fork();
            if fork.parse::<E>().is_ok() {
                input.parse::<E>()?;
                break;
            }

            // Parse the next item
            items.push_value(input.parse()?);

            // Check if we've reached the terminator after parsing an item
            if input.is_empty() {
                break;
            }
            
            let fork = input.fork();
            if fork.parse::<E>().is_ok() {
                input.parse::<E>()?;
                break;
            }

            // If not at terminator, expect a punctuation
            if !input.is_empty() {
                items.push_punct(input.parse()?);
            }
        }

        Ok(items)
    }
}

/// Parses a comma-separated list of items.
///
/// # Examples
///
/// ```ignore
/// use syn::Ident;
///
/// let result = parse_comma_separated::<Ident>(input)?;
/// ```
pub fn parse_comma_separated<T: Parse>(input: ParseStream) -> Result<Punctuated<T, Token![,]>> {
    Punctuated::parse_terminated(input)
}

/// Parses a comma-separated list that allows a trailing comma.
pub fn parse_comma_separated_trailing<T: Parse>(
    input: ParseStream,
) -> Result<Punctuated<T, Token![,]>> {
    Punctuated::parse_separated_nonempty(input)
}

/// Parses an optional comma-separated list.
///
/// Returns an empty list if the input is empty.
pub fn parse_optional_comma_separated<T: Parse>(
    input: ParseStream,
) -> Result<Punctuated<T, Token![,]>> {
    if input.is_empty() {
        Ok(Punctuated::new())
    } else {
        Punctuated::parse_terminated(input)
    }
}

/// Parses items until a specific token is encountered.
///
/// The terminator token is consumed.
pub fn parse_until<T, E>(input: ParseStream, mut parse_item: impl FnMut(ParseStream) -> Result<T>) -> Result<Vec<T>>
where
    E: Parse,
{
    let mut items = Vec::new();
    
    while !input.is_empty() {
        // Try to parse the terminator
        let fork = input.fork();
        if fork.parse::<E>().is_ok() {
            input.parse::<E>()?;
            break;
        }
        
        items.push(parse_item(input)?);
    }
    
    Ok(items)
}

/// Helper to parse an optional attribute value.
///
/// Returns `None` if the value is not present.
pub fn parse_optional<T: Parse>(input: ParseStream) -> Result<Option<T>> {
    if input.is_empty() {
        Ok(None)
    } else {
        Ok(Some(input.parse()?))
    }
}

/// Parses a parenthesized list of items.
///
/// # Examples
///
/// ```ignore
/// use syn::Ident;
///
/// // Parses: (foo, bar, baz)
/// let result = parse_parenthesized_list::<Ident>(input)?;
/// ```
pub fn parse_parenthesized_list<T: Parse>(input: ParseStream) -> Result<Vec<T>> {
    let content;
    syn::parenthesized!(content in input);
    
    let mut items = Vec::new();
    while !content.is_empty() {
        items.push(content.parse()?);
        
        if !content.is_empty() {
            content.parse::<Token![,]>()?;
        }
    }
    
    Ok(items)
}

/// Parses a bracketed list of items.
///
/// # Examples
///
/// ```ignore
/// use syn::Ident;
///
/// // Parses: [foo, bar, baz]
/// let result = parse_bracketed_list::<Ident>(input)?;
/// ```
pub fn parse_bracketed_list<T: Parse>(input: ParseStream) -> Result<Vec<T>> {
    let content;
    syn::bracketed!(content in input);
    
    let mut items = Vec::new();
    while !content.is_empty() {
        items.push(content.parse()?);
        
        if !content.is_empty() {
            content.parse::<Token![,]>()?;
        }
    }
    
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::Ident;

    #[test]
    fn test_parse_comma_separated() {
        let parser = |input: ParseStream| parse_comma_separated::<Ident>(input);
        let input = quote::quote! { foo, bar, baz };
        let result: Punctuated<Ident, Token![,]> = parser.parse2(input).unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_parse_optional_comma_separated_empty() {
        let input = quote::quote! {};
        let parser = |input: ParseStream| parse_optional_comma_separated::<Ident>(input);
        let result: Punctuated<Ident, Token![,]> = parser.parse2(input).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_optional_comma_separated_with_items() {
        let input = quote::quote! { foo, bar };
        let parser = |input: ParseStream| parse_optional_comma_separated::<Ident>(input);
        let result: Punctuated<Ident, Token![,]> = parser.parse2(input).unwrap();
        assert_eq!(result.len(), 2);
    }
}