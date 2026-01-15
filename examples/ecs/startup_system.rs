//! This example illustrates how to use startup systems.

use autozig_ecs::prelude::*;

fn main() {
    let mut app = App::new();
    
    // Run once for demonstration
    app.add_plugins(ScheduleRunnerPlugin::run_once());

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // Explicitly typed systems
    let startup_sys: ParamFunctionSystem<FunctionMarker<((),)>, _> = startup_system.into_system();
    let normal_sys: ParamFunctionSystem<FunctionMarker<((),)>, _> = normal_system.into_system();

    app.add_systems(Startup, startup_sys);
    app.add_systems(Update, normal_sys);
    
    println!("Starting Startup System Example...");
    app.run();
}

fn startup_system() {
    println!("Startup system ran! (This should run once before normal systems)");
}

fn normal_system() {
    println!("Normal system ran! (This should run after startup systems)");
}
