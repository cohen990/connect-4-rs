use std::fmt::Display;

use super::win_conditions::WinCondition;

pub const DEFAULT_COLUMNS: usize = 7;
pub const DEFAULT_ROWS: usize = 6;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Player {
    None,
    One,
    Two,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::None => f.write_str("None"),
            Player::One => f.write_str("One"),
            Player::Two => f.write_str("Two"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum GameStatus {
    Started,
    Completed,
    Draw,
}

#[derive(Debug)]
pub struct GameError<'a> {
    pub message: String,
    pub previous_state: Game<'a>,
}

impl<'a> GameError<'a> {
    pub fn with_message(message: &str, previous_state: Game<'a>) -> Self {
        Self {
            message: message.to_owned(),
            previous_state,
        }
    }
}

#[derive(Clone)]
pub struct Game<'a> {
    game_board: Vec<Vec<Player>>,
    win_conditions: &'a Vec<Box<dyn WinCondition>>,
    pub winner: Option<Player>,
    pub status: GameStatus,
    pub current: Player,
}

impl<'a> std::fmt::Debug for Game<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game")
            .field("current", &self.current)
            .field("game_board", &self.game_board)
            .field("winner", &self.winner)
            .field("status", &self.status)
            .finish()
    }
}

impl<'a> std::fmt::Display for Game<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: String = "\n".to_owned();
        for j in (0..self.game_board[0].len()).rev() {
            for i in 0..self.game_board.len() {
                output += match self.game_board[i][j] {
                    Player::None => ".",
                    Player::One => "x",
                    Player::Two => "o",
                }
            }
            output += "\n"
        }
        for i in 0..self.game_board.len() {
            output += &i.to_string();
        }
        output += "\n";
        f.write_str(&output)
    }
}

impl<'a> Game<'a> {
    pub fn initialise(
        columns: usize,
        rows: usize,
        win_conditions: &'a Vec<Box<dyn WinCondition>>,
    ) -> Self {
        Game {
            current: Player::One,
            game_board: vec![vec![Player::None; rows]; columns],
            winner: None,
            status: GameStatus::Started,
            win_conditions,
        }
    }

    // Plays on the column - zero indexed
    pub fn play_on_column(self, column: usize) -> Result<Self, GameError<'a>> {
        let mut active_state = self.clone();
        let old_state = self;
        if column >= old_state.game_board.len() {
            return Err(GameError::with_message(
                "Game board does not have that many columns.",
                old_state,
            ));
        }

        if let Some(error) = active_state.place_piece(column).err() {
            return Err(GameError::with_message(error, old_state));
        }

        let mut available_move_count = 0;
        for column in 0..active_state.game_board.len() {
            for row in 0..active_state.game_board[column].len() {
                match active_state.game_board[column][row] {
                    Player::None => available_move_count += 1,
                    _ => {
                        if active_state.has_four_connected(column, row) {
                            active_state.status = GameStatus::Completed;
                            active_state.winner = Some(active_state.current);
                            return Ok(active_state);
                        }
                    }
                }
            }
        }

        if active_state.status != GameStatus::Completed && available_move_count == 0 {
            active_state.status = GameStatus::Draw;
            return Ok(active_state);
        }

