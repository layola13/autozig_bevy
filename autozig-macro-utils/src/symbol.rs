//! Symbol types for efficient string comparison in procedural macros.
//!
//! Symbols are interned string identifiers that can be compared efficiently
//! using pointer equality instead of string comparison.

use std::fmt::{self, Display};
use syn::{Ident, Path};

/// A symbol representing an interned string identifier.
///
/// Symbols are useful in procedural macros for efficiently comparing
/// identifier names without repeated string allocations.
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Symbol(&'static str);

impl Symbol {
    /// Creates a new symbol from a static string.
    pub const fn new(s: &'static str) -> Self {
        Symbol(s)
    }

    /// Returns the string representation of this symbol.
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    /// Checks if this symbol matches the given identifier.
    pub fn matches_ident(self, ident: &Ident) -> bool {
        ident == self.0
    }

    /// Checks if this symbol matches the last segment of a path.
    pub fn matches_path(self, path: &Path) -> bool {
        path.segments.last().map_or(false, |segment| {
            segment.ident == self.0
        })
    }

    /// Checks if this symbol matches any of the given symbols.
    pub fn matches_any(self, symbols: &[Symbol]) -> bool {
        symbols.iter().any(|&s| s == self)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Symbol(\"{}\")", self.0)
    }
}

impl From<&'static str> for Symbol {
    fn from(s: &'static str) -> Self {
        Symbol::new(s)
    }
}

impl PartialEq<str> for Symbol {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Symbol {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for Symbol {
    fn eq(&self, other: &String) -> bool {
        self.0 == other.as_str()
    }
}

impl PartialEq<Ident> for Symbol {
    fn eq(&self, other: &Ident) -> bool {
        self.0 == other.to_string()
    }
}

/// Macro to define commonly used symbols as constants.
///
/// # Examples
///
/// ```ignore
/// symbols! {
///     pub MY_SYMBOL = "my_symbol";
///     pub ANOTHER = "another";
/// }
/// ```
#[macro_export]
macro_rules! symbols {
    ($($(#[$attr:meta])* $vis:vis $name:ident = $value:literal;)*) => {
        $(
            $(#[$attr])*
            $vis const $name: $crate::Symbol = $crate::Symbol::new($value);
        )*
    };
}

// Common symbols used in Bevy macros
symbols! {
    pub CLONE = "Clone";
    pub COPY = "Copy";
    pub DEBUG = "Debug";
    pub DEFAULT = "Default";
    pub DEREF = "Deref";
    pub DEREF_MUT = "DerefMut";
    pub DISPLAY = "Display";
    pub EQ = "Eq";
    pub FROM = "From";
    pub HASH = "Hash";
    pub INTO = "Into";
    pub ORD = "Ord";
    pub PARTIAL_EQ = "PartialEq";
    pub PARTIAL_ORD = "PartialOrd";
    pub SEND = "Send";
    pub SYNC = "Sync";
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_symbol_creation() {
        let sym = Symbol::new("test");
        assert_eq!(sym.as_str(), "test");
    }

    #[test]
    fn test_symbol_equality() {
        let sym1 = Symbol::new("test");
        let sym2 = Symbol::new("test");
        assert_eq!(sym1, sym2);
    }

    #[test]
    fn test_symbol_matches_ident() {
        let sym = Symbol::new("test");
        let ident: Ident = parse_quote!(test);
        assert!(sym.matches_ident(&ident));
    }

    #[test]
    fn test_symbol_matches_path() {
        let sym = Symbol::new("test");
        let path: Path = parse_quote!(module::test);
        assert!(sym.matches_path(&path));
    }

    #[test]
    fn test_symbol_matches_any() {
        let sym = Symbol::new("test");
        let symbols = [Symbol::new("foo"), Symbol::new("test"), Symbol::new("bar")];
        assert!(sym.matches_any(&symbols));
    }

    #[test]
    fn test_predefined_symbols() {
        assert_eq!(CLONE.as_str(), "Clone");
        assert_eq!(DEBUG.as_str(), "Debug");
        assert_eq!(DEFAULT.as_str(), "Default");
    }
}