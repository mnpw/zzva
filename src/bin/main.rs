use zzva::game::*;

fn main() {
    const SIZE: usize = 4;
    const MAX_TILE: usize = 2048;

    let mut game = Game::init(SIZE, MAX_TILE);

    loop {
        println!("{}", game.board);
        println!("Enter input (up, down, left, right):");
        let mut play = String::new();
        std::io::stdin().read_line(&mut play).unwrap();
        let play = play.trim();
        game.play(&play);
        println!("\n\n");
    }
}
