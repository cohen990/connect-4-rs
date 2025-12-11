use std::io;

use crate::overengineered::{
    game::{Game, GameStatus, DEFAULT_COLUMNS, DEFAULT_ROWS},
    win_conditions::{
        default_win_conditions, DiagonalWinCondition, HorizontalWinCondition,
        ReverseDiagonalWinCondition, VerticalWinCondition, WinCondition,
    },
};

fn prompt_yes_no(stdin: &io::Stdin, input: &mut String, prompt: &str) -> bool {
    println!("{}", prompt);
    input.clear();
    stdin.read_line(input).expect("Error reading from stdio");
    input.trim() != "n"
}

fn select_win_conditions(
    stdin: &io::Stdin,
    input: &mut String,
) -> Vec<Box<dyn WinCondition<DEFAULT_COLUMNS, DEFAULT_ROWS>>> {
    if prompt_yes_no(stdin, input, "Would you play to play with the standard ruleset? Y/n") {
        return default_win_conditions();
    }

    let condition_configs: [(&str, &dyn Fn() -> Box<dyn WinCondition<DEFAULT_COLUMNS, DEFAULT_ROWS>>); 4] = [
        ("Do you want to allow for vertical connect 4s? Y/n", &|| VerticalWinCondition::boxed()),
        ("Do you want to allow for horizontal connect 4s? Y/n", &|| HorizontalWinCondition::boxed()),
        ("Do you want to allow for forward diagonal connect 4s? Y/n", &|| DiagonalWinCondition::boxed()),
        ("Do you want to allow for backwards diagonal connect 4s? Y/n", &|| ReverseDiagonalWinCondition::boxed()),
    ];

    condition_configs
        .iter()
        .filter(|(prompt, _)| prompt_yes_no(stdin, input, prompt))
        .map(|(_, constructor)| constructor())
        .collect()
}

fn read_column_input(stdin: &io::Stdin, input: &mut String) -> Option<usize> {
    input.clear();
    stdin.read_line(input).expect("Error reading from stdio");
    match input.trim().parse() {
        Ok(column) => Some(column),
        Err(_) => {
            eprintln!(
                "The input <{}> could not be parsed as a usize. Please try again.",
                input
            );
            None
        }
    }
}

fn check_game_status(game: &Game<DEFAULT_COLUMNS, DEFAULT_ROWS>) -> bool {
    match game.status {
        GameStatus::Started => true,
        GameStatus::Completed => {
            println!(
                "Player {} wins!",
                game.winner
                    .expect("Game has been win with no winner. Invalid state.")
            );
            false
        }
        GameStatus::Draw => {
            println!("It's a draw!");
            false
        }
    }
}

fn execute_move(
    game: Game<DEFAULT_COLUMNS, DEFAULT_ROWS>,
    column: usize,
) -> Game<DEFAULT_COLUMNS, DEFAULT_ROWS> {
    match game.play_on_column(column) {
        Ok(game) => game,
        Err(error) => {
            eprintln!("{}", error.message);
            error.previous_state
        }
    }
}

fn run_game(
    stdin: &io::Stdin,
    input: &mut String,
    win_conditions: &Vec<Box<dyn WinCondition<DEFAULT_COLUMNS, DEFAULT_ROWS>>>,
) {
    let mut game = Game::initialise(win_conditions);
    loop {
        println!("{}", game);
        if !check_game_status(&game) {
            break;
        }
        println!(
            "Player {}'s turn. Which column would you like to play in? 0-{}",
            game.current,
            DEFAULT_COLUMNS - 1
        );
        let column = match read_column_input(stdin, input) {
            Some(column) => column,
            None => continue,
        };
        game = execute_move(game, column);
    }
}

fn setup_and_play_game(stdin: &io::Stdin, input: &mut String) -> bool {
    if !prompt_yes_no(stdin, input, "Would you like to play with a default gameboard? Y/n") {
        println!("Different game boards feature coming soon. Starting over.");
        return true;
    }

    let win_conditions = select_win_conditions(stdin, input);

    let printable_win_conditions: Vec<String> =
        win_conditions.iter().map(|x| format!("{}", x)).collect();

    println!(
        "Beginning a game with the following win conditions: {}",
        printable_win_conditions.join(", ")
    );

    run_game(stdin, input, &win_conditions);

    prompt_yes_no(stdin, input, "Would you like to play again? Y/n")
}

pub fn play() {
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        println!("<<Customisable Ruleset Mode>>");
        if !setup_and_play_game(&stdin, input) {
            println!("Returning to the main menu.\n");
            break;
        }
    }
}
