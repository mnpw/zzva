use std::{convert::Infallible, fmt::Display, io::Error};

use crate::{board::*, tile::Tile};

pub struct Game {
    size: usize,
    board: Board,
    win_tile: Tile,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
    }
}

#[derive(PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    Invalid,
}

impl Move {
    fn from(direction: &str) -> Self {
        let mv = match direction.to_ascii_lowercase().as_str() {
            "up" => Self::Up,
            "down" => Self::Down,
            "left" => Self::Left,
            "right" => Self::Right,
            _ => Self::Invalid,
        };

        mv
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            &Self::Up => "⬆",
            &Self::Down => "⬇",
            &Self::Left => "⬅",
            &Self::Right => "➡",
            _ => "Invalid move ❌",
        };

        write!(f, "{}", display)
    }
}

impl Game {
    pub fn init(size: usize, max_tile: usize) -> Result<Self, &'static str> {
        // game: inits with two tiles
        // two tiles are the smallest two tiles
        // the likelihood of smaller tile occuring is more
        // from a trial game it seems like the ratio is 5:1 for 2:4 tiles

        // game starts off with two tiles, then tile is spawned at every cycle

        let mut board = Board::new(size);
        board.spawn()?;
        board.spawn()?;

        Ok(Game {
            size,
            board,
            win_tile: Tile::new(max_tile),
        })
    }

    pub fn from(size: usize, max_tile: usize, state: &str) -> Self {
        let board = Board::from_state(size, state);

        Game {
            size,
            board,
            win_tile: Tile::new(max_tile),
        }
    }

    // pub fn run(&mut self)

    pub fn play(&mut self, direction: &str) {
        let mv = Move::from(direction);
        println!("You chose: {}", mv);

        // Play the move
        if let Err(e) = self.board.play(&mv) {
            eprintln!("[error] {}", e);
            return;
        }

        // Evaluate the game state
        if let Ok(state) = self.board.check(&self.win_tile) {
            let status = match state {
                GameState::Won => "You Won! Game finished. 🥳🥂",
                GameState::Lost => "You Lost! :(",
                GameState::InProgress => {
                    self.board.spawn();
                    return;
                }
            };
            println!("{}", status);
        }
    }

    pub fn spawn(&mut self) {
        self.board.check(&self.win_tile);
    }

    pub fn check(&mut self) -> Result<GameState, Infallible> {
        self.board.check(&self.win_tile)
    }
}
