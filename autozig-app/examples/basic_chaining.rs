//! Basic example demonstrating method chaining in autozig-app
//!
//! This example shows how to use the fluent API to configure an App
//! with plugins, resources, and systems using method chaining.

#![forbid(unsafe_code)]

use autozig_app::{App, Plugin, PluginGroup, PluginGroupBuilder, AppExit};

/// Core game plugin
struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, _app: &mut App) {
        println!("Building CorePlugin");
    }
    
    fn name(&self) -> &'static str {
        "CorePlugin"
    }
}

/// Physics simulation plugin
struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, _app: &mut App) {
        println!("Building PhysicsPlugin");
    }
    
    fn name(&self) -> &'static str {
        "PhysicsPlugin"
    }
}

/// Rendering plugin
struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, _app: &mut App) {
        println!("Building RenderPlugin");
    }
    
    fn name(&self) -> &'static str {
        "RenderPlugin"
    }
}

/// Audio plugin
struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, _app: &mut App) {
        println!("Building AudioPlugin");
    }
    
    fn name(&self) -> &'static str {
        "AudioPlugin"
    }
}

/// Game settings resource
#[derive(Debug, Clone)]
struct GameSettings {
    resolution: (u32, u32),
    fullscreen: bool,
    volume: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            resolution: (1920, 1080),
            fullscreen: false,
            volume: 0.8,
        }
    }
}

/// Player state resource
#[derive(Debug, Clone)]
struct PlayerState {
    health: i32,
    score: u32,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            health: 100,
            score: 0,
        }
    }
}

/// Default plugin group containing core plugins
struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin)
            .add(PhysicsPlugin)
            .add(RenderPlugin)
            .add(AudioPlugin)
    }
    
    fn name() -> &'static str {
        "DefaultPlugins"
    }
}

fn main() {
    println!("=== Basic Chaining Example ===\n");
    
    // Example 1: Single plugin chaining
    println!("Example 1: Adding single plugins");
    let mut app = App::new();
    app.add_plugin(CorePlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(RenderPlugin);
    println!("✓ Single plugin chaining works!\n");
    
    // Example 2: Tuple plugin chaining
    println!("Example 2: Adding multiple plugins as tuple");
    let mut app = App::new();
    app.add_plugins((CorePlugin, PhysicsPlugin, RenderPlugin, AudioPlugin));
    println!("✓ Tuple plugin chaining works!\n");
    
    // Example 3: Plugin group chaining
    println!("Example 3: Adding plugin group");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    println!("✓ Plugin group chaining works!\n");
    
    // Example 4: Mixed chaining with resources
    println!("Example 4: Mixed chaining with plugins and resources");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(PlayerState {
            health: 150,
            score: 1000,
        })
        .init_resource::<GameSettings>();
    println!("✓ Mixed chaining works!\n");
    
    // Example 5: Complex chaining with multiple resources
    println!("Example 5: Complex chaining with multiple resources");
    let mut app = App::new();
    app.add_plugins((CorePlugin, PhysicsPlugin))
        .insert_resource(GameSettings {
            resolution: (2560, 1440),
            fullscreen: true,
            volume: 1.0,
        })
        .add_plugin(RenderPlugin)
        .insert_resource(PlayerState::default())
        .init_resource::<GameSettings>();
    println!("✓ Complex chaining with resources works!\n");
    
    // Example 6: Demonstrating fluent API style
    println!("Example 6: Fluent API style (Bevy-like)");
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .insert_resource(GameSettings::default())
        .insert_resource(PlayerState {
            health: 100,
            score: 0,
        });
    println!("✓ Fluent API style works!\n");
    
    println!("=== All chaining examples completed successfully! ===");
}