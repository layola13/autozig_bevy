//! This example demonstrates you can create a custom runner (to update an app manually). It reads
//! lines from stdin and prints them from within the ecs.

use autozig_app::{App, AppExit, Update};
use autozig_ecs::prelude::*;
use std::io;

#[derive(Debug)]
struct Input(String);

fn print_system(input: Res<Input>) {
    println!("You typed: {}", input.0);
}

fn exit_system(input: Res<Input>, mut exit_events: EventWriter<AppExit>) {
    if input.0 == "exit" {
        exit_events.send(AppExit::Success);
    }
}

fn main() {
    let mut app = App::new();
    app.insert_resource(Input(String::new()));
    
    // Manually register AppExit events since we are running a minimal setup
    // and want to use the standard EventWriter pattern in Rust systems.
    app.insert_resource(Events::<AppExit>::default());
    
    app.add_systems(Update, (print_system, exit_system));

    // Initialize the app
    app.finish();
    app.cleanup();

    println!("Type stuff into the console");
    for line in io::stdin().lines() {
        {
            let mut input = app.get_resource_mut::<Input>().expect("Input resource missing");
            input.0 = line.unwrap();
        }

        app.update();

        // Check for exit event
        // We access the Events resource directly to see if any exit events were sent
        let events = app.world.resource::<Events<AppExit>>();
        if !events.is_empty() {
            println!("Exiting with Success");
            break;
        }
    }
}
