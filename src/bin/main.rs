use zzva::game::*;

fn main() {
    const SIZE: usize = 4;
    const MAX_TILE: usize = 16;

    let mut game = Game::init(SIZE, MAX_TILE).unwrap();

    loop {
        println!("{}", game);
        println!("Enter input (up, down, left, right):");

        let mut play = String::new();
        std::io::stdin().read_line(&mut play).unwrap();
        let play = play.trim();

        game.play(&play);
        println!("\n\n");
    }
}
