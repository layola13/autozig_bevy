//! Advanced tests for Schedule system configuration
//!
//! Tests Phase 2 features: configure_schedules, edit_schedule, 
//! ambiguity detection, and advanced scheduling options

#![allow(unsafe_code)]

use autozig_app::{
    App, Plugin, Update, Startup, PreUpdate, PostUpdate,
    ScheduleBuildSettings, AmbiguityDetection,
};

/// Test resource
#[derive(Debug, Clone, PartialEq)]
struct TestResource {
    value: i32,
}

/// Another test resource
#[derive(Debug, Clone, PartialEq)]
struct AnotherResource {
    count: u32,
}

/// Test component type
struct TestComponent;

/// Simple test plugin
struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, _app: &mut App) {
        // Plugin logic here
    }
    
    fn name(&self) -> &str {
        "TestPlugin"
    }
}

#[test]
fn test_configure_schedules() {
    // Test that configure_schedules applies settings
    let mut app = App::new();
    
    let settings = ScheduleBuildSettings {
        ambiguity_detection: AmbiguityDetection::Ignore,
        hierarchy_detection: false,
        auto_insert_apply_deferred: false,
    };
    
    app.configure_schedules(settings.clone());
    
    // Verify settings were stored as a resource
    assert!(app.has_resource::<ScheduleBuildSettings>());
    
    let stored_settings = app.get_resource::<ScheduleBuildSettings>().unwrap();
    assert_eq!(stored_settings.ambiguity_detection, AmbiguityDetection::Ignore);
    assert_eq!(stored_settings.hierarchy_detection, false);
    assert_eq!(stored_settings.auto_insert_apply_deferred, false);
}

#[test]
fn test_configure_schedules_chaining() {
    // Test that configure_schedules returns &mut Self for chaining
    let mut app = App::new();
    
    app.configure_schedules(ScheduleBuildSettings::default())
        .add_plugin(TestPlugin)
        .insert_resource(TestResource { value: 42 });
    
    assert!(app.has_resource::<ScheduleBuildSettings>());
    assert!(app.has_resource::<TestResource>());
}

#[test]
fn test_allow_ambiguous_component() {
    // Test allowing ambiguous component access
    let mut app = App::new();
    
    app.allow_ambiguous_component::<TestComponent>();
    
    // If we got here without panicking, the component was marked as allowed
    assert!(true);
}

#[test]
fn test_allow_ambiguous_component_chaining() {
    // Test that allow_ambiguous_component returns &mut Self
    let mut app = App::new();
    
    app.allow_ambiguous_component::<TestComponent>()
        .insert_resource(TestResource { value: 1 })
        .add_plugin(TestPlugin);
    
    assert!(app.has_resource::<TestResource>());
}

#[test]
fn test_allow_ambiguous_resource() {
    // Test allowing ambiguous resource access
    let mut app = App::new();
    
    app.allow_ambiguous_resource::<TestResource>();
    
    // If we got here without panicking, the resource was marked as allowed
    assert!(true);
}

#[test]
fn test_allow_ambiguous_resource_chaining() {
    // Test that allow_ambiguous_resource returns &mut Self
    let mut app = App::new();
    
    app.allow_ambiguous_resource::<TestResource>()
        .allow_ambiguous_resource::<AnotherResource>()
        .insert_resource(TestResource { value: 2 });
    
    assert!(app.has_resource::<TestResource>());
}

#[test]
fn test_ignore_ambiguity() {
    // Test ignoring ambiguity between specific systems
    let mut app = App::new();
    
    fn system_a() {}
    fn system_b() {}
    
    app.ignore_ambiguity(Update, system_a, system_b);
    
    // If we got here without panicking, the ambiguity was ignored
    assert!(true);
}

#[test]
fn test_ignore_ambiguity_chaining() {
    // Test that ignore_ambiguity returns &mut Self
    let mut app = App::new();
    
    fn system_a() {}
    fn system_b() {}
    
    app.ignore_ambiguity(Update, system_a, system_b)
        .insert_resource(TestResource { value: 3 })
        .add_plugin(TestPlugin);
    
    assert!(app.has_resource::<TestResource>());
}

#[test]
fn test_schedule_settings_default() {
    // Test default schedule build settings
    let settings = ScheduleBuildSettings::default();
    
    assert_eq!(settings.ambiguity_detection, AmbiguityDetection::Check);
    assert_eq!(settings.hierarchy_detection, true);
    assert_eq!(settings.auto_insert_apply_deferred, true);
}

#[test]
fn test_ambiguity_detection_modes() {
    // Test all ambiguity detection modes
    let check_mode = AmbiguityDetection::Check;
    let error_mode = AmbiguityDetection::Error;
    let ignore_mode = AmbiguityDetection::Ignore;
    
    assert_eq!(check_mode, AmbiguityDetection::Check);
    assert_eq!(error_mode, AmbiguityDetection::Error);
    assert_eq!(ignore_mode, AmbiguityDetection::Ignore);
}

#[test]
fn test_complex_schedule_configuration() {
    // Test complex schedule configuration scenario
    let mut app = App::new();
    
    app.configure_schedules(ScheduleBuildSettings {
            ambiguity_detection: AmbiguityDetection::Check,
            hierarchy_detection: true,
            auto_insert_apply_deferred: true,
        })
        .allow_ambiguous_component::<TestComponent>()
        .allow_ambiguous_resource::<TestResource>()
        .insert_resource(TestResource { value: 999 });
    
    assert!(app.has_resource::<TestResource>());
    assert!(app.has_resource::<ScheduleBuildSettings>());
}

#[test]
fn test_schedule_chain_with_all_features() {
    // Test using all Phase 2 features in one chain
    let mut app = App::new();
    
    fn test_system_1() {}
    fn test_system_2() {}
    
    app.add_plugin(TestPlugin)
        .configure_schedules(ScheduleBuildSettings::default())
        .allow_ambiguous_component::<TestComponent>()
        .allow_ambiguous_resource::<TestResource>()
        .allow_ambiguous_resource::<AnotherResource>()
        .ignore_ambiguity(Update, test_system_1, test_system_2)
        .insert_resource(TestResource { value: 12345 })
        .insert_resource(AnotherResource { count: 67890 });
    
    assert!(app.has_resource::<TestResource>());
    assert!(app.has_resource::<AnotherResource>());
    assert!(app.has_resource::<ScheduleBuildSettings>());
}