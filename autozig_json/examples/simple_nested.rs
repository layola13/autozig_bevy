//! Simple nested struct test

use autozig_json::{parse, AutoDeserialize};

// Simple nested struct
#[derive(AutoDeserialize, Debug)]
struct Inner {
    x: f32,
    y: f32,
}

#[derive(AutoDeserialize, Debug)]
struct Outer {
    name: String,
    pos: Inner,
}

fn main() {
    println!("=== Simple Nested Test ===\n");

    let json = r#"{
        "name": "test",
        "pos": { "x": 1.0, "y": 2.0 }
    }"#;

    println!("Parsing...");
    
    match parse::<Outer>(json) {
        Ok(outer) => {
            println!("✓ Success!");
            println!("  Name: {}", outer.name);
            println!("  Pos: ({}, {})", outer.pos.x, outer.pos.y);
        },
        Err(e) => println!("Error: {}", e),
    }
}
