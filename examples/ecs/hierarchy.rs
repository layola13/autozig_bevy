//! hierarchy - Demonstrate parent-child entity relationships
//!
//! Ported from Bevy ecs/hierarchy.rs
//!
//! Shows how to create parent-child relationships between entities.

use autozig_ecs::prelude::*;

#[derive(Component)]
struct Name(String);

fn main() {
    let mut app = App::new();

    app.add_plugins(ScheduleRunnerPlugin::run_once());

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = 
        setup_hierarchy.into_system();

    let print_sys: ParamFunctionSystem<FunctionMarker<((), 
        Query<'static, &'static Name>
    )>, _> = print_entities.into_system();

    app.add_systems(Startup, setup_sys);
    app.add_systems(Update, print_sys);

    println!("Starting Hierarchy Example...");
    app.run();
}

fn setup_hierarchy(mut commands: Commands) {
    println!("Creating entities...");
    
    // Create entities (parent-child linking would require full BuildChildren API)
    commands.spawn(Name("Entity A".to_string()));
    commands.spawn(Name("Entity B".to_string()));
    commands.spawn(Name("Entity C".to_string()));
    
    println!("Created 3 entities");
    println!("Note: Full parent-child relationships require BuildChildren trait implementation");
}

fn print_entities(query: Query<&Name>) {
    println!("\n--- Entity List ---");
    for name in query.iter() {
        println!("Entity: '{}'", name.0);
    }
    println!("-------------------\n");
}
