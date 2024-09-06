use std::io;

use crate::remove_consts::{
    game::{Game, GameStatus, DEFAULT_COLUMNS, DEFAULT_ROWS},
    win_conditions::{
        default_win_conditions, DiagonalWinCondition, HorizontalWinCondition,
        ReverseDiagonalWinCondition, VerticalWinCondition, WinCondition,
    },
};

pub fn play() {
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        println!("<<Fully Customisable Mode>>");
        println!("Would you like to play with a default gameboard? Y/n");
        input.clear();
        stdin.read_line(input).expect("Error reading from stdio");
        let mut columns: usize = DEFAULT_COLUMNS;
        let mut rows: usize = DEFAULT_ROWS;
        if input.trim() == "n" {
            loop {
                println!("How many columns?");
                input.clear();
                stdin.read_line(input).expect("Error reading from stdio");
                match input.trim().parse::<usize>() {
                    Ok(parsed) => {
                        columns = parsed;
                        break;
                    }
                    Err(_) => {
                        eprintln!(
                            "The input <{}> could not be parsed as a usize. Please try again.",
                            input
                        );
                        continue;
                    }
                };
            }
            loop {
                println!("How many rows?");
                input.clear();
                stdin.read_line(input).expect("Error reading from stdio");
                match input.trim().parse::<usize>() {
                    Ok(parsed) => {
                        rows = parsed;
                        break;
                    }
                    Err(_) => {
                        eprintln!(
                            "The input <{}> could not be parsed as a usize. Please try again.",
                            input
                        );
                        continue;
                    }
                };
            }
        }
        println!("Would you play to play with the standard ruleset? Y/n");
        input.clear();
        stdin.read_line(input).expect("Error reading from stdio");
        let mut win_conditions: Vec<Box<dyn WinCondition>> = vec![];
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
            "Beginning a game of board size: [{},{}], with the following win conditions: {}",
            columns,
            rows,
            printable_win_conditions.join(", ")
        );

        let mut game = Game::initialise(columns, rows, &win_conditions);
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
            println!("Returning to the main menu.\n");
            break;
        }
    }
}
