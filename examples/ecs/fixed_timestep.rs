//! fixed_timestep - Demonstrate fixed timestep systems
//!
//! Ported from Bevy ecs/fixed_timestep.rs

use autozig_ecs::prelude::*;
use autozig_time::{Time, Fixed};

fn main() {
    let mut app = App::new();

    // Run for a limited number of iterations to demonstrate
    app.add_plugins(ScheduleRunnerPlugin::run_loop(std::time::Duration::from_millis(100)));
    
    // Use explicit type annotations for system conversion
    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
    
    // Regular update system - runs every frame
    let frame_sys: ParamFunctionSystem<FunctionMarker<((), 
        Local<'static, f32>, 
        Res<'static, Time>
    )>, _> = frame_update.into_system();
    
    // Fixed update system - runs at fixed intervals
    let fixed_sys: ParamFunctionSystem<FunctionMarker<((),
        Local<'static, u32>,
        Res<'static, Time>
    )>, _> = fixed_update.into_system();

    app.add_systems(Update, frame_sys);
    app.add_systems(FixedUpdate, fixed_sys);
    
    // Configure fixed timestep to run at 2 Hz (every 0.5 seconds)
    app.insert_resource(Fixed::from_seconds(0.5));

    println!("Starting Fixed Timestep Example...");
    println!("Frame updates run every tick, fixed updates run at 2 Hz");
    app.run();
}

fn frame_update(mut frame_count: Local<f32>, time: Res<Time>) {
    *frame_count += 1.0;
    println!(
        "[Frame {}] elapsed: {:.3}s, delta: {:.3}s",
        *frame_count as u32,
        time.elapsed_seconds(),
        time.delta_seconds()
    );
}

fn fixed_update(mut tick_count: Local<u32>, time: Res<Time>) {
    *tick_count += 1;
    println!(
        "  [Fixed Tick {}] elapsed: {:.3}s, delta: {:.3}s",
        *tick_count,
        time.elapsed_seconds(),
        time.delta_seconds()
    );
}
