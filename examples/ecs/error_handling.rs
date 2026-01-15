//! error_handling - Demonstrate error handling in systems
//!
//! Ported from Bevy ecs/error_handling.rs
//!
//! Shows how systems can return Result types and handle errors gracefully.
//! Note: Full error handling requires Result-returning system support.

use autozig_ecs::prelude::*;

#[derive(Resource)]
struct Config {
    value: String,
}

#[derive(Resource, Default)]
struct ParsedValue(Option<i32>);

fn main() {
    let mut app = App::new();

    app.add_plugins(ScheduleRunnerPlugin::run_once());
    
    // Insert config with a parseable value
    app.insert_resource(Config { value: "42".to_string() });
    app.init_resource::<ParsedValue>();

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // System that parses and handles errors internally
    let parse_sys: ParamFunctionSystem<FunctionMarker<((), 
        Res<'static, Config>,
        ResMut<'static, ParsedValue>
    )>, _> = parse_config.into_system();

    // System that uses parsed value
    let use_sys: ParamFunctionSystem<FunctionMarker<((), 
        Res<'static, ParsedValue>
    )>, _> = use_parsed_value.into_system();

    app.add_systems(Update, (parse_sys, use_sys).chain());

    println!("Starting Error Handling Example...");
    println!("Note: Full Result<()> returning systems require additional implementation");
    app.run();
}

fn parse_config(config: Res<Config>, mut parsed: ResMut<ParsedValue>) {
    println!("Attempting to parse config value: '{}'", config.value);
    
    match config.value.parse::<i32>() {
        Ok(value) => {
            println!("Successfully parsed: {}", value);
            parsed.0 = Some(value);
        }
        Err(e) => {
            println!("Parse error: {:?}", e);
            parsed.0 = None;
        }
    }
}

fn use_parsed_value(parsed: Res<ParsedValue>) {
    match parsed.0 {
        Some(value) => println!("Using parsed value: {} * 2 = {}", value, value * 2),
        None => println!("No valid parsed value available"),
    }
}
