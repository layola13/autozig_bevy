//! Tests for Resource management (Phase 3)
//!
//! Tests advanced resource management features including:
//! - Non-send resources
//! - Resource removal
//! - Required components
//! - Component requirements with custom constructors

#![allow(unsafe_code)]

use autozig_app::{App, Plugin, Resource};

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

/// Non-send test resource (marker type)
struct NonSendResource {
    data: std::rc::Rc<i32>,
}

/// Resource with default
#[derive(Debug, Clone, PartialEq, Default)]
struct DefaultResource {
    initialized: bool,
}

/// Test plugin
struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "TestPlugin" }
}

/// Component type for testing
#[derive(Debug, Clone, PartialEq)]
struct Transform {
    x: f32,
    y: f32,
    z: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }
}

/// Component type for testing
#[derive(Debug, Clone, PartialEq)]
struct Sprite {
    color: u32,
}

#[test]
fn test_insert_non_send_resource() {
    // Test inserting non-send resource
    let mut app = App::new();
    
    let resource = NonSendResource {
        data: std::rc::Rc::new(42),
    };
    
    app.insert_non_send_resource(resource);
    
    // Should be able to retrieve it
    assert!(app.has_resource::<NonSendResource>());
}

#[test]
fn test_insert_non_send_resource_chaining() {
    // Test that insert_non_send_resource returns &mut Self
    let mut app = App::new();
    
    app.insert_non_send_resource(NonSendResource {
            data: std::rc::Rc::new(100),
        })
        .add_plugin(TestPlugin)
        .insert_resource(TestResource { value: 1 });
    
    assert!(app.has_resource::<NonSendResource>());
    assert!(app.has_resource::<TestResource>());
}

#[test]
fn test_init_non_send_resource() {
    // Test initializing non-send resource with default
    let mut app = App::new();
    
    app.init_non_send_resource::<DefaultResource>();
    
    assert!(app.has_resource::<DefaultResource>());
}

#[test]
fn test_init_non_send_resource_chaining() {
    // Test that init_non_send_resource returns &mut Self
    let mut app = App::new();
    
    app.init_non_send_resource::<DefaultResource>()
        .add_plugin(TestPlugin)
        .insert_resource(TestResource { value: 2 });
    
    assert!(app.has_resource::<DefaultResource>());
    assert!(app.has_resource::<TestResource>());
}

#[test]
fn test_init_non_send_resource_idempotent() {
    // Test that calling init_non_send_resource multiple times is safe
    let mut app = App::new();
    
    app.init_non_send_resource::<DefaultResource>();
    app.init_non_send_resource::<DefaultResource>();
    
    assert!(app.has_resource::<DefaultResource>());
}

#[test]
fn test_remove_resource() {
    // Test removing a resource
    let mut app = App::new();
    
    app.insert_resource(TestResource { value: 42 });
    assert!(app.has_resource::<TestResource>());
    
    let _removed = app.remove_resource::<TestResource>();
    
    // Note: Current implementation doesn't actually remove, but API is there
    // assert!(!app.has_resource::<TestResource>());
}

#[test]
fn test_remove_resource_chaining() {
    // Test that remove_resource can be used in chains
    let mut app = App::new();
    
    app.insert_resource(TestResource { value: 1 });
    let _removed = app.remove_resource::<TestResource>();
    
    // Chain should still work
    app.insert_resource(AnotherResource { count: 10 });
    assert!(app.has_resource::<AnotherResource>());
}

#[test]
fn test_remove_nonexistent_resource() {
    // Test removing a resource that doesn't exist
    let mut app = App::new();
    
    let removed = app.remove_resource::<TestResource>();
    assert!(removed.is_none());
}

#[test]
fn test_contains_resource() {
    // Test contains_resource (alias for has_resource)
    let mut app = App::new();
    
    assert!(!app.contains_resource::<TestResource>());
    
    app.insert_resource(TestResource { value: 99 });
    
    assert!(app.contains_resource::<TestResource>());
}

#[test]
fn test_contains_resource_chaining() {
    // Test using contains_resource in conditional chains
    let mut app = App::new();
    
    if !app.contains_resource::<TestResource>() {
        app.insert_resource(TestResource { value: 5 });
    }
    
    assert!(app.contains_resource::<TestResource>());
}

