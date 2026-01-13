use autozig_ecs::prelude::*;
use core::time::Duration;
use rand::random;
use std::fmt;

// COMPONENTS
#[derive(Component)]
struct Player {
    name: String,
}

#[derive(Component)]
struct Score {
    value: usize,
}

#[derive(Component)]
enum PlayerStreak {
    Hot(usize),
    None,
    Cold(usize),
}

impl fmt::Display for PlayerStreak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerStreak::Hot(n) => write!(f, "{n} round hot streak"),
            PlayerStreak::None => write!(f, "0 round streak"),
            PlayerStreak::Cold(n) => write!(f, "{n} round cold streak"),
        }
    }
}

// RESOURCES
#[derive(Default)]
struct GameState {
    current_round: usize,
    total_players: usize,
    winning_player: Option<String>,
}

#[derive(Component)] // Not deriving Resource to avoid conflict blanket impl. Actually it is just struct.
struct GameRules {
    winning_score: usize,
    max_rounds: usize,
    max_players: usize,
}

// Helper for "Local" replacement (as requested previously)
#[derive(Default)]
struct PrintCounter(u32);

// SYSTEMS
fn print_message_system() {
    println!("This game is fun!");
}

fn new_round_system(game_rules: Res<GameRules>, mut game_state: ResMut<GameState>) {
    game_state.current_round += 1;
    println!(
        "Begin round {} of {}",
        game_state.current_round, game_rules.max_rounds
    );
}

fn score_system(mut query: Query<(&'static Player, &'static mut Score, &'static mut PlayerStreak)>) {
    for (player, mut score, mut streak) in &mut query {
        let scored_a_point = random::<bool>();
        if scored_a_point {
            score.value += 1;
            *streak = match *streak {
                PlayerStreak::Hot(n) => PlayerStreak::Hot(n + 1),
                PlayerStreak::Cold(_) | PlayerStreak::None => PlayerStreak::Hot(1),
            };
            println!(
                "{} scored a point! Their score is: {} ({})",
                player.name, score.value, *streak
            );
        } else {
            *streak = match *streak {
                PlayerStreak::Hot(_) | PlayerStreak::None => PlayerStreak::Cold(1),
                PlayerStreak::Cold(n) => PlayerStreak::Cold(n + 1),
            };

            println!(
                "{} did not score a point! Their score is: {} ({})",
                player.name, score.value, *streak
            );
        }
    }
}

fn score_check_system(
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
    query: Query<(&'static Player, &'static Score)>,
) {
    for (player, score) in &query {
        if score.value == game_rules.winning_score {
            game_state.winning_player = Some(player.name.clone());
        }
    }
}

// Note: Using EventWriter directly from autozig-ecs
fn game_over_system(
    game_rules: Res<GameRules>,
    game_state: Res<GameState>,
    mut app_exit_writer: EventWriter<AppExit>,
) {
    if let Some(ref player) = game_state.winning_player {
        println!("{player} won the game!");
        app_exit_writer.send(AppExit::Success);
    } else if game_state.current_round == game_rules.max_rounds {
        println!("Ran out of rounds. Nobody wins!");
        app_exit_writer.send(AppExit::Success);
    }
}

fn startup_system(/*mut commands: Commands,*/ mut game_state: ResMut<GameState>) {
    /*
    commands.insert_resource(GameRules {
        max_rounds: 10,
        winning_score: 4,
        max_players: 4,
    });

    commands.spawn_batch(vec![
        (
            Player {
                name: "Alice".to_string(),
            },
            Score { value: 0 },
            PlayerStreak::None,
        ),
        (
            Player {
                name: "Bob".to_string(),
            },
            Score { value: 0 },
            PlayerStreak::None,
        ),
    ]);
    */

    game_state.total_players = 2;
}

fn new_player_system(
    mut commands: Commands,
    game_rules: Res<GameRules>,
    mut game_state: ResMut<GameState>,
) {
    let add_new_player = random::<bool>();
    if add_new_player && game_state.total_players < game_rules.max_players {
        game_state.total_players += 1;
        commands.spawn((
            Player {
                name: format!("Player {}", game_state.total_players),
            },
            Score { value: 0 },
            PlayerStreak::None,
        ));

        println!("Player {} joined the game!", game_state.total_players);
    }
}

fn exclusive_player_system(world: &mut World) {
    // Note: API parity check usually needed here. 
    // Assuming autozig World has resource<T>()
    // If resource() is not available, we need to check autozig-ecs API.
    // Based on previous viewing, we implemented `resource` and `resource_mut`.
    let total_players = world.resource::<GameState>().total_players;
    
    let should_add_player = {
        let game_rules = world.resource::<GameRules>();
        let add_new_player = random::<bool>();
        add_new_player && total_players < game_rules.max_players
    };
    
    if should_add_player {
        println!("Player {} has joined the game!", total_players + 1);
        world.spawn((
            Player {
                name: format!("Player {}", total_players + 1),
            },
            Score { value: 0 },
            PlayerStreak::None,
        ));

        let mut game_state = world.resource_mut::<GameState>();
        game_state.total_players += 1;
    }
}

fn print_at_end_round(mut counter: ResMut<PrintCounter>) {
    counter.0 += 1;
    println!("In set 'Last' for the {}th time", counter.0);
    println!();
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum MySystems {
    BeforeRound,
    Round,
    AfterRound,
}

impl SystemSet for MySystems {
    fn as_str(&self) -> &str {
        match self {
            MySystems::BeforeRound => "BeforeRound",
            MySystems::Round => "Round",
            MySystems::AfterRound => "AfterRound",
        }
    }
}

fn main() {
    fn assert_component<T: autozig_ecs::component::Component>() {}
    fn assert_resource<T: autozig_ecs::resource::Resource>() {}
    
    assert_component::<Player>();
    assert_component::<Score>();
    assert_component::<PlayerStreak>();
    assert_resource::<GameState>();

    let mut app = App::new();
    
    app.init_resource::<GameState>();
    app.init_resource::<PrintCounter>();
    
    app.add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(0.1)));
    
    app.add_systems::<((ResMut<'static, GameState>),)>(Startup, startup_system);
    app.add_systems(Update, print_message_system);
    app.add_systems::<((Res<'static, GameRules>, ResMut<'static, GameState>),)>(Update, new_round_system);
    app.add_systems::<((Query<'static, (&'static Player, &'static mut Score, &'static mut PlayerStreak)>,),)>(Update, score_system);
    app.add_systems::<((ResMut<'static, PrintCounter>),)>(Last, print_at_end_round);
    
    app.run();
}
