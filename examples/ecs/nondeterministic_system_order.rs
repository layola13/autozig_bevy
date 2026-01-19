//! nondeterministic_system_order.rs - Demonstrate system order ambiguity detection
//!
//! Ported from Bevy examples/ecs/nondeterministic_system_order.rs

use autozig_ecs::prelude::*;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use autozig_ecs::schedule::{LogLevel, ScheduleBuildSettings};

#[derive(Debug, Default)]
struct A(usize);

#[derive(Debug, Default)]
struct B(usize);

fn main() {
    let mut app = App::new();
    
    app.edit_schedule(Update, |schedule| {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..Default::default()
            });
        });
        
    app.init_resource::<A>();
    app.init_resource::<B>();
    
    // Systems
    let reads_a_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, A>)>, _> = reads_a.into_system();
    let writes_a_sys: ParamFunctionSystem<FunctionMarker<((), ResMut<'static, A>)>, _> = writes_a.into_system();
    
    let adds_one_to_b_sys: ParamFunctionSystem<FunctionMarker<((), ResMut<'static, B>)>, _> = adds_one_to_b.into_system();
    let doubles_b_sys: ParamFunctionSystem<FunctionMarker<((), ResMut<'static, B>)>, _> = doubles_b.into_system();
    let reads_b_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, B>)>, _> = reads_b.into_system();
    
    let reads_a_and_b_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, A>, Res<'static, B>)>, _> = reads_a_and_b.into_system();

    app.add_systems(
            Update,
            (
                reads_a_sys,
                writes_a_sys,
                adds_one_to_b_sys,
                // Manually specifying system names as strings because FunctionSystem doesn't implement IntoSystemSet directly in this port
                doubles_b_sys.after("nondeterministic_system_order::adds_one_to_b"),
                reads_b_sys.after("nondeterministic_system_order::doubles_b"),
                // ambiguous_with takes string in our mocked impl
                reads_a_and_b_sys.ambiguous_with("nondeterministic_system_order::adds_one_to_b"),
            ),
        );
        
    app.add_plugins(DefaultPlugins);

    println!("Running nondeterministic system order example...");
    app.set_runner(|mut app| {
        for _ in 0..5 {
            app.update();
        }
    });

    app.run();
}

fn reads_a(_a: Res<A>) {
    // Read A
}

fn writes_a(mut a: ResMut<A>) {
    a.0 += 1;
}

fn adds_one_to_b(mut b: ResMut<B>) {
    b.0 = b.0.saturating_add(1);
}

fn doubles_b(mut b: ResMut<B>) {
    b.0 = b.0.saturating_mul(2);
}

fn reads_b(b: Res<B>) {
    // This invariant is always true,
    // because we've fixed the system order so doubling always occurs after adding.
    assert!((b.0 % 2 == 0) || (b.0 == usize::MAX));
}

fn reads_a_and_b(a: Res<A>, b: Res<B>) {
    // Only display the first few steps to avoid burying the ambiguities in the console
    if b.0 < 10 {
        println!("A: {}, B: {}", a.0, b.0);
    }
}
