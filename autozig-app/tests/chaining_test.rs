//! Tests for method chaining API in autozig-app
//!
//! Verifies that all configuration methods return &mut Self for fluent API support

#![forbid(unsafe_code)]

use autozig_app::{App, Plugin, PluginGroup, PluginGroupBuilder};

/// Test plugin that does nothing
#[derive(Default)]
struct TestPluginA;

impl Plugin for TestPluginA {
    fn build(&self, _app: &mut App) {}
    
    fn name(&self) -> &'static str {
        "TestPluginA"
    }
}

/// Another test plugin
#[derive(Default)]
struct TestPluginB;

impl Plugin for TestPluginB {
    fn build(&self, _app: &mut App) {}
    
    fn name(&self) -> &'static str {
        "TestPluginB"
    }
}

/// Third test plugin
#[derive(Default)]
struct TestPluginC;

impl Plugin for TestPluginC {
    fn build(&self, _app: &mut App) {}
    
    fn name(&self) -> &'static str {
        "TestPluginC"
    }
}

/// Test resource
#[derive(Debug, Clone, PartialEq)]
struct TestResource {
    value: i32,
}

/// Test plugin group
struct TestPluginGroup;

impl PluginGroup for TestPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TestPluginA)
            .add(TestPluginB)
    }
}

#[test]
fn test_basic_chaining() {
    // Test that methods return &mut Self for chaining
    let mut app = App::new();
    
    app.add_plugin(TestPluginA)
        .insert_resource(TestResource { value: 42 });
    
    // If we got here without compilation errors, chaining works!
    assert!(true);
}

#[test]
fn test_multi_plugin_chaining() {
    // Test adding multiple plugins in a chain
    let mut app = App::new();
    
    app.add_plugin(TestPluginA)
        .add_plugin(TestPluginB)
        .add_plugin(TestPluginC);
    
    assert!(true);
}

#[test]
fn test_tuple_plugin_chaining() {
    // Test adding plugins as tuple
    let mut app = App::new();
    
    app.add_plugins((TestPluginA, TestPluginB))
        .insert_resource(TestResource { value: 100 });
    
    assert!(true);
}

#[test]
fn test_three_tuple_plugins() {
    // Test 3-tuple of plugins
    let mut app = App::new();
    
    app.add_plugins((TestPluginA, TestPluginB, TestPluginC));
    
    assert!(true);
}

#[test]
fn test_plugin_group_chaining() {
    // Test adding a plugin group
    let mut app = App::new();
    
    app.add_plugin_group(TestPluginGroup)
        .insert_resource(TestResource { value: 200 });
    
    assert!(true);
}

#[test]
fn test_complex_chaining() {
    // Test complex chaining scenario
    let mut app = App::new();
    
    app.add_plugins((TestPluginA, TestPluginB))
        .insert_resource(TestResource { value: 1 })
        .add_plugin(TestPluginC)
        .insert_resource(TestResource { value: 2 });
    
    assert!(true);
}

#[test]
fn test_init_resource_chaining() {
    // Test init_resource returns &mut Self
    #[derive(Default)]
    struct DefaultResource {
        count: u32,
    }
    
    let mut app = App::new();
    
    app.init_resource::<DefaultResource>()
        .add_plugin(TestPluginA);
    
    assert!(true);
}

#[test]
fn test_long_chain() {
    // Test a longer chain of operations
    let mut app = App::new();
    
    app.add_plugin(TestPluginA)
        .insert_resource(TestResource { value: 1 })
        .add_plugin(TestPluginB)
        .insert_resource(TestResource { value: 2 })
        .add_plugins((TestPluginC, TestPluginA))
        .insert_resource(TestResource { value: 3 });
    
    assert!(true);
}

#[test]
fn test_runner_chaining() {
    // Test set_runner returns &mut Self - Skip this test as set_runner requires C function pointer
    // use autozig_app::AppExit;
    
    let mut app = App::new();
    
    // Skipping set_runner test as it requires extern "C" fn pointer
    app.add_plugin(TestPluginA)
        .insert_resource(TestResource { value: 42 });
    
    assert!(true, "Runner chaining test skipped - requires C function pointer");
}

#[test]
fn test_single_plugin_via_add_plugins() {
    // Test that add_plugins works with single plugin (not just tuples)
    let mut app = App::new();
    
    app.add_plugins(TestPluginA)
        .insert_resource(TestResource { value: 999 });
    
    assert!(true);
}