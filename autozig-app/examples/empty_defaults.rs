//! An empty application with default plugins.

use autozig_app::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
