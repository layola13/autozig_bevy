//! Complex game data example demonstrating nested structs, Vec<T>, and Option<T>

use autozig_json::{parse, AutoDeserialize};

// 1. Nested struct: Transform
#[derive(AutoDeserialize, Debug)]
struct Transform {
    x: f32,
    y: f32,
    z: f32,
}

// 2. Nested struct: Weapon
#[derive(AutoDeserialize, Debug)]
struct Weapon {
    name: String,
    damage: i32,
}

// 3. Complex struct: Enemy with all features
#[derive(AutoDeserialize, Debug)]
struct Enemy {
    id: u32,
    name: String,
    // Nested struct
    transform: Transform,
    // Option: might be null
    loot_drop: Option<String>,
    // Vec<f32>: patrol points
    patrol_path: Vec<f32>,
    // Vec<Struct>: weapons list
    weapons: Vec<Weapon>,
}

// 4. Root struct: Level data
#[derive(AutoDeserialize, Debug)]
struct LevelData {
    level_name: String,
    difficulty: i32,
    enemies: Vec<Enemy>,
}

fn main() {
    println!("=== AutoZig Complex JSON Test ===\n");

    let json = r#"{
        "level_name": "Dungeon_01",
        "difficulty": 5,
        "enemies": [
            {
                "id": 101,
                "name": "Goblin Scout",
                "transform": { "x": 10.5, "y": 0.0, "z": -5.0 },
                "loot_drop": "Gold Coin",
                "patrol_path": [10.0, 20.0, 15.0],
                "weapons": [
                    { "name": "Dagger", "damage": 5 }
                ]
            },
            {
                "id": 102,
                "name": "Goblin Boss",
                "transform": { "x": 50.0, "y": 0.0, "z": 50.0 },
                "loot_drop": null,
                "patrol_path": [],
                "weapons": [
                    { "name": "Axe", "damage": 15 },
                    { "name": "Shield", "damage": 2 }
                ]
            }
        ]
    }"#;

    println!("Parsing Level Data ({} bytes)...", json.len());
    
    let start = std::time::Instant::now();
    
    match parse::<LevelData>(json) {
        Ok(level) => {
            println!("✓ Parse Success in {:?}\n", start.elapsed());
            
            println!("Level: {}", level.level_name);
            println!("Difficulty: {}", level.difficulty);
            println!("Enemies Count: {}", level.enemies.len());
            
            for (i, enemy) in level.enemies.iter().enumerate() {
                println!("\n  [Enemy #{}] {}", i, enemy.name);
                println!("    Pos: ({}, {}, {})", enemy.transform.x, enemy.transform.y, enemy.transform.z);
                println!("    Loot: {:?}", enemy.loot_drop);
                println!("    Path Points: {:?}", enemy.patrol_path);
                println!("    Weapons:");
                for w in &enemy.weapons {
                    println!("      - {} (DMG: {})", w.name, w.damage);
                }
            }
        },
        Err(e) => println!("Error: {}", e),
    }
    
    println!("\n=== Done! ===");
}
