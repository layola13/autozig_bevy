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
#[derive(Default, Debug)]
struct GameState {
    current_round: usize,
    total_players: usize,
    winning_player: Option<String>,
}

#[derive(Debug)]
struct GameRules {
    winning_score: usize,
    max_rounds: usize,
    max_players: usize,
}

// Helper for "Local" replacement (as requested previously)
#[derive(Default, Debug)]
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
    for (player, mut score, mut streak) in query.iter() {
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
    for (player, score) in query.iter() {
        if score.value >= game_rules.winning_score {
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
    } else if game_state.current_round >= game_rules.max_rounds {
        println!("Ran out of rounds. Nobody wins!");
        app_exit_writer.send(AppExit::Success);
    }
}

fn startup_system(mut commands: Commands, mut game_state: ResMut<GameState>) {
    commands.spawn((
        Player {
            name: "Alice".to_string(),
        },
        Score { value: 0 },
        PlayerStreak::None,
    ));
    commands.spawn((
        Player {
            name: "Bob".to_string(),
        },
        Score { value: 0 },
        PlayerStreak::None,
    ));

    game_state.total_players = 2;
    println!("Spawned initial players: Alice and Bob");
}

fn print_at_end_round(mut counter: ResMut<PrintCounter>) {
    counter.0 += 1;
    println!("End of frame counter: {}", counter.0);
    println!("----------------------------------");
}

fn main() {
    println!("DEBUG: Entered main");
    let mut app = App::new();
    
    app.init_resource::<GameState>();
    app.init_resource::<PrintCounter>();
    app.insert_resource(GameRules {
        winning_score: 4,
        max_rounds: 5,
        max_players: 4,
    });
    
    app.add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(50)));
    
    app.add_systems(Startup, startup_system);
    app.add_systems(Update, (
        new_round_system,
        score_system,
        score_check_system,
        game_over_system,
    ));
    
    app.add_systems(Last, print_at_end_round);
    
    println!("Starting Bevy ECS Parity Demo...");
    use std::io::Write;
    let _ = std::io::stdout().flush();
    app.update();
    println!("Update finished.");
}
