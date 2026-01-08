//! JSON Serializer - convert Value to JSON string

use crate::value::Value;

/// Convert a Value to a JSON string
/// 
/// # Example
/// ```
/// use autozig_json::{Value, to_string};
/// 
/// let value = Value::Object(std::collections::HashMap::from([
///     ("name".to_string(), Value::String("AutoZig".to_string())),
///     ("version".to_string(), Value::Number(1.0)),
/// ]));
/// 
/// let json = to_string(&value);
/// ```
pub fn to_string(value: &Value) -> String {
    value.to_string()
}

/// Convert a Value to a pretty-printed JSON string
pub fn to_string_pretty(value: &Value) -> String {
    value.to_string_pretty()
}

/// Convert a Value to a JSON byte vector
pub fn to_vec(value: &Value) -> Vec<u8> {
    value.to_string().into_bytes()
}

/// Convert a Value to a pretty-printed JSON byte vector
pub fn to_vec_pretty(value: &Value) -> Vec<u8> {
    value.to_string_pretty().into_bytes()
}
