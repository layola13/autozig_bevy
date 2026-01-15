//! Safe SIMD-Style Benchmark - No `unsafe` keyword!
//! Demonstrates high-performance iteration using safe patterns.
//!
//! This shows how AutoZig's Query API can achieve good performance
//! without any unsafe code in user space.

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

    println!("=== Safe SIMD-Style AutoZig ECS Benchmark ===");
    println!("No `unsafe` keyword in user code!");
    println!("SIMD optimizations happen inside Zig backend");
    println!("");
    println!("Spawning {} entities...", ENTITY_COUNT);
    setup(app.world_mut());
    app.world_mut().update_archetypes();

    // Register the movement system
    let move_sys: ParamFunctionSystem<FunctionMarker<((), Query<'static, (&'static mut Position, &'static Velocity)>)>, _> = 
        simd_style_movement.into_system();
    app.add_systems(Update, move_sys);

    println!("Benchmarking for {} frames...", FRAMES);

    let start = std::time::Instant::now();
    for _ in 0..FRAMES {
        app.update();
    }
    let duration = start.elapsed();

    println!("");
    println!("AutoZig ECS SAFE SIMD-Style Result:");
    println!("  Total Time: {:.2?}", duration);
    println!("  Avg Frame:  {:.2?}", duration / FRAMES as u32);
    println!("  Entities:   {}", ENTITY_COUNT);
    println!("");
    println!("✓ Zero unsafe code in user space!");
    println!("✓ SIMD happens in Zig backend (32-byte aligned storage)");
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

/// Safe SIMD-style movement system
/// The Zig backend uses 32-byte aligned storage enabling SIMD
/// All unsafe operations are encapsulated in the ECS library
fn simd_style_movement(mut query: Query<(&mut Position, &Velocity)>) {
    let dt = 0.016;
    
    // This loop benefits from:
    // 1. Zig's 32-byte aligned component storage (AVX-ready)
    // 2. Cache-friendly table iteration  
    // 3. Compiler auto-vectorization hints from LTO
    for (mut pos, vel) in &mut query {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }
}
