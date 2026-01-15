//! system_stepping.rs - Demonstrate stepping through systems
//!
//! Ported from Bevy examples/ecs/system_stepping.rs

use autozig_ecs::prelude::*;
use autozig_ecs::schedule::Stepping;
use autozig_ecs::into_system::IntoSystem;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            (
                update_system_one,
                // establish a dependency here to simplify descriptions below
                update_system_two.after("system_stepping::update_system_one"),
                update_system_three.after("system_stepping::update_system_two"),
                update_system_four,
            ),
        )
        .add_systems(PreUpdate, pre_update_system);

    // For the simplicity of this example, we directly modify the `Stepping`
    // resource here and run the systems with `App::update()`.
    println!(
        r#"
    Actions: call app.update()
     Result: All systems run normally"#
    );
    app.update();

    println!(
        r#"
    Actions: Add the Stepping resource then call app.update()
     Result: All systems run normally.  Stepping has no effect unless explicitly
             configured for a Schedule, and Stepping has been enabled."#
    );
    app.insert_resource(Stepping::new());
    app.update();

    println!(
        r#"
    Actions: Add the Update Schedule to Stepping; enable Stepping; call
             app.update()
     Result: Only the systems in PreUpdate run.
    "#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.add_schedule(Update).enable();
    app.update();

    println!(
        r#"
    Actions: call Stepping.step_frame(); call app.update()
     Result: The PreUpdate systems run, and one Update system will run.
    "#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.step_frame();
    app.update();

    println!(
        r#"
    Actions: call app.update()
     Result: Only the PreUpdate systems run.
    "#
    );
    app.update();

    println!(
        r#"
    Actions: call Stepping::continue_frame(); call app.update()
     Result: PreUpdate system will run, and all remaining Update systems will run.
    "#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.continue_frame();
    app.update();

    println!(
        r#"
    Actions: call Stepping::step_frame() & app.update() four times in a row
     Result: PreUpdate system runs every time we call app.update(), along with
             one system from the Update schedule each time.
    "#
    );
    for _ in 0..4 {
        let mut stepping = app.world_mut().resource_mut::<Stepping>();
        stepping.step_frame();
        app.update();
    }

    println!(
        r#"
    Actions: Stepping::always_run(Update, update_system_two); step through all
             systems
     Result: PreUpdate system and update_system_two() will run every time we
             call app.update().
    "#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.always_run(Update, "system_stepping::update_system_two");
    for _ in 0..3 {
        let mut stepping = app.world_mut().resource_mut::<Stepping>();
        stepping.step_frame();
        app.update();
    }

    println!(
        r#"
    Actions: Stepping::never_run(Update, update_system_two); continue through
             all systems
     Result: All systems except update_system_two() will execute.
    "#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.never_run(Update, "system_stepping::update_system_two");
    stepping.continue_frame();
    app.update();

    println!(
        r#"
    Actions: Stepping::set_breakpoint(Update, update_system_two); continue,
             step, continue
     Result: Stepping stops system execution at breakpoint.
    "#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.set_breakpoint(Update, "system_stepping::update_system_two");
    stepping.continue_frame();
    app.update();
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.step_frame();
    app.update();
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.continue_frame();
    app.update();

    println!(
        r#"
    Actions: Stepping::clear_breakpoint(Update, update_system_two); continue
             through all systems
     Result: All systems will run"#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.clear_breakpoint(Update, "system_stepping::update_system_two");
    stepping.continue_frame();
    app.update();

    println!(
        r#"
    Actions: Stepping::disable(); app.update()
     Result: All systems will run.
    "#
    );
    let mut stepping = app.world_mut().resource_mut::<Stepping>();
    stepping.disable();
    app.update();
}

fn pre_update_system() {
    println!("▶ pre_update_system");
}
fn update_system_one() {
    println!("▶ update_system_one");
}
fn update_system_two() {
    println!("▶ update_system_two");
}
fn update_system_three() {
    println!("▶ update_system_three");
}
fn update_system_four() {
    println!("▶ update_system_four");
}
