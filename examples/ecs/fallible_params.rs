//! fallible_params - Demonstrate fallible system parameters
//!
//! Ported from Bevy ecs/fallible_params.rs
//!
//! Shows how Option<Res<T>> and other fallible parameters work.
//! These parameters don't cause panics when resources/entities don't exist.

use autozig_ecs::prelude::*;

#[derive(Resource)]
struct PlayerData {
    name: String,
    score: u32,
}

#[derive(Resource)]
struct GameConfig {
    difficulty: u32,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(ScheduleRunnerPlugin::run_once());
    
    // Only insert PlayerData, NOT GameConfig
    // This demonstrates fallible params - GameConfig won't exist
    app.insert_resource(PlayerData {
        name: "Hero".to_string(),
        score: 100,
    });

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // System with Option<Res<T>> - won't panic if resource doesn't exist
    let check_sys: ParamFunctionSystem<FunctionMarker<((), 
        Option<Res<'static, PlayerData>>,
        Option<Res<'static, GameConfig>>
    )>, _> = check_resources.into_system();

    // System that always expects PlayerData (would panic if missing)
    let player_sys: ParamFunctionSystem<FunctionMarker<((), 
        Res<'static, PlayerData>
    )>, _> = print_player.into_system();

    app.add_systems(Update, check_sys);
    app.add_systems(Update, player_sys);

    println!("Starting Fallible Params Example...");
    println!("GameConfig intentionally NOT inserted to demonstrate Option<Res<T>>");
    app.run();
}

fn check_resources(
    player: Option<Res<PlayerData>>,
    config: Option<Res<GameConfig>>,
) {
    println!("\n--- Checking Optional Resources ---");
    
    match player {
        Some(p) => println!("PlayerData exists: {} (score: {})", p.name, p.score),
        None => println!("PlayerData does NOT exist"),
    }
    
    match config {
        Some(c) => println!("GameConfig exists: difficulty={}", c.difficulty),
        None => println!("GameConfig does NOT exist (this is expected!)"),
    }
    
    println!("-----------------------------------\n");
}

fn print_player(player: Res<PlayerData>) {
    println!("Player: {} with score {}", player.name, player.score);
}
