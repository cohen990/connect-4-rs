use std::io;

use crate::overengineered::{
    game::{Game, DEFAULT_COLUMNS, DEFAULT_ROWS},
    win_conditions::{
        default_win_conditions, DiagonalWinCondition, HorizontalWinCondition,
        ReverseDiagonalWinCondition, VerticalWinCondition, WinCondition,
    },
};

pub fn play() {
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        println!("Welcome to connect 4");
        println!("Would you like to play with a default gameboard? Y/n");
        input.clear();
        stdin.read_line(input).expect("Error reading from stdio");
        if input.trim() == "n" {
            println!("Different game boards feature coming soon. Starting over.");
            continue;
        }
        println!("Would you play to play with the standard ruleset? Y/n");
        input.clear();
        stdin.read_line(input).expect("Error reading from stdio");
        let mut win_conditions: Vec<Box<dyn WinCondition<DEFAULT_COLUMNS, DEFAULT_ROWS>>> = vec![];
        if input.trim() == "n" {
            println!("Do you want to allow for vertical connect 4s? Y/n");
            input.clear();
            stdin.read_line(input).expect("Error reading from stdio");
            if input.trim() != "n" {
                win_conditions.push(VerticalWinCondition::boxed())
            }

            println!("Do you want to allow for horizontal connect 4s? Y/n");
            input.clear();
            stdin.read_line(input).expect("Error reading from stdio");
            if input.trim() != "n" {
                win_conditions.push(HorizontalWinCondition::boxed())
            }

            println!("Do you want to allow for forward diagonal connect 4s? Y/n");
            input.clear();
            stdin.read_line(input).expect("Error reading from stdio");
            if input.trim() != "n" {
                win_conditions.push(DiagonalWinCondition::boxed())
            }
            println!("Do you want to allow for backwards diagonal connect 4s? Y/n");
            input.clear();
            stdin.read_line(input).expect("Error reading from stdio");
            if input.trim() != "n" {
                win_conditions.push(ReverseDiagonalWinCondition::boxed())
            }
        } else {
            win_conditions = default_win_conditions()
        }

        let printable_win_conditions: Vec<String> =
            win_conditions.iter().map(|x| format!("{}", x)).collect();

        println!(
            "Beginning a game with the following win conditions: {}",
            printable_win_conditions.join(", ")
        );

        let mut game = Game::initialise(&win_conditions);
        loop {
            println!("{}", game);
            match game.status {
                crate::overengineered::game::GameStatus::Started => (),
                crate::overengineered::game::GameStatus::Completed => {
                    println!(
                        "Player {} wins!",
                        game.winner
                            .expect("Game has been win with no winner. Invalid state.")
                    );
                    break;
                }
                crate::overengineered::game::GameStatus::Draw => {
                    println!("It's a draw!");
                    break;
                }
            }
            println!(
                "Player {}'s turn. Which column would you like to play in? 0-{}",
                game.current,
                DEFAULT_COLUMNS - 1
            );
            input.clear();
            stdin.read_line(input).expect("Error reading from stdio");
            let column: usize = match input.trim().parse() {
                Ok(column) => column,
                Err(_) => {
                    eprintln!(
                        "The input <{}> could not be parsed as a usize. Please try again.",
                        input
                    );
                    continue;
                }
            };
            game = match game.play_on_column(column) {
                Ok(game) => game,
                Err(error) => {
                    eprintln!("{}", error.message);
                    error.previous_state
                }
            }
        }

        println!("Would you like to play again? Y/n");
        input.clear();
        stdin.read_line(input).expect("Error reading from stdio");
        if input.trim() == "n" {
            break;
        }
    }
}
