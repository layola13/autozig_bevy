use autozig_ecs::prelude::*;
use autozig_ecs::schedule::{ScheduleRunnerPlugin, Update};
use autozig_ecs::system::BoxedSystem;
use autozig_ecs::into_system::IntoSystem;

#[derive(Default, Clone)]
struct OrderLog(Vec<String>);

fn sys_a(mut log: ResMut<OrderLog>) {
    log.0.push("A".to_string());
}

fn sys_b(mut log: ResMut<OrderLog>) {
    log.0.push("B".to_string());
}

#[test]
fn test_ordering_before() {
    let mut app = App::new();
    app.init_resource::<OrderLog>();
    app.add_plugins(ScheduleRunnerPlugin::run_once());

    // Explicitly name SystemB
    let sys_b_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_b);
    let sys_b = BoxedSystem::new(sys_b_sys, "SystemB");
    
    // Explicitly name SystemA just in case, but rely on .before("SystemB")
    let sys_a_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_a);
    let sys_a = BoxedSystem::new(sys_a_sys, "SystemA");

    // Add B first? No, add dependency constraint.
    // If we add (B, A.before(B)), order should be A -> B.
    // Standard registration order without dependencies would usually be B -> A (or A -> B? implementation defined).
    // Let's force B registered first to ensure "natural" order is B -> A, then sort flips it.
    
    // Note: add_systems calls are sequential.
    app.add_systems(Update, sys_b);
    app.add_systems(Update, sys_a.before("SystemB"));

    app.update();

    let log = app.world_mut().resource::<OrderLog>();
    assert_eq!(log.0, vec!["A", "B"], "SystemA should run before SystemB");
}

#[test]
fn test_ordering_after() {
    let mut app = App::new();
    app.init_resource::<OrderLog>();
    app.add_plugins(ScheduleRunnerPlugin::run_once());

    let sys_a_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_a);
    let sys_a = BoxedSystem::new(sys_a_sys, "SystemA");

    let sys_b_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_b);
    let sys_b = BoxedSystem::new(sys_b_sys, "SystemB");

    // Register A. Then B after A.
    app.add_systems(Update, sys_a);
    app.add_systems(Update, sys_b.after("SystemA"));

    app.update();

    let log = app.world_mut().resource::<OrderLog>();
    assert_eq!(log.0, vec!["A", "B"], "SystemB should run after SystemA");
}

#[test]
fn test_ordering_reverse() {
    // Verify that we can force reverse "natural" order
    let mut app = App::new();
    app.init_resource::<OrderLog>();
    app.add_plugins(ScheduleRunnerPlugin::run_once());

    let sys_a_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_a);
    let sys_a = BoxedSystem::new(sys_a_sys, "SystemA");

    let sys_b_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_b);
    let sys_b = BoxedSystem::new(sys_b_sys, "SystemB");

    // Register A then B. But enforce A AFTER B.
    app.add_systems(Update, sys_a.after("SystemB"));
    app.add_systems(Update, sys_b); // B registered later, but A depends on B.
    
    // Note: Zig dependency graph allows out-of-order registration.
    // "SystemB" is referenced by name in A's constraint. B is registered later.
    // This tests the "Lazy Node" logic.

    app.update();

    let log = app.world_mut().resource::<OrderLog>();
    assert_eq!(log.0, vec!["B", "A"], "SystemA should run after SystemB");
}
