//! Shows how to return to the calling function after a windowed Bevy app has exited.
//!
//! In windowed *Bevy* applications, executing code below a call to `App::run()` is
//! not recommended because:
//! - `App::run()` will never return on iOS and Web.
//! - It is not possible to recreate a window after the event loop has been terminated.

use autozig_app::{prelude::*, default_plugins::WindowPlugin};
macro_rules! info { ($($arg:tt)*) => { println!("INFO: {}", format_args!($($arg)*)); } }

fn main() {
    println!("Running Bevy App");
    App::new()
        .add_plugins(DefaultPlugins) // WindowPlugin config removed as placeholder doesn't support it yet
        .add_systems(Update, system)
        .run();
    println!("Bevy App has exited. We are back in our main function.");
}

fn system() {
    info!("Logging from Bevy App");
}
