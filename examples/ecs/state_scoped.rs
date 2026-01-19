//! state_scoped.rs - Demonstrate state scoped entities
//!
//! Ported from Bevy examples/ecs/state_scoped.rs (Simplified)

use autozig_ecs::prelude::*;
use autozig_ecs::into_system::{IntoSystem, ParamFunctionSystem, FunctionMarker};
use autozig_ecs::state::{States, State, NextState, OnEnter, OnExit};
use autozig_time::{Time, Timer, TimerMode};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
enum GameState {
    #[default]
    A,
    B,
}



struct TickTock(Timer);

// Custom component to mark entities for despawn
#[derive(Component)]
struct DespawnOnExitA;

#[derive(Component)]
struct DespawnOnExitB;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    
    // Manual State Init
    app.init_state::<GameState>();
    
    // Manually register state schedules
    {
        use autozig_ecs::schedule::{Schedule, Schedules};
        if let Some(mut schedules) = app.world_mut().get_resource_mut::<Schedules>() {
            println!("Registering state schedules...");
            schedules.insert(Schedule::new(OnEnter(GameState::A)));
            schedules.insert(Schedule::new(OnExit(GameState::A)));
            schedules.insert(Schedule::new(OnEnter(GameState::B)));
            schedules.insert(Schedule::new(OnExit(GameState::B)));
        } else {
             println!("Schedules resource NOT FOUND during manual init!");
        }
    }

    app.insert_resource(TickTock(Timer::from_seconds(1.0, TimerMode::Repeating)));

    // Components registration
    app.world_mut().register_component::<DespawnOnExitA>();
    app.world_mut().register_component::<DespawnOnExitB>();
    
    // Systems
    let setup_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = setup_camera.into_system();
    app.add_systems(Startup, setup_sys);
    
    let on_a_enter_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = on_a_enter.into_system();
    app.add_systems(OnEnter(GameState::A), on_a_enter_sys);

    let on_b_enter_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>)>, _> = on_b_enter.into_system();
    app.add_systems(OnEnter(GameState::B), on_b_enter_sys);
    
    let despawn_a_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>, Query<'static, Entity, With<DespawnOnExitA>>)>, _> = despawn_a.into_system();
    app.add_systems(OnExit(GameState::A), despawn_a_sys);
    
    let despawn_b_sys: ParamFunctionSystem<FunctionMarker<((), Commands<'static>, Query<'static, Entity, With<DespawnOnExitB>>)>, _> = despawn_b.into_system();
    app.add_systems(OnExit(GameState::B), despawn_b_sys);
    
    let toggle_sys: ParamFunctionSystem<FunctionMarker<((), Res<'static, Time>, ResMut<'static, TickTock>, Res<'static, State<GameState>>, ResMut<'static, NextState<GameState>>)>, _> = toggle.into_system();
    app.add_systems(Update, toggle_sys);

    println!("Starting State Scoped Example...");
    app.set_runner(|mut app| {
        println!("Game Loop...");
        for _ in 0..10 {
           app.update();
           std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    app.run();
}

fn setup_camera(mut commands: Commands) {
    // commands.spawn(Camera3d::default()); // No Camera3d in minimal ecs
    println!("Camera setup");
}

fn on_a_enter(mut commands: Commands) {
    println!("State A Enter: Spawning entities...");
    commands.spawn(DespawnOnExitA); // Marker
}

fn on_b_enter(mut commands: Commands) {
     println!("State B Enter: Spawning entities...");
    commands.spawn(DespawnOnExitB); // Marker
}

fn despawn_a(mut commands: Commands, query: Query<Entity, With<DespawnOnExitA>>) {
    println!("State A Exit: Despawning...");
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

fn despawn_b(mut commands: Commands, query: Query<Entity, With<DespawnOnExitB>>) {
    println!("State B Exit: Despawning...");
    for e in query.iter() {
        commands.entity(e).despawn();
    }
}

fn toggle(
    time: Res<Time>,
    mut timer: ResMut<TickTock>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.0.tick(time.delta_nanos());
    if timer.0.just_finished() {
        let current = state.get();
        let next = match current {
            GameState::A => GameState::B,
            GameState::B => GameState::A,
        };
        println!("Switching state to {:?}", next);
        next_state.set(next);
    }
}
