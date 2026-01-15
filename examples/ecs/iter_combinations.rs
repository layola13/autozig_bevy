//! iter_combinations - Demonstrate iterating over combinations of entities
//!
//! Ported from Bevy ecs/iter_combinations.rs

use autozig_ecs::prelude::*;

#[derive(Component)]
struct Position(f32, f32);

#[derive(Component)]
struct Velocity(f32, f32);

fn main() {
    let mut app = App::new();

    app.add_plugins(ScheduleRunnerPlugin::run_once());

    // Use explicit type annotations for system conversion (required workaround)
    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
    
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = setup.into_system();
    let repulsion_sys: ParamFunctionSystem<FunctionMarker<((), Query<'static, (&'static Position, &'static mut Velocity)>)>, _> = calculate_repulsion.into_system();

    app.add_systems(Startup, setup_sys);
    app.add_systems(Update, repulsion_sys);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Position(0.0, 0.0), Velocity(0.0, 0.0)));
    commands.spawn((Position(1.0, 1.0), Velocity(0.0, 0.0)));
    commands.spawn((Position(2.0, 0.0), Velocity(0.0, 0.0)));
}

fn calculate_repulsion(query: Query<(&Position, &mut Velocity)>) {
    // We want to calculate repulsion between all pairs of entities
    // In Bevy this is query.iter_combinations_mut::<2>()
    // For now we use our implementation
    
    // Note: Our current implementation of combinations is read-only or disjoint-safe
    // But since we have &mut Velocity, we need iter_combinations_mut or similar.
    // However, iter_combinations on a Query with &mut works if the implementation
    // handles it correctly.
    
    let mut combinations = query.iter_combinations::<2>();
    while let Some([entity_a, entity_b]) = combinations.next() {
        let (pos_a, mut vel_a) = entity_a;
        let (pos_b, mut vel_b) = entity_b;

        let dx = pos_a.0 - pos_b.0;
        let dy = pos_a.1 - pos_b.1;
        let distance_sq = dx * dx + dy * dy;

        if distance_sq > 0.0 {
            let force = 1.0 / distance_sq;
            vel_a.0 += force * dx;
            vel_a.1 += force * dy;
            vel_b.0 -= force * dx;
            vel_b.1 -= force * dy;
        }
    }
    
    println!("Repulsion calculated for all pairs.");
}
