//! entity_disabling.rs - Demonstrate entity disabling
//!
//! Ported from Bevy examples/ecs/entity_disabling.rs (Manual Implementation)

use autozig_ecs::prelude::*;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

#[derive(Component)]
struct Disabled;

#[derive(Component)]
struct Name(String);

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    
    app.world_mut().register_component::<Disabled>();
    app.world_mut().register_component::<Name>();

    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = setup.into_system();
    app.add_systems(Startup, setup_sys);

    let list_sys: ParamFunctionSystem<FunctionMarker<((), Query<'static, &'static Name, Without<Disabled>>)>, _> = list_active_entities.into_system();
    app.add_systems(Update, list_sys);
    
    // Simulate toggling disabling for demonstration
    // app.add_systems(Update, toggle_disabled.into_system());

    println!("Starting Entity Disabling Example...");
    app.set_runner(|mut app| {
        for _ in 0..5 {
            app.update();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Name("Alice (Active)".to_string()));
    commands.spawn((
        Name("Bob (Disabled)".to_string()),
        Disabled
    ));
    commands.spawn(Name("Charlie (Active)".to_string()));
}

fn list_active_entities(query: Query<&Name, Without<Disabled>>) {
    println!("Active entities:");
    for name in query.iter() {
        println!(" - {}", name.0);
    }
}
