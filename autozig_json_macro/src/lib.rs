//! Hand-written proc-macro for AutoZig JSON
//!
//! ZERO dependencies - only uses std proc_macro.
//! This is the ultimate "造轮子" approach for speed.

use proc_macro::{Delimiter, Group, Ident, TokenStream, TokenTree};

/// Derive macro for automatic JSON deserialization
///
/// # Example
/// ```ignore
/// #[derive(AutoDeserialize)]
/// struct User {
///     id: u32,
///     name: String,
///     active: bool,
/// }
/// ```
#[proc_macro_derive(AutoDeserialize)]
pub fn derive_auto_deserialize(input: TokenStream) -> TokenStream {
    match parse_and_generate(input) {
        Ok(output) => output,
        Err(err) => {
            // Generate compile error
            format!("compile_error!(\"{}\");", err).parse().unwrap()
        }
    }
}

fn parse_and_generate(input: TokenStream) -> Result<TokenStream, String> {
    let mut tokens = input.into_iter().peekable();
    let mut struct_name = String::new();
    let mut fields: Vec<(String, String)> = Vec::new();

    // Skip attributes, visibility, etc. until we find "struct"
    while let Some(token) = tokens.next() {
        if let TokenTree::Ident(ident) = &token {
            let name = ident.to_string();
            if name == "struct" {
                // Next token is struct name
                if let Some(TokenTree::Ident(name_ident)) = tokens.next() {
                    struct_name = name_ident.to_string();
                }
                break;
            }
        }
    }

    if struct_name.is_empty() {
        return Err("Could not find struct name".to_string());
    }

    // Find the braces containing fields
    while let Some(token) = tokens.next() {
        if let TokenTree::Group(group) = token {
            if group.delimiter() == Delimiter::Brace {
                parse_struct_fields(group, &mut fields)?;
                break;
            }
        }
    }

    if fields.is_empty() {
        return Err("No fields found in struct".to_string());
    }

    // Generate the implementation code
    generate_impl(&struct_name, &fields)
}

fn parse_struct_fields(group: Group, fields: &mut Vec<(String, String)>) -> Result<(), String> {
    let mut iter = group.stream().into_iter().peekable();

    loop {
        // Skip any attributes on field (like #[...])
        while let Some(TokenTree::Punct(p)) = iter.peek() {
            if p.as_char() == '#' {
                iter.next(); // Skip #
                if let Some(TokenTree::Group(_)) = iter.next() {
                    // Skip the attribute group
                }
            } else {
                break;
            }
        }

        // Skip 'pub' if present
        if let Some(TokenTree::Ident(ident)) = iter.peek() {
            if ident.to_string() == "pub" {
                iter.next();
                // Skip optional (crate) or (super) etc
                if let Some(TokenTree::Group(_)) = iter.peek() {
                    iter.next();
                }
            }
        }

        // Field name
        let field_name = match iter.next() {
            Some(TokenTree::Ident(ident)) => ident.to_string(),
            None => break, // End of fields
            _ => continue, // Skip unexpected tokens
        };

        // Colon
        match iter.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ':' => {}
            _ => continue,
        }

        // Type - can be simple ident or complex path
        let field_type = parse_type(&mut iter)?;

        fields.push((field_name, field_type));

        // Skip comma if present
        if let Some(TokenTree::Punct(p)) = iter.peek() {
            if p.as_char() == ',' {
                iter.next();
            }
        }
    }

    Ok(())
}

