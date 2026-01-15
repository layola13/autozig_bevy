//! generic_system.rs - demonstrate generic systems
//!
//! Ported from Bevy ecs/generic_system.rs

use autozig_ecs::prelude::*;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use autozig_time::{Time, Timer, TimerMode};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Component)]
struct TextToPrint(String);

#[derive(Component)]
struct PrinterTick(Timer);

// Deref/DerefMut support for PrinterTick
impl std::ops::Deref for PrinterTick {
    type Target = Timer;
    fn deref(&self) -> &Self::Target { &self.0 }
}
impl std::ops::DerefMut for PrinterTick {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[derive(Component)]
struct MenuClose;

#[derive(Component)]
struct LevelUnload;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    
    // Check if init_state is available, if not simulate it
    // app.init_state::<AppState>(); 
    // Manual state init if needed:
    app.insert_resource(State::<AppState>::default());
    app.insert_resource(NextState::<AppState>::default());
    app.init_resource::<crate::state::State<AppState>>();

    // Register components (autozig-ecs safety)
    app.world_mut().register_component::<TextToPrint>();
    app.world_mut().register_component::<PrinterTick>();
    app.world_mut().register_component::<MenuClose>();
    app.world_mut().register_component::<LevelUnload>();

    // Systems
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = 
        setup_system.into_system();
    app.add_systems(Startup, setup_sys);

    let print_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, Time>, Query<'static, (&'static mut PrinterTick, &'static TextToPrint)>)>, _> = 
        print_text_system.into_system();
    
    let transition_sys: ParamFunctionSystem<FunctionMarker<((), ResMut<'static, NextState<AppState>>)>, _> = 
         transition_to_in_game_system.into_system();
         // Note: .run_if(in_state(...)) support
    
    // Combine with run_if
    use autozig_ecs::state::in_state;
    // let transition_sys = transition_sys.run_if(in_state(AppState::MainMenu));
    // Implementation details for proper piping/conditions might vary slightly.
    // For now add without condition to test compilation or add if conditions work.
    
    app.add_systems(Update, print_sys);
    // app.add_systems(Update, transition_sys.run_if(in_state(AppState::MainMenu)));
    
    // Generic cleanup systems
    let cleanup_menu: ParamFunctionSystem<FunctionMarker<((), Commands<'static>, Query<'static, Entity, With<MenuClose>>)>, _> = 
        cleanup_system::<MenuClose>.into_system();
    
    let cleanup_level: ParamFunctionSystem<FunctionMarker<((), Commands<'static>, Query<'static, Entity, With<LevelUnload>>)>, _> = 
        cleanup_system::<LevelUnload>.into_system();

    app.add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    app.add_systems(OnExit(AppState::InGame), cleanup_level);
    
    // Simulate Input for transition?
    // Since we don't have real Input resource from winit, we'll simulate explicit state change after some time 
    // or just run for a bit.
    
    println!("Starting Generic System Example...");
    // app.run(); 
    // Use ScheduleRunner
    app.set_runner(|mut app| {
        println!("Running app loop...");
        for _ in 0..10 {
            app.update();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    app.run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn((
        PrinterTick(Timer::from_seconds(1.0, TimerMode::Repeating)),
        TextToPrint("I will print until you press space (or state changes).".to_string()),
        MenuClose,
    ));

    commands.spawn((
        PrinterTick(Timer::from_seconds(1.0, TimerMode::Repeating)),
        TextToPrint("I will always print".to_string()),
        LevelUnload,
    ));
}

    for (mut timer, text) in &mut query {
        timer.tick(time.delta_nanos());
        if timer.just_finished() {
            println!("{}", text.0);
        }
    }
}

fn transition_to_in_game_system(
    mut next_state: ResMut<NextState<AppState>>,
    // keyboard_input: Res<ButtonInput<KeyCode>>, // Missing Input resource in minimal example
) {
    // Simulate input
    // if keyboard_input.pressed(KeyCode::Space) {
    //    next_state.set(AppState::InGame);
    // }
}

fn cleanup_system<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}
