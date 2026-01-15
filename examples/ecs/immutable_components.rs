//! immutable_components.rs - Demonstrate immutable components and hooks
//!
//! Ported from Bevy examples/ecs/immutable_components.rs

use autozig_ecs::prelude::*;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use autozig_ecs::component_advanced::HookContext;
use autozig_ecs::world::DeferredWorld;
use std::collections::HashMap;

// Components
#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
struct Name(String);

#[derive(Component, Debug, Clone)]
struct ImmutableTag; 

// Resource
#[derive(Resource, Default)]
struct NameIndex {
    name_to_entity: HashMap<String, Entity>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    
    app.init_resource::<NameIndex>();
    
    // Register component hooks using builder pattern
    // Note: register_component_hooks returns a builder in autozig-ecs
    app.world_mut().register_component_hooks::<Name>()
        .on_insert(|mut world: DeferredWorld, context: HookContext| {
            println!("Hook: On Insert Name for entity {:?}", context.entity);
        })
        .on_remove(|world: DeferredWorld, context: HookContext| {
            println!("Hook: On Remove Name for entity {:?}", context.entity);
        });
    
    // Systems
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = setup_names.into_system();
    app.add_systems(Startup, setup_sys);
    
    let check_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, NameIndex>)>, _> = check_index.into_system();
    app.add_systems(Update, check_sys);

    println!("Starting Immutable Components Example...");
    app.set_runner(|mut app| {
        app.update();
    });
    app.run();
}

fn setup_names(mut commands: Commands) {
    let e1 = commands.spawn(Name("Alice".to_string())).id();
    let e2 = commands.spawn(Name("Bob".to_string())).id();
    println!("Spawned Alice {:?} and Bob {:?}", e1, e2);
}

fn check_index(index: Res<NameIndex>) {
    println!("Index check placeholder.");
}
