//! JSON Deserializer using Zig SIMD backend

use std::collections::HashMap;

use autozig::include_zig;

use crate::error::{Error, Result};
use crate::value::Value;

// Import Zig scanner functions
include_zig!("src/json_tape.zig", {
    fn next_token(ptr: *const u8, total_len: usize, cursor: usize) -> Token;
    fn parse_number_f64(ptr: *const u8, len: usize) -> f64;
    fn parse_number_i64(ptr: *const u8, len: usize) -> i64;
});

/// Token type returned by Zig scanner
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Error = 0,
    ObjectStart = 1,
    ObjectEnd = 2,
    ArrayStart = 3,
    ArrayEnd = 4,
    String = 5,
    Number = 6,
    True = 7,
    False = 8,
    Null = 9,
    Colon = 10,
    Comma = 11,
    Eof = 12,
}

/// Token struct matching Zig's extern struct
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub kind: TokenType,
    pub start: usize,
    pub len: usize,
    pub next_cursor: usize,
}

/// JSON Deserializer with Zig SIMD backend
pub struct Deserializer<'a> {
    input: &'a [u8],
    cursor: usize,
}

impl<'a> Deserializer<'a> {
    /// Create a new deserializer from a byte slice
    pub fn new(input: &'a [u8]) -> Self {
        Self { input, cursor: 0 }
    }

    /// Create a new deserializer from a string slice
    pub fn from_str(input: &'a str) -> Self {
        Self::new(input.as_bytes())
    }

    /// Get the next token from the Zig scanner
    fn next_token(&mut self) -> Token {
        let token = next_token(self.input.as_ptr(), self.input.len(), self.cursor);
        self.cursor = token.next_cursor;
        token
    }

    /// Peek at the next token without consuming it
    fn peek_token(&self) -> Token {
        next_token(self.input.as_ptr(), self.input.len(), self.cursor)
    }

    /// Parse any JSON value
    pub fn parse_value(&mut self) -> Result<Value> {
        let token = self.next_token();
        self.parse_value_from_token(token)
    }

    /// Parse a value given an already-consumed token
    fn parse_value_from_token(&mut self, token: Token) -> Result<Value> {
        match token.kind {
            TokenType::Null => Ok(Value::Null),
            TokenType::True => Ok(Value::Bool(true)),
            TokenType::False => Ok(Value::Bool(false)),
            TokenType::Number => self.parse_number(token),
            TokenType::String => self.parse_string(token),
            TokenType::ArrayStart => self.parse_array(),
            TokenType::ObjectStart => self.parse_object(),
            TokenType::Eof => Err(Error::UnexpectedEof),
            TokenType::Error => Err(Error::SyntaxError {
                position: token.start,
                message: "invalid token".to_string(),
            }),
            _ => Err(Error::SyntaxError {
                position: token.start,
                message: format!("unexpected token: {:?}", token.kind),
            }),
        }
    }

    /// Parse a number using Zig's number parser
    fn parse_number(&self, token: Token) -> Result<Value> {
        let ptr = unsafe { self.input.as_ptr().add(token.start) };
        let num = parse_number_f64(ptr, token.len);
        Ok(Value::Number(num))
    }

    /// Parse a string, handling escape sequences
    fn parse_string(&self, token: Token) -> Result<Value> {
        let slice = &self.input[token.start..token.start + token.len];
        
        // Check if we need to process escapes
        if slice.contains(&b'\\') {
            let s = self.unescape_string(slice)?;
            Ok(Value::String(s))
        } else {
            // Fast path: no escapes
            let s = std::str::from_utf8(slice)
                .map_err(|_| Error::InvalidUtf8 { position: token.start })?;
            Ok(Value::String(s.to_string()))
        }
    }

    /// Unescape a JSON string with escape sequences
    fn unescape_string(&self, slice: &[u8]) -> Result<String> {
        let mut result = String::with_capacity(slice.len());
        let mut i = 0;

        while i < slice.len() {
            if slice[i] == b'\\' && i + 1 < slice.len() {
                i += 1;
                match slice[i] {
                    b'"' => result.push('"'),
                    b'\\' => result.push('\\'),
                    b'/' => result.push('/'),
                    b'b' => result.push('\x08'),
                    b'f' => result.push('\x0c'),
                    b'n' => result.push('\n'),
                    b'r' => result.push('\r'),
                    b't' => result.push('\t'),
                    b'u' => {
                        if i + 4 >= slice.len() {
                            return Err(Error::InvalidEscape { position: i });
                        }
                        let hex = &slice[i + 1..i + 5];
                        let hex_str = std::str::from_utf8(hex)
                            .map_err(|_| Error::InvalidEscape { position: i })?;
                        let code = u32::from_str_radix(hex_str, 16)
                            .map_err(|_| Error::InvalidEscape { position: i })?;
                        if let Some(c) = char::from_u32(code) {
                            result.push(c);
                        } else {
                            return Err(Error::InvalidEscape { position: i });
                        }
                        i += 4;
                    }
                    _ => {
                        return Err(Error::InvalidEscape { position: i });
                    }
                }
                i += 1;
            } else {
                result.push(slice[i] as char);
                i += 1;
            }
        }

        Ok(result)
    }

