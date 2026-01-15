//! system_closure.rs - Demonstrate uses of closures as systems
//!
//! Ported from Bevy examples/ecs/system_closure.rs

use autozig_ecs::prelude::*;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use autozig_time::{Time, Timer, TimerMode};

fn main() {
    // create a simple closure.
    let simple_closure = || {
        // this is a closure that does nothing.
        println!("Hello from a simple closure!");
    };

    // create a closure, with an 'input' value.
    let complex_closure = |mut value: String| {
        move || {
            println!("Hello from a complex closure! {}", value);

            // we can modify the value inside the closure. this will be saved between calls.
            value = format!("{value} - updated");
        }
    };

    let outside_variable = "bar".to_string();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    
    // Explicit type hints needed for closures in autozig-ecs until full inference parity
    let simple_closure_sys: ParamFunctionSystem<FunctionMarker<((),)>, _> = simple_closure.into_system();
    
    app.add_systems(Update, simple_closure_sys);

    let complex_closure_instance = complex_closure("foo".into());
    let complex_closure_sys: ParamFunctionSystem<FunctionMarker<((),)>, _> = complex_closure_instance.into_system();

    app.add_systems(Update, complex_closure_sys);

    // we can also inline a closure - but need explicit cast currently
    let inline_closure = || {
         println!("Hello from an inlined closure!");
    };
    let inline_closure_sys: ParamFunctionSystem<FunctionMarker<((),)>, _> = inline_closure.into_system();
    app.add_systems(Update, inline_closure_sys);

    // or use variables outside a closure
    let moved_closure = move || {
        println!(
            "Hello from an inlined closure that captured the 'outside_variable'! {}",
            outside_variable
        );
    };
    let moved_closure_sys: ParamFunctionSystem<FunctionMarker<((),)>, _> = moved_closure.into_system();
    app.add_systems(Update, moved_closure_sys);
    
    // Run limited frames
    app.set_runner(|mut app| {
        println!("Running 3 frames...");
        for _ in 0..3 {
            app.update();
        }
    });

    app.run();
}
