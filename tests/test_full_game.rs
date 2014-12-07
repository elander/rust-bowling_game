extern crate bowling_game;
use bowling_game::Game;

#[test]
fn test_gutter_game() {
    let mut game = Game::new();
    roll_many(&mut game, 20i, 0i);

    assert_eq!(0, game.score());
}

#[test]
fn test_all_ones() {
    let mut game = Game::new();
    
    roll_many(&mut game, 20i, 1i);

    assert_eq!(20, game.score());
}

#[test]
fn test_one_spare() {
    let mut game = Game::new();

    roll_spare(&mut game);
    game.roll(3);
    roll_many(&mut game, 17i, 0i);

    assert_eq!(16i, game.score());
}

#[test]
fn test_one_strike() {
    let mut game = Game::new();

    roll_strike(&mut game);
    game.roll(3);
    game.roll(4);
    roll_many(&mut game, 16, 0);

    assert_eq!(24, game.score());
}

#[test]
fn test_perfect_game() {
    let mut game = Game::new();

    roll_many(&mut game, 12, 10);

    assert_eq!(300, game.score());
}

fn roll_many(game: &mut Game, n: int, pins: int) {
    for i in range(0i, n) {
        game.roll(pins);
    }
}

fn roll_spare(game: &mut Game) {
    game.roll(5);
    game.roll(5);
}

fn roll_strike(game: &mut Game) {
    game.roll(10);
}
