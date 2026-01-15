//! dynamic - Demonstrate dynamic component handling
//!
//! Ported from Bevy ecs/dynamic.rs
//!
//! Shows how to work with dynamic bundles and component reflection.
//! Note: Full dynamic component API requires reflection implementation.

use autozig_ecs::prelude::*;
use std::any::TypeId;

#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Velocity { x: f32, y: f32 }

#[derive(Component)]
struct Name(String);

fn main() {
    let mut app = App::new();

    app.add_plugins(ScheduleRunnerPlugin::run_once());

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // Spawn entities with different component combinations
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = 
        setup.into_system();

    // Query and print component info
    let print_sys: ParamFunctionSystem<FunctionMarker<((), 
        Query<'static, (Entity, &'static Position)>,
        Query<'static, (Entity, &'static Velocity)>,
        Query<'static, (Entity, &'static Name)>
    )>, _> = print_components.into_system();

    app.add_systems(Startup, setup_sys);
    app.add_systems(Update, print_sys);

    println!("Starting Dynamic Components Example...");
    println!("Note: Full DynamicBundle API requires reflection implementation");
    app.run();
}

fn setup(mut commands: Commands) {
    println!("Spawning entities with different component combinations...\n");
    
    // Entity with all components
    commands.spawn((
        Name("Full Entity".to_string()),
        Position { x: 1.0, y: 2.0 },
        Velocity { x: 0.5, y: 0.5 },
    ));
    
    // Entity with position only
    commands.spawn((
        Name("Position Only".to_string()),
        Position { x: 10.0, y: 20.0 },
    ));
    
    // Entity with velocity only
    commands.spawn((
        Name("Velocity Only".to_string()),
        Velocity { x: 1.0, y: -1.0 },
    ));
    
    // Entity with just name
    commands.spawn(Name("Name Only".to_string()));
    
    println!("Spawned 4 entities with different component combinations");
}

fn print_components(
    pos_query: Query<(Entity, &Position)>,
    vel_query: Query<(Entity, &Velocity)>,
    name_query: Query<(Entity, &Name)>,
) {
    println!("\n--- Component Report ---");
    
    println!("Entities with Position ({}):", pos_query.iter().count());
    for (entity, pos) in pos_query.iter() {
        println!("  {:?}: ({:.1}, {:.1})", entity, pos.x, pos.y);
    }
    
    println!("Entities with Velocity ({}):", vel_query.iter().count());
    for (entity, vel) in vel_query.iter() {
        println!("  {:?}: ({:.1}, {:.1})", entity, vel.x, vel.y);
    }
    
    println!("Entities with Name ({}):", name_query.iter().count());
    for (entity, name) in name_query.iter() {
        println!("  {:?}: '{}'", entity, name.0);
    }
    
    println!("------------------------\n");
}
