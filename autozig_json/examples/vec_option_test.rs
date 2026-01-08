//! Test Vec and Option

use autozig_json::{parse, AutoDeserialize};

#[derive(AutoDeserialize, Debug)]
struct Item {
    name: String,
    value: i32,
}

#[derive(AutoDeserialize, Debug)]
struct Container {
    items: Vec<Item>,
    optional: Option<String>,
}

fn main() {
    println!("=== Vec/Option Test ===\n");

    let json = r#"{
        "items": [
            { "name": "A", "value": 1 },
            { "name": "B", "value": 2 }
        ],
        "optional": "hello"
    }"#;

    println!("Parsing...");
    
    match parse::<Container>(json) {
        Ok(c) => {
            println!("✓ Success!");
            println!("  Items: {:?}", c.items);
            println!("  Optional: {:?}", c.optional);
        },
        Err(e) => println!("Error: {}", e),
    }
}
