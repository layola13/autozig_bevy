//! Derive macro example for autozig_json

use autozig_json::{parse, AutoDeserialize};

// 使用手写的 proc-macro，编译时间极快！
#[derive(AutoDeserialize, Debug)]
struct User {
    id: u32,
    name: String,
    active: bool,
}

#[derive(AutoDeserialize, Debug)]
struct Product {
    sku: String,
    price: f64,
    quantity: i32,
}

fn main() {
    println!("=== AutoZig JSON Derive Macro Demo ===\n");

    // Example 1: Parse User struct
    println!("1. Parsing User struct:");
    let json = r#"{"id": 101, "name": "ZigSpeed", "active": true}"#;

    match parse::<User>(json) {
        Ok(user) => {
            println!("  Parsed: {:?}", user);
            println!("  ID: {}", user.id);
            println!("  Name: {}", user.name);
            println!("  Active: {}", user.active);
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Example 2: Parse Product struct
    println!("2. Parsing Product struct:");
    let json = r#"{"sku": "ZIG-001", "price": 99.99, "quantity": 50}"#;

    match parse::<Product>(json) {
        Ok(product) => {
            println!("  Parsed: {:?}", product);
            println!("  SKU: {}", product.sku);
            println!("  Price: ${:.2}", product.price);
            println!("  Quantity: {}", product.quantity);
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();

    // Example 3: Show compilation speed benefit
    println!("3. Compilation Speed:");
    println!("  ✓ Zero syn/quote dependencies");
    println!("  ✓ Hand-written TokenStream parser");
    println!("  ✓ Zig SIMD backend for runtime speed");
    println!();

    println!("=== Done! ===");
}
