pub struct Game {
    player_one: String,
    player_two: String,
    pub is_complete: bool,
    pub winner: Option<String>,
}

impl Game {
    pub fn init(player_one: &str, player_two: &str) -> Self {
        Game {
            player_one: player_one.to_owned(),
            player_two: player_two.to_owned(),
            is_complete: false,
            winner: None,
        }
    }

    pub fn play_on_column(self, column: usize) -> Self {
        self
    }
}
