//! Basic usage example for autozig_json

use autozig_json::{from_str, json, to_string_pretty, Value};

fn main() {
    println!("=== AutoZig JSON Example ===\n");

    // Example 1: Parse JSON string
    println!("1. Parsing JSON string:");
    let json_str = r#"
    {
        "name": "AutoZig",
        "version": 1.0,
        "features": ["simd", "fast", "safe"],
        "active": true
    }
    "#;

    let value = from_str(json_str).unwrap();
    println!("  Name: {}", value["name"]);
    println!("  Version: {}", value["version"]);
    println!("  Active: {}", value["active"]);
    println!("  First feature: {}", value["features"][0]);
    println!();

    // Example 2: Use json! macro
    println!("2. Building JSON with macro:");
    let new_value = json!({
        "id": 42,
        "message": "Hello from AutoZig!",
        "data": [1, 2, 3]
    });
    println!("  Created: {}", new_value);
    println!();

    // Example 3: Pretty print
    println!("3. Pretty printing:");
    let pretty = to_string_pretty(&new_value);
    println!("{}", pretty);
    println!();

    // Example 4: Access nested data
    println!("4. Nested data access:");
    let nested = from_str(r#"
    {
        "users": [
            {"id": 1, "name": "Alice"},
            {"id": 2, "name": "Bob"}
        ]
    }
    "#).unwrap();
    
    println!("  User 0: {}", nested["users"][0]["name"]);
    println!("  User 1 ID: {}", nested["users"][1]["id"]);

    println!("\n=== Done! ===");
}