    /// Parse a JSON array
    fn parse_array(&mut self) -> Result<Value> {
        let mut arr = Vec::new();

        loop {
            let token = self.peek_token();
            
            if token.kind == TokenType::ArrayEnd {
                self.next_token(); // consume ]
                return Ok(Value::Array(arr));
            }

            if token.kind == TokenType::Eof {
                return Err(Error::UnexpectedEof);
            }

            // Parse element
            let value = self.parse_value()?;
            arr.push(value);

            // Check for comma or end
            let next = self.peek_token();
            if next.kind == TokenType::Comma {
                self.next_token(); // consume comma
            } else if next.kind != TokenType::ArrayEnd {
                return Err(Error::ExpectedToken {
                    expected: ", or ]",
                    found: token_kind_name(next.kind),
                    position: next.start,
                });
            }
        }
    }

    /// Parse a JSON object
    fn parse_object(&mut self) -> Result<Value> {
        let mut map = HashMap::new();

        loop {
            let token = self.peek_token();
            
            if token.kind == TokenType::ObjectEnd {
                self.next_token(); // consume }
                return Ok(Value::Object(map));
            }

            if token.kind == TokenType::Eof {
                return Err(Error::UnexpectedEof);
            }

            // Parse key
            let key_token = self.next_token();
            if key_token.kind != TokenType::String {
                return Err(Error::ExpectedToken {
                    expected: "string key",
                    found: token_kind_name(key_token.kind),
                    position: key_token.start,
                });
            }
            let key = self.extract_string(key_token)?;

            // Expect colon
            let colon = self.next_token();
            if colon.kind != TokenType::Colon {
                return Err(Error::ExpectedToken {
                    expected: ":",
                    found: token_kind_name(colon.kind),
                    position: colon.start,
                });
            }

            // Parse value
            let value = self.parse_value()?;
            map.insert(key, value);

            // Check for comma or end
            let next = self.peek_token();
            if next.kind == TokenType::Comma {
                self.next_token(); // consume comma
            } else if next.kind != TokenType::ObjectEnd {
                return Err(Error::ExpectedToken {
                    expected: ", or }",
                    found: token_kind_name(next.kind),
                    position: next.start,
                });
            }
        }
    }

    /// Extract string content from a token
    fn extract_string(&self, token: Token) -> Result<String> {
        let slice = &self.input[token.start..token.start + token.len];
        if slice.contains(&b'\\') {
            self.unescape_string(slice)
        } else {
            std::str::from_utf8(slice)
                .map(|s| s.to_string())
                .map_err(|_| Error::InvalidUtf8 { position: token.start })
        }
    }
}

/// Get human-readable name for token type
fn token_kind_name(kind: TokenType) -> &'static str {
    match kind {
        TokenType::Error => "error",
        TokenType::ObjectStart => "{",
        TokenType::ObjectEnd => "}",
        TokenType::ArrayStart => "[",
        TokenType::ArrayEnd => "]",
        TokenType::String => "string",
        TokenType::Number => "number",
        TokenType::True => "true",
        TokenType::False => "false",
        TokenType::Null => "null",
        TokenType::Colon => ":",
        TokenType::Comma => ",",
        TokenType::Eof => "end of input",
    }
}

// ============================================================================
// Public API Functions
// ============================================================================

/// Parse a JSON string into a Value
/// 
/// # Example
/// ```
/// use autozig_json::from_str;
/// 
/// let json = r#"{"name": "AutoZig", "version": 1}"#;
/// let value = from_str(json).unwrap();
/// assert_eq!(value["name"].as_str(), Some("AutoZig"));
/// ```
pub fn from_str(s: &str) -> Result<Value> {
    let mut de = Deserializer::from_str(s);
    de.parse_value()
}

/// Parse a JSON byte slice into a Value
pub fn from_slice(slice: &[u8]) -> Result<Value> {
    let mut de = Deserializer::new(slice);
    de.parse_value()
}
