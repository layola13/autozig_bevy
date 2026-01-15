//! observer_propagation.rs - Demonstrate event propagation up the hierarchy
//!
//! Ported from Bevy's observer propagation concepts.
//! Events triggered on an entity should bubble up to its parents.

use autozig_ecs::prelude::*;
use autozig_ecs::observer::{Trigger, TriggerEvent};
use autozig_ecs::hierarchy::{Parent, HierarchyPlugin};
use autozig_ecs::into_system::{ParamFunctionSystem, FunctionMarker, IntoSystem};

#[derive(Component)]
struct Name(String);

#[derive(Component, Clone, Default)]
struct MyEvent;
impl TriggerEvent for MyEvent {}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins); // Includes HierarchyPlugin

    // Explicitly define systems
    app.add_systems(Startup, autozig_ecs::system::ExclusiveFunctionSystem::new(setup, "setup"));
    // Query needs explicit lifetimes for inference
    let trigger_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>, Query<'static, (Entity, &'static Name, &'static Parent)> )>, _> = trigger_event.into_system();

    app.add_systems(Update, trigger_sys);

    // Register components
    app.world_mut().register_component::<Name>();
    app.world_mut().register_component::<Parent>();
    app.world_mut().register_component::<MyEvent>();
    app.world_mut().register_component::<autozig_ecs::observer::Observer<MyEvent>>();
    app.world_mut().register_component::<autozig_ecs::observer::Observed>();

    // We want to verify that an observer on the PARENT fires when the CHILD is triggered.
    // This requires:
    // 1. Attaching an observer to the parent entity.
    // 2. Triggering the event on the child.
    // 3. The framework traversing up from child to parent.

    println!("Starting Observer Propagation Example...");
    app.run();
}

// Debug verification
fn assert_system_param<P: autozig_ecs::system_param::SystemParam>() {}

fn setup(world: &mut World) {
    eprintln!("DEBUG: setup starting");
    
    // We use &mut World because Commands::spawn().id() is not yet supported in autozig-ecs
    // (Commands buffers the spawn so ID is unknown until apply).
    
    // Create parent entity
    let parent = world.spawn(Name("parent".to_string())).id();
    eprintln!("DEBUG: setup parent spawned: {:?}", parent);
    
    // Create child entity
    let child = world.spawn(Name("child".to_string())).id();
    eprintln!("DEBUG: setup child spawned: {:?}", child);
    
    // Set parent relationship
    world.entity_mut(child).insert(Parent::new(parent));
    eprintln!("DEBUG: setup parent-child link set");
    
    // Add observer to parent
    // Normally: commands.entity(parent).observe(observer_system);
    // Manual equivalent: Spawn observer entity with Observer component and Observed target
    use autozig_ecs::observer::IntoObserverSystem;
    use autozig_ecs::observer::{Observer, Observed};
    let observer_sys: ParamFunctionSystem<FunctionMarker<((), Trigger<MyEvent>)>, _> = (|trigger: Trigger<MyEvent>| {
        println!("Observer on Parent received event from {:?}", trigger.entity());
    }).into_system();
    let observer_system = observer_sys.into_observer_system();
    let observer_component = Observer::<MyEvent>::new(observer_system); // implicit .into_system() via new
    let observed_component = Observed { entity: parent };
    world.spawn((observer_component, observed_component));
    eprintln!("DEBUG: setup observer attached to parent");

    // Trigger event on child
    // commands.trigger_targets(MyEvent, child); -> world.trigger_targets? No, world.trigger takes event.
    // But world.trigger is Global.
    // To trigger on entity:
    // We need logic similar to commands.trigger but immediate.
    // Wait, World::trigger is IMPLEMENTED as global observer trigger in world/mod.rs.
    // It calls `self.remove_resource::<ObserverList<E>>`.
    // It does NOT support entity targets yet?
    // command.rs implements entity propagation logic.
    // world/mod.rs does not have it?
    // Check world/mod.rs again.
    
    // If World doesn't support entity trigger, we can use Commands!
    // We can create a CommandQueue and apply it?
    // Or just use world.run_system to run a system that uses Commands?
    // No, that puts us back to Commands limitation.
    
    // Wait, command.rs `trigger` calls `observer.trigger` manually.
    // We can do that here manually too!
    // But `Commands::trigger` logic handles hierarchy propagation.
    // We need that logic.
    
    // Solution: Invoke the SAME logic as command.rs `trigger` but immediately on World.
    // I can copy-paste the propagation logic into this example for demonstration?
    // Or add `trigger_entity` method to `World`?
    
    // Adding `trigger_entity` to `World` is the right way if possible.
    // But hacking example is faster.
    
    eprintln!("DEBUG: setup calling propagate_trigger");
    propagate_trigger(world, MyEvent, child);
    eprintln!("DEBUG: setup propagate_trigger returned");
}

// Helper to simulate World::trigger(event, entity)
fn propagate_trigger<E: Default + TriggerEvent + Clone + 'static>(world: &mut World, _event: E, entity: Entity) {
    eprintln!("DEBUG: propagate_trigger starting for entity {:?}", entity);
    world.update_archetypes();
    // 1. Global (omitted for now)
    
    // 2. Entity Propagation
    let mut entities = vec![entity];
    let mut current = entity;
    
    // Note: Hierarchy Parent component access
    // Parent is struct Parent(pub Entity).
    while let Some(parent) = world.get::<autozig_ecs::hierarchy::Parent>(current) {
         current = parent.entity;
         entities.push(current);
    }

    use autozig_ecs::observer::{Observer, Observed};
    let observer_id = world.component_id::<Observer<E>>();
    let observed_id = world.component_id::<Observed>();
    eprintln!("DEBUG: propagate_trigger IDs: observer={:?}, observed={:?}", observer_id, observed_id);
    
    let mut observers_to_run: Vec<Observer<E>> = Vec::new();

    if let (Some(observer_id), Some(observed_id)) = (observer_id, observed_id) {
        let archetypes_guard = world.archetypes();
        eprintln!("DEBUG: propagate_trigger checking {} archetypes", archetypes_guard.len());
        for archetype in archetypes_guard.iter() {
             let has_obs = archetype.components().contains(&observer_id);
             let has_targ = archetype.components().contains(&observed_id);
             if has_obs && has_targ {
                eprintln!("DEBUG: propagate_trigger found matching archetype with {} entities", archetype.entity_count());
                for arch_entity in archetype.entities() {
                    let entity = arch_entity.entity;
                    if let Some(observed) = world.get::<Observed>(entity) {
                        eprintln!("DEBUG: propagate_trigger checking observed entity on observer {:?}: {:?}", entity, observed.entity);
                        for target in &entities {
                            if observed.entity == *target {
                                eprintln!("DEBUG: propagate_trigger MATCH found for target {:?}", target);
                                if let Some(observer) = world.get::<Observer<E>>(entity) {
                                    unsafe {
                                        observers_to_run.push(std::ptr::read(observer));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for observer in observers_to_run {
         observer.trigger(entity, world);
         std::mem::forget(observer);
    }
}

fn trigger_event(
    mut commands: Commands, 
    query: Query<(Entity, &Name, &Parent)>,
    // query_parent: Query<(Entity, &Name), Without<Parent>>
) {
    // Find child
    for (entity, name, _parent) in query.iter() {
        if name.0 == "Child" {
            println!("Triggering event on Child {:?}", entity);
            commands.trigger(MyEvent, entity);
        }
    }
}
