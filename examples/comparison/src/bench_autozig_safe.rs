//! Safe Raw Iteration Benchmark - No `unsafe` keyword!
//! Uses standard Query API for maximum safety while still being fast.
//!
//! This demonstrates the performance you get with pure safe Rust
//! using AutoZig ECS's optimized Query implementation.

use autozig_ecs::prelude::*;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use rand::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
struct Position { x: f32, y: f32 }

#[derive(Component, Clone, Copy, Debug)]
#[repr(C)]
struct Velocity { x: f32, y: f32 }

const ENTITY_COUNT: usize = 100_000;
const FRAMES: usize = 1000;

#[no_mangle]
pub extern "C" fn __zig_probe_stack() {}

fn main() {
    let mut app = App::new();
    
    // Register components
    app.world_mut().register_component::<Position>();
    app.world_mut().register_component::<Velocity>();

    println!("=== Safe AutoZig ECS Benchmark ===");
    println!("No `unsafe` keyword used in this file!");
    println!("");
    println!("Spawning {} entities...", ENTITY_COUNT);
    setup(app.world_mut());
    app.world_mut().update_archetypes();

    // Register the movement system
    let move_sys: ParamFunctionSystem<FunctionMarker<((), Query<'static, (&'static mut Position, &'static Velocity)>)>, _> = 
        movement_system.into_system();
    app.add_systems(Update, move_sys);

    println!("Benchmarking for {} frames...", FRAMES);

    let start = std::time::Instant::now();
    for _ in 0..FRAMES {
        app.update();
    }
    let duration = start.elapsed();

    println!("");
    println!("AutoZig ECS SAFE Result:");
    println!("  Total Time: {:.2?}", duration);
    println!("  Avg Frame:  {:.2?}", duration / FRAMES as u32);
    println!("  Entities:   {}", ENTITY_COUNT);
    println!("");
    println!("✓ No unsafe code used!");
}

fn setup(world: &mut World) {
    let mut rng = rand::thread_rng();
    for _ in 0..ENTITY_COUNT {
        world.spawn((
            Position { x: rng.gen(), y: rng.gen() },
            Velocity { x: rng.gen(), y: rng.gen() }
        ));
    }
}

/// Pure safe movement system using standard Query API
fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    let dt = 0.016;
    for (mut pos, vel) in &mut query {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }
}
