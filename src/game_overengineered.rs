use std::fmt::Error;

pub const DEFAULT_COLUMNS: usize = 7;
pub const DEFAULT_ROWS: usize = 6;

trait WinCondition<const COLUMNS: usize, const ROWS: usize>: Sized {
    fn is_met(board: &[[Player; ROWS]; COLUMNS], column: usize, row: usize) -> bool;
}

#[derive(Debug, Clone)]
struct VerticalWinCondition {}

impl<const COLUMNS: usize, const ROWS: usize> WinCondition<COLUMNS, ROWS> for VerticalWinCondition {
    fn is_met(board: &[[Player; ROWS]; COLUMNS], column: usize, row: usize) -> bool {
        row + 3 < board[column].len()
            && board[column][row] != Player::None
            && board[column][row] == board[column][row + 1]
            && board[column][row] == board[column][row + 2]
            && board[column][row] == board[column][row + 3]
    }
}

#[derive(Debug, Clone)]
struct HorizontalWinCondition {}

impl<const COLUMNS: usize, const ROWS: usize> WinCondition<COLUMNS, ROWS>
    for HorizontalWinCondition
{
    fn is_met(board: &[[Player; ROWS]; COLUMNS], column: usize, row: usize) -> bool {
        column + 3 < board.len()
            && board[column][row] != Player::None
            && board[column][row] == board[column + 1][row]
            && board[column][row] == board[column + 2][row]
            && board[column][row] == board[column + 3][row]
    }
}

#[derive(Debug, Clone)]
struct DiagonalWinCondition {}

impl<const COLUMNS: usize, const ROWS: usize> WinCondition<COLUMNS, ROWS> for DiagonalWinCondition {
    fn is_met(board: &[[Player; ROWS]; COLUMNS], column: usize, row: usize) -> bool {
        column + 3 < board.len()
            && row + 3 < board[column].len()
            && board[column][row] != Player::None
            && board[column][row] == board[column + 1][row + 1]
            && board[column][row] == board[column + 2][row + 2]
            && board[column][row] == board[column + 3][row + 3]
    }
}

#[derive(Debug, Clone)]
struct ReverseDiagonalWinCondition {}

impl<const COLUMNS: usize, const ROWS: usize> WinCondition<COLUMNS, ROWS>
    for ReverseDiagonalWinCondition
{
    fn is_met(board: &[[Player; ROWS]; COLUMNS], column: usize, row: usize) -> bool {
        column >= 3
            && row + 3 < board[column].len()
            && board[column][row] != Player::None
            && board[column][row] == board[column - 1][row + 1]
            && board[column][row] == board[column - 2][row + 2]
            && board[column][row] == board[column - 3][row + 3]
    }
}

#[derive(Debug, Clone)]
enum WinConditions {
    Vertical(VerticalWinCondition),
    Horizontal(HorizontalWinCondition),
    Diagonal(DiagonalWinCondition),
    ReverseDiagonal(ReverseDiagonalWinCondition),
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Player {
    None,
    One,
    Two,
}

#[derive(PartialEq, Debug, Clone)]
pub enum GameStatus {
    Started,
    Completed,
    Draw,
}

#[derive(Debug)]
pub struct GameError<const COLUMNS: usize, const ROWS: usize> {
    message: String,
    previous_state: Game<COLUMNS, ROWS>,
}

impl<const COLUMNS: usize, const ROWS: usize> GameError<COLUMNS, ROWS> {
    pub fn with_message(message: &str, previous_state: Game<COLUMNS, ROWS>) -> Self {
        Self {
            message: message.to_owned(),
            previous_state,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Game<const COLUMNS: usize, const ROWS: usize> {
    current: Player,
    game_board: [[Player; ROWS]; COLUMNS],
    win_conditions: Vec<WinConditions>,
    pub winner: Option<Player>,
    pub status: GameStatus,
}

impl<const COLUMNS: usize, const ROWS: usize> Game<COLUMNS, ROWS> {
    pub fn initialise() -> Self {
        Game {
            current: Player::One,
            game_board: [[Player::None; ROWS]; COLUMNS],
            winner: None,
            win_conditions: vec![WinConditions::Vertical(VerticalWinCondition {})],
            status: GameStatus::Started,
        }
    }

    // Plays on the column - zero indexed
    pub fn play_on_column(self, column: usize) -> Result<Self, GameError<COLUMNS, ROWS>> {
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
        for row in 0..self.game_board[column].len() {
            if self.game_board[column][row] == Player::None {
                self.game_board[column][row] = self.current;
                return Ok(());
            }
        }
        Err("Column is full")
    }

    // I couldn't resist
    // I can't make it work how I want.
    // I'll remove the file from the crate so it doesn't compile and block. I may come back to it later.
    fn has_four_connected(&self, column: usize, row: usize) -> bool {
        for condition in self.win_conditions {
            if let condition(inner) = thing {
                return thing.is_met(self, column, row);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::game_overengineered::{GameStatus, Player, DEFAULT_COLUMNS, DEFAULT_ROWS};

    use super::Game;

    #[test]
    fn game_starts_with_the_player_one_playing_first() {
        let game = Game::<DEFAULT_COLUMNS, DEFAULT_ROWS>::initialise();
        assert_eq!(GameStatus::Started, game.status);
        assert_eq!(Player::One, game.current);
    }

    #[test]
    fn player_one_and_player_two_take_turns() {
        let mut game = Game::<DEFAULT_COLUMNS, DEFAULT_ROWS>::initialise();
        assert_eq!(Player::One, game.current);
        game = game.play_on_column(1).unwrap();
        assert_eq!(Player::Two, game.current);
        game = game.play_on_column(1).unwrap();
        assert_eq!(Player::One, game.current);
    }

    #[test]
    fn cannot_play_on_a_column_outside_the_board() {
        let game = Game::<1, DEFAULT_ROWS>::initialise();
        let result = game.play_on_column(1);
        assert!(result.is_err())
    }

    #[test]
    fn cannot_stack_a_column_beyond_the_row_size_of_the_board() {
        let mut game = Game::<2, 1>::initialise();
        game = game.play_on_column(0).unwrap();
        let result = game.play_on_column(0);
        assert!(result.is_err())
    }

    #[test]
    fn draws_the_game_if_all_positions_have_been_played() {
        let mut game = Game::<1, 1>::initialise();
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
        let mut game = Game::<4, 4>::initialise();
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
        let mut game = Game::<4, 4>::initialise();
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
        let mut game = Game::<4, 4>::initialise();
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
        let mut game = Game::<4, 4>::initialise();
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
        let mut game = Game::<1, 1>::initialise();
        let result = game.play_on_column(3);
        assert!(result.is_err());
        game = result.unwrap_err().previous_state;
        let result = game.play_on_column(0);
        assert!(result.is_ok())
    }
}
