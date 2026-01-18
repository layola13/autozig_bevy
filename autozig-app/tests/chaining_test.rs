use autozig_app::{App, Plugin, PluginGroup, PluginGroupBuilder};

struct MockPlugin1;
impl Plugin for MockPlugin1 {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "MockPlugin1" }
}

struct MockPlugin2;
impl Plugin for MockPlugin2 {
    fn build(&self, _app: &mut App) {}
    fn name(&self) -> &str { "MockPlugin2" }
}

struct MockPluginGroup;
impl PluginGroup for MockPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MockPlugin1)
            .add(MockPlugin2)
    }
}

#[test]
fn test_chaining_syntax() {
    let mut app = App::new();
    
    app
        // Test single plugin
        .add_plugins(MockPlugin1)
        // Test tuple
        .add_plugins((MockPlugin1, MockPlugin2))
        // Test PluginGroup
        .add_plugins(MockPluginGroup)
        // Test configure_sub_app (assuming Main subapp exists or we act on app itself if it had subapps)
        // .configure_sub_app(...) // Skipping as getting subapp might fail if not created
        ;
}

#[test]
fn test_add_plugin_group_directly() {
    let mut app = App::new();
    // This verifies that PluginGroup implements Plugins
    app.add_plugins(MockPluginGroup);
}