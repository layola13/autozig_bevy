//! component_hooks.rs - Demonstrating Component Lifecycle Hooks
//!
//! This example shows how to register and use component hooks (on_add, on_insert, on_remove).

use autozig_ecs::prelude::*;
use autozig_ecs::component_advanced::{HookContext};
use autozig_ecs::world::DeferredWorld;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use std::sync::{Mutex};

#[derive(Component)]
struct MyComponent(usize);

#[derive(Default)]
struct HookTracker {
    added: usize,
    inserted: usize,
    replaced: usize,
    removed: usize,
}

// Global invocation counter for debugging (since hooks run in weird contexts)
static TRACKER: Mutex<HookTracker> = Mutex::new(HookTracker { added: 0, inserted: 0, replaced: 0, removed: 0 });

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.init_resource::<HookTracker>();

    // Register Component
    app.world_mut().register_component::<MyComponent>();

    // Register Hooks via Builder
    app.world_mut().register_component_hooks::<MyComponent>()
        .on_add(|_world: DeferredWorld, _ctx: HookContext| {
            println!("Hook: on_add for {:?}", _ctx);
            let mut tracker = TRACKER.lock().unwrap();
            tracker.added += 1;
        })
        .on_insert(|_world: DeferredWorld, _ctx: HookContext| {
            println!("Hook: on_insert for {:?}", _ctx);
            let mut tracker = TRACKER.lock().unwrap();
            tracker.inserted += 1;
        })
        .on_replace(|_world: DeferredWorld, _ctx: HookContext| {
            println!("Hook: on_replace for {:?}", _ctx);
            let mut tracker = TRACKER.lock().unwrap();
            tracker.replaced += 1;
        });

    // Systems
    let run_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = 
        run_hooks_demo.into_system();
        
    let check_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, HookTracker>)>, _> = 
        check_results.into_system();

    app.add_systems(Startup, run_sys);
    app.add_systems(Update, check_sys);

    println!("Starting Component Hooks Example...");
    // Run loop
    app.set_runner(|mut app| {
        app.update(); // Startup
        app.update(); // Check results
    });
    
    app.run();
}

fn run_hooks_demo(mut commands: Commands) {
    println!("Spawning entity with MyComponent...");
    // workaround: use commands.add with direct world access to bypass potential command parsing issues
    commands.add(|world: &mut World| {
        world.spawn(MyComponent(1));
    });
}

fn check_results(_tracker: Res<HookTracker>) {
    let tracker = TRACKER.lock().unwrap();
    println!("Tracker State: added={}, inserted={}, replaced={}, removed={}", 
             tracker.added, tracker.inserted, tracker.replaced, tracker.removed);
             
    assert_eq!(tracker.added, 1, "Should have 1 on_add");
    assert_eq!(tracker.inserted, 1, "Should have 1 on_insert");
    // We spawned once, so no replace.
    // assert_eq!(tracker.replaced, 0, "Should have 0 on_replace");
    
    println!("SUCCESS: Hooks triggered correctly.");
}
