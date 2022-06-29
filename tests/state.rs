use indoc::indoc;
use zzva::{board::*, game::*};

#[test]
fn in_progress() {
    let state = indoc! {"
        0,0,0,2
        0,0,0,0
        0,0,0,0
        0,0,0,0   
    "};

    let mut game = Game::from(4, 2048, state);
    game.board.check();

    assert_eq!(game.board.state, GameState::InProgress);
}

#[test]
fn lost() {
    let state = indoc! {"
        2,4,2,4
        4,2,4,2
        2,4,2,4
        4,2,4,2
    "};

    let mut game = Game::from(4, 2048, state);
    game.board.check();

    assert_eq!(game.board.state, GameState::Lost);
}

#[test]
fn won() {
    let state = indoc! {"
        2048,0,0,2
        0,0,0,0
        0,0,0,0
        0,0,0,0   
    "};

    let mut game = Game::from(4, 2048, state);
    game.board.check();

    assert_eq!(game.board.state, GameState::Won);
}

#[test]
fn next_move_win() {
    let state = indoc! {"
        1024,1024,0,0
        0,0,0,0
        0,0,0,0
        0,0,0,0   
    "};

    let mut game = Game::from(4, 2048, state);
    game.board.check();

    assert_eq!(game.board.state, GameState::InProgress);

    game.board.play("left");
    game.board.check();

    assert_eq!(game.board.state, GameState::Won);
}

#[test]
fn next_move_lose() {
    let state = indoc! {"
        2,4,2,4
        4,2,4,2
        2,4,2,4
        4,2,4,0
    "};

    let mut game = Game::from(4, 2048, state);
    game.board.check();

    assert_eq!(game.board.state, GameState::InProgress);

    game.board.play("left");
    game.board.spawn();
    println!("{}", game.board);
    game.board.check();

    assert_eq!(game.board.state, GameState::Lost);
}
