//! Integration tests for PluginGroup functionality

use autozig_app::{
    App, Plugin, PluginGroup, PluginGroupBuilder, PluginGroupExt,
    DefaultPlugins, MinimalPlugins, SimplePlugin,
    default_plugins::*,
};

// Test plugins
struct TestPluginA;
impl Plugin for TestPluginA {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "TestPluginA" }
}

struct TestPluginB;
impl Plugin for TestPluginB {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "TestPluginB" }
}

struct TestPluginC;
impl Plugin for TestPluginC {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "TestPluginC" }
}

// Test plugin group
struct TestPluginGroup;
impl PluginGroup for TestPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TestPluginA)
            .add(TestPluginB)
            .add(TestPluginC)
    }
}

#[test]
fn test_plugin_group_basic() {
    let builder = PluginGroupBuilder::start::<TestPluginGroup>();
    assert_eq!(builder.len(), 0);
    assert!(builder.is_empty());
}

#[test]
fn test_plugin_group_add() {
    let builder = PluginGroupBuilder::start::<TestPluginGroup>()
        .add(TestPluginA)
        .add(TestPluginB);
    
    assert_eq!(builder.len(), 2);
    assert!(!builder.is_empty());
    assert!(builder.contains::<TestPluginA>());
    assert!(builder.contains::<TestPluginB>());
    assert!(!builder.contains::<TestPluginC>());
}

#[test]
fn test_plugin_group_contains() {
    let builder = TestPluginGroup.build();
    
    assert!(builder.contains::<TestPluginA>());
    assert!(builder.contains::<TestPluginB>());
    assert!(builder.contains::<TestPluginC>());
    assert_eq!(builder.len(), 3);
}

#[test]
fn test_plugin_group_enabled() {
    let builder = TestPluginGroup.build();
    
    assert!(builder.enabled::<TestPluginA>());
    assert!(builder.enabled::<TestPluginB>());
    assert!(builder.enabled::<TestPluginC>());
}

#[test]
fn test_plugin_group_disable() {
    let builder = TestPluginGroup.build()
        .disable::<TestPluginB>();
    
    assert!(builder.contains::<TestPluginB>());
    assert!(!builder.enabled::<TestPluginB>());
    assert!(builder.enabled::<TestPluginA>());
    assert!(builder.enabled::<TestPluginC>());
    
    // Enabled count should be 2 (A and C)
    assert_eq!(builder.enabled_count(), 2);
}

#[test]
fn test_plugin_group_enable_after_disable() {
    let builder = TestPluginGroup.build()
        .disable::<TestPluginB>()
        .enable::<TestPluginB>();
    
    assert!(builder.enabled::<TestPluginB>());
    assert_eq!(builder.enabled_count(), 3);
}

#[test]
fn test_plugin_group_add_before() {
    let plugin_c = TestPluginC;
    let builder = PluginGroupBuilder::start::<TestPluginGroup>()
        .add(TestPluginA)
        .add(TestPluginB)
        .add_before::<TestPluginB, _>(plugin_c);
    
    // All three plugins should be present
    assert_eq!(builder.len(), 3);
    assert!(builder.contains::<TestPluginA>());
    assert!(builder.contains::<TestPluginB>());
    assert!(builder.contains::<TestPluginC>());
}

#[test]
#[should_panic(expected = "Plugin does not exist in group")]
fn test_plugin_group_add_before_nonexistent() {
    PluginGroupBuilder::start::<TestPluginGroup>()
        .add(TestPluginA)
        .add_before::<TestPluginB, _>(TestPluginC);
}

#[test]
fn test_plugin_group_add_after() {
    let plugin_c = TestPluginC;
    let builder = PluginGroupBuilder::start::<TestPluginGroup>()
        .add(TestPluginA)
        .add(TestPluginB)
        .add_after::<TestPluginA, _>(plugin_c);
    
    // All three plugins should be present
    assert_eq!(builder.len(), 3);
    assert!(builder.contains::<TestPluginA>());
    assert!(builder.contains::<TestPluginB>());
    assert!(builder.contains::<TestPluginC>());
}

#[test]
#[should_panic(expected = "Plugin does not exist in group")]
fn test_plugin_group_add_after_nonexistent() {
    PluginGroupBuilder::start::<TestPluginGroup>()
        .add(TestPluginA)
        .add_after::<TestPluginB, _>(TestPluginC);
}

