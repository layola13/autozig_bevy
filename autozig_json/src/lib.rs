//! # AutoZig JSON
//!
//! High-performance JSON parsing library with Zig SIMD backend.
//! Drop-in replacement for serde_json without any serde dependencies.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use autozig_json::{from_str, Value, AutoDeserialize};
//!
//! // Parse JSON string to Value
//! let json = r#"{"name": "AutoZig", "score": 99.5}"#;
//! let value = from_str(json).unwrap();
//! println!("Name: {}", value["name"]);
//!
//! // Or use derive macro for typed parsing
//! #[derive(AutoDeserialize)]
//! struct User {
//!     id: u32,
//!     name: String,
//! }
//!
//! let user: User = autozig_json::parse(r#"{"id": 1, "name": "Alice"}"#).unwrap();
//! ```
//!
//! ## Features
//!
//! - **SIMD Optimization**: Zig backend uses SIMD for fast parsing
//! - **Tape-based Parsing**: Fastest JSON parsing architecture
//! - **Zero Dependencies**: No serde, no syn, no quote
//! - **Ultra-fast Compilation**: Hand-written proc-macro

mod de;
mod error;
mod ser;
pub mod tape;
mod value;

// Re-export public API
pub use de::{from_slice, from_str, Deserializer, Token, TokenType};
pub use error::{Error, Result};
pub use ser::{to_string, to_string_pretty, to_vec, to_vec_pretty};
pub use value::Value;

// Tape API
pub use tape::{
    parse, parse_borrow, ArrayIter, AutoDeserialize, BorrowDeserialize, Node, NodeType, ObjectIter,
    TapeRef,
};

// Re-export derive macros
pub use autozig_json_macro::AutoDeserialize;
pub use autozig_json_macro::AutoBorrowDeserialize;

/// json! macro for constructing Value literals
///
/// # Example
/// ```
/// use autozig_json::json;
///
/// let value = json!({
///     "name": "AutoZig",
///     "version": 1,
///     "features": ["simd", "fast", "safe"]
/// });
/// ```
#[macro_export]
macro_rules! json {
    // null
    (null) => {
        $crate::Value::Null
    };
    // boolean
    (true) => {
        $crate::Value::Bool(true)
    };
    (false) => {
        $crate::Value::Bool(false)
    };
    // array
    ([ $($elem:tt),* $(,)? ]) => {
        $crate::Value::Array(vec![ $( $crate::json!($elem) ),* ])
    };
    // object
    ({ $($key:tt : $value:tt),* $(,)? }) => {
        $crate::Value::Object({
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $crate::json!($value));
            )*
            map
        })
    };
    // string literal
    ($s:literal) => {
        $crate::Value::from($s)
    };
    // expression (for variables)
    ($e:expr) => {
        $crate::Value::from($e)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        let value = from_str("null").unwrap();
        assert!(value.is_null());
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(from_str("true").unwrap(), Value::Bool(true));
        assert_eq!(from_str("false").unwrap(), Value::Bool(false));
    }

    #[test]
    fn test_parse_number() {
        let value = from_str("42").unwrap();
        assert_eq!(value.as_f64(), Some(42.0));

        let value = from_str("-3.14").unwrap();
        assert!((value.as_f64().unwrap() - (-3.14)).abs() < 0.001);
    }

    #[test]
    fn test_parse_string() {
        let value = from_str(r#""hello world""#).unwrap();
        assert_eq!(value.as_str(), Some("hello world"));
    }

    #[test]
    fn test_parse_string_escape() {
        let value = from_str(r#""hello\nworld""#).unwrap();
        assert_eq!(value.as_str(), Some("hello\nworld"));
    }

    #[test]
    fn test_parse_array() {
        let value = from_str("[1, 2, 3]").unwrap();
        let arr = value.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].as_f64(), Some(1.0));
    }

    #[test]
    fn test_parse_object() {
        let value = from_str(r#"{"name": "test", "value": 42}"#).unwrap();
        assert_eq!(value["name"].as_str(), Some("test"));
        assert_eq!(value["value"].as_f64(), Some(42.0));
    }

    #[test]
    fn test_nested() {
        let json = r#"
        {
            "users": [
                {"id": 1, "name": "Alice"},
                {"id": 2, "name": "Bob"}
            ]
        }
        "#;
        let value = from_str(json).unwrap();
        assert_eq!(value["users"][0]["name"].as_str(), Some("Alice"));
        assert_eq!(value["users"][1]["id"].as_f64(), Some(2.0));
    }

    #[test]
    fn test_json_macro() {
        let value = json!({
            "name": "AutoZig",
            "version": 1,
            "active": true,
            "tags": ["fast", "safe"]
        });

        assert_eq!(value["name"].as_str(), Some("AutoZig"));
        assert_eq!(value["active"].as_bool(), Some(true));
    }

    #[test]
    fn test_to_string() {
        let value = json!({
            "name": "test"
        });
        let s = to_string(&value);
        assert!(s.contains("\"name\""));
        assert!(s.contains("\"test\""));
    }
}
