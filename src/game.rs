const DEFAULT_COLUMNS: usize = 7;
pub struct Game {
    player_one: String,
    player_two: String,
    current: String,
    columns: usize,
    pub is_complete: bool,
    pub winner: Option<String>,
}

impl Game {
    pub fn init(player_one: &str, player_two: &str) -> Self {
        Game {
            player_one: player_one.to_owned(),
            player_two: player_two.to_owned(),
            current: player_one.to_owned(),
            is_complete: false,
            winner: None,
            columns: DEFAULT_COLUMNS,
        }
    }

    pub fn play_on_column(self, column: usize) -> Self {
        if column >= self.columns {
            panic!("Play out of bounds")
        }
        let new_player = match self.current == self.player_one {
            true => self.player_two.to_owned(),
            false => self.player_one.to_owned(),
        };
        Self {
            player_one: self.player_one,
            player_two: self.player_two,
            current: new_player,
            columns: self.columns,
            is_complete: false,
            winner: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn game_initialises_with_the_player_one_playing_first() {
        let game = Game::init("x", "y");
        assert_eq!(game.player_one, game.current)
    }

    #[test]
    fn player_one_and_player_two_take_turns() {
        let mut game = Game::init("x", "y");
        assert_eq!(game.player_one, game.current);
        game = game.play_on_column(1);
        assert_eq!(game.player_two, game.current);
        game = game.play_on_column(1);
        assert_eq!(game.player_one, game.current);
    }

    #[test]
    #[should_panic]
    fn cannot_play_on_a_column_outside_the_board() {
        let game = Game::init("x", "y");
        game.play_on_column(7);
    }
}
