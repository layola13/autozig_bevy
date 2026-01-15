//! run_conditions - Demonstrate conditional system execution
//!
//! Ported from Bevy ecs/run_conditions.rs
//!
//! Shows how to use run conditions to control when systems run.

use autozig_ecs::prelude::*;

#[derive(Resource, Default)]
struct Counter(u32);

#[derive(Resource)]
struct EnableSystem(bool);

fn main() {
    let mut app = App::new();

    app.add_plugins(ScheduleRunnerPlugin::run_once());
    app.init_resource::<Counter>();
    app.insert_resource(EnableSystem(true));

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // System that increments counter
    let increment_sys: ParamFunctionSystem<FunctionMarker<((), ResMut<'static, Counter>)>, _> = 
        increment_counter.into_system();

    // System that prints counter
    let print_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, Counter>)>, _> = 
        print_counter.into_system();

    // Conditional system - only runs if enabled
    let conditional_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, EnableSystem>)>, _> = 
        conditional_message.into_system();

    // Add systems
    app.add_systems(Update, increment_sys);
    app.add_systems(Update, print_sys);
    app.add_systems(Update, conditional_sys);

    println!("Starting Run Conditions Example...");
    println!("(Note: .run_if() requires full Condition API implementation)");
    app.run();
}

fn increment_counter(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("Counter incremented to: {}", counter.0);
}

fn print_counter(counter: Res<Counter>) {
    println!("Current counter value: {}", counter.0);
}

fn conditional_message(enable: Res<EnableSystem>) {
    if enable.0 {
        println!("Conditional system is ENABLED and running!");
    }
}
