use autozig_ecs::prelude::*;
use autozig_ecs::schedule::{ScheduleRunnerPlugin, Update};
use autozig_ecs::system::BoxedSystem;
use autozig_ecs::into_system::IntoSystem;
use autozig_ecs::system_set::SystemSet;

#[derive(Default, Clone)]
struct OrderLog(Vec<String>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SetA;
impl SystemSet for SetA { fn as_str(&self) -> &str { "SetA" } }

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SetB;
impl SystemSet for SetB { fn as_str(&self) -> &str { "SetB" } }

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ParentSet;
impl SystemSet for ParentSet { fn as_str(&self) -> &str { "ParentSet" } }

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ChildSet;
impl SystemSet for ChildSet { fn as_str(&self) -> &str { "ChildSet" } }


fn sys_a(mut log: ResMut<OrderLog>) { log.0.push("A".to_string()); }
fn sys_b(mut log: ResMut<OrderLog>) { log.0.push("B".to_string()); }
fn sys_c(mut log: ResMut<OrderLog>) { log.0.push("C".to_string()); }

#[test]
fn test_set_ordering() {
    let mut app = App::new();
    app.init_resource::<OrderLog>();
    app.add_plugins(ScheduleRunnerPlugin::run_once());

    // Configure SetA before SetB
    app.configure_sets(Update, SetA.before(SetB));

    // Register SysA in SetA, SysB in SetB
    // Note: use explicit casting to avoid inference issues seen before
    let sys_a_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_a);
    let sys_a_boxed = BoxedSystem::new(sys_a_sys, "SysA");
    
    let sys_b_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_b);
    let sys_b_boxed = BoxedSystem::new(sys_b_sys, "SysB");

    app.add_systems(Update, sys_b_boxed.in_set(SetB)); // Added first, but should run after A
    app.add_systems(Update, sys_a_boxed.in_set(SetA));

    app.update();

    let log = app.world_mut().resource::<OrderLog>();
    assert_eq!(log.0, vec!["A", "B"], "SetA (SysA) should run before SetB (SysB)");
}

#[test]
fn test_nested_sets() {
    let mut app = App::new();
    app.init_resource::<OrderLog>();
    app.add_plugins(ScheduleRunnerPlugin::run_once());

    // Child in Parent. Start(Parent) -> Start(Child) ... End(Child) -> End(Parent)
    app.configure_sets(Update, ChildSet.in_set(ParentSet));
    
    // Parent before SetB. End(Parent) -> Start(SetB)
    app.configure_sets(Update, ParentSet.before(SetB));

    let sys_a_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_a);
    let sys_a_boxed = BoxedSystem::new(sys_a_sys, "SysChild"); // In ChildSet

    let sys_b_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_b);
    let sys_b_boxed = BoxedSystem::new(sys_b_sys, "SysB"); // In SetB

    app.add_systems(Update, sys_b_boxed.in_set(SetB));
    app.add_systems(Update, sys_a_boxed.in_set(ChildSet));

    app.update();

    let log = app.world_mut().resource::<OrderLog>();
    assert_eq!(log.0, vec!["A", "B"], "ChildSet (in ParentSet) should run before SetB");
}

#[test]
fn test_system_before_set() {
    let mut app = App::new();
    app.init_resource::<OrderLog>();
    app.add_plugins(ScheduleRunnerPlugin::run_once());

    // SysA before SetB
    let sys_a_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_a);
    let sys_a_boxed = BoxedSystem::new(sys_a_sys, "SysA");

    let sys_b_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_b);
    let sys_b_boxed = BoxedSystem::new(sys_b_sys, "SysB");

    // Add SysA with explicit constraint before SetB
    app.add_systems(Update, sys_a_boxed.before(SetB));
    // Add SysB in SetB
    app.add_systems(Update, sys_b_boxed.in_set(SetB));

    app.update();

    let log = app.world_mut().resource::<OrderLog>();
    assert_eq!(log.0, vec!["A", "B"], "SysA should run before SetB (SysB)");
}

#[test]
#[should_panic(expected = "Failed to build schedule: dependency cycle detected")]
fn test_cycle_detection() {
    let mut app = App::new();
    app.add_plugins(ScheduleRunnerPlugin::run_once());

    let sys_a_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_a);
    let sys_a_boxed = BoxedSystem::new(sys_a_sys, "SysA");

    let sys_b_sys = IntoSystem::<(ResMut<OrderLog>,)>::into_system(sys_b);
    let sys_b_boxed = BoxedSystem::new(sys_b_sys, "SysB");

    // A before B, B before A -> Cycle
    app.add_systems(Update, (
        sys_a_boxed.before("SysB"),
        sys_b_boxed.before("SysA"),
    ));

    app.update();
}
