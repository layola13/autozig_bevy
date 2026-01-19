//! Demonstrates the creation and registration of a custom plugin.
//!
//! Plugins are the foundation of Bevy. They are scoped sets of components, resources, and systems
//! that provide a specific piece of functionality (generally the smaller the scope, the better).
//! This example illustrates how to create a simple plugin that prints out a message.

use autozig_app::prelude::*;
use core::time::Duration;
macro_rules! info { ($($arg:tt)*) => { println!("INFO: {}", format_args!($($arg)*)); } }

fn main() {
    App::new()
        // plugins are registered as part of the "app building" process
        .add_plugins((
            DefaultPlugins,
            PrintMessagePlugin {
                wait_duration: Duration::from_secs(1),
                message: "This is an example plugin".to_string(),
            },
        ))
        .run();
}

// This "print message plugin" prints a `message` every `wait_duration`
struct PrintMessagePlugin {
    // Put your plugin configuration here
    wait_duration: Duration,
    message: String,
}

impl Plugin for PrintMessagePlugin {
    // this is where we set up our plugin
    fn build(&self, app: &mut App) {
        let state = PrintMessageState {
            message: self.message.clone(),
        };
        app.insert_resource(state)
            .add_systems(Update, print_message_system);
    }
}

struct PrintMessageState {
    message: String,
}

fn print_message_system(state: Res<PrintMessageState>) {
    // Just print every frame for now
    info!("{}", state.message);
}