#[test]
fn test_plugin_group_readd() {
    let builder = PluginGroupBuilder::start::<TestPluginGroup>()
        .add(TestPluginA)
        .add(TestPluginB)
        .add(TestPluginC)
        .add(TestPluginB); // Re-add B
    
    // Should still have 3 plugins (B moved to end)
    assert_eq!(builder.len(), 3);
    assert!(builder.contains::<TestPluginA>());
    assert!(builder.contains::<TestPluginB>());
    assert!(builder.contains::<TestPluginC>());
}

#[test]
fn test_default_plugins_basic() {
    let builder = DefaultPlugins.build();
    
    assert!(builder.len() > 0);
    assert!(!builder.is_empty());
}

#[test]
fn test_default_plugins_contains_core() {
    let builder = DefaultPlugins.build();
    
    assert!(builder.contains::<TaskPoolPlugin>());
    assert!(builder.contains::<TypeRegistrationPlugin>());
    assert!(builder.contains::<TimePlugin>());
    assert!(builder.contains::<FrameCountPlugin>());
}

#[test]
fn test_default_plugins_all_enabled() {
    let builder = DefaultPlugins.build();
    
    assert!(builder.enabled::<TaskPoolPlugin>());
    assert!(builder.enabled::<TypeRegistrationPlugin>());
    assert!(builder.enabled::<TimePlugin>());
    assert_eq!(builder.enabled_count(), builder.len());
}

#[test]
fn test_default_plugins_disable_specific() {
    let builder = DefaultPlugins.build()
        .disable::<WindowPlugin>();
    
    assert!(builder.contains::<WindowPlugin>());
    assert!(!builder.enabled::<WindowPlugin>());
    assert!(builder.enabled::<TaskPoolPlugin>());
    assert_eq!(builder.enabled_count(), builder.len() - 1);
}

#[test]
fn test_minimal_plugins() {
    let builder = MinimalPlugins.build();
    
    assert!(builder.len() > 0);
    assert!(builder.contains::<TaskPoolPlugin>());
    assert!(builder.contains::<TimePlugin>());
}

#[test]
fn test_app_add_plugin_group() {
    let mut app = App::empty();
    app.add_plugins(MinimalPlugins);
    
    // App should be configured with the plugin group
    // This test just verifies it doesn't panic
}

#[test]
fn test_app_add_custom_plugin_group() {
    let mut app = App::empty();
    app.add_plugins(TestPluginGroup);
    
    // Should successfully add custom plugin group
}

#[test]
fn test_plugin_group_set() {
    let new_plugin = TestPluginA;
    let builder = TestPluginGroup.build()
        .set(new_plugin);
    
    // Plugin should be replaced
    assert!(builder.contains::<TestPluginA>());
}

#[test]
#[should_panic(expected = "does not exist in this PluginGroup")]
fn test_plugin_group_set_nonexistent() {
    struct NonexistentPlugin;
    impl Plugin for NonexistentPlugin {
        fn build(&self, _app: &mut App) {}
        fn name(&self) -> &str { "NonexistentPlugin" }
    }
    
    PluginGroupBuilder::start::<TestPluginGroup>()
        .add(TestPluginA)
        .set(NonexistentPlugin);
}

#[test]
fn test_plugin_group_builder_counts() {
    let builder = TestPluginGroup.build()
        .disable::<TestPluginB>();
    
    assert_eq!(builder.len(), 3);
    assert_eq!(builder.enabled_count(), 2);
}

#[test]
fn test_empty_plugin_group() {
    struct EmptyGroup;
    impl PluginGroup for EmptyGroup {
        fn build(self) -> PluginGroupBuilder {
            PluginGroupBuilder::start::<Self>()
        }
    }
    
    let builder = EmptyGroup.build();
    assert_eq!(builder.len(), 0);
    assert!(builder.is_empty());
    assert_eq!(builder.enabled_count(), 0);
}

#[test]
fn test_plugin_group_finish() {
    let mut app = App::empty();
    let builder = TestPluginGroup.build();
    
    // This should not panic
    builder.finish(&mut app);
}

#[test]
fn test_simple_plugin_in_group() {
    let plugin = SimplePlugin::new("simple", |_| {});
    let builder = PluginGroupBuilder::start::<TestPluginGroup>()
        .add(plugin);
    
    assert_eq!(builder.len(), 1);
    assert!(builder.contains::<SimplePlugin>());
}