fn parse_type(
    iter: &mut std::iter::Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<String, String> {
    let mut type_str = String::new();

    loop {
        match iter.peek() {
            Some(TokenTree::Ident(ident)) => {
                type_str.push_str(&ident.to_string());
                iter.next();
            }
            Some(TokenTree::Punct(p)) => {
                let ch = p.as_char();
                if ch == ',' || ch == '}' {
                    break;
                } else if ch == ':' {
                    // Path separator ::
                    type_str.push_str("::");
                    iter.next();
                    if let Some(TokenTree::Punct(_)) = iter.peek() {
                        iter.next(); // Skip second :
                    }
                } else if ch == '<' || ch == '>' {
                    type_str.push(ch);
                    iter.next();
                } else {
                    break;
                }
            }
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Bracket => {
                // Array type like [u8; 32]
                type_str.push_str(&format!("{}", g));
                iter.next();
            }
            _ => break,
        }
    }

    if type_str.is_empty() {
        return Err("Could not parse type".to_string());
    }

    Ok(type_str)
}

fn generate_impl(struct_name: &str, fields: &[(String, String)]) -> Result<TokenStream, String> {
    // Generate field initializations
    let mut field_inits = String::new();
    let mut field_matches = String::new();
    let mut field_unwraps = String::new();

    for (name, ty) in fields {
        // Declare as Option
        field_inits.push_str(&format!("let mut _field_{}: Option<{}> = None;\n", name, ty));

        // Determine if type is Option (special handling for unwrap)
        let is_option = ty.starts_with("Option<");
        
        // All types now use the recursive AutoDeserialize trait
        // This handles: basic types, Vec<T>, Option<T>, nested structs
        let extract = format!(
            "_field_{name} = Some(<{ty} as autozig_json::AutoDeserialize>::from_tape(__json, __tape, val_idx)?);",
            name = name,
            ty = ty
        );

        field_matches.push_str(&format!(
            r#""{name}" => {{ {extract} }},"#,
            name = name,
            extract = extract
        ));

        // Generate field unwrap - Option fields don't need error
        if is_option {
            field_unwraps.push_str(&format!(
                r#"{name}: _field_{name}.flatten(),"#,
                name = name
            ));
        } else {
            field_unwraps.push_str(&format!(
                r#"{name}: _field_{name}.ok_or_else(|| autozig_json::Error::KeyNotFound {{ key: "{name}".to_string() }})?,"#,
                name = name
            ));
        }
    }

    let code = format!(
        r#"
impl autozig_json::AutoDeserialize for {struct_name} {{
    fn from_tape(
        __json: &str,
        __tape: &autozig_json::TapeRef,
        __root_idx: usize,
    ) -> autozig_json::Result<Self> {{
        {field_inits}
        
        let root = __tape.get(__root_idx);
        if root.tag != autozig_json::NodeType::Object {{
            return Err(autozig_json::Error::TypeMismatch {{
                expected: "object",
                found: "non-object",
            }});
        }}
        
        // Iterate through key-value pairs
        let mut key_idx = root.child as usize;
        while key_idx != 0 {{
            let key_node = __tape.get(key_idx);
            let val_idx = key_node.child as usize;
            let val_node = __tape.get(val_idx);
            
            if let Some(key) = __tape.get_str(__json, key_node) {{
                match key {{
                    {field_matches}
                    _ => {{}} // Ignore unknown fields
                }}
            }}
            
            key_idx = key_node.next as usize;
        }}
        
        Ok({struct_name} {{
            {field_unwraps}
        }})
    }}
}}
"#,
        struct_name = struct_name,
        field_inits = field_inits,
        field_matches = field_matches,
        field_unwraps = field_unwraps
    );

    code.parse()
        .map_err(|e| format!("Failed to parse generated code: {:?}", e))
}

// ============================================================================
// Zero-Copy BorrowDeserialize Derive Macro
// ============================================================================

/// Derive macro for zero-copy JSON deserialization
/// 
/// This macro generates BorrowDeserialize implementation for structs
/// that contain `&'a str` fields, allowing zero-copy parsing.
/// 
/// # Example
/// ```ignore
/// #[derive(AutoBorrowDeserialize)]
/// struct User<'a> {
///     name: &'a str,  // Zero-copy!
///     id: u32,
/// }
/// ```
#[proc_macro_derive(AutoBorrowDeserialize)]
pub fn derive_auto_borrow_deserialize(input: TokenStream) -> TokenStream {
    match parse_and_generate_borrow(input) {
        Ok(output) => output,
        Err(err) => {
            format!("compile_error!(\"{}\");", err).parse().unwrap()
        }
    }
}

fn parse_and_generate_borrow(input: TokenStream) -> Result<TokenStream, String> {
    let mut tokens = input.into_iter().peekable();
    let mut struct_name = String::new();
    let mut fields: Vec<(String, String)> = Vec::new();

    // Skip until we find "struct"
    while let Some(token) = tokens.next() {
        if let TokenTree::Ident(ident) = &token {
            let name = ident.to_string();
            if name == "struct" {
                if let Some(TokenTree::Ident(name_ident)) = tokens.next() {
                    struct_name = name_ident.to_string();
                }
                break;
            }
        }
    }

    if struct_name.is_empty() {
        return Err("Could not find struct name".to_string());
    }

    // Skip optional lifetime <'a>
    while let Some(token) = tokens.next() {
        if let TokenTree::Group(group) = &token {
            if group.delimiter() == Delimiter::Brace {
                parse_struct_fields(group.clone(), &mut fields)?;
                break;
            }
        }
    }

    if fields.is_empty() {
        return Err("No fields found in struct".to_string());
    }

    generate_borrow_impl(&struct_name, &fields)
}

fn generate_borrow_impl(struct_name: &str, fields: &[(String, String)]) -> Result<TokenStream, String> {
    let mut field_inits = String::new();
    let mut field_matches = String::new();
    let mut field_unwraps = String::new();

    for (name, ty) in fields {
        field_inits.push_str(&format!("let mut _field_{}: Option<{}> = None;\n", name, ty));

        let is_option = ty.starts_with("Option<");
        
        // Use BorrowDeserialize for all types
        let extract = format!(
            "_field_{name} = Some(<{ty} as autozig_json::BorrowDeserialize>::borrow_from_tape(__json, __tape, val_idx)?);",
            name = name,
            ty = ty
        );

        field_matches.push_str(&format!(
            r#""{name}" => {{ {extract} }},"#,
            name = name,
            extract = extract
        ));

        if is_option {
            field_unwraps.push_str(&format!(
                r#"{name}: _field_{name}.flatten(),"#,
                name = name
            ));
        } else {
            field_unwraps.push_str(&format!(
                r#"{name}: _field_{name}.ok_or_else(|| autozig_json::Error::KeyNotFound {{ key: "{name}".to_string() }})?,\"#,
                name = name
            ));
        }
    }

    let code = format!(
        r#"
impl<'a> autozig_json::BorrowDeserialize<'a> for {struct_name}<'a> {{
    fn borrow_from_tape(
        __json: &'a str,
        __tape: &autozig_json::TapeRef<'a>,
        __root_idx: usize,
    ) -> autozig_json::Result<Self> {{
        {field_inits}
        
        let root = __tape.get(__root_idx);
        if root.tag != autozig_json::NodeType::Object {{
            return Err(autozig_json::Error::TypeMismatch {{
                expected: "object",
                found: "non-object",
            }});
        }}
        
        let mut key_idx = root.child as usize;
        while key_idx != 0 {{
            let key_node = __tape.get(key_idx);
            let val_idx = key_node.child as usize;
            
            if let Some(key) = __tape.get_str(__json, key_node) {{
                match key {{
                    {field_matches}
                    _ => {{}}
                }}
            }}
            
            key_idx = key_node.next as usize;
        }}
        
        Ok({struct_name} {{
            {field_unwraps}
        }})
    }}
}}
"#,
        struct_name = struct_name,
        field_inits = field_inits,
        field_matches = field_matches,
        field_unwraps = field_unwraps
    );

    code.parse()
        .map_err(|e| format!("Failed to parse generated code: {:?}", e))
}
