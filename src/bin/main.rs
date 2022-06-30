use clap::Parser;
use zzva::{cli::Args, game::*, state::GameState};

fn main() {
    let args = Args::parse();

    let size = args.board;
    let max_tile = args.max;

    let mut game = Game::init(size.into(), max_tile.into()).unwrap();

    loop {
        println!("{}", game);
        println!("Enter input (up, down, left, right):");

        let mut play = String::new();
        std::io::stdin().read_line(&mut play).unwrap();
        let play = play.trim();

        match game.play(&play) {
            Ok(state) => {
                println!("{}", state.message);
                match state.game_state {
                    GameState::Won | GameState::Lost => break,
                    GameState::InProgress => continue,
                }
            }
            Err(message) => eprintln!("[ERROR] {}", message),
        }
        println!("\n\n");
    }
}
