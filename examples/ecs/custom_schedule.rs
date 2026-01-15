//! custom_schedule.rs - Demonstrate custom schedules
//!
//! Ported from Bevy ecs/custom_schedule.rs

use autozig_ecs::prelude::*;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use autozig_ecs::schedule::{ExecutorKind, ScheduleLabel};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct SingleThreadedUpdate;

impl ScheduleLabel for SingleThreadedUpdate {
    fn label(&self) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("SingleThreadedUpdate")
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct CustomStartup;

impl ScheduleLabel for CustomStartup {
    fn label(&self) -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("CustomStartup")
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Create a new Schedule
    use autozig_ecs::schedule::{Schedule, Schedules};
    let custom_update_schedule = Schedule::new(SingleThreadedUpdate);
    // custom_update_schedule.set_executor_kind(ExecutorKind::SingleThreaded); // Not supported
    // app.add_schedule(custom_update_schedule); // Not implemented
    app.world_mut().resource_mut::<Schedules>().insert(custom_update_schedule);
    
    let custom_startup_schedule = Schedule::new(CustomStartup);
    app.world_mut().resource_mut::<Schedules>().insert(custom_startup_schedule);

    // Configure Main schedule order
    // Accessing MainScheduleOrder resource. 
    // autozig-ecs might not have MainScheduleOrder exposed or implemented exactly like Bevy.
    // However, App::run() uses fixed schedule execution: Startup -> Update -> Last.
    // If we want to insert custom schedules, autozig-ecs App need to support dynamic schedule ordering 
    // OR we just manually run them in a Runner or a System?
    // Bevy's MainScheduleOrder allows inserting schedules into the main loop.
    // autozig-ecs App::run uses hardcoded `run_schedule(&mut self.world, Update);`.
    
    // For this example to work as intended (custom schedules in main loop), we might need to assume 
    // the user wants to see them running.
    // Since autozig-ecs is simplified, maybe we can run them manually in a system?
    // OR `app.add_systems` to `Update` that runs `world.run_schedule`.
    
    // Workaround: Run custom schedules via systems in standard schedules.
    // This is a common pattern for sub-schedules anyway.
    
    let run_custom_update: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = 
        (|mut commands: Commands| {
            // commands.run_schedule(SingleThreadedUpdate); // If Commands supports it
            // Or use World access? System doesn't have checks.
            println!("(Simulating Scheduler behavior via system placeholder)");
        }).into_system();
        
    // Actually, `App` in autozig-ecs doesn't strictly support `MainScheduleOrder` resource yet.
    // So we'll skip the "insert_after" part and just run them if possible, or demonstrate creation.
    
    // Let's just add systems to the custom schedule and manually run it once at the end.
    
    app.add_systems(SingleThreadedUpdate, single_threaded_update_system.into_system());
    app.add_systems(CustomStartup, custom_startup_system.into_system());
    
    app.add_systems(Startup, startup_system.into_system());
    app.add_systems(Update, update_system.into_system());
    
    // Manually run schedules for demonstration
    println!("Running CustomStartup schedule...");
    let _ = app.world_mut().try_run_schedule(CustomStartup);
    
    println!("Running SingleThreadedUpdate schedule...");
    let _ = app.world_mut().try_run_schedule(SingleThreadedUpdate);

    println!("Starting Custom Schedule Example (Standard Loop)...");
    
    // Use runner to limit execution
    app.set_runner(|mut app| {
        // Run standard startup
        app.update(); // This runs Update
    });
    
    app.run();
}

fn startup_system() {
    println!("Startup");
}

fn custom_startup_system() {
    println!("Custom Startup");
}

fn update_system() {
    println!("Update");
}

fn single_threaded_update_system() {
    println!("Single Threaded Update");
}
