//! system_piping - Demonstrate system output piping
//!
//! Ported from Bevy ecs/system_piping.rs
//!
//! Shows how to chain systems where one system's output becomes another's input.

use autozig_ecs::prelude::*;


struct Message(String);

fn main() {
    let mut app = App::new();

    app.add_plugins(ScheduleRunnerPlugin::run_once());
    app.insert_resource(Message("42".to_string()));

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // In the full Bevy API, you'd use .pipe() to chain systems
    // For now, we demonstrate the concept with separate systems
    
    let parse_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, Message>)>, _> = parse_message.into_system();
    let handle_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, Message>)>, _> = handle_message.into_system();

    app.add_systems(Update, (parse_sys, handle_sys).chain());

    println!("Starting System Piping Example...");
    println!("Note: Full .pipe() API requires In<T> system input support.");
    app.run();
}

fn parse_message(message: Res<Message>) {
    match message.0.parse::<usize>() {
        Ok(value) => println!("Parsed message as number: {}", value),
        Err(err) => println!("Failed to parse message: {:?}", err),
    }
}

fn handle_message(message: Res<Message>) {
    println!("Message value: {}", message.0);
    if message.0.parse::<usize>().is_ok() {
        println!("Message is a valid number!");
    }
}
