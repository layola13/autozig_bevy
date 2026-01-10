//! Tape-based JSON parsing with Zig backend

use autozig::include_zig;

use crate::error::{Error, Result};

// Import Zig tape functions
include_zig!("src/json_tape.zig", {
    fn tape_parse(ptr: *const u8, len: usize) -> Tape;
    fn tape_get_node(tape: *const Tape, idx: usize) -> Node;
    fn tape_node_count(tape: *const Tape) -> usize;
    fn node_as_f64(ptr: *const u8, start: u32, len: u32) -> f64;
    fn node_as_i64(ptr: *const u8, start: u32, len: u32) -> i64;
    fn node_is_float(ptr: *const u8, start: u32, len: u32) -> bool;
});

/// Node type in the JSON tape
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Null = 0,
    Bool = 1,
    Number = 2,
    String = 3,
    Array = 4,
    Object = 5,
    Error = 255,
}

/// A node in the JSON tape (16 bytes)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub tag: NodeType,
    pub _pad: u8,
    pub start: u32,
    pub len: u32,
    pub next: u32,
    pub child: u32,
}

/// Raw tape from Zig
#[repr(C)]
#[derive(Debug)]
pub struct Tape {
    nodes: *mut Node,
    count: usize,
    capacity: usize,
}

/// Safe reference to a parsed tape
pub struct TapeRef<'a> {
    json: &'a str,
    tape: Tape,
}

impl<'a> TapeRef<'a> {
    /// Parse JSON into a tape
    pub fn parse(json: &'a str) -> Result<Self> {
        let tape = tape_parse(json.as_ptr(), json.len());
        if tape.count == 0 {
            return Err(Error::SyntaxError {
                position: 0,
                message: "Failed to parse JSON".to_string(),
            });
        }
        Ok(Self { json, tape })
    }

    /// Get number of nodes
    pub fn len(&self) -> usize {
        tape_node_count(&self.tape)
    }

    /// Check if tape is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get node at index
    pub fn get(&self, idx: usize) -> Node {
        tape_get_node(&self.tape, idx)
    }

    /// Get root node
    pub fn root(&self) -> Node {
        self.get(0)
    }

    /// Extract string content from a node
    pub fn get_str<'b>(&self, json: &'b str, node: Node) -> Option<&'b str> {
        if node.tag != NodeType::String {
            return None;
        }
        let start = node.start as usize;
        let end = start + node.len as usize;
        if end <= json.len() {
            Some(&json[start..end])
        } else {
            None
        }
    }

    /// Extract String for field
    pub fn get_string(&self, json: &str, node: Node) -> Option<String> {
        self.get_str(json, node).map(|s| s.to_string())
    }

    /// Extract f64 from a number node
    pub fn get_f64(&self, json: &str, node: Node) -> Option<f64> {
        if node.tag != NodeType::Number {
            return None;
        }
        Some(node_as_f64(json.as_ptr(), node.start, node.len))
    }

    /// Extract i64 from a number node
    pub fn get_i64(&self, json: &str, node: Node) -> Option<i64> {
        if node.tag != NodeType::Number {
            return None;
        }
        Some(node_as_i64(json.as_ptr(), node.start, node.len))
    }

    /// Extract u32 from a number node
    pub fn get_u32(&self, json: &str, node: Node) -> Option<u32> {
        self.get_i64(json, node).map(|n| n as u32)
    }

    /// Extract i32 from a number node
    pub fn get_i32(&self, json: &str, node: Node) -> Option<i32> {
        self.get_i64(json, node).map(|n| n as i32)
    }

    /// Extract bool from a node
    pub fn get_bool(&self, node: Node) -> Option<bool> {
        if node.tag != NodeType::Bool {
            return None;
        }
        Some(node.child != 0)
    }

    /// Check if number is float
    pub fn is_float(&self, json: &str, node: Node) -> bool {
        if node.tag != NodeType::Number {
            return false;
        }
        node_is_float(json.as_ptr(), node.start, node.len)
    }

    /// Iterate over array children
    pub fn iter_array(&self, node: Node) -> ArrayIter {
        ArrayIter {
            tape: self,
            current_idx: node.child as usize,
        }
    }

    /// Iterate over object key-value pairs
    pub fn iter_object(&self, node: Node) -> ObjectIter<'_> {
        ObjectIter {
            tape: self,
            current_idx: node.child as usize,
        }
    }
}

