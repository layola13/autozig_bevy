//! one_shot_systems - Demonstrate one-shot system execution
//!
//! Ported from Bevy ecs/one_shot_systems.rs
//!
//! Shows how to run systems on-demand rather than every frame.
//! Note: Full one-shot API (SystemId, run_system) requires additional implementation.

use autozig_ecs::prelude::*;

#[derive(Default)]
struct Counter(u32);


struct ShouldRunOnce(bool);

fn main() {
    let mut app = App::new();

    app.add_plugins(ScheduleRunnerPlugin::run_once());
    app.init_resource::<Counter>();
    app.insert_resource(ShouldRunOnce(true));

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // Regular system that runs every tick
    let regular_sys: ParamFunctionSystem<FunctionMarker<((), ResMut<'static, Counter>)>, _> = 
        regular_system.into_system();

    // Conditional "one-shot" style system
    let oneshot_sys: ParamFunctionSystem<FunctionMarker<((), 
        ResMut<'static, ShouldRunOnce>,
        ResMut<'static, Counter>
    )>, _> = oneshot_style_system.into_system();

    app.add_systems(Update, regular_sys);
    app.add_systems(Update, oneshot_sys);

    println!("Starting One-Shot Systems Example...");
    println!("Note: Full run_system_once() API requires SystemId implementation");
    app.run();
}

fn regular_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("Regular system ran. Counter: {}", counter.0);
}

fn oneshot_style_system(
    mut should_run: ResMut<ShouldRunOnce>,
    mut counter: ResMut<Counter>,
) {
    if should_run.0 {
        counter.0 += 100;
        println!("One-shot system triggered! Counter jumped to: {}", counter.0);
        should_run.0 = false; // Prevent running again
    }
}
