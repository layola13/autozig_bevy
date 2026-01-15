use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
struct Position { x: f32, y: f32 }

#[derive(Component, Clone, Copy, Debug)]
struct Velocity { x: f32, y: f32 }

const ENTITY_COUNT: usize = 100_000;
const FRAMES: usize = 1000;

fn main() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    
    println!("Spawning {} entities...", ENTITY_COUNT);
    setup(&mut app);

    app.add_systems(Update, movement_system);

    println!("Benchmarking Native Bevy ECS for {} frames...", FRAMES);

    let start = std::time::Instant::now();
    for _ in 0..FRAMES {
        app.update();
    }
    let duration = start.elapsed();

    println!("Native Bevy ECS Result:");
    println!("  Total Time: {:.2?}", duration);
    println!("  Avg Frame:  {:.2?}", duration / FRAMES as u32);
    println!("  Entities:   {}", ENTITY_COUNT);
}

fn setup(app: &mut App) {
    let mut rng = rand::thread_rng();
    let world = app.world_mut();
    for _ in 0..ENTITY_COUNT {
        world.spawn((
            Position { x: rng.gen(), y: rng.gen() },
            Velocity { x: rng.gen(), y: rng.gen() }
        ));
    }
}

fn movement_system(mut query: Query<(&mut Position, &Velocity)>) {
    let dt = 0.016;
    for (mut pos, vel) in &mut query {
        pos.x += vel.x * dt;
        pos.y += vel.y * dt;
    }
}
