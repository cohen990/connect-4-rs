use crate::game::{Game, GameStatus, DEFAULT_COLUMNS, DEFAULT_ROWS};

#[test]
fn plays_full_game() {
    /* given a full game of connect 4
    the system should allow the players to input plays and should recognise a victory
    the system should award a point to the winning player
    */

    let mut game = Game::<DEFAULT_COLUMNS, DEFAULT_ROWS>::init("x", "o");
    game = game.play_on_column(1);
    game = game.play_on_column(2);
    game = game.play_on_column(1);
    game = game.play_on_column(3);
    game = game.play_on_column(1);
    game = game.play_on_column(4);
    game = game.play_on_column(1);

    assert_eq!(game.status, GameStatus::Completed);
    assert!(game.winner.is_some());
    assert_eq!("x", game.winner.unwrap());
}
