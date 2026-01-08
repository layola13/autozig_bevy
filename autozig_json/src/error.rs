//! Error types for autozig_json

use std::fmt;

/// Error type for JSON parsing
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// Unexpected end of input
    UnexpectedEof,
    /// Invalid JSON syntax at position
    SyntaxError { position: usize, message: String },
    /// Expected a specific token type
    ExpectedToken { expected: &'static str, found: &'static str, position: usize },
    /// Invalid escape sequence
    InvalidEscape { position: usize },
    /// Invalid number format
    InvalidNumber { position: usize },
    /// Invalid UTF-8 sequence
    InvalidUtf8 { position: usize },
    /// Key not found in object
    KeyNotFound { key: String },
    /// Type mismatch
    TypeMismatch { expected: &'static str, found: &'static str },
    /// Custom error message
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedEof => write!(f, "unexpected end of input"),
            Error::SyntaxError { position, message } => {
                write!(f, "syntax error at position {}: {}", position, message)
            }
            Error::ExpectedToken { expected, found, position } => {
                write!(f, "expected {} but found {} at position {}", expected, found, position)
            }
            Error::InvalidEscape { position } => {
                write!(f, "invalid escape sequence at position {}", position)
            }
            Error::InvalidNumber { position } => {
                write!(f, "invalid number format at position {}", position)
            }
            Error::InvalidUtf8 { position } => {
                write!(f, "invalid UTF-8 sequence at position {}", position)
            }
            Error::KeyNotFound { key } => {
                write!(f, "key not found: {}", key)
            }
            Error::TypeMismatch { expected, found } => {
                write!(f, "type mismatch: expected {} but found {}", expected, found)
            }
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

/// Result type alias for autozig_json
pub type Result<T> = std::result::Result<T, Error>;
