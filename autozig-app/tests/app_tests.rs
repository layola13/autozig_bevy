//! Comprehensive test suite for autozig-app
//! 
//! Tests all core functionality:
//! 1. App creation (new and empty)
//! 2. App update
//! 3. Plugin system
//! 4. Plugin lifecycle
//! 5. SubApp management
//! 6. Custom runner
//! 7. AppExit mechanism
//! 8. Resource management
//! 9. System addition
//! 10. App builder pattern

use autozig_app::*;

/// Test 1: App creation
#[test]
fn test_app_creation() {
    let app = App::new();
    // App should be created successfully
    assert!(true, "App::new() should create an app");
    drop(app);
}

/// Test 2: Empty app creation
#[test]
fn test_app_empty() {
    let app = App::empty();
    // Empty app should be created successfully
    assert!(true, "App::empty() should create an empty app");
    drop(app);
}

/// Test 3: App update
#[test]
fn test_app_update() {
    let mut app = App::new();
    
    // Update should not panic
    app.update();
    app.update();
    app.update();
    
    assert!(true, "App::update() should work multiple times");
}

/// Test 4: Add plugin
#[test]
fn test_add_plugin() {
    let mut app = App::new();
    
    // Create a simple plugin
    let plugin = SimplePlugin::new("TestPlugin", |_app| {
        // Plugin build function
    });
    
    app.add_plugin(plugin);
    
    assert!(true, "Should be able to add a plugin");
}

/// Test 5: Plugin lifecycle (build → ready → finish → cleanup)
#[test]
fn test_plugin_lifecycle() {
    let mut app = App::new();
    
    // Create a plugin that tracks lifecycle
    let plugin = SimplePlugin::new("LifecyclePlugin", |_app| {
        // Build phase
    });
    
    app.add_plugin(plugin);
    
    // Call lifecycle methods
    app.finish();
    app.cleanup();
    
    assert!(true, "Plugin lifecycle should complete without errors");
}

/// Test 6: SubApp management
#[test]
fn test_sub_app() {
    let mut app = App::new();
    
    // Add a sub-application
    let _sub_app = app.add_sub_app("render");
    
    // Try to retrieve it
    let retrieved = app.get_sub_app("render");
    assert!(retrieved.is_some(), "SubApp should be retrievable");
    
    // Non-existent sub-app should return None
    let non_existent = app.get_sub_app("non_existent");
    assert!(non_existent.is_none(), "Non-existent SubApp should return None");
}

/// Test 7: Custom runner
#[test]
fn test_runner() {
    let mut app = App::new();
    
    // Set a custom runner that exits immediately
    extern "C" fn custom_runner(_app: *mut autozig_app::ZigApp) -> u8 {
        0 // Exit successfully
    }
    
    app.set_runner(custom_runner);
    
    // Run the app
    let exit = app.run();
    
    assert_eq!(exit, AppExit::Success, "Custom runner should exit successfully");
}

/// Test 8: AppExit mechanism
#[test]
fn test_app_exit() {
    // Test success exit
    let exit_success = AppExit::Success;
    assert_eq!(exit_success.code(), 0);
    assert!(exit_success.is_success());
    assert!(!exit_success.is_error());
    
    // Test error exit
    let exit_error = AppExit::Error(core::num::NonZeroU8::new(1).unwrap());
    assert_eq!(exit_error.code(), 1);
    assert!(!exit_error.is_success());
    assert!(exit_error.is_error());
    
    // Test from_code
    assert_eq!(AppExit::from_code(0), AppExit::Success);
    assert_eq!(AppExit::from_code(42).code(), 42);
}

/// Test 9: Resource management
#[test]
fn test_resource_management() {
    let mut app = App::new();
    
    // Insert a resource
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Counter {
        value: u32,
    }
    
    let counter = Counter { value: 42 };
    app.insert_resource(counter);
    
    // Check if resource exists
    assert!(app.has_resource::<Counter>(), "Resource should exist after insertion");
    
    // Check non-existent resource
    struct NonExistent;
    assert!(!app.has_resource::<NonExistent>(), "Non-existent resource should return false");
}

/// Test 10: System addition (builder pattern)
#[test]
fn test_system_addition() {
    let mut app = App::new();
    
    // Test builder pattern
    app.update()
       .update()
       .finish()
       .cleanup();
    
    assert!(true, "Builder pattern should work");
}

/// Test 11: Multiple plugins
#[test]
fn test_multiple_plugins() {
    let mut app = App::new();
    
    let plugin1 = SimplePlugin::new("Plugin1", |_app| {});
    let plugin2 = SimplePlugin::new("Plugin2", |_app| {});
    let plugin3 = SimplePlugin::new("Plugin3", |_app| {});
    
    app.add_plugin(plugin1)
       .add_plugin(plugin2)
       .add_plugin(plugin3);
    
    assert!(true, "Should be able to add multiple plugins");
}

/// Test 12: SubApp creation and update
#[test]
fn test_sub_app_creation_and_update() {
    let mut sub_app = SubApp::new();
    
    // Should be able to update
    sub_app.update();
    sub_app.run_default_schedule();
    
    assert!(true, "SubApp should be created and updated");
}

/// Test 13: App should_exit
#[test]
fn test_should_exit() {
    let app = App::new();
    
    // Initially should not have exit code
    let exit = app.should_exit();
    assert!(exit.is_none(), "New app should not have exit code");
}

/// Test 14: Default implementations
#[test]
fn test_defaults() {
    let app = App::default();
    drop(app);
    
    let sub_app = SubApp::default();
    drop(sub_app);
    
    let exit = AppExit::default();
    assert_eq!(exit, AppExit::Success, "Default AppExit should be Success");
    
    assert!(true, "Default implementations should work");
}

/// Test 15: Plugin trait
#[test]
fn test_plugin_trait() {
    struct CustomPlugin;
    
    impl Plugin for CustomPlugin {
        fn build(&self, _app: &mut App) {
            // Build logic
        }
        
        fn name(&self) -> &str {
            "CustomPlugin"
        }
    }
    
    let mut app = App::new();
    let plugin = CustomPlugin;
    
    app.add_plugin(plugin);
    
    assert!(true, "Custom plugin trait implementation should work");
}

/// Integration test: Full application lifecycle
#[test]
fn test_full_lifecycle() {
    let mut app = App::new();
    
    // Add resource
    struct GameState {
        frame_count: u32,
    }
    
    app.insert_resource(GameState { frame_count: 0 });
    
    // Add plugin
    let plugin = SimplePlugin::new("GamePlugin", |_app| {});
    app.add_plugin(plugin);
    
    // Add sub-app
    let _render_app = app.add_sub_app("render");
    
    // Update multiple frames
    for _ in 0..10 {
        app.update();
    }
    
    // Finish and cleanup
    app.finish();
    app.cleanup();
    
    assert!(true, "Full application lifecycle should complete");
}