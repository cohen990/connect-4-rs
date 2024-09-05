pub struct Game {
    player_one: String,
    player_two: String,
    current: String,
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
        }
    }

    pub fn play_on_column(self, column: usize) -> Self {
        self
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
}
