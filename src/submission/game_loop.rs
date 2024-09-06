use std::io;

use crate::submission::game::GameStatus;

use super::game::{Game, DEFAULT_COLUMNS, DEFAULT_ROWS};

pub fn play() {
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        println!("<<Normal Mode>>");

        let mut game: Game<DEFAULT_COLUMNS, DEFAULT_ROWS> = Game::initialise();

        loop {
            println!("{}", game);
            match game.status {
                GameStatus::Started => (),
                GameStatus::Completed => {
                    println!(
                        "Player {} wins!",
                        game.winner
                            .expect("Game has been win with no winner. Invalid state.")
                    );
                    break;
                }
                GameStatus::Draw => {
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
            println!("Returning to the main menu.");
            break;
        }
    }
}