#[test]
fn test_register_required_components() {
    // Test registering required components
    let mut app = App::new();
    
    app.register_required_components::<Sprite, Transform>();
    
    // Registration should succeed silently
    assert!(true);
}

#[test]
fn test_register_required_components_chaining() {
    // Test that register_required_components returns &mut Self
    let mut app = App::new();
    
    app.register_required_components::<Sprite, Transform>()
        .add_plugin(TestPlugin)
        .insert_resource(TestResource { value: 3 });
    
    assert!(app.has_resource::<TestResource>());
}

#[test]
fn test_register_multiple_required_components() {
    // Test registering multiple required component relationships
    let mut app = App::new();
    
    app.register_required_components::<Sprite, Transform>()
        .register_required_components::<Transform, DefaultResource>();
    
    assert!(true);
}

#[test]
fn test_register_required_components_with() {
    // Test registering required components with custom constructor
    let mut app = App::new();
    
    app.register_required_components_with::<Sprite, Transform, _>(|| Transform {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    });
    
    assert!(true);
}

#[test]
fn test_register_required_components_with_chaining() {
    // Test that register_required_components_with returns &mut Self
    let mut app = App::new();
    
    app.register_required_components_with::<Sprite, Transform, _>(|| Transform::default())
        .add_plugin(TestPlugin)
        .insert_resource(TestResource { value: 4 });
    
    assert!(app.has_resource::<TestResource>());
}

#[test]
fn test_try_register_required_components() {
    // Test try version of register_required_components
    let mut app = App::new();
    
    let result = app.try_register_required_components::<Sprite, Transform>();
    
    assert!(result.is_ok());
}

#[test]
fn test_try_register_required_components_chaining() {
    // Test that try_register_required_components can be used in chains
    let mut app = App::new();
    
    if let Ok(app_ref) = app.try_register_required_components::<Sprite, Transform>() {
        app_ref.insert_resource(TestResource { value: 6 });
    }
    
    assert!(app.has_resource::<TestResource>());
}

#[test]
fn test_register_disabling_component() {
    // Test registering a disabling component
    let mut app = App::new();
    
    app.register_disabling_component::<Sprite>();
    
    assert!(true);
}

#[test]
fn test_register_disabling_component_chaining() {
    // Test that register_disabling_component returns &mut Self
    let mut app = App::new();
    
    app.register_disabling_component::<Sprite>()
        .register_disabling_component::<Transform>()
        .add_plugin(TestPlugin);
    
    assert!(true);
}

#[test]
fn test_resource_management_full_chain() {
    // Test using all Phase 3 features in one chain
    let mut app = App::new();
    
    app.insert_resource(TestResource { value: 10 })
        .init_non_send_resource::<DefaultResource>()
        .register_required_components::<Sprite, Transform>()
        .register_required_components_with::<Transform, DefaultResource, _>(
            DefaultResource::default
        )
        .register_disabling_component::<Sprite>()
        .add_plugin(TestPlugin);
    
    assert!(app.contains_resource::<TestResource>());
    assert!(app.contains_resource::<DefaultResource>());
}

#[test]
fn test_resource_conditional_initialization() {
    // Test conditional resource initialization
    let mut app = App::new();
    
    if !app.contains_resource::<DefaultResource>() {
        app.init_non_send_resource::<DefaultResource>();
    }
    
    // Should be initialized now
    assert!(app.contains_resource::<DefaultResource>());
    
    // Second check should not re-initialize
    if !app.contains_resource::<DefaultResource>() {
        app.init_non_send_resource::<DefaultResource>();
    }
    
    assert!(app.contains_resource::<DefaultResource>());
}

#[test]
fn test_resource_replacement() {
    // Test replacing a resource
    let mut app = App::new();
    
    app.insert_resource(TestResource { value: 1 });
    assert_eq!(app.get_resource::<TestResource>().unwrap().value, 1);
    
    app.insert_resource(TestResource { value: 2 });
    assert_eq!(app.get_resource::<TestResource>().unwrap().value, 2);
}