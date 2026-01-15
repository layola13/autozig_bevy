use autozig_ecs::prelude::*;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use rand::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
struct Position { x: f32, y: f32 }

#[derive(Component, Clone, Copy, Debug)]
struct Velocity { x: f32, y: f32 }

const ENTITY_COUNT: usize = 100_000;
const FRAMES: usize = 1000;

#[no_mangle]
pub extern "C" fn __zig_probe_stack() {}

fn main() {
    let mut app = App::new();
    
    // Register components manual registration required for Zig layout
    app.world_mut().register_component::<Position>();
    app.world_mut().register_component::<Velocity>();

    println!("Spawning {} entities...", ENTITY_COUNT);
    setup(app.world_mut());
    // Manual sync required because component registration via direct World access 
    // updates Zig backend but doesn't automatically sync Rust archetype cache
    app.world_mut().update_archetypes();

    // Register active system
    let move_sys: ParamFunctionSystem<FunctionMarker<((), Query<'static, (&'static mut Position, &'static Velocity)>)>, _> = 
        movement_system.into_system();
    app.add_systems(Update, move_sys);

    println!("Benchmarking AutoZig ECS for {} frames...", FRAMES);

    let start = std::time::Instant::now();
    for _ in 0..FRAMES {
        app.update();
    }
    let duration = start.elapsed();

    println!("AutoZig ECS Result:");
    println!("  Total Time: {:.2?}", duration);
    println!("  Avg Frame:  {:.2?}", duration / FRAMES as u32);
    println!("  Entities:   {}", ENTITY_COUNT);
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

fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    let dt = 0.016;
    static mut ONCE: bool = false;
    unsafe { if !ONCE { println!("System running! Count: {}", query.iter().count()); ONCE = true; } }
    
    for (mut pos, vel) in &mut query {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }
}
