//! This example illustrates how to react to component and resource changes.

use autozig_ecs::prelude::*;
use autozig_ecs::change_detection::{Ref, DetectChanges};
use rand::{Rng, random};
use std::time::Duration; // No Res<Time> yet in pure ECS, simulating time or using simple counter

// Simulating Time resource for pure ECS example
#[derive(Default)]
struct Time {
    seconds: f32,
}

impl Time {
    fn elapsed_secs(&self) -> f32 {
        self.seconds
    }
}

fn time_updater_system(mut time: ResMut<Time>) {
    time.seconds += 0.1;
}

fn main() {
    let mut app = App::new();
    app.add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(100)));
    
    app.init_resource::<Time>();

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // Setup system
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = setup.into_system();
    app.add_systems(Startup, setup_sys);

    // Time Updater
    let time_sys: ParamFunctionSystem<FunctionMarker<((), ResMut<'static, Time>)>, _> = time_updater_system.into_system();

    // Change Component
    let change_comp_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, Time>, Query<'static, (Entity, &'static mut MyComponent)>)>, _> = change_component.into_system();
    
    // Change Component 2
    let change_comp_2_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, Time>, Query<'static, (Entity, &'static mut MyComponent)>)>, _> = change_component_2.into_system();

    // Change Resource
    let change_res_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, Time>, ResMut<'static, MyResource>)>, _> = change_resource.into_system();

    // Change Detection
    let detect_sys: ParamFunctionSystem<FunctionMarker<((), Query<'static, Ref<'static, MyComponent>, Changed<MyComponent>>, Res<'static, MyResource>)>, _> = change_detection.into_system();

    app.add_systems(
        Update,
        (
            time_sys,
            change_comp_sys,
            change_comp_2_sys,
            change_res_sys,
            detect_sys,
        ).chain(),
    );
    
    println!("Starting Change Detection Example...");
    app.run();
}

#[derive(Component, PartialEq, Debug)]
struct MyComponent(f32);

#[derive(PartialEq, Debug, Default)]
struct MyResource(f32);

fn setup(mut commands: Commands) {
    // Note the first change detection log correctly points to this line because the component is
    // added. Although commands are deferred, they are able to track the original calling location.
    commands.spawn(MyComponent(0.0));
    commands.insert_resource(MyResource(0.0));
}

fn change_component(time: Res<Time>, mut query: Query<(Entity, &'static mut MyComponent)>) {
    for (entity, mut component) in query.iter() {
        if random::<bool>() { // 50% chance for demo speed
            let new_component = MyComponent(time.elapsed_secs().round());
            println!("New value: {:?} {:?}", new_component, entity);
            // Change detection occurs on mutable dereference, and does not consider whether or not
            // a value is actually equal. To avoid triggering change detection when nothing has
            // actually changed, you can use the `set_if_neq` method on any component or resource
            // that implements PartialEq.
            component.set_if_neq(new_component);
        }
    }
}

/// This is a duplicate of the `change_component` system
fn change_component_2(time: Res<Time>, mut query: Query<(Entity, &'static mut MyComponent)>) {
    for (entity, mut component) in query.iter() {
        if random::<bool>() {
            let new_component = MyComponent(time.elapsed_secs().round());
            println!("New value (sys2): {:?} {:?}", new_component, entity);
            component.set_if_neq(new_component);
        }
    }
}

/// Change detection concepts for components apply similarly to resources.
fn change_resource(time: Res<Time>, mut my_resource: ResMut<MyResource>) {
    if random::<bool>() {
        let new_resource = MyResource(time.elapsed_secs().round());
        println!("New resource value: {:?}", new_resource);
        my_resource.set_if_neq(new_resource);
    }
}

/// Query filters like [`Changed<T>`] and [`Added<T>`] ensure only entities matching these filters
/// will be returned by the query.
///
/// Using the [`Ref<T>`] system param allows you to access change detection information, but does
/// not filter the query.
fn change_detection(
    changed_components: Query<Ref<'static, MyComponent>, Changed<MyComponent>>,
    my_resource: Res<MyResource>,
) {
    for component in changed_components.iter() {
        println!(
            "Change detected!\n\t-> value: {:?}\n\t-> added: {}\n\t-> changed: {}",
            component,
            component.is_added(),
            component.is_changed(),
        );
    }

    if my_resource.is_changed() {
        println!(
            "Resource change detected!\n\t-> value: {:?}\n\t-> added: {}\n\t-> changed: {}",
            my_resource,
            my_resource.is_added(),
            my_resource.is_changed(),
        );
    }
}
