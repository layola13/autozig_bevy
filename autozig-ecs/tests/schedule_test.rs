use autozig_ecs::prelude::*;
use autozig_ecs::schedule::{ScheduleRunnerPlugin, Update};

#[derive(Default)]
struct Counter(i32);

#[derive(Component)]
struct Marker;

fn increment_system(mut counter: ResMut<Counter>) {
    counter.0 += 1;
}

fn spawn_system(mut commands: Commands) {
    commands.spawn(Marker);
}

fn query_system(query: Query<&'static Marker>, mut counter: ResMut<Counter>) {
    // For each marker, add 10 to counter
    for _ in query.iter() {
        counter.0 += 10;
    }
}

#[test]
fn test_simple_schedule() {
    let mut app = App::new();
    app.init_resource::<Counter>();
    app.world_mut().register_component::<Marker>();
    app.add_plugins(ScheduleRunnerPlugin::run_once());
    
    // Explicit cast needed for inference
    let inc_sys = IntoSystem::<(ResMut<'static, Counter>,)>::into_system(increment_system);
    app.add_systems(Update, inc_sys);
    
    app.update();
    
    let counter = app.world_mut().resource::<Counter>();
    assert_eq!(counter.0, 1, "System should have run once");
}

#[test]
fn test_complex_schedule() {
    let mut app = App::new();
    app.init_resource::<Counter>();
    app.world_mut().register_component::<Marker>();
    app.add_plugins(ScheduleRunnerPlugin::run_once());
    
    // Explicit cast needed for inference
    use autozig_ecs::into_system::IntoSystem;
    let system = IntoSystem::<(Commands<'static>,)>::into_system(spawn_system);
    app.add_systems(Update, system);
    
    // Manually spawn a marker so query has something to iterate
    app.world_mut().spawn(Marker);
    
    let query_sys = IntoSystem::<(Query<'static, &'static Marker>, ResMut<'static, Counter>)>::into_system(query_system);
    app.add_systems(Update, query_sys); // Adds 10
    
    let inc_sys = IntoSystem::<(ResMut<'static, Counter>,)>::into_system(increment_system);
    app.add_systems(Update, inc_sys); // Adds 1
    
    app.update();
    
    let counter = app.world_mut().resource::<Counter>();
    assert_eq!(counter.0, 11, "Systems should run in order and update resource");
}
