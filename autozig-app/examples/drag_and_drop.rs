//! An example that shows how to handle drag and drop of files in an app.

use autozig_app::prelude::*;

#[derive(Debug, Clone, Event)]
struct FileDragAndDrop(String);

macro_rules! info { ($($arg:tt)*) => { println!("INFO: {}", format_args!($($arg)*)); } }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<FileDragAndDrop>()
        .add_systems(Update, file_drag_and_drop_system)
        .run();
}

fn file_drag_and_drop_system(mut drag_and_drop_reader: EventReader<FileDragAndDrop>) {
    for drag_and_drop in drag_and_drop_reader.read() {
        info!("{:?}", drag_and_drop);
    }
}
