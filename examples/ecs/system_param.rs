//! This example illustrates how to use different system parameters.

use autozig_ecs::prelude::*;

#[derive(Resource)]
pub struct MyResource(pub u32);

#[derive(Component)]
pub struct MyComponent(pub u32);

#[derive(Resource, Clone, Copy, Debug)]
pub struct MyEvent(pub u32);

fn main() {
    let mut app = App::new();
    
    // Run once for demonstration
    app.add_plugins(ScheduleRunnerPlugin::run_once());
    app.insert_resource(MyResource(10));
    app.insert_resource(Events::<MyEvent>::new());

    use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};

    // Explicitly typed systems
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = setup.into_system();
    let complex_sys: ParamFunctionSystem<FunctionMarker<((), 
        Res<'static, MyResource>, 
        Query<'static, &MyComponent>, 
        EventWriter<'static, MyEvent>, 
        SystemName<'static>
    )>, _> = system_with_many_params.into_system();

    app.add_systems(Startup, setup_sys);
    app.add_systems(Update, complex_sys);
    
    println!("Starting System Param Example...");
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(MyComponent(20));
}

fn system_with_many_params(
    res: Res<MyResource>,
    query: Query<&'static MyComponent>,
    mut event_writer: EventWriter<MyEvent>,
    name: SystemName,
) {
    let component = query.single().expect("Component not found");
    println!("System Name: {}", name.0);
    println!("Resource Value: {}", res.0);
    println!("Component Value: {}", component.0);
    event_writer.send(MyEvent(res.0 + component.0));
    println!("Event sent with value: {}", res.0 + component.0);
}
