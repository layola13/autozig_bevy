//! removal_detection - Demonstrate removal detection
//!
//! Ported from Bevy ecs/removal_detection.rs
//!
//! Shows how to detect when a component has been removed from an entity.

use autozig_ecs::prelude::*;

#[derive(Component)]
struct MyComponent;

#[derive(Component)]
struct Marked;

fn main() {
    let mut app = App::new();

    app.add_plugins(ScheduleRunnerPlugin::run_once());

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // Setup system spawns entities with MyComponent
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = setup.into_system();

    // This system removes a component
    let remove_sys: ParamFunctionSystem<FunctionMarker<((), 
        Commands<'static>,
        Query<'static, Entity, With<MyComponent>>
    )>, _> = remove_component.into_system();

    // This system detects removed components
    let detect_sys: ParamFunctionSystem<FunctionMarker<((), 
        RemovedComponents<'static, MyComponent>
    )>, _> = detect_removal.into_system();

    app.add_systems(Startup, setup_sys);
    app.add_systems(Update, (remove_sys, detect_sys).chain());

    println!("Starting Removal Detection Example...");
    app.run();
}

fn setup(mut commands: Commands) {
    println!("Spawning 3 entities with MyComponent...");
    commands.spawn(MyComponent);
    commands.spawn(MyComponent);
    commands.spawn(MyComponent);
}

fn remove_component(
    mut commands: Commands,
    query: Query<Entity, With<MyComponent>>,
) {
    // Remove MyComponent from the first entity found
    if let Some(entity) = query.iter().next() {
        println!("Removing MyComponent from entity {:?}", entity);
        commands.entity(entity).remove::<MyComponent>();
    }
}

fn detect_removal(removed: RemovedComponents<MyComponent>) {
    for entity in removed.iter() {
        println!("Detected removal of MyComponent from entity {:?}", entity);
    }
}
