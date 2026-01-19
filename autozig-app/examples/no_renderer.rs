//! An application that runs with default plugins and displays an empty
//! window, but without an actual renderer.
//! This can be very useful for integration tests or CI.
//!
//! See also the `headless` example which does not display a window.

use autozig_app::{prelude::*, default_plugins::{RenderPlugin, WgpuSettings}};

// Mock WgpuSettings
#[derive(Default, Clone)]
pub struct WgpuSettings {
    pub backends: Option<u32>,
}
impl From<WgpuSettings> for RenderPlugin {
    fn from(_: WgpuSettings) -> Self { RenderPlugin }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
        )
        .run();
}