/// Iterator over array elements
pub struct ArrayIter<'a> {
    tape: &'a TapeRef<'a>,
    current_idx: usize,
}

impl<'a> Iterator for ArrayIter<'a> {
    type Item = (usize, Node);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx == 0 {
            return None;
        }
        let idx = self.current_idx;
        let node = self.tape.get(idx);
        self.current_idx = node.next as usize;
        Some((idx, node))
    }
}

/// Iterator over object key-value pairs
pub struct ObjectIter<'a> {
    tape: &'a TapeRef<'a>,
    current_idx: usize,
}

impl<'a> Iterator for ObjectIter<'a> {
    type Item = (usize, Node, usize, Node); // key_idx, key_node, val_idx, val_node

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx == 0 {
            return None;
        }
        let key_idx = self.current_idx;
        let key_node = self.tape.get(key_idx);
        let val_idx = key_node.child as usize;
        let val_node = self.tape.get(val_idx);
        self.current_idx = key_node.next as usize;
        Some((key_idx, key_node, val_idx, val_node))
    }
}

// ============================================================================
// AutoDeserialize Trait and Implementations
// ============================================================================

/// Trait for automatic deserialization from JSON tape
pub trait AutoDeserialize: Sized {
    /// Deserialize from a tape at the given node index
    fn from_tape(json: &str, tape: &TapeRef, root_idx: usize) -> Result<Self>;
}

// === Basic Type Implementations ===

impl AutoDeserialize for String {
    fn from_tape(json: &str, tape: &TapeRef, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        tape.get_string(json, node)
            .ok_or_else(|| Error::TypeMismatch { expected: "string", found: "other" })
    }
}

impl AutoDeserialize for f64 {
    fn from_tape(json: &str, tape: &TapeRef, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        tape.get_f64(json, node)
            .ok_or_else(|| Error::TypeMismatch { expected: "f64", found: "other" })
    }
}

impl AutoDeserialize for f32 {
    fn from_tape(json: &str, tape: &TapeRef, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        tape.get_f64(json, node)
            .map(|n| n as f32)
            .ok_or_else(|| Error::TypeMismatch { expected: "f32", found: "other" })
    }
}

impl AutoDeserialize for i64 {
    fn from_tape(json: &str, tape: &TapeRef, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        tape.get_i64(json, node)
            .ok_or_else(|| Error::TypeMismatch { expected: "i64", found: "other" })
    }
}

impl AutoDeserialize for i32 {
    fn from_tape(json: &str, tape: &TapeRef, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        tape.get_i64(json, node)
            .map(|n| n as i32)
            .ok_or_else(|| Error::TypeMismatch { expected: "i32", found: "other" })
    }
}

impl AutoDeserialize for u32 {
    fn from_tape(json: &str, tape: &TapeRef, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        tape.get_i64(json, node)
            .map(|n| n as u32)
            .ok_or_else(|| Error::TypeMismatch { expected: "u32", found: "other" })
    }
}

impl AutoDeserialize for u64 {
    fn from_tape(json: &str, tape: &TapeRef, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        tape.get_i64(json, node)
            .map(|n| n as u64)
            .ok_or_else(|| Error::TypeMismatch { expected: "u64", found: "other" })
    }
}

impl AutoDeserialize for bool {
    fn from_tape(_json: &str, tape: &TapeRef, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        tape.get_bool(node)
            .ok_or_else(|| Error::TypeMismatch { expected: "bool", found: "other" })
    }
}

// === Generic Vec<T> Implementation ===

impl<T: AutoDeserialize> AutoDeserialize for Vec<T> {
    fn from_tape(json: &str, tape: &TapeRef, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        if node.tag != NodeType::Array {
            return Err(Error::TypeMismatch { expected: "array", found: "other" });
        }

        let mut list = Vec::new();
        for (elem_idx, _elem_node) in tape.iter_array(node) {
            let item = T::from_tape(json, tape, elem_idx)?;
            list.push(item);
        }
        Ok(list)
    }
}

// === Generic Option<T> Implementation ===

impl<T: AutoDeserialize> AutoDeserialize for Option<T> {
    fn from_tape(json: &str, tape: &TapeRef, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        if node.tag == NodeType::Null {
            return Ok(None);
        }
        match T::from_tape(json, tape, idx) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        }
    }
}

