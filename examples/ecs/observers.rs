//! observers - Demonstrate observer pattern
//!
//! Ported from Bevy ecs/observers.rs
//!
//! Shows how to use observers to react to triggers/events.

use autozig_ecs::prelude::*;
use autozig_ecs::observer::{Trigger, TriggerEvent, OnAdd, OnRemove};

#[derive(Component, Debug, Clone)]
struct Mine {
    size: f32,
}

#[derive(Component)]
struct Position { x: f32, y: f32 }

// Custom trigger event
#[derive(Clone, Default, Component)]
struct Explode;
impl TriggerEvent for Explode {}

fn main() {
    let mut app = App::new();

    app.add_plugins(ScheduleRunnerPlugin::run_once());

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // Setup system
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = 
        setup.into_system();

    // Trigger system
    let trigger_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>, Query<'static, Entity, With<Mine>>)>, _> = 
        trigger_explosion.into_system();

    // Register components
    println!("Registering components...");
    let mine_id = app.world_mut().register_component::<Mine>();
    let pos_id = app.world_mut().register_component::<Position>();
    let explode_id = app.world_mut().register_component::<Explode>();
    println!("Registered components: Mine={:?}, Pos={:?}, Explode={:?}", mine_id, pos_id, explode_id);
    
    // Verify valid IDs immediately
    let world = app.world_mut();
    let mine_valid = world.components().get_valid_id(std::any::TypeId::of::<Mine>());
    println!("Mine valid ID check: {:?}", mine_valid);
    
    // Register Observer components
    world.register_component::<autozig_ecs::observer::Observer<Explode>>();
    world.register_component::<autozig_ecs::observer::Observer<OnAdd<Mine>>>();
    world.register_component::<autozig_ecs::observer::Observer<OnRemove<Mine>>>();

    app.add_systems(Startup, setup_sys);
    app.add_systems(Update, trigger_sys);
    
    // Add observers
    // Note: Manual closure conversion might be needed if IntoObserverSystem isn't fully robust for closures yet
    // But let's try direct implementation first.
    
    // Observer for custom event `Explode`
    // Manual conversion to help type inference
    let explode_observer: ParamFunctionSystem<FunctionMarker<((), Trigger<Explode>, Query<'static, &'static Mine>)>, _> = (|trigger: Trigger<Explode>, query: Query<&Mine>| {
        let entity = trigger.entity();
        if let Ok(mine) = query.get(entity) {
             println!("BOOM! Mine at {:?} exploded! Size: {}", entity, mine.size);
        } else {
             println!("Explosion triggered on {:?}, but no mine found!", entity);
        }
    }).into_system();
    app.add_observer::<Explode, _>(explode_observer);

    // Observer for OnAdd component lifecycle
    let add_observer: ParamFunctionSystem<FunctionMarker<((), Trigger<OnAdd<Mine>>)>, _> = (|trigger: Trigger<OnAdd<Mine>>| {
        println!("Mine added to entity {:?}", trigger.entity());
    }).into_system();
    app.add_observer::<OnAdd<Mine>, _>(add_observer);
    
    // Observer for OnRemove component lifecycle
    let remove_observer: ParamFunctionSystem<FunctionMarker<((), Trigger<OnRemove<Mine>>)>, _> = (|trigger: Trigger<OnRemove<Mine>>| {
        println!("Mine removed from entity {:?}", trigger.entity());
    }).into_system();
    app.add_observer::<OnRemove<Mine>, _>(remove_observer);

    println!("Starting Observers Example...");
    app.run();
}

fn setup(mut commands: Commands) {
    println!(" planting mines...");
    commands.spawn((
        Mine { size: 10.0 },
        Position { x: 0.0, y: 0.0 }
    ));
    commands.spawn((
        Mine { size: 5.0 },
        Position { x: 10.0, y: 10.0 }
    ));
}

fn trigger_explosion(mut commands: Commands, query: Query<Entity, With<Mine>>) {
    println!("Triggering explosions...");
    // Trigger Explode event for all mines
    for entity in query.iter() {
        commands.trigger(Explode, entity);
    }
}
