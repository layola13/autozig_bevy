//! Zero-copy JSON parsing example
//!
//! This example demonstrates using &'a str fields to borrow
//! directly from the JSON input, avoiding memory allocation.

use autozig_json::{parse_borrow, BorrowDeserialize, Error, NodeType, Result, TapeRef};

// Zero-copy struct - borrows strings directly from JSON
#[derive(Debug)]
struct WeaponZC<'a> {
    name: &'a str,  // Zero-copy!
    damage: i32,
}

// Manual impl for zero-copy deserialization
impl<'a> BorrowDeserialize<'a> for WeaponZC<'a> {
    fn borrow_from_tape(json: &'a str, tape: &TapeRef<'a>, root_idx: usize) -> Result<Self> {
        let root = tape.get(root_idx);
        if root.tag != NodeType::Object {
            return Err(Error::TypeMismatch { expected: "object", found: "other" });
        }
        
        let mut name: Option<&'a str> = None;
        let mut damage: Option<i32> = None;
        
        let mut key_idx = root.child as usize;
        while key_idx != 0 {
            let key_node = tape.get(key_idx);
            let val_idx = key_node.child as usize;
            
            if let Some(key) = tape.get_str(json, key_node) {
                match key {
                    "name" => name = Some(<&str as BorrowDeserialize>::borrow_from_tape(json, tape, val_idx)?),
                    "damage" => damage = Some(<i32 as BorrowDeserialize>::borrow_from_tape(json, tape, val_idx)?),
                    _ => {}
                }
            }
            key_idx = key_node.next as usize;
        }
        
        Ok(WeaponZC {
            name: name.ok_or_else(|| Error::KeyNotFound { key: "name".to_string() })?,
            damage: damage.ok_or_else(|| Error::KeyNotFound { key: "damage".to_string() })?,
        })
    }
}

#[derive(Debug)]
struct PlayerZC<'a> {
    name: &'a str,  // Zero-copy!
    health: i32,
    weapons: Vec<WeaponZC<'a>>,
}

impl<'a> BorrowDeserialize<'a> for PlayerZC<'a> {
    fn borrow_from_tape(json: &'a str, tape: &TapeRef<'a>, root_idx: usize) -> Result<Self> {
        let root = tape.get(root_idx);
        if root.tag != NodeType::Object {
            return Err(Error::TypeMismatch { expected: "object", found: "other" });
        }
        
        let mut name: Option<&'a str> = None;
        let mut health: Option<i32> = None;
        let mut weapons: Option<Vec<WeaponZC<'a>>> = None;
        
        let mut key_idx = root.child as usize;
        while key_idx != 0 {
            let key_node = tape.get(key_idx);
            let val_idx = key_node.child as usize;
            
            if let Some(key) = tape.get_str(json, key_node) {
                match key {
                    "name" => name = Some(<&str as BorrowDeserialize>::borrow_from_tape(json, tape, val_idx)?),
                    "health" => health = Some(<i32 as BorrowDeserialize>::borrow_from_tape(json, tape, val_idx)?),
                    "weapons" => weapons = Some(<Vec<WeaponZC> as BorrowDeserialize>::borrow_from_tape(json, tape, val_idx)?),
                    _ => {}
                }
            }
            key_idx = key_node.next as usize;
        }
        
        Ok(PlayerZC {
            name: name.ok_or_else(|| Error::KeyNotFound { key: "name".to_string() })?,
            health: health.ok_or_else(|| Error::KeyNotFound { key: "health".to_string() })?,
            weapons: weapons.ok_or_else(|| Error::KeyNotFound { key: "weapons".to_string() })?,
        })
    }
}

fn main() {
    println!("=== Zero-Copy JSON Parsing ===\n");

    let json = r#"{
        "name": "Hero",
        "health": 100,
        "weapons": [
            { "name": "Sword of Light", "damage": 50 },
            { "name": "Shield of Dawn", "damage": 10 }
        ]
    }"#;

    println!("JSON ({} bytes):", json.len());
    println!("{}\n", json);

    // Parse with zero-copy
    let tape = TapeRef::parse(json).unwrap();
    
    let start = std::time::Instant::now();
    
    match parse_borrow::<PlayerZC>(&json, &tape) {
        Ok(player) => {
            println!("✓ Parse Success in {:?}\n", start.elapsed());
            println!("Player: {}", player.name);
            println!("Health: {}", player.health);
            println!("Weapons:");
            for w in &player.weapons {
                println!("  - {} (DMG: {})", w.name, w.damage);
            }
            
            // Prove it's zero-copy: check pointer addresses
            println!("\n=== Zero-Copy Proof ===");
            let json_ptr = json.as_ptr();
            let name_ptr = player.name.as_ptr();
            let offset = name_ptr as usize - json_ptr as usize;
            println!("JSON starts at:   {:?}", json_ptr);
            println!("player.name at:   {:?} (offset: {})", name_ptr, offset);
            println!("→ Strings point directly into JSON buffer!");
        },
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n=== Done! ===");
}