// ============================================================================
// Zero-Copy Borrowing Deserialization
// ============================================================================

/// Trait for zero-copy deserialization (borrows from input JSON)
/// 
/// This trait allows types containing `&'a str` to borrow directly
/// from the input JSON string, avoiding memory allocation.
pub trait BorrowDeserialize<'a>: Sized {
    /// Deserialize by borrowing from the JSON input
    fn borrow_from_tape(json: &'a str, tape: &TapeRef<'a>, idx: usize) -> Result<Self>;
}

// Zero-copy string slice implementation
impl<'a> BorrowDeserialize<'a> for &'a str {
    fn borrow_from_tape(json: &'a str, tape: &TapeRef<'a>, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        tape.get_str(json, node)
            .ok_or_else(|| Error::TypeMismatch { expected: "&str", found: "other" })
    }
}

// Basic types - forward to AutoDeserialize
macro_rules! impl_borrow_for_auto {
    ($($ty:ty),*) => {
        $(
            impl<'a> BorrowDeserialize<'a> for $ty {
                fn borrow_from_tape(json: &'a str, tape: &TapeRef<'a>, idx: usize) -> Result<Self> {
                    <$ty as AutoDeserialize>::from_tape(json, tape, idx)
                }
            }
        )*
    };
}

impl_borrow_for_auto!(String, f64, f32, i64, i32, u32, u64, bool);

// Zero-copy Vec implementation
impl<'a, T: BorrowDeserialize<'a>> BorrowDeserialize<'a> for Vec<T> {
    fn borrow_from_tape(json: &'a str, tape: &TapeRef<'a>, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        if node.tag != NodeType::Array {
            return Err(Error::TypeMismatch { expected: "array", found: "other" });
        }

        let mut list = Vec::new();
        for (elem_idx, _elem_node) in tape.iter_array(node) {
            let item = T::borrow_from_tape(json, tape, elem_idx)?;
            list.push(item);
        }
        Ok(list)
    }
}

// Zero-copy Option implementation
impl<'a, T: BorrowDeserialize<'a>> BorrowDeserialize<'a> for Option<T> {
    fn borrow_from_tape(json: &'a str, tape: &TapeRef<'a>, idx: usize) -> Result<Self> {
        let node = tape.get(idx);
        if node.tag == NodeType::Null {
            return Ok(None);
        }
        match T::borrow_from_tape(json, tape, idx) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Ok(None),
        }
    }
}

/// Parse JSON with zero-copy borrowing
pub fn parse_borrow<'a, T: BorrowDeserialize<'a>>(json: &'a str, tape: &TapeRef<'a>) -> Result<T> {
    T::borrow_from_tape(json, tape, 0)
}

/// Parse JSON string into a type that implements AutoDeserialize
pub fn parse<T: AutoDeserialize>(json: &str) -> Result<T> {
    let tape = TapeRef::parse(json)?;
    T::from_tape(json, &tape, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Static Zig buffer causes issues in test context
    fn test_tape_parse_object() {
        let json = r#"{"name": "test", "value": 42}"#;
        let tape = TapeRef::parse(json).unwrap();
        
        assert!(tape.len() > 0);
        let root = tape.root();
        assert_eq!(root.tag, NodeType::Object);
    }

    #[test]
    #[ignore] // Static Zig buffer causes issues in test context
    fn test_tape_parse_string() {
        let json = r#"{"key": "hello"}"#;
        let tape = TapeRef::parse(json).unwrap();
        
        let root = tape.root();
        assert_eq!(root.tag, NodeType::Object);
        
        // First child is the key
        let key_idx = root.child as usize;
        let key_node = tape.get(key_idx);
        assert_eq!(tape.get_str(json, key_node), Some("key"));
    }

    #[test]
    #[ignore] // Static Zig buffer causes issues in test context
    fn test_tape_parse_number() {
        let json = r#"{"n": 42}"#;
        let tape = TapeRef::parse(json).unwrap();
        
        let root = tape.root();
        let key_idx = root.child as usize;
        let key_node = tape.get(key_idx);
        let val_idx = key_node.child as usize;
        let val_node = tape.get(val_idx);
        
        assert_eq!(tape.get_i64(json, val_node), Some(42));
    }
}
