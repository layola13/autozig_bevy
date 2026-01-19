//! hotpatching_systems.rs - Demonstrating Dynamic System Hot-Swapping
//!
//! This example shows how to implement a system that can be replaced at runtime.
//! We use an ExclusiveSystem as a "Proxy" that looks up the actual system implementation
//! from a Resource and executes it.

use autozig_ecs::prelude::*;
use autozig_ecs::system::{BoxedSystem, ExclusiveFunctionSystem};
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use std::collections::HashMap;

// Resource to hold our dynamic systems
#[derive(Default)]
struct HotSwapRegistry {
    systems: HashMap<String, BoxedSystem>,
}

// Proxy system that runs the dynamic system
fn proxy_runner(world: &mut World) {
    // 1. Temporarily take the registry out of the world to avoid double-borrow
    if let Some(mut registry) = world.remove_resource::<HotSwapRegistry>() {
        
        // 2. Find and run the system
        if let Some(system) = registry.systems.get_mut("logic") {
            system.initialize(world); // Ensure initialized
            system.run(world);
        } else {
            println!("Proxy: No system found for key 'logic'");
        }
        
        // 3. Put registry back
        world.insert_resource(registry);
    }
}

// System Version A
fn logic_version_a(mut commands: Commands) {
    println!("Logic A: Spawning entity...");
    commands.add(|world: &mut World| {
         world.spawn(Name("Entity_A".to_string()));
    });
}

// System Version B
fn logic_version_b(query: Query<&Name>) {
    println!("Logic B: Counting entities...");
    for name in query.iter() {
        println!(" - Found: {:?}", name);
    }
}

// Component for demo
#[derive(Component, Debug)]
struct Name(String);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    
    // Register component
    app.world_mut().register_component::<Name>();
    
    // Initialize Registry
    let mut registry = HotSwapRegistry::default();
    
    // Start with Version A
    println!("--- Hotpatching Demo: Load Version A ---");
    let sys_a: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = logic_version_a.into_system();
    registry.systems.insert("logic".to_string(), BoxedSystem::new(sys_a, "logic_version_a"));
    
    app.insert_resource(registry);
    
    // Register the Proxy as an ExclusiveSystem
    // Note: In a real app you might arguably wrappers to make this generic
    app.add_systems(Update, ExclusiveFunctionSystem::new(proxy_runner, "proxy_runner"));
    
    // Run a few frames
    app.set_runner(|mut app| {
        println!("Frame 1 (Version A):");
        app.update();
        
        println!("Frame 2 (Version A):");
        app.update();
        
        // --- HOT SWAP ---
        println!("\n--- Hotpatching Demo: Swapping to Version B ---");
        if let Some(mut registry) = app.world_mut().remove_resource::<HotSwapRegistry>() {
            let sys_b: ParamFunctionSystem<FunctionMarker<((), Query<'static, &'static Name, ()>)>, _> = logic_version_b.into_system();
            registry.systems.insert("logic".to_string(), BoxedSystem::new(sys_b, "logic_version_b"));
            app.world_mut().insert_resource(registry);
        }
        
        println!("Frame 3 (Version B):");
        app.update();
        
        println!("Frame 4 (Version B):");
        app.update();
    });
    
    app.run();
}