        active_state.current = match active_state.current {
            Player::One => Player::Two,
            Player::Two => Player::One,
            Player::None => panic!("Invalid game state"),
        };
        Ok(active_state)
    }

    fn place_piece(&mut self, column: usize) -> Result<(), &str> {
        dbg!(&self);
        for row in 0..self.game_board[column].len() {
            if self.game_board[column][row] == Player::None {
                self.game_board[column][row] = self.current;
                dbg!(&self);
                return Ok(());
            }
        }
        Err("Column is full")
    }

    fn has_four_connected(&self, column: usize, row: usize) -> bool {
        for win_condition in self.win_conditions {
            if win_condition.is_met(&self.game_board, column, row) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::remove_consts::{
        game::{GameStatus, Player, DEFAULT_COLUMNS, DEFAULT_ROWS},
        win_conditions::default_win_conditions,
    };

    use super::Game;

    #[test]
    fn game_starts_with_the_player_one_playing_first() {
        let win_conditions = default_win_conditions();
        let game = Game::initialise(DEFAULT_COLUMNS, DEFAULT_ROWS, &win_conditions);
        assert_eq!(GameStatus::Started, game.status);
        assert_eq!(Player::One, game.current);
    }

    #[test]
    fn player_one_and_player_two_take_turns() {
        let win_conditions = default_win_conditions();
        let mut game = Game::initialise(DEFAULT_COLUMNS, DEFAULT_ROWS, &win_conditions);
        assert_eq!(Player::One, game.current);
        game = game.play_on_column(1).unwrap();
        assert_eq!(Player::Two, game.current);
        game = game.play_on_column(1).unwrap();
        assert_eq!(Player::One, game.current);
    }

    #[test]
    fn cannot_play_on_a_column_outside_the_board() {
        let win_conditions = default_win_conditions();
        let game = Game::initialise(1, DEFAULT_ROWS, &win_conditions);
        let result = game.play_on_column(1);
        assert!(result.is_err())
    }

    #[test]
    fn cannot_stack_a_column_beyond_the_row_size_of_the_board() {
        let win_conditions = default_win_conditions();
        let mut game = Game::initialise(2, 1, &win_conditions);
        game = game.play_on_column(0).unwrap();
        let result = game.play_on_column(0);
        assert!(result.is_err())
    }

    #[test]
    fn draws_the_game_if_all_positions_have_been_played() {
        let win_conditions = default_win_conditions();
        let mut game = Game::initialise(1, 1, &win_conditions);
        game = game.play_on_column(0).unwrap();
        assert_eq!(game.status, GameStatus::Draw)
    }

    /*
    o . . .
    o . . .
    o . . .
    x x x x
    */
    #[test]
    fn recognises_a_win_along_the_horizontal() {
        let win_conditions = default_win_conditions();
        let mut game = Game::initialise(4, 4, &win_conditions);
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(1).unwrap();
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(2).unwrap();
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(3).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert!(game.winner.is_some());
        assert_eq!(game.winner.unwrap(), Player::One);
    }

    /*
    x . . .
    x . . .
    x . . .
    x o o o
    */
    #[test]
    fn recognises_a_win_along_the_vertical() {
        let win_conditions = default_win_conditions();
        let mut game = Game::initialise(4, 4, &win_conditions);
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(1).unwrap();
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(2).unwrap();
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(3).unwrap();
        game = game.play_on_column(0).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert!(game.winner.is_some());
        assert_eq!(game.winner.unwrap(), Player::One);
    }

    /*
    . . . x
    x x x o
    x x o o
    x o o o
    */
    #[test]
    fn recognises_a_win_along_the_positive_diagonal() {
        let win_conditions = default_win_conditions();
        let mut game = Game::initialise(4, 4, &win_conditions);
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(1).unwrap();
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(2).unwrap();
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(2).unwrap();
        game = game.play_on_column(1).unwrap();
        game = game.play_on_column(3).unwrap();
        game = game.play_on_column(1).unwrap();
        game = game.play_on_column(3).unwrap();
        game = game.play_on_column(2).unwrap();
        game = game.play_on_column(3).unwrap();
        game = game.play_on_column(3).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert!(game.winner.is_some());
        assert_eq!(game.winner.unwrap(), Player::One);
    }

    /*
    x . . .
    o x x x
    o o x x
    o o o x
    */
    #[test]
    fn recognises_a_win_along_the_negative_diagonal() {
        let win_conditions = default_win_conditions();
        let mut game = Game::initialise(4, 4, &win_conditions);
        game = game.play_on_column(3).unwrap();
        game = game.play_on_column(2).unwrap();
        game = game.play_on_column(3).unwrap();
        game = game.play_on_column(1).unwrap();
        game = game.play_on_column(3).unwrap();
        game = game.play_on_column(1).unwrap();
        game = game.play_on_column(2).unwrap();
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(2).unwrap();
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(1).unwrap();
        game = game.play_on_column(0).unwrap();
        game = game.play_on_column(0).unwrap();
        assert_eq!(game.status, GameStatus::Completed);
        assert!(game.winner.is_some());
        assert_eq!(game.winner.unwrap(), Player::One);
    }

    #[test]
    fn when_playing_an_invalid_move_can_try_to_find_a_different_move() {
        let win_conditions = default_win_conditions();
        let mut game = Game::initialise(1, 1, &win_conditions);
        let result = game.play_on_column(3);
        assert!(result.is_err());
        game = result.unwrap_err().previous_state;
        let result = game.play_on_column(0);
        assert!(result.is_ok())
    }
}
