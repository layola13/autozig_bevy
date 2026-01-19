//! ecs_guide.rs - Comprehensive ECS guide example
//!
//! Ported from Bevy examples/ecs/ecs_guide.rs

use autozig_ecs::prelude::*;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use autozig_time::{Time, Timer, TimerMode};
use rand::Rng; // rand 0.8 is in Cargo.toml

// Components
#[derive(Component, Debug)]
struct Position { x: f32, y: f32 }

#[derive(Component, Debug)]
struct Velocity { x: f32, y: f32 }

// Resources

struct Score { value: usize }


struct StarSpawnTimer(Timer);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    
    // Register Components (autozig-ecs safety)
    app.world_mut().register_component::<Position>();
    app.world_mut().register_component::<Velocity>();
    
    // Register Resources
    app.init_resource::<Score>(); // Uses Default
    app.insert_resource(StarSpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));

    // Systems
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = 
        setup.into_system();
    
    let move_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, Time>, Query<'static, (&'static mut Position, &'static Velocity)>)>, _> = 
        movement.into_system();
        
    let score_sys: ParamFunctionSystem<FunctionMarker<((), ResMut<'static, Score>)>, _> = 
         update_score.into_system();
         
    // spawn_stars system needs Random
    let spawn_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>, Res<'static, Time>, ResMut<'static, StarSpawnTimer>)>, _> = 
         spawn_stars.into_system();

    app.add_systems(Startup, setup_sys);
    app.add_systems(Update, move_sys);
    app.add_systems(Update, score_sys);
    app.add_systems(Update, spawn_sys);

    println!("Starting ECS Guide Example...");
    // Runner loop
    app.set_runner(|mut app| {
        println!("Game Loop Running...");
        for i in 0..10 {
            app.update();
            std::thread::sleep(std::time::Duration::from_millis(100));
            // Print score from world if possible? 
            // We can add a print system or just trust internal prints.
        }
    });
    
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 1.0, y: 1.0 }
    ));
}

fn movement(time: Res<Time>, mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.x += vel.x * time.delta_seconds();
        pos.y += vel.y * time.delta_seconds();
        // println!("Entity moved to {:?}", pos);
    }
}

// Default impl for Score
impl Default for Score {
    fn default() -> Self { Score { value: 0 } }
}

fn update_score(mut score: ResMut<Score>) {
    score.value += 1;
    println!("Score: {}", score.value);
}

fn spawn_stars(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<StarSpawnTimer>
) {
    timer.0.tick(time.delta_nanos());
    if timer.0.just_finished() {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-10.0..10.0);
        let y = rng.gen_range(-10.0..10.0);
        
        commands.spawn((
            Position { x, y },
            Velocity { x: 0.0, y: 0.0 } // Stationary star
        ));
        println!("Swpaned star at ({}, {})", x, y);
    }
}
