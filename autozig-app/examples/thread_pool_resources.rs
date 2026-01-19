//! This example illustrates how to customize the thread pool used internally (e.g. to only use a
//! certain number of threads).

use autozig_app::{prelude::*, default_plugins::TaskPoolPlugin, TaskPoolOptions};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // TaskPoolOptions not fully exposed/configurable in placeholder yet, simplifiying
        .run();
}
