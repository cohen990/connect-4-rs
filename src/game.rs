pub const DEFAULT_COLUMNS: usize = 7;
pub const DEFAULT_ROWS: usize = 6;

#[derive(PartialEq, Debug)]
enum Player {
    One,
    Two,
}

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Started,
    Completed,
    Draw,
}

pub struct Game<const COLUMNS: usize, const ROWS: usize> {
    player_one_symbol: String,
    player_two_symbol: String,
    current: Player,
    game_board: [[usize; ROWS]; COLUMNS],
    pub winner: Option<String>,
    pub status: GameStatus,
}

impl<const COLUMNS: usize, const ROWS: usize> Game<COLUMNS, ROWS> {
    pub fn init(player_one_symbol: &str, player_two_symbol: &str) -> Self {
        Game {
            player_one_symbol: player_one_symbol.to_owned(),
            player_two_symbol: player_two_symbol.to_owned(),
            current: Player::One,
            game_board: [[0; ROWS]; COLUMNS],
            winner: None,
            status: GameStatus::Started,
        }
    }

    // Plays on the column - zero indexed
    pub fn play_on_column(self, column: usize) -> Self {
        if column >= self.game_board.len() {
            panic!("Play out of bounds")
        }

        let mut game = Self {
            player_one_symbol: self.player_one_symbol,
            player_two_symbol: self.player_two_symbol,
            current: self.current,
            game_board: self.game_board,
            winner: None,
            status: self.status,
        };

        let mut height = 0;
        loop {
            if height >= self.game_board[0].len() {
                // TODO: replace this panic with a Result pattern - the old state can be stashed with the result - or something
                panic!("Play out of bounds");
            }

            let next_empty_cell_in_column = self.game_board[column][height];
            if next_empty_cell_in_column == 0 {
                match game.current {
                    Player::One => game.game_board[column][height] = 1,
                    Player::Two => game.game_board[column][height] = 2,
                }
                break;
            }
            height += 1;
        }
        let mut available_move_count = 0;
        for i in 0..game.game_board.len() {
            for j in 0..game.game_board[i].len() {
                if i + 3 < game.game_board.len()
                    && game.game_board[i][j] != 0
                    && game.game_board[i][j] == game.game_board[i + 1][j]
                    && game.game_board[i][j] == game.game_board[i + 2][j]
                    && game.game_board[i][j] == game.game_board[i + 3][j]
                {
                    game.status = GameStatus::Completed;
                    game.winner = match game.current {
                        Player::One => Some(game.player_one_symbol.to_owned()),
                        Player::Two => Some(game.player_two_symbol.to_owned()),
                    };
                    return game;
                }
                if j + 3 < game.game_board[i].len()
                    && game.game_board[i][j] != 0
                    && game.game_board[i][j] == game.game_board[i][j + 1]
                    && game.game_board[i][j] == game.game_board[i][j + 2]
                    && game.game_board[i][j] == game.game_board[i][j + 3]
                {
                    game.status = GameStatus::Completed;
                    game.winner = match game.current {
                        Player::One => Some(game.player_one_symbol.to_owned()),
                        Player::Two => Some(game.player_two_symbol.to_owned()),
                    };
                    return game;
                }
                if i + 3 < game.game_board.len()
                    && j + 3 < game.game_board[i].len()
                    && game.game_board[i][j] != 0
                    && game.game_board[i][j] == game.game_board[i + 1][j + 1]
                    && game.game_board[i][j] == game.game_board[i + 2][j + 2]
                    && game.game_board[i][j] == game.game_board[i + 3][j + 3]
                {
                    game.status = GameStatus::Completed;
                    game.winner = match game.current {
                        Player::One => Some(game.player_one_symbol.to_owned()),
                        Player::Two => Some(game.player_two_symbol.to_owned()),
                    };
                    return game;
                }
                if i >= 3
                    && j + 3 < game.game_board[i].len()
                    && game.game_board[i][j] != 0
                    && game.game_board[i][j] == game.game_board[i - 1][j + 1]
                    && game.game_board[i][j] == game.game_board[i - 2][j + 2]
                    && game.game_board[i][j] == game.game_board[i - 3][j + 3]
                {
                    game.status = GameStatus::Completed;
                    game.winner = match game.current {
                        Player::One => Some(game.player_one_symbol.to_owned()),
                        Player::Two => Some(game.player_two_symbol.to_owned()),
                    };
                    return game;
                }
                if game.game_board[i][j] == 0 {
                    available_move_count += 1;
                }
            }
        }
        if game.status == GameStatus::Started && available_move_count == 0 {
            game.status = GameStatus::Draw
        }
        let new_player = match game.current {
            Player::One => Player::Two,
            Player::Two => Player::One,
        };
        game.current = new_player;
        game
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{GameStatus, Player, DEFAULT_COLUMNS, DEFAULT_ROWS};

    use super::Game;

    #[test]
    fn game_starts_with_the_player_one_playing_first() {
        let game = Game::<DEFAULT_COLUMNS, DEFAULT_ROWS>::init("x", "y");
        assert_eq!(GameStatus::Started, game.status);
        assert_eq!(Player::One, game.current);
    }

    #[test]
    fn player_one_and_player_two_take_turns() {
        let mut game = Game::<DEFAULT_COLUMNS, DEFAULT_ROWS>::init("x", "y");
        assert_eq!(Player::One, game.current);
        game = game.play_on_column(1);
        assert_eq!(Player::Two, game.current);
        game = game.play_on_column(1);
        assert_eq!(Player::One, game.current);
    }

    #[test]
    #[should_panic]
    fn cannot_play_on_a_column_outside_the_board() {
        let game = Game::<1, DEFAULT_ROWS>::init("x", "y");
        game.play_on_column(1);
    }

    #[test]
    #[should_panic]
    fn cannot_stack_a_column_beyond_the_row_size_of_the_board() {
        let mut game = Game::<2, 1>::init("x", "y");
        game = game.play_on_column(0);
        game.play_on_column(0);
    }

    #[test]
    fn draws_the_game_if_all_positions_have_been_played() {
        let mut game = Game::<1, 1>::init("x", "y");
        game = game.play_on_column(0);
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
        let mut game = Game::<4, 4>::init("x", "y");
        game = game.play_on_column(0);
        game = game.play_on_column(0);
        game = game.play_on_column(1);
        game = game.play_on_column(0);
        game = game.play_on_column(2);
        game = game.play_on_column(0);
        game = game.play_on_column(3);
        assert_eq!(game.status, GameStatus::Completed);
        assert!(game.winner.is_some());
        assert_eq!(game.winner.unwrap(), game.player_one_symbol);
    }

    /*
    x . . .
    x . . .
    x . . .
    x o o o
    */
    #[test]
    fn recognises_a_win_along_the_vertical() {
        let mut game = Game::<4, 4>::init("x", "y");
        game = game.play_on_column(0);
        game = game.play_on_column(1);
        game = game.play_on_column(0);
        game = game.play_on_column(2);
        game = game.play_on_column(0);
        game = game.play_on_column(3);
        game = game.play_on_column(0);
        assert_eq!(game.status, GameStatus::Completed);
        assert!(game.winner.is_some());
        assert_eq!(game.winner.unwrap(), game.player_one_symbol);
    }

    /*
    . . . x
    x x x o
    x x o o
    x o o o
    */
    #[test]
    fn recognises_a_win_along_the_positive_diagonal() {
        let mut game = Game::<4, 4>::init("x", "y");
        game = game.play_on_column(0);
        game = game.play_on_column(1);
        game = game.play_on_column(0);
        game = game.play_on_column(2);
        game = game.play_on_column(0);
        game = game.play_on_column(2);
        game = game.play_on_column(1);
        game = game.play_on_column(3);
        game = game.play_on_column(1);
        game = game.play_on_column(3);
        game = game.play_on_column(2);
        game = game.play_on_column(3);
        game = game.play_on_column(3);
        assert_eq!(game.status, GameStatus::Completed);
        assert!(game.winner.is_some());
        assert_eq!(game.winner.unwrap(), game.player_one_symbol);
    }

    /*
    x . . .
    o x x x
    o o x x
    o o o x
    */
    #[test]
    fn recognises_a_win_along_the_negative_diagonal() {
        let mut game = Game::<4, 4>::init("x", "y");
        game = game.play_on_column(3);
        game = game.play_on_column(2);
        game = game.play_on_column(3);
        game = game.play_on_column(1);
        game = game.play_on_column(3);
        game = game.play_on_column(1);
        game = game.play_on_column(2);
        game = game.play_on_column(0);
        game = game.play_on_column(2);
        game = game.play_on_column(0);
        game = game.play_on_column(1);
        game = game.play_on_column(0);
        game = game.play_on_column(0);
        assert_eq!(game.status, GameStatus::Completed);
        assert!(game.winner.is_some());
        assert_eq!(game.winner.unwrap(), game.player_one_symbol);
    }
}
