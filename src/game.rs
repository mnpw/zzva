use crate::{board::*, tile::Tile};

pub struct Game {
    size: usize,
    pub board: Board,
    win_tile: Tile,
}

impl Game {
    pub fn init(size: usize, max_tile: usize) -> Self {
        // game: inits with two tiles
        // two tiles are the smallest two tiles
        // the likelihood of smaller tile occuring is more
        // from a trial game it seems like the ratio is 5:1 for 2:4 tiles

        // game starts off with two tiles, then tile is spawned at every cycle

        let mut board = Board::new(size);
        board.spawn();
        board.spawn();

        Game {
            size,
            board,
            win_tile: Tile::new(max_tile),
        }
    }

    pub fn from(size: usize, max_tile: usize, state: &str) -> Self {
        let board = Board::from_state(size, state);

        Game {
            size,
            board,
            win_tile: Tile::new(max_tile),
        }
    }

    pub fn play(&mut self, direction: &str) {
        println!("You chose: {}", direction);
        self.board.play(direction);
        self.board.check(&self.win_tile);
        self.board.spawn();
    }

    pub fn check(&mut self) {
        self.board.check(&self.win_tile);
    }
}
