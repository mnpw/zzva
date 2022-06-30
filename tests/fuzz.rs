use rand::Rng;
use zzva::{game::Game, state::GameState};

#[test]
fn fuzz() {
    let moves_num = 10000;
    let moves = ["up", "down", "left", "right"];
    let sizes = [4, 8, 16];
    let winning_tiles = [4, 16, 128, 1024, 2048, 4096];

    for size in sizes {
        for win_tile in winning_tiles {
            let mut game = Game::new(size, win_tile).unwrap();

            let vals: Vec<usize> = (0..moves_num)
                .map(|_| rand::thread_rng().gen_range(0..moves.len()))
                .collect();

            let mut id = 0;

            loop {
                let play = moves[vals[id]];
                match game.play(&play) {
                    Ok(state) => match state.game_state {
                        GameState::Won | GameState::Lost => break,
                        GameState::InProgress => continue,
                    },
                    Err(_) => (),
                }
                id += 1;
                if id >= moves_num {
                    break;
                }
            }
        }
    }
}